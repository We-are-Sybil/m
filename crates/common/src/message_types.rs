use serde::{Deserialize, Serialize};

/// Simplified message type enum for Kafka pipeline
/// Maps from WebhookMessageType for event processing
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MessageType {
    Text,
    Image,
    Audio,
    Document,
    Video,
    Location,
    Contact,
    Interactive,
    Reaction,
    Sticker,
    Referral,
    Error,
}

/// Simplified message structure for Kafka events
/// Transforms complex webhook messages into streamlined formats
/// (useful for AI processing and event handling)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MessageContent {
    Text { 
        body: String 
    },
    Image { 
        media_id: Option<String>, 
        caption: Option<String>,
        mime_type: Option<String>,
        sha256: Option<String>,
    },
    Audio { 
        media_id: Option<String>,
        mime_type: Option<String>,
        sha256: Option<String>,
    },
    Document { 
        media_id: Option<String>, 
        filename: Option<String>,
        caption: Option<String>,
        mime_type: Option<String>,
        sha256: Option<String>,
    },
    Video {
        media_id: Option<String>,
        caption: Option<String>,
        mime_type: Option<String>,
        sha256: Option<String>,
    },
    Location { 
        latitude: f64, 
        longitude: f64, 
        name: Option<String>,
        address: Option<String>,
    },
    Contact { 
        name: String,
        phones: Vec<String>,
        emails: Vec<String>,
    },
    Interactive { 
        interaction_type: String,
        button_reply: Option<super::webhook_types::ButtonReply>,
        list_reply: Option<super::webhook_types::ListReply>,
    },
    Reaction {
        message_id: String,
        emoji: String,
    },
    Sticker {
        media_id: Option<String>,
        mime_type: Option<String>,
        sha256: Option<String>,
    },
    Referral {
        source_url: String,
        source_type: String,
        headline: Option<String>,
        body: Option<String>,
    },
    Unknown { 
        raw_data: String 
    },
}
