use crate::{
    errors::WhatsAppResult,
    client::{
        validation::{validate_phone_number, validate_text_message},
        message_types::mtrait::Message,
    },
};
use serde::{Serialize, Deserialize};

/// A text message that can be sent via WhatsApp
/// 
/// This represents a simple text message with optional link preview functionality.
/// Text messages are the most basic message type and form the foundation for
/// most conversational interactions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient type - always "individual" for direct messages
    recipient_type: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Text content and settings
    text: TextContent,
}

impl Message for TextMessage {
    /// Get the recipient phone number
    fn recipient(&self) -> &str {
        &self.to
    }

    /// Get the message type identifier
    fn message_type(&self) -> &str {
        "text"
    }
    
}

/// Text message content structure
/// 
/// This contains the actual message text and optional settings for link previews.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct TextContent {
    /// The message text (up to 4096 characters)
    body: String,
    /// Whether to show link previews for URLs in the message
    #[serde(skip_serializing_if = "Option::is_none")]
    preview_url: Option<bool>,
}

impl TextMessage {
    /// Create a new text message
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format (+1234567890)
    /// * `message` - Text content (up to 4096 characters)
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::TextMessage;
    /// let message = TextMessage::new("+1234567890", "Hello, world!")?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn new(to: &str, message: &str) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_text_message(message)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "text".to_string(),
            text: TextContent {
                body: message.to_string(),
                preview_url: None,
            },
        })
    }
    
    /// Create a new text message with link preview enabled
    /// 
    /// When link preview is enabled, WhatsApp will attempt to generate
    /// a preview for the first URL found in the message text.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::TextMessage;
    /// let message = TextMessage::with_preview(
    ///     "+1234567890", 
    ///     "Check out our website: https://example.com"
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_preview(to: &str, message: &str) -> WhatsAppResult<Self> {
        let mut text_message = Self::new(to, message)?;
        text_message.text.preview_url = Some(true);
        Ok(text_message)
    }
    
    /// Create a new text message with link preview explicitly disabled
    /// 
    /// This ensures that no link previews are shown even if URLs are present.
    pub fn without_preview(to: &str, message: &str) -> WhatsAppResult<Self> {
        let mut text_message = Self::new(to, message)?;
        text_message.text.preview_url = Some(false);
        Ok(text_message)
    }
    
    /// Get the message text
    pub fn message(&self) -> &str {
        &self.text.body
    }
    
    /// Check if link preview is enabled
    pub fn has_preview_enabled(&self) -> Option<bool> {
        self.text.preview_url
    }
    
    /// Get the length of the message text
    pub fn message_length(&self) -> usize {
        self.text.body.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_basic_text_message_creation() {
        let message = TextMessage::new("+1234567890", "Hello, world!").unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.message(), "Hello, world!");
        assert_eq!(message.has_preview_enabled(), None);
        assert_eq!(message.message_length(), 13);
    }
    
    #[test]
    fn test_text_message_with_preview() {
        let message = TextMessage::with_preview(
            "+1234567890", 
            "Check this out: https://example.com"
        ).unwrap();
        
        assert_eq!(message.has_preview_enabled(), Some(true));
    }
    
    #[test]
    fn test_text_message_without_preview() {
        let message = TextMessage::without_preview(
            "+1234567890", 
            "No preview: https://example.com"
        ).unwrap();
        
        assert_eq!(message.has_preview_enabled(), Some(false));
    }
    
    #[test]
    fn test_text_message_serialization() {
        let message = TextMessage::with_preview(
            "+1234567890", 
            "Hello with link: https://example.com"
        ).unwrap();
        
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["recipient_type"], "individual");
        assert_eq!(json["to"], "+1234567890");
        assert_eq!(json["type"], "text");
        assert_eq!(json["text"]["body"], "Hello with link: https://example.com");
        assert_eq!(json["text"]["preview_url"], true);
    }
    
    #[test]
    fn test_invalid_phone_number() {
        let result = TextMessage::new("invalid", "Hello");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_empty_message() {
        let result = TextMessage::new("+1234567890", "");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_message_too_long() {
        let long_message = "x".repeat(4097);
        let result = TextMessage::new("+1234567890", &long_message);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_max_length_message() {
        let max_message = "x".repeat(4096);
        let result = TextMessage::new("+1234567890", &max_message);
        assert!(result.is_ok());
        
        let message = result.unwrap();
        assert_eq!(message.message_length(), 4096);
    }

    #[test]
    fn test_text_message_json_format_matches_api() {
        let message = TextMessage::new("+16505551234", "Hello, world!").unwrap();
        let json_output = serde_json::to_string(&message).unwrap();
        
        let expected_json = r#"{"messaging_product":"whatsapp","recipient_type":"individual","to":"+16505551234","type":"text","text":{"body":"Hello, world!"}}"#;
        
        assert_eq!(json_output, expected_json);
    }
    
    #[test]
    fn test_text_message_with_preview_json_format() {
        let message = TextMessage::with_preview("+16505551234", "Check out: https://example.com").unwrap();
        let json_output = serde_json::to_string(&message).unwrap();
        
        let expected_json = r#"{"messaging_product":"whatsapp","recipient_type":"individual","to":"+16505551234","type":"text","text":{"body":"Check out: https://example.com","preview_url":true}}"#;
        
        assert_eq!(json_output, expected_json);
    }
    
    #[test]
    fn test_text_message_without_preview_json_format() {
        let message = TextMessage::without_preview("+16505551234", "No preview").unwrap();
        let json_output = serde_json::to_string(&message).unwrap();
        
        let expected_json = r#"{"messaging_product":"whatsapp","recipient_type":"individual","to":"+16505551234","type":"text","text":{"body":"No preview","preview_url":false}}"#;
        
        assert_eq!(json_output, expected_json);
    }
}
