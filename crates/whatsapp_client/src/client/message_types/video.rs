use crate::{
    errors::WhatsAppResult,
    client::validation::{
        validate_phone_number, validate_media_id, validate_url, 
        validate_mime_type, validate_file_size, validate_caption, MediaType
    },
};
use serde::Serialize;

/// A video message that can be sent via WhatsApp
/// 
/// Video messages display as inline videos within the chat conversation with playback controls.
/// They can be sent using either uploaded media (recommended) or hosted media.
/// Videos support captions up to 1024 characters and must use H.264 codec with AAC audio.
#[derive(Debug, Clone, Serialize)]
pub struct VideoMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient type - always "individual" for direct messages
    recipient_type: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Video content configuration
    video: VideoContent,
}

/// Video message content structure
/// 
/// This contains either a media ID (for uploaded videos) or a URL (for hosted videos).
/// The media ID approach is recommended for better performance and reliability.
/// Videos can include an optional caption.
#[derive(Debug, Clone, Serialize)]
struct VideoContent {
    /// Media ID for uploaded video (recommended approach)
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// URL for hosted video (not recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,
    /// Optional caption text (max 1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
}

impl VideoMessage {
    /// Create a new video message using uploaded media ID
    /// 
    /// This is the recommended approach for sending video messages.
    /// The video must be uploaded to WhatsApp first using the media upload API.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `media_id` - ID of the uploaded video file from WhatsApp's media API
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::VideoMessage;
    /// let message = VideoMessage::from_media_id("+1234567890", "1013859600285441")?;
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
            message_type: "video".to_string(),
            video: VideoContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: None,
            },
        })
    }
    
    /// Create a new video message using a hosted URL
    /// 
    /// This approach is not recommended due to performance implications.
    /// WhatsApp will need to download the video from your server, which
    /// adds latency and potential failure points. Videos are typically large files.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `video_url` - URL to the hosted video file
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::VideoMessage;
    /// let message = VideoMessage::from_url(
    ///     "+1234567890", 
    ///     "https://example.com/video.mp4"
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn from_url(to: &str, video_url: &str) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_url(video_url)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "video".to_string(),
            video: VideoContent {
                id: None,
                link: Some(video_url.to_string()),
                caption: None,
            },
        })
    }
    
    /// Add a caption to the video message
    /// 
    /// Captions help explain what the video shows and are displayed
    /// below the video in WhatsApp. Maximum 1024 characters.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::VideoMessage;
    /// let message = VideoMessage::from_media_id("+1234567890", "123456")?
    ///     .with_caption("Here's the demo video you requested")?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_caption(mut self, caption: &str) -> WhatsAppResult<Self> {
        validate_caption(caption)?;
        self.video.caption = Some(caption.to_string());
        Ok(self)
    }
    
    /// Get the recipient phone number
    pub fn recipient(&self) -> &str {
        &self.to
    }
    
    /// Get the media ID if this message uses uploaded media
    pub fn media_id(&self) -> Option<&str> {
        self.video.id.as_deref()
    }
    
    /// Get the URL if this message uses hosted media
    pub fn media_url(&self) -> Option<&str> {
        self.video.link.as_deref()
    }
    
    /// Get the caption text if set
    pub fn caption(&self) -> Option<&str> {
        self.video.caption.as_deref()
    }
    
    /// Check if this message uses uploaded media (recommended)
    pub fn uses_uploaded_media(&self) -> bool {
        self.video.id.is_some()
    }
    
    /// Validate video file properties
    /// 
    /// This can be used to validate video files before upload.
    /// Note: This validation is performed at the application level,
    /// WhatsApp will perform its own validation when receiving the message.
    /// 
    /// Videos must use H.264 video codec and AAC audio codec.
    /// Single audio stream or no audio stream only.
    pub fn validate_video_file(
        mime_type: &str,
        file_size_bytes: u64,
    ) -> WhatsAppResult<()> {
        validate_mime_type(mime_type, MediaType::Video)?;
        validate_file_size(file_size_bytes, MediaType::Video)?;
        Ok(())
    }
    
    /// Get supported video formats
    /// 
    /// Returns the list of MIME types supported by WhatsApp for video messages.
    /// All videos must use H.264 video codec and AAC audio codec.
    pub fn supported_formats() -> &'static [&'static str] {
        &[
            "video/3gpp", // 3GPP format
            "video/mp4",  // MP4 format
        ]
    }
    
    /// Get video codec requirements
    /// 
    /// Returns information about the required codecs for video messages.
    /// WhatsApp has strict requirements for video encoding.
    pub fn codec_requirements() -> &'static str {
        "H.264 video codec and AAC audio codec. Single audio stream or no audio stream only."
    }
    
    /// Get maximum file size for video messages
    /// 
    /// Returns the maximum file size in bytes (16 MB for videos).
    pub fn max_file_size() -> u64 {
        16 * 1024 * 1024 // 16 MB
    }
    
    /// Check if video file meets WhatsApp's technical requirements
    /// 
    /// This is a comprehensive validation that checks MIME type, file size,
    /// and provides guidance on codec requirements. Use this before uploading
    /// videos to avoid rejection by WhatsApp's servers.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::VideoMessage;
    /// // Check if a video file is suitable for WhatsApp
    /// let is_valid = VideoMessage::validate_for_whatsapp(
    ///     "video/mp4", 
    ///     10 * 1024 * 1024 // 10MB
    /// );
    /// 
    /// match is_valid {
    ///     Ok(_) => println!("Video is ready for WhatsApp"),
    ///     Err(e) => println!("Video needs adjustment: {}", e),
    /// }
    /// ```
    pub fn validate_for_whatsapp(
        mime_type: &str,
        file_size_bytes: u64,
    ) -> WhatsAppResult<()> {
        // Validate MIME type and file size
        Self::validate_video_file(mime_type, file_size_bytes)?;
        
        // Additional validation could be added here for codec checking
        // if video metadata parsing is available in the future
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_video_message_from_media_id() {
        let message = VideoMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert_eq!(message.caption(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_video_message_from_url() {
        let message = VideoMessage::from_url(
            "+1234567890", 
            "https://example.com/video.mp4"
        ).unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), None);
        assert_eq!(message.media_url(), Some("https://example.com/video.mp4"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_video_message_with_caption() {
        let message = VideoMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_caption("Product demonstration video")
            .unwrap();
        
        assert_eq!(message.caption(), Some("Product demonstration video"));
    }
    
    #[test]
    fn test_video_message_serialization_with_media_id() {
        let message = VideoMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["recipient_type"], "individual");
        assert_eq!(json["to"], "+1234567890");
        assert_eq!(json["type"], "video");
        assert_eq!(json["video"]["id"], "1013859600285441");
        assert!(json["video"]["link"].is_null());
        assert!(json["video"]["caption"].is_null());
    }
    
    #[test]
    fn test_video_message_serialization_with_url_and_caption() {
        let message = VideoMessage::from_url(
            "+1234567890", 
            "https://example.com/video.mp4"
        ).unwrap()
        .with_caption("Check out this amazing video")
        .unwrap();
        
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["video"]["link"], "https://example.com/video.mp4");
        assert_eq!(json["video"]["caption"], "Check out this amazing video");
        assert!(json["video"]["id"].is_null());
    }
    
    #[test]
    fn test_invalid_phone_number() {
        let result = VideoMessage::from_media_id("invalid", "123456");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_media_id() {
        let result = VideoMessage::from_media_id("+1234567890", "invalid_id");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_url() {
        let result = VideoMessage::from_url("+1234567890", "not-a-url");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_caption() {
        let long_caption = "x".repeat(1025); // Over 1024 character limit
        let result = VideoMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_caption(&long_caption);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_video_file_validation() {
        // Valid video formats
        assert!(VideoMessage::validate_video_file("video/mp4", 1024 * 1024).is_ok());
        assert!(VideoMessage::validate_video_file("video/3gpp", 5 * 1024 * 1024).is_ok());
        
        // Invalid MIME type
        assert!(VideoMessage::validate_video_file("video/avi", 1024).is_err());
        assert!(VideoMessage::validate_video_file("video/webm", 1024).is_err());
        
        // File too large (over 16MB)
        assert!(VideoMessage::validate_video_file("video/mp4", 17 * 1024 * 1024).is_err());
    }
    
    #[test]
    fn test_whatsapp_validation() {
        // Valid video
        assert!(VideoMessage::validate_for_whatsapp("video/mp4", 10 * 1024 * 1024).is_ok());
        
        // Invalid format
        assert!(VideoMessage::validate_for_whatsapp("video/avi", 1024 * 1024).is_err());
        
        // Too large
        assert!(VideoMessage::validate_for_whatsapp("video/mp4", 20 * 1024 * 1024).is_err());
    }
    
    #[test]
    fn test_supported_formats() {
        let formats = VideoMessage::supported_formats();
        assert!(formats.contains(&"video/mp4"));
        assert!(formats.contains(&"video/3gpp"));
        assert_eq!(formats.len(), 2); // Only MP4 and 3GPP are supported
    }
    
    #[test]
    fn test_codec_requirements() {
        let requirements = VideoMessage::codec_requirements();
        assert!(requirements.contains("H.264"));
        assert!(requirements.contains("AAC"));
    }
    
    #[test]
    fn test_max_file_size() {
        assert_eq!(VideoMessage::max_file_size(), 16 * 1024 * 1024);
    }
}
