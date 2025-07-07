use crate::{
    errors::WhatsAppResult,
    client::message_types::ImageMessage,
};

/// Builder for creating image messages with fluent interface
/// 
/// This builder provides a discoverable way to create image messages
/// with either uploaded media IDs (recommended) or hosted URLs,
/// plus optional caption support. Images are perfect for visual
/// communication and can include descriptive text.
/// 
/// # Example
/// ```
/// # use whatsapp_client::client::builders::ImageMessageBuilder;
/// // Simple image without caption
/// let message = ImageMessageBuilder::new()
///     .to("+1234567890")
///     .media_id("1013859600285441")
///     .build()?;
/// 
/// // Image with descriptive caption
/// let message = ImageMessageBuilder::new()
///     .to("+1234567890")
///     .media_url("https://example.com/sunset.jpg")
///     .caption("Beautiful sunset from our office!")
///     .build()?;
/// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
/// ```
#[derive(Debug, Default)]
pub struct ImageMessageBuilder {
    to: Option<String>,
    media_id: Option<String>,
    media_url: Option<String>,
    caption: Option<String>,
}

impl ImageMessageBuilder {
    /// Create a new image message builder
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
    
    /// Set the media ID for uploaded image (recommended approach)
    /// 
    /// Use this when you've already uploaded the image to WhatsApp's
    /// media servers. This approach offers better performance, reliability,
    /// and faster loading for recipients.
    /// 
    /// # Arguments
    /// * `id` - Media ID returned from WhatsApp's media upload API
    /// 
    /// # Note
    /// Cannot be used together with `media_url()`. If both are set,
    /// `media_id` takes precedence as it's the recommended approach.
    pub fn media_id(mut self, id: &str) -> Self {
        self.media_id = Some(id.to_string());
        // Clear URL if previously set - ID takes precedence
        self.media_url = None;
        self
    }
    
    /// Set the URL for hosted image (not recommended for production)
    /// 
    /// Use this for quick testing or when you can't upload to WhatsApp.
    /// The image must be publicly accessible via HTTPS. WhatsApp will
    /// download the file, which adds latency and potential failure points.
    /// 
    /// # Arguments
    /// * `url` - HTTPS URL to the image file (JPEG or PNG only)
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
    
    /// Add a caption to describe the image
    /// 
    /// Captions appear below the image and help explain what the image
    /// shows. They're especially useful for accessibility and context.
    /// Maximum 1024 characters.
    /// 
    /// # Arguments
    /// * `text` - Caption text (up to 1024 characters)
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::ImageMessageBuilder;
    /// let message = ImageMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .media_id("123456")
    ///     .caption("Our new product launch event! 🎉")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn caption(mut self, text: &str) -> Self {
        self.caption = Some(text.to_string());
        self
    }
    
    /// Remove any previously set caption
    /// 
    /// This is useful if you want to conditionally add captions
    /// and need to clear a previously set value.
    pub fn without_caption(mut self) -> Self {
        self.caption = None;
        self
    }
    
    /// Build the image message
    /// 
    /// This validates all the configuration and creates the final ImageMessage.
    /// Returns an error if required fields are missing or invalid.
    /// 
    /// # Validation
    /// - Recipient phone number must be set and valid E.164 format
    /// - Either media_id OR media_url must be set (but not both)
    /// - Caption (if provided) must be 1024 characters or less
    /// - All WhatsApp validation rules are applied (file size, format, etc.)
    pub fn build(self) -> WhatsAppResult<ImageMessage> {
        let to = self.to.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Recipient phone number is required".to_string()
            )
        })?;
        
        // Create the base message using the appropriate method
        let mut message = match (self.media_id, self.media_url) {
            (Some(id), _) => {
                // Media ID takes precedence (recommended approach)
                ImageMessage::from_media_id(&to, &id)?
            },
            (None, Some(url)) => {
                // Fall back to URL approach
                ImageMessage::from_url(&to, &url)?
            },
            (None, None) => {
                return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                    "Either media_id or media_url must be provided".to_string()
                ));
            }
        };
        
        // Add caption if provided
        if let Some(caption_text) = self.caption {
            message = message.with_caption(&caption_text)?;
        }
        
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_image_message_with_media_id() {
        let message = ImageMessageBuilder::new()
            .to("+1234567890")
            .media_id("1013859600285441")
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert_eq!(message.caption(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_image_message_with_url_and_caption() {
        let message = ImageMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/sunset.jpg")
            .caption("Beautiful sunset!")
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), None);
        assert_eq!(message.media_url(), Some("https://example.com/sunset.jpg"));
        assert_eq!(message.caption(), Some("Beautiful sunset!"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_caption_without_caption_toggle() {
        let message = ImageMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption("First caption")
            .without_caption() // Should remove the caption
            .build()
            .unwrap();
        
        assert_eq!(message.caption(), None);
    }
    
    #[test]
    fn test_media_id_precedence_with_caption() {
        let message = ImageMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/image.jpg")
            .caption("Test caption")
            .media_id("1013859600285441") // This should override the URL
            .build()
            .unwrap();
        
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert_eq!(message.caption(), Some("Test caption"));
    }
    
    #[test]
    fn test_missing_recipient() {
        let result = ImageMessageBuilder::new()
            .media_id("123456")
            .caption("Test")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Recipient phone number is required"));
    }
    
    #[test]
    fn test_missing_media() {
        let result = ImageMessageBuilder::new()
            .to("+1234567890")
            .caption("Test caption")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Either media_id or media_url must be provided"));
    }
    
    #[test]
    fn test_invalid_caption_length() {
        let long_caption = "x".repeat(1025); // Over 1024 character limit
        let result = ImageMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption(&long_caption)
            .build();
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_fluent_interface_all_methods() {
        // Test that all methods can be chained in any order
        let message = ImageMessageBuilder::new()
            .caption("Beautiful photo")
            .media_id("123456")
            .to("+1234567890")
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("123456"));
        assert_eq!(message.caption(), Some("Beautiful photo"));
    }
    
    #[test]
    fn test_conditional_caption_building() {
        // Simulate conditional caption logic
        let add_caption = true;
        
        let mut builder = ImageMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456");
        
        if add_caption {
            builder = builder.caption("Dynamic caption");
        }
        
        let message = builder.build().unwrap();
        assert_eq!(message.caption(), Some("Dynamic caption"));
    }
}
