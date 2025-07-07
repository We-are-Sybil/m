use crate::{
    errors::WhatsAppResult,
    client::message_types::VideoMessage,
};

/// Builder for creating video messages with fluent interface
/// 
/// This builder handles the complexities of video message creation,
/// including WhatsApp's strict codec requirements and file size limits.
/// Videos are powerful for demonstrations, tutorials, and visual
/// communication but require careful handling due to their size and
/// technical constraints.
/// 
/// # Technical Requirements
/// Videos must use H.264 video codec with AAC audio codec.
/// Single audio stream or no audio stream only. Maximum 16MB file size.
/// 
/// # Example
/// ```
/// # use whatsapp_client::client::builders::VideoMessageBuilder;
/// // Product demo video with context
/// let message = VideoMessageBuilder::new()
///     .to("+1234567890")
///     .media_id("1013859600285441")
///     .caption("Product demo: new features walkthrough (2 min)")
///     .build()?;
/// 
/// // Simple video without caption
/// let message = VideoMessageBuilder::new()
///     .to("+1234567890")
///     .media_url("https://example.com/demo.mp4")
///     .build()?;
/// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
/// ```
#[derive(Debug, Default)]
pub struct VideoMessageBuilder {
    to: Option<String>,
    media_id: Option<String>,
    media_url: Option<String>,
    caption: Option<String>,
}

impl VideoMessageBuilder {
    /// Create a new video message builder
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
    
    /// Set the media ID for uploaded video (strongly recommended)
    /// 
    /// Use this when you've uploaded the video to WhatsApp's media servers.
    /// This is especially important for videos due to their large file sizes
    /// and WhatsApp's strict codec requirements. Pre-uploading ensures
    /// the video is properly processed and optimized.
    /// 
    /// # Arguments
    /// * `id` - Media ID returned from WhatsApp's media upload API
    /// 
    /// # Performance Note
    /// Videos sent via media ID load faster for recipients and are less
    /// likely to fail due to network issues during transmission.
    /// 
    /// # Technical Requirements
    /// - H.264 video codec required
    /// - AAC audio codec required
    /// - Maximum 16MB file size
    /// - MP4 or 3GPP container formats only
    pub fn media_id(mut self, id: &str) -> Self {
        self.media_id = Some(id.to_string());
        // Clear URL if previously set - ID takes precedence
        self.media_url = None;
        self
    }
    
    /// Set the URL for hosted video (use with caution)
    /// 
    /// Use this only for testing or when upload isn't possible.
    /// Video files are typically large (approaching the 16MB limit),
    /// making URL-based delivery slower and more prone to failure.
    /// 
    /// # Arguments
    /// * `url` - HTTPS URL to the video file
    /// 
    /// # Important Considerations
    /// - WhatsApp must download the entire video before delivery
    /// - Network timeouts are more likely with large files
    /// - Your server must handle WhatsApp's download requests
    /// - Video must meet codec requirements before upload
    /// 
    /// # Codec Requirements
    /// Ensure your hosted video uses H.264 video codec and AAC audio codec
    /// before setting the URL, as WhatsApp won't perform conversion.
    pub fn media_url(mut self, url: &str) -> Self {
        // Only set URL if no media ID is already set
        if self.media_id.is_none() {
            self.media_url = Some(url.to_string());
        }
        self
    }
    
