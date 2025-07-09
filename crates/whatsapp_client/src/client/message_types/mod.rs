pub mod mtrait;

pub mod text;
pub mod audio;
pub mod contacts;
pub mod document;
pub mod image;
pub mod interactive;
pub mod location;
pub mod video;

pub use mtrait::Message;
pub use text::TextMessage;
pub use audio::AudioMessage;
pub use contacts::ContactMessage;
pub use document::DocumentMessage;
pub use image::ImageMessage;
pub use interactive::InteractiveMessage;
pub use location::LocationMessage;
pub use video::VideoMessage;

use serde::{Deserialize, Serialize};
use common::message_bus::Event;

/// A response message to be sent via WhatsApp
/// 
/// This struct represents a complete WhatsApp message response that includes
/// the original message context, recipient information, message content, and
/// metadata for proper message routing and delivery prioritization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsAppMessageSend {
    /// ID of the original message that this response is related to
    pub original_message_id: String,
    /// The WhatsApp message to be sent as a response
    pub message: WhatsAppMessage,
    /// Date and time when the message was generated (UTC)
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Priority level for message delivery (Low, Normal, Urgent)
    pub priority: ResponsePriority,
}

impl Event for WhatsAppMessageSend {
    const TOPIC: &'static str = "conversation.responses";
    const VERSION: &'static str = "1.0";
    /// Partitioning by `to_phone` allows us to group responses
    /// to the same recipient together.
    fn partition_key(&self) -> Option<String> {
        let to_phone = match &self.message {
            WhatsAppMessage::Text(msg) => msg.recipient(),
            WhatsAppMessage::Audio(msg) => msg.recipient(),
            WhatsAppMessage::Contact(msg) => msg.recipient(),
            WhatsAppMessage::Document(msg) => msg.recipient(),
            WhatsAppMessage::Image(msg) => msg.recipient(),
            WhatsAppMessage::Interactive(msg) => msg.recipient(),
            WhatsAppMessage::Location(msg) => msg.recipient(),
            WhatsAppMessage::Video(msg) => msg.recipient(),
        };
        Some(to_phone.to_string())
    }
}

impl WhatsAppMessageSend {
    /// Create a new WhatsApp message response
    /// 
    /// The recipient phone number is automatically extracted from the message.
    /// 
    /// # Arguments
    /// * `original_message_id` - ID of the original message this response relates to
    /// * `message` - The WhatsApp message content to send
    /// * `priority` - Delivery priority level
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::*;
    /// let text_msg = TextMessage::new("+1234567890", "Hello!")?;
    /// let response = WhatsAppMessageResponse::new(
    ///     "msg_12345".to_string(),
    ///     WhatsAppMessage::Text(text_msg),
    ///     ResponsePriority::Normal
    /// );
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn new(
        original_message_id: String,
        message: WhatsAppMessage,
        priority: ResponsePriority,
    ) -> Self {
 
        Self { 
            original_message_id,
            message, 
            generated_at: chrono::Utc::now(),
            priority
        }
     }
 }
 
 
 /// Union type for all supported WhatsApp message types
 /// 
 /// This enum represents all the different message types that can be sent
 /// through the WhatsApp Business API. Each variant contains the specific
 /// message data and formatting for that message type.
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub enum WhatsAppMessage {
     /// Plain text message with optional link preview
     Text(TextMessage),
     /// Audio message with media upload or URL
     Audio(AudioMessage),
     /// Contact information sharing
     Contact(ContactMessage),
     /// Document file sharing with optional caption
     Document(DocumentMessage),
     /// Image message with optional caption
     Image(ImageMessage),
     /// Interactive message with buttons or lists
     Interactive(InteractiveMessage),
     /// Location sharing with coordinates and address
     Location(LocationMessage),
     /// Video message with optional caption
     Video(VideoMessage),
 }
 
 /// Priority level for message delivery
 /// 
 /// This enum defines the urgency level for message responses, which can
 /// be used by the message processing system to prioritize delivery.
 #[derive(Debug, Clone, Serialize, Deserialize)]
 pub enum ResponsePriority {
     /// Low priority - can be delayed for batch processing
     Low,
     /// Normal priority - standard delivery timing
     Normal,
     /// Urgent priority - should be processed immediately
     Urgent,
 }
