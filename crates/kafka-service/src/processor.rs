use crate::config::KafkaConfig;
use common::{
    AIResponse,
    AIResponseContent,
    InteractiveButton,
    MessageContent,
    ProcessingError,
    WebhookEvent,
};
use tracing::{info, debug};
use std::collections::HashMap;

/// Handles business logic for processing webhook events
///
/// This is where one can implement the transformation logic between
/// webhook events and the different services that consume them.
pub struct MessageProcessor {
    config: KafkaConfig,
}


impl MessageProcessor {
    pub fn new(config: KafkaConfig) -> Self {
        info!("🔧 Initializing MessageProcessor with config: {:?}", config);
        Self { config }
    }

    /// Process a webhook event into an AI request
    ///
    /// This is a placeholder implementation. That should be replaced with
    /// the actual logic that transforms the incoming webhook event
    /// into a request for the AI service.
    pub async fn process_webhook_event(
        &self,
        event: WebhookEvent,
    ) -> Result<AIResponse, ProcessingError> {
        debug!("🔍 Processing webhook event: {:?}", event);
        
        // Simulate some processing logic based on message type
        let response_content = match &event.content {
            MessageContent::Text { body } => {
                debug!("📄 Processing text content: {}", body);
                self.process_text_message(body, &event).await
            }
            _ => {
                info!("🔄 Processing generic message type: {:?}", event.content);
                AIResponseContent::Text {
                    message: "I received your message and I'm processing it.".to_string(),
                }
            }
        };

        Ok(AIResponse {
            original_event_id: event.event_id,
            processed_at: chrono::Utc::now(),
            response: response_content,
            ai_metadata: HashMap::new(),
        })
    }

    /// Place holder.
    async fn process_text_message(&self, body: &str, _event: &WebhookEvent) -> AIResponseContent {
        if body.to_lowercase().contains("help") {
            AIResponseContent::Interactive {
                body_text: "How can I assist you?".to_string(),
                buttons: vec![
                    InteractiveButton {
                        id: "support".to_string(),
                        title: "Support".to_string(),
                    },
                    InteractiveButton {
                        id: "faq".to_string(),
                        title: "FAQ".to_string(),
                    },
                ],
            }
        } else {
            AIResponseContent::Text {
                message: format!("You said: {}", body),
            }
        }
    }
}



