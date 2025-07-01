use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use crate::{
    state::AppState,
    types::{WebhookVerifyQuery, WebhookPayload},
    event_publisher::WebhookEventPublisher,
};

use tracing::{error, info, warn};

/// Verify webhook subscription requests from WhatsApp
///
/// WhatsApp sends a GET request with specific query parameters to verify
/// that the webhook endpoint is valid and owned by the user.
pub async fn verify_webhook(
    Query(query): Query<WebhookVerifyQuery>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    match (query.mode.as_deref(), &query.verify_token, &query.challenge) {
        (Some("subscribe"), Some(token), Some(challenge)) => {
            if token == &state.config.verify_token {
                info!("✅ Webhook verification successful");
                Ok(challenge.clone())
            } else {
                warn!("❌ Invalid verify token: {}", token);
                Err(StatusCode::FORBIDDEN)
            }
        }
        _ => {
            error!("❌ Invalid webhook verification request: {:?}", query);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

/// Handle incoming WhatsApp webhook messages and transform them into domain events
///
/// THis is the main webhook endpoint that receives all WhatsApp messages, 
/// interactions,and status updates. It processes each message and publishes
/// appropriate domain events to Kafka for downstream services to consume.
pub async fn handle_webhook(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> Result<StatusCode, StatusCode> {
    info!("📨 Received webhook payload with {} entries", payload.entry.len());

    let event_publisher = WebhookEventPublisher::new(state.event_bus.clone());

    // Only process message changes (ignore status changes, etc.)
    for entry in payload.entry {
        info!("🔄 Processing entry {} with {} changes", entry.id, entry.changes.len());

        for change in entry.changes {
            if change.field != "messages" {
                warn!("⚠️ Unsupported field in change: {}", change.field);
                continue;
            }

            if let Some(messages) = change.value.messages {
                for message in messages {
                    // Extract content message ID if present 
                    // (for replies/interactions)
                    let context_message_id = message.context
                        .as_ref()
                        .and_then(|ctx| ctx.id.clone());

                    let webhook_message_type = message.get_message_type();

                    // Publish message as a domain event
                    match event_publisher.process_message(
                        message.id.clone(),
                        message.from.clone(),
                        message.timestamp.clone(),
                        webhook_message_type, 
                        context_message_id,
                    ).await {
                        Ok(()) => {
                            info!("✅ Successfully processed message {} from {}", 
                                  message.id, message.from);
                        }
                        Err(e) => {
                            error!("❌ Failed to process message {} from {}: {}", 
                                   message.id, message.from, e);
                            
                            // Continue processing other messages even if one fails
                            // The event publisher handles retries and dead letter queues
                        }
                    }
                }
            }
        }
    }

    // Always return 200 OK to WhatsApp to acknowledge receipt
    // Even if some message processing failed, we don't want WhatsApp 
    // to retry the entire webhook payload since failures are handled 
    // by our retry mechanisms
    Ok(StatusCode::OK)
}
