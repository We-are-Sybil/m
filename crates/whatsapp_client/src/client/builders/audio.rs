use crate::{
    errors::WhatsAppResult,
    client::message_types::AudioMessage,
};

/// Builder for creating audio messages with fluent interface
/// 
/// This builder provides a discoverable way to create audio messages
/// with either uploaded media IDs (recommended) or hosted URLs.
/// It guides developers through the available options and validates
/// the configuration before building.
/// 
/// Audio messages don't support captions, making this builder simpler
/// than image/video/document builders.
/// 
/// # Example
/// ```
/// # use whatsapp_client::client::builders::AudioMessageBuilder;
/// // Using uploaded media (recommended)
/// let message = AudioMessageBuilder::new()
///     .to("+1234567890")
///     .media_id("1013859600285441")
///     .build()?;
/// 
/// // Using hosted URL (not recommended)
/// let message = AudioMessageBuilder::new()
///     .to("+1234567890")
///     .media_url("https://example.com/audio.mp3")
///     .build()?;
/// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
/// ```
#[derive(Debug, Default)]
pub struct AudioMessageBuilder {
    to: Option<String>,
    media_id: Option<String>,
    media_url: Option<String>,
}

impl AudioMessageBuilder {
    /// Create a new audio message builder
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
    
    /// Set the media ID for uploaded audio (recommended approach)
    /// 
    /// Use this when you've already uploaded the audio file to WhatsApp's
    /// media servers. This approach offers better performance and reliability.
    /// 
    /// # Arguments
    /// * `id` - Media ID returned from WhatsApp's media upload API
    /// 
    /// # Note
    /// Cannot be used together with `media_url()`. If both are set,
    /// `media_id` takes precedence.
    pub fn media_id(mut self, id: &str) -> Self {
        self.media_id = Some(id.to_string());
        // Clear URL if previously set - ID takes precedence
        self.media_url = None;
        self
    }
    
    /// Set the URL for hosted audio (not recommended)
    /// 
    /// Use this when your audio is hosted on your own servers.
    /// WhatsApp will download the file, which adds latency and
    /// potential failure points.
    /// 
    /// # Arguments
    /// * `url` - HTTPS URL to the audio file
    /// 
    /// # Note
    /// Cannot be used together with `media_id()`. If both are set,
    /// `media_id` takes precedence.
    pub fn media_url(mut self, url: &str) -> Self {
        // Only set URL if no media ID is already set
        if self.media_id.is_none() {
            self.media_url = Some(url.to_string());
        }
        self
    }
    
    /// Build the audio message
    /// 
    /// This validates all the configuration and creates the final AudioMessage.
    /// Returns an error if required fields are missing or invalid.
    /// 
    /// # Validation
    /// - Recipient phone number must be set and valid
    /// - Either media_id OR media_url must be set (but not both)
    /// - All WhatsApp validation rules are applied
    pub fn build(self) -> WhatsAppResult<AudioMessage> {
        let to = self.to.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Recipient phone number is required".to_string()
            )
        })?;
        
        // Determine which creation method to use
        match (self.media_id, self.media_url) {
            (Some(id), _) => {
                // Media ID takes precedence (recommended approach)
                AudioMessage::from_media_id(&to, &id)
            },
            (None, Some(url)) => {
                // Fall back to URL
                AudioMessage::from_url(&to, &url)
            },
            (None, None) => {
                Err(crate::errors::WhatsAppError::InvalidMessageContent(
                    "Either media_id or media_url must be provided".to_string()
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_message_with_media_id() {
        let message = AudioMessageBuilder::new()
            .to("+1234567890")
            .media_id("1013859600285441")
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_audio_message_with_media_url() {
        let message = AudioMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/audio.mp3")
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), None);
        assert_eq!(message.media_url(), Some("https://example.com/audio.mp3"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_media_id_takes_precedence() {
        // If both are set, media_id should take precedence
        let message = AudioMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/audio.mp3")
            .media_id("1013859600285441") // This should override the URL
            .build()
            .unwrap();
        
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_url_ignored_when_id_set_first() {
        // Setting URL after ID should not override ID
        let message = AudioMessageBuilder::new()
            .to("+1234567890")
            .media_id("1013859600285441")
            .media_url("https://example.com/audio.mp3") // This should be ignored
            .build()
            .unwrap();
        
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
    }
    
    #[test]
    fn test_missing_recipient() {
        let result = AudioMessageBuilder::new()
            .media_id("123456")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Recipient phone number is required"));
    }
    
    #[test]
    fn test_missing_media() {
        let result = AudioMessageBuilder::new()
            .to("+1234567890")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Either media_id or media_url must be provided"));
    }
    
    #[test]
    fn test_fluent_interface_different_orders() {
        // Test that methods can be called in any order
        let message1 = AudioMessageBuilder::new()
            .media_id("123456")
            .to("+1234567890")
            .build()
            .unwrap();
        
        let message2 = AudioMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .build()
            .unwrap();
        
        // Both should produce equivalent results
        assert_eq!(message1.recipient(), message2.recipient());
        assert_eq!(message1.media_id(), message2.media_id());
    }
    
    #[test]
    fn test_invalid_phone_number() {
        let result = AudioMessageBuilder::new()
            .to("invalid-phone")
            .media_id("123456")
            .build();
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_media_id() {
        let result = AudioMessageBuilder::new()
            .to("+1234567890")
            .media_id("invalid_id")
            .build();
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_media_url() {
        let result = AudioMessageBuilder::new()
            .to("+1234567890")
            .media_url("invalid-url")
            .build();
        
        assert!(result.is_err());
    }
}
