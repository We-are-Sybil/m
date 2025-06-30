use crate::message_bus::Event;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;


/// Represents when a message is received from Whatsapp.
/// This is the primary event that triggers most business logic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReceived {
    pub message_id: String,
    pub from_phone: String,
    pub message_type: MessageType,
    pub content: MessageContent,
    pub received_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

impl Event for MessageReceived {
    const TOPIC: &'static str = "conversation.messages";
    const VERSION: &'static str = "1.0";
    /// Partitioning by `from_phone` allows us to group messages from
    /// the same sender together.
    fn partition_key(&self) -> Option<String> {
        Some(self.from_phone.clone())
    }
}

/// Represents when a user interacts with buttons or lists.
/// This way we can handle interactive responses in a structured way.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionReceived {
    pub original_message_id: String,
    pub from_phone: String,
    pub interaction_type: InteractionType,
    pub selection: InteractionSelection,
    pub received_at: chrono::DateTime<chrono::Utc>,
}

impl Event for InteractionReceived {
    const TOPIC: &'static str = "conversation.interactions";
    const VERSION: &'static str = "1.0";
    /// Partitioning by `from_phone` allows us to group interactions
    /// from the same sender together.
    fn partition_key(&self) -> Option<String> {
        Some(self.from_phone.clone())
    }
}

/// Represents when the AI or any service has something to say
/// back to the user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseReady {
    /// ID of the original message this is responding to
    pub original_message_id: String,

    pub to_phone: String,
    pub response_type: ResponseType,
    pub content: ResponseContent,
    pub generated_at: chrono::DateTime<chrono::Utc>,

    /// Priority level (normal, urgent, low)
    pub priority: ResponsePriority,
}

impl Event for ResponseReady {
    const TOPIC: &'static str = "conversation.responses";
    const VERSION: &'static str = "1.0";
    /// Partitioning by `to_phone` allows us to group responses
    /// to the same recipient together.
    fn partition_key(&self) -> Option<String> {
        Some(self.to_phone.clone())
    }
    
}

/// Represents when a message fails to process after all retries.
/// TODO: This should trigger human intervention or alerting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFailed {
    pub message_id: String,
    pub phone: String,
    pub failure_type: FailureType,
    pub error_details: String,
    pub attempt_count: u32,
    pub failed_at: chrono::DateTime<chrono::Utc>,
}

impl Event for MessageFailed {
    const TOPIC: &'static str = "conversation.messages.failed";
    const VERSION: &'static str = "1.0";
    /// Partitioning by `phone` allows us to group failures
    /// for the same recipient together.
    fn partition_key(&self) -> Option<String> {
        Some(self.phone.clone())
    }
}

// ====> Supporting types for the events <=====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    Audio,
    Document,
    Video,
    Location,
    Contact,
    Sticker,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text {
        body: String,
    },
    Media {
        media_id: String,
        caption: Option<String>,
        mime_type: String,
    },
    Location {
        latitude: f64,
        longitude: f64,
        name: Option<String>,
        address: Option<String>,
    },
    Contact {
        name: String,
        phone_number: String,
        email: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    ButtonReply,
    ListReply,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionSelection {
    Button {
        id: String,
        title: String,
    },
    List {
        id: String,
        title: String,
        description: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseType {
    Text,
    Interactive,
    Media,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseContent {
    Text {
        message: String,
    },
    Interactive {
        body_text: String,
        buttons: Vec<ResponseButton>,
    },
    List {
        body_text: String,
        button_text: String,
        sections: Vec<ResponseSection>,
    },
    Media {
        media_id: String,
        caption: Option<String>,
    },
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseButton {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseSection {
    pub title: String,
    pub rows: Vec<ResponseRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRow {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponsePriority {
    Low,
    Normal,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FailureType {
    SerializationError,
    ProcessingTimeout,
    ExternalServiceError,
    ValidationError,
    UnknownError,
}
