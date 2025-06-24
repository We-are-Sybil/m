use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use crate::{
    state::AppState,
    types::{WebhookVerifyQuery, WebhookPayload, WhatsAppMessage, TextBody, MessageContext, MessageStatus},
};
use tracing::{error, info, warn};
use reqwest::Client;

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

