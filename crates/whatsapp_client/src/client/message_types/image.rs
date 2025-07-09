use crate::{
    errors::WhatsAppResult,
    client::{
        validation::{
            validate_phone_number, validate_media_id, validate_url, 
            validate_mime_type, validate_file_size, validate_caption, MediaType
        },
        message_types::mtrait::Message,
    },
};
use serde::{Serialize, Deserialize};

/// An image message that can be sent via WhatsApp
/// 
/// Image messages display as inline images within the chat conversation.
/// They can be sent using either uploaded media (recommended) or hosted media.
/// Images support captions up to 1024 characters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient type - always "individual" for direct messages
    recipient_type: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Image content configuration
    image: ImageContent,
}

impl Message for ImageMessage {
    /// Get the recipient phone number
    fn recipient(&self) -> &str {
        &self.to
    }

    /// Get the message type identifier
    fn message_type(&self) -> &str {
        "image"
    }
}

/// Image message content structure
/// 
/// This contains either a media ID (for uploaded images) or a URL (for hosted images).
/// The media ID approach is recommended for better performance and reliability.
/// Images can include an optional caption.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ImageContent {
    /// Media ID for uploaded image (recommended approach)
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// URL for hosted image (not recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,
    /// Optional caption text (max 1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
}

impl ImageMessage {
    /// Create a new image message using uploaded media ID
    /// 
    /// This is the recommended approach for sending image messages.
    /// The image must be uploaded to WhatsApp first using the media upload API.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `media_id` - ID of the uploaded image file from WhatsApp's media API
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::ImageMessage;
    /// let message = ImageMessage::from_media_id("+1234567890", "1013859600285441")?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn from_media_id(to: &str, media_id: &str) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_media_id(media_id)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "image".to_string(),
            image: ImageContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: None,
            },
        })
    }
    
    /// Create a new image message using a hosted URL
    /// 
    /// This approach is not recommended due to performance implications.
    /// WhatsApp will need to download the image from your server, which
    /// adds latency and potential failure points.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `image_url` - URL to the hosted image file
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::ImageMessage;
    /// let message = ImageMessage::from_url(
    ///     "+1234567890", 
    ///     "https://example.com/image.jpg"
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn from_url(to: &str, image_url: &str) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_url(image_url)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "image".to_string(),
            image: ImageContent {
                id: None,
                link: Some(image_url.to_string()),
                caption: None,
            },
        })
    }
    
    /// Add a caption to the image message
    /// 
    /// Captions help explain what the image shows and are displayed
    /// below the image in WhatsApp. Maximum 1024 characters.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::ImageMessage;
    /// let message = ImageMessage::from_media_id("+1234567890", "123456")?
    ///     .with_caption("Here's the photo you requested")?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_caption(mut self, caption: &str) -> WhatsAppResult<Self> {
        validate_caption(caption)?;
        self.image.caption = Some(caption.to_string());
        Ok(self)
    }
    
    /// Get the media ID if this message uses uploaded media
    pub fn media_id(&self) -> Option<&str> {
        self.image.id.as_deref()
    }
    
    /// Get the URL if this message uses hosted media
    pub fn media_url(&self) -> Option<&str> {
        self.image.link.as_deref()
    }
    
    /// Get the caption text if set
    pub fn caption(&self) -> Option<&str> {
        self.image.caption.as_deref()
    }
    
    /// Check if this message uses uploaded media (recommended)
    pub fn uses_uploaded_media(&self) -> bool {
        self.image.id.is_some()
    }
    
    /// Validate image file properties
    /// 
    /// This can be used to validate image files before upload.
    /// Note: This validation is performed at the application level,
    /// WhatsApp will perform its own validation when receiving the message.
    pub fn validate_image_file(
        mime_type: &str,
        file_size_bytes: u64,
    ) -> WhatsAppResult<()> {
        validate_mime_type(mime_type, MediaType::Image)?;
        validate_file_size(file_size_bytes, MediaType::Image)?;
        Ok(())
    }
    
    /// Get supported image formats
    /// 
    /// Returns the list of MIME types supported by WhatsApp for image messages.
    pub fn supported_formats() -> &'static [&'static str] {
        &[
            "image/jpeg", // JPEG format
            "image/png",  // PNG format
        ]
    }
    
    /// Get maximum file size for image messages
    /// 
    /// Returns the maximum file size in bytes (5 MB for images).
    pub fn max_file_size() -> u64 {
        5 * 1024 * 1024 // 5 MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_image_message_from_media_id() {
        let message = ImageMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert_eq!(message.caption(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_image_message_from_url() {
        let message = ImageMessage::from_url(
            "+1234567890", 
            "https://example.com/image.jpg"
        ).unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), None);
        assert_eq!(message.media_url(), Some("https://example.com/image.jpg"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_image_message_with_caption() {
        let message = ImageMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_caption("Beautiful sunset")
            .unwrap();
        
        assert_eq!(message.caption(), Some("Beautiful sunset"));
    }
    
    #[test]
    fn test_image_message_serialization_with_media_id() {
        let message = ImageMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["recipient_type"], "individual");
        assert_eq!(json["to"], "+1234567890");
        assert_eq!(json["type"], "image");
        assert_eq!(json["image"]["id"], "1013859600285441");
        assert!(json["image"]["link"].is_null());
        assert!(json["image"]["caption"].is_null());
    }
    
    #[test]
    fn test_image_message_serialization_with_url_and_caption() {
        let message = ImageMessage::from_url(
            "+1234567890", 
            "https://example.com/image.jpg"
        ).unwrap()
        .with_caption("Check out this photo")
        .unwrap();
        
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["image"]["link"], "https://example.com/image.jpg");
        assert_eq!(json["image"]["caption"], "Check out this photo");
        assert!(json["image"]["id"].is_null());
    }
    
    #[test]
    fn test_invalid_phone_number() {
        let result = ImageMessage::from_media_id("invalid", "123456");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_media_id() {
        let result = ImageMessage::from_media_id("+1234567890", "invalid_id");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_url() {
        let result = ImageMessage::from_url("+1234567890", "not-a-url");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_caption() {
        let long_caption = "x".repeat(1025); // Over 1024 character limit
        let result = ImageMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_caption(&long_caption);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_image_file_validation() {
        // Valid image formats
        assert!(ImageMessage::validate_image_file("image/jpeg", 1024 * 1024).is_ok());
        assert!(ImageMessage::validate_image_file("image/png", 2 * 1024 * 1024).is_ok());
        
        // Invalid MIME type
        assert!(ImageMessage::validate_image_file("image/gif", 1024).is_err());
        assert!(ImageMessage::validate_image_file("image/webp", 1024).is_err());
        
        // File too large (over 5MB)
        assert!(ImageMessage::validate_image_file("image/jpeg", 6 * 1024 * 1024).is_err());
    }
    
    #[test]
    fn test_supported_formats() {
        let formats = ImageMessage::supported_formats();
        assert!(formats.contains(&"image/jpeg"));
        assert!(formats.contains(&"image/png"));
        assert_eq!(formats.len(), 2); // Only JPEG and PNG are supported
    }
    
    #[test]
    fn test_max_file_size() {
        assert_eq!(ImageMessage::max_file_size(), 5 * 1024 * 1024);
    }
}
