use crate::{
    errors::WhatsAppResult,
    client::message_types::TextMessage,
};

/// Builder for creating text messages with fluent interface
/// 
/// This builder provides a discoverable way to create text messages
/// with optional features. It guides developers through the available
/// options and validates the configuration before building.
/// 
/// # Example
/// ```
/// # use whatsapp_client::client::builders::TextMessageBuilder;
/// let message = TextMessageBuilder::new()
///     .to("+1234567890")
///     .message("Check out our website: https://example.com")
///     .with_preview()
///     .build()?;
/// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
/// ```
#[derive(Debug, Default)]
pub struct TextMessageBuilder {
    to: Option<String>,
    message: Option<String>,
    preview_enabled: Option<bool>,
}

impl TextMessageBuilder {
    /// Create a new text message builder
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the recipient phone number
    /// 
    /// # Arguments
    /// * `phone` - Phone number in E.164 format (+1234567890)
    pub fn to(mut self, phone: &str) -> Self {
        self.to = Some(phone.to_string());
        self
    }
    
    /// Set the message text
    /// 
    /// # Arguments
    /// * `text` - Message content (up to 4096 characters)
    pub fn message(mut self, text: &str) -> Self {
        self.message = Some(text.to_string());
        self
    }
    
    /// Enable link preview for URLs in the message
    /// 
    /// When enabled, WhatsApp will attempt to generate a preview
    /// for the first URL found in the message text.
    pub fn with_preview(mut self) -> Self {
        self.preview_enabled = Some(true);
        self
    }
    
    /// Explicitly disable link preview
    /// 
    /// This ensures no link previews are shown even if URLs are present.
    pub fn without_preview(mut self) -> Self {
        self.preview_enabled = Some(false);
        self
    }
    
    /// Build the text message
    /// 
    /// This validates all the configuration and creates the final TextMessage.
    /// Returns an error if required fields are missing or invalid.
    pub fn build(self) -> WhatsAppResult<TextMessage> {
        let to = self.to.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Recipient phone number is required".to_string()
            )
        })?;
        
        let message = self.message.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Message text is required".to_string()
            )
        })?;
        
        // Create the message using the appropriate method based on preview setting
        match self.preview_enabled {
            Some(true) => TextMessage::with_preview(&to, &message),
            Some(false) => TextMessage::without_preview(&to, &message),
            None => TextMessage::new(&to, &message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_text_message_builder() {
        let message = TextMessageBuilder::new()
            .to("+1234567890")
            .message("Hello, world!")
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.message(), "Hello, world!");
        assert_eq!(message.has_preview_enabled(), None);
    }
    
    #[test]
    fn test_text_message_with_preview() {
        let message = TextMessageBuilder::new()
            .to("+1234567890")
            .message("Visit: https://example.com")
            .with_preview()
            .build()
            .unwrap();
        
        assert_eq!(message.has_preview_enabled(), Some(true));
    }
    
    #[test]
    fn test_text_message_without_preview() {
        let message = TextMessageBuilder::new()
            .to("+1234567890")
            .message("No preview: https://example.com")
            .without_preview()
            .build()
            .unwrap();
        
        assert_eq!(message.has_preview_enabled(), Some(false));
    }
    
    #[test]
    fn test_missing_recipient() {
        let result = TextMessageBuilder::new()
            .message("Hello")
            .build();
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_missing_message() {
        let result = TextMessageBuilder::new()
            .to("+1234567890")
            .build();
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_fluent_interface() {
        // Test that we can chain methods in any order
        let message1 = TextMessageBuilder::new()
            .message("Hello")
            .to("+1234567890")
            .with_preview()
            .build()
            .unwrap();
        
        let message2 = TextMessageBuilder::new()
            .with_preview()
            .to("+1234567890")
            .message("Hello")
            .build()
            .unwrap();
        
        // Both should produce equivalent results
        assert_eq!(message1.recipient(), message2.recipient());
        assert_eq!(message1.message(), message2.message());
        assert_eq!(message1.has_preview_enabled(), message2.has_preview_enabled());
    }
    
    #[test]
    fn test_builder_json_format_matches_api() {
        let message = TextMessageBuilder::new()
            .to("+16505551234")
            .message("Hello, world!")
            .build()
            .unwrap();
        
        let json_output = serde_json::to_string(&message).unwrap();
        let expected_json = r#"{"messaging_product":"whatsapp","recipient_type":"individual","to":"+16505551234","type":"text","text":{"body":"Hello, world!"}}"#;
        
        assert_eq!(json_output, expected_json);
    }
    
    #[test]
    fn test_builder_with_preview_json_format() {
        let message = TextMessageBuilder::new()
            .to("+16505551234")
            .message("Check out: https://example.com")
            .with_preview()
            .build()
            .unwrap();
        
        let json_output = serde_json::to_string(&message).unwrap();
        let expected_json = r#"{"messaging_product":"whatsapp","recipient_type":"individual","to":"+16505551234","type":"text","text":{"body":"Check out: https://example.com","preview_url":true}}"#;
        
        assert_eq!(json_output, expected_json);
    }
    
    #[test]
    fn test_builder_without_preview_json_format() {
        let message = TextMessageBuilder::new()
            .to("+16505551234")
            .message("No preview")
            .without_preview()
            .build()
            .unwrap();
        
        let json_output = serde_json::to_string(&message).unwrap();
        let expected_json = r#"{"messaging_product":"whatsapp","recipient_type":"individual","to":"+16505551234","type":"text","text":{"body":"No preview","preview_url":false}}"#;
        
        assert_eq!(json_output, expected_json);
    }
    
    #[test]
    fn test_builder_and_direct_create_same_json() {
        let builder_message = TextMessageBuilder::new()
            .to("+16505551234")
            .message("Test message")
            .with_preview()
            .build()
            .unwrap();
        
        let direct_message = TextMessage::with_preview("+16505551234", "Test message").unwrap();
        
        let builder_json = serde_json::to_string(&builder_message).unwrap();
        let direct_json = serde_json::to_string(&direct_message).unwrap();
        
        assert_eq!(builder_json, direct_json);
    }
}