    /// Add a caption describing the video content
    /// 
    /// Captions are especially valuable for videos since recipients
    /// can't quickly scan the content like they can with images or text.
    /// Use captions to provide context, duration, or content summaries.
    /// 
    /// # Arguments
    /// * `text` - Caption text (up to 1024 characters)
    /// 
    /// # Best Practices for Video Captions
    /// - Include video duration if it's longer than 30 seconds
    /// - Describe what the video demonstrates or teaches
    /// - Mention if audio is required for understanding
    /// - Indicate if the video contains sensitive or time-sensitive content
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::VideoMessageBuilder;
    /// let message = VideoMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .media_id("123456")
    ///     .caption("Tutorial: Setting up your account (3 min) - audio recommended")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn caption(mut self, text: &str) -> Self {
        self.caption = Some(text.to_string());
        self
    }
    
    /// Remove any previously set caption
    /// 
    /// Useful for conditional caption logic where business rules
    /// determine whether context is needed. For example, tutorial
    /// videos might need captions while promotional videos might not.
    pub fn without_caption(mut self) -> Self {
        self.caption = None;
        self
    }
    
    /// Build the video message
    /// 
    /// This validates all configuration and creates the final VideoMessage.
    /// Video messages have additional validation due to WhatsApp's strict
    /// technical requirements for video content.
    /// 
    /// # Validation Process
    /// 1. Recipient phone number must be valid E.164 format
    /// 2. Either media_id OR media_url must be provided (media_id preferred)
    /// 3. Caption (if provided) must be ≤1024 characters
    /// 4. Video codec and format validation (H.264 + AAC required)
    /// 5. File size validation (≤16MB)
    /// 
    /// # Error Scenarios
    /// - Missing recipient or media source
    /// - Invalid phone number format
    /// - Caption too long
    /// - Video doesn't meet technical requirements
    /// - File size exceeds 16MB limit
    /// 
    /// # Performance Recommendations
    /// For the best user experience, ensure videos are:
    /// - Compressed efficiently (aim for under 10MB when possible)
    /// - Encoded with H.264 baseline profile for compatibility
    /// - Have clear audio if speech is included
    /// - Are under 60 seconds for optimal engagement
    pub fn build(self) -> WhatsAppResult<VideoMessage> {
        let to = self.to.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Recipient phone number is required for video messages".to_string()
            )
        })?;
        
        // Create the base message using the appropriate method
        let mut message = match (self.media_id, self.media_url) {
            (Some(id), _) => {
                // Media ID takes precedence (strongly recommended for videos)
                VideoMessage::from_media_id(&to, &id)?
            },
            (None, Some(url)) => {
                // Fall back to URL approach (discouraged for videos)
                VideoMessage::from_url(&to, &url)?
            },
            (None, None) => {
                return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                    "Either media_id or media_url must be provided for video messages. \
                     Media ID is strongly recommended for videos due to file size and codec requirements.".to_string()
                ));
            }
        };
        
        // Add caption if provided (this validates caption length)
        if let Some(caption_text) = self.caption {
            message = message.with_caption(&caption_text)?;
        }
        
        Ok(message)
    }
    
    /// Validate video file before building (utility method)
    /// 
    /// This helper method lets you validate video files before even
    /// starting the builder process. Useful for pre-flight checks
    /// in upload workflows.
    /// 
    /// # Arguments
    /// * `mime_type` - Video MIME type (should be "video/mp4" or "video/3gpp")
    /// * `file_size_bytes` - File size in bytes (must be ≤16MB)
    /// 
    /// # Returns
    /// Ok(()) if video meets requirements, detailed error otherwise
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::VideoMessageBuilder;
    /// // Check video before starting upload process
    /// match VideoMessageBuilder::validate_video_requirements("video/mp4", 12_000_000) {
    ///     Ok(_) => println!("Video meets WhatsApp requirements"),
    ///     Err(e) => println!("Video needs adjustment: {}", e),
    /// }
    /// ```
    pub fn validate_video_requirements(
        mime_type: &str,
        file_size_bytes: u64,
    ) -> WhatsAppResult<()> {
        VideoMessage::validate_for_whatsapp(mime_type, file_size_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_video_message_with_media_id() {
        let message = VideoMessageBuilder::new()
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
    fn test_video_message_with_caption() {
        let message = VideoMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption("Product demo video - 2 minutes")
            .build()
            .unwrap();
        
        assert_eq!(message.caption(), Some("Product demo video - 2 minutes"));
    }
    
    #[test]
    fn test_video_message_with_url() {
        let message = VideoMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/video.mp4")
            .caption("External video")
            .build()
            .unwrap();
        
        assert_eq!(message.media_url(), Some("https://example.com/video.mp4"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_media_id_precedence() {
        let message = VideoMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/video.mp4")
            .caption("Test video")
            .media_id("1013859600285441") // This should override the URL
            .build()
            .unwrap();
        
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
    }
    
    #[test]
    fn test_caption_toggle() {
        let message = VideoMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption("First caption")
            .without_caption()
            .build()
            .unwrap();
        
        assert_eq!(message.caption(), None);
    }
    
    #[test]
    fn test_missing_recipient_error() {
        let result = VideoMessageBuilder::new()
            .media_id("123456")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Recipient phone number is required"));
    }
    
    #[test]
    fn test_missing_media_error() {
        let result = VideoMessageBuilder::new()
            .to("+1234567890")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Either media_id or media_url must be provided"));
        assert!(error_msg.contains("Media ID is strongly recommended"));
    }
    
    #[test]
    fn test_conditional_caption_for_video_types() {
        // Simulate business logic for different video types
        let video_type = "tutorial"; // Could be "promotional", "tutorial", "demo"
        let video_duration_seconds = 180; // 3 minutes
        
        let mut builder = VideoMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456");
        
        // Add context-aware captions based on video type
        match video_type {
            "tutorial" => {
                builder = builder.caption(&format!(
                    "Tutorial video ({} min) - audio recommended", 
                    video_duration_seconds / 60
                ));
            },
            "demo" => {
                builder = builder.caption("Product demonstration - no audio required");
            },
            "promotional" => {
                // Promotional videos might not need captions
            },
            _ => {}
        }
        
        let message = builder.build().unwrap();
        assert_eq!(message.caption(), Some("Tutorial video (3 min) - audio recommended"));
    }
    
    #[test]
    fn test_video_validation_utility() {
        // Test the static validation method
        assert!(VideoMessageBuilder::validate_video_requirements("video/mp4", 10_000_000).is_ok());
        assert!(VideoMessageBuilder::validate_video_requirements("video/3gpp", 5_000_000).is_ok());
        
        // Test invalid format
        assert!(VideoMessageBuilder::validate_video_requirements("video/avi", 1_000_000).is_err());
        
        // Test file too large (over 16MB)
        assert!(VideoMessageBuilder::validate_video_requirements("video/mp4", 17_000_000).is_err());
    }
    
    #[test]
    fn test_fluent_interface_flexibility() {
        // Test that methods can be called in any logical order
        let message1 = VideoMessageBuilder::new()
            .caption("Test video")
            .media_id("123456")
            .to("+1234567890")
            .build()
            .unwrap();
        
        let message2 = VideoMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption("Test video")
            .build()
            .unwrap();
        
        assert_eq!(message1.recipient(), message2.recipient());
        assert_eq!(message1.media_id(), message2.media_id());
        assert_eq!(message1.caption(), message2.caption());
    }
}
