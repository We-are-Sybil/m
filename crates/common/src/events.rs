use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::message_types::{MessageType, MessageContent};

/// Event sent through Kafka pipeline
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WebhookEvent {
    pub event_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub from_phone: String,
    pub message_type: MessageType,
    pub content: MessageContent,
    pub metadata: HashMap<String, String>,
}

/// AI service response
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AIResponse {
    pub original_event_id: String,
    pub processed_at: chrono::DateTime<chrono::Utc>,
    pub response: AIResponseContent,
    pub ai_metadata: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum AIResponseContent {
    Text { 
        message: String 
    },
    Interactive {
        body_text: String,
        buttons: Vec<InteractiveButton>,
    },
    NoResponse,
    Error { 
        error_message: String,
        should_retry: bool 
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveButton {
    pub id: String,
    pub title: String,
}

impl WebhookEvent {
    pub fn new(
        from_phone: String,
        message_type: MessageType,
        content: MessageContent,
    ) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            from_phone,
            message_type,
            content,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}
