use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use crate::{
    state::AppState,
    types::{WebhookVerifyQuery, WebhookPayload},
};

use common::WebhookMessageType;

use tracing::{error, info, warn};

pub async fn verify_webhook(
    Query(query): Query<WebhookVerifyQuery>,
    State(state): State<AppState>,
) -> Result<String, StatusCode> {
    match (query.mode.as_deref(), &query.verify_token, &query.challenge) {
        (Some("subscribe"), Some(token), Some(challenge)) => {
            if token == &state.config.verify_token {
                info!("Webhook verification successful");
                Ok(challenge.clone())
            } else {
                warn!("Invalid verify token: {}", token);
                Err(StatusCode::FORBIDDEN)
            }
        }
        _ => {
            error!("Invalid query parameters: {:?}", query);
            Err(StatusCode::BAD_REQUEST)
        }
    }
}

pub async fn handle_webhook(
    State(_): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> Result<StatusCode, StatusCode> {
    info!("Received webhook payload: {:?}", payload);

    for entry in payload.entry {
        for change in entry.changes {
            if let Some(messages) = change.value.messages {
                for message in messages {
                    match message.get_message_type() {
                        Some(message_type) => {
                            match message_type {
                                WebhookMessageType::Text(text_msg) => {
                                    handle_text_message(&message.from, &text_msg).await;
                                }
                                WebhookMessageType::Reaction(reaction_msg) => {
                                    handle_reaction_message(&message.from, &reaction_msg).await;
                                }
                                WebhookMessageType::Image(media_msg) => {
                                    handle_image_message(&message.from, &media_msg).await;
                                }
                                WebhookMessageType::Sticker(media_msg) => {
                                    handle_sticker_message(&message.from, &media_msg).await;
                                }
                                WebhookMessageType::Location(location_msg) => {
                                    handle_location_message(&message.from, &location_msg).await;
                                }
                                WebhookMessageType::Contact(contact_msgs) => {
                                    handle_contact_message(&message.from, &contact_msgs).await;
                                }
                                WebhookMessageType::Interactive(interactive_msg) => {
                                    handle_interactive_message(&message.from, &interactive_msg).await;
                                }
                                WebhookMessageType::Referral(referral_msg) => {
                                    handle_referral_message(&message.from, &referral_msg).await;
                                }
                                WebhookMessageType::Unknown(errors) => {
                                    handle_unknown_message(&message.from, &errors).await;
                                }
                            }
                        }
                        None => {
                            warn!("Could not determine message type for message: {:?}", message);
                        }
                    }
                }
            }
        }
    }

    Ok(StatusCode::OK)
}

async fn handle_text_message(from: &str, text_msg: &common::TextMessage) {
    info!("Handling text message from {}: {}", from, text_msg.body);
}

async fn handle_reaction_message(from: &str, reaction_msg: &common::ReactionMessage) {
    info!("Handling reaction message from {}: {} to message {}", from, reaction_msg.emoji, reaction_msg.message_id);
}

async fn handle_image_message(from: &str, media_msg: &common::MediaMessage) {
    info!("Handling image message from {}: {:?}", from, media_msg.id);
}

async fn handle_sticker_message(from: &str, media_msg: &common::MediaMessage) {
    info!("Handling sticker message from {}: {:?}", from, media_msg.id);
}

async fn handle_location_message(from: &str, location_msg: &common::LocationMessage) {
    info!("Handling location message from {}: {}, {}", from, location_msg.latitude, location_msg.longitude);
}

async fn handle_contact_message(from: &str, contact_msgs: &[common::ContactMessage]) {
    info!("Handling contact message from {} with {} contacts", from, contact_msgs.len());
}

async fn handle_interactive_message(from: &str, interactive_msg: &common::InteractiveMessage) {
    info!("Handling interactive message from {}: type {}", from, interactive_msg.interactive_type);
}

async fn handle_referral_message(from: &str, referral_msg: &common::ReferralMessage) {
    info!("Handling referral message from {}: {}", from, referral_msg.source_url);
}

async fn handle_unknown_message(from: &str, errors: &[common::MessageError]) {
    if errors.is_empty() {
        warn!("Handling unknown message type from {}", from);
    } else {
        error!("Handling message with errors from {}: {:?}", from, errors);
    }
}

