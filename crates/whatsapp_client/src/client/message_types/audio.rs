use crate::{
    errors::WhatsAppResult,
    client::validation::{
        validate_phone_number, validate_media_id, validate_url, 
        validate_mime_type, validate_file_size, MediaType
    },
};
use serde::Serialize;

/// An audio message that can be sent via WhatsApp
/// 
/// Audio messages display an audio icon and allow playback within WhatsApp.
/// They can be sent using either uploaded media (recommended) or hosted media.
#[derive(Debug, Clone, Serialize)]
pub struct AudioMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient type - always "individual" for direct messages
    recipient_type: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Audio content configuration
    audio: AudioContent,
}

/// Audio message content structure
/// 
/// This contains either a media ID (for uploaded audio) or a URL (for hosted audio).
/// The media ID approach is recommended for better performance and reliability.
#[derive(Debug, Clone, Serialize)]
struct AudioContent {
    /// Media ID for uploaded audio (recommended approach)
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// URL for hosted audio (not recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,
}

impl AudioMessage {
    /// Create a new audio message using uploaded media ID
    /// 
    /// This is the recommended approach for sending audio messages.
    /// The audio must be uploaded to WhatsApp first using the media upload API.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `media_id` - ID of the uploaded audio file from WhatsApp's media API
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::AudioMessage;
    /// let message = AudioMessage::from_media_id("+1234567890", "1013859600285441")?;
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
            message_type: "audio".to_string(),
            audio: AudioContent {
                id: Some(media_id.to_string()),
                link: None,
            },
        })
    }
    
    /// Create a new audio message using a hosted URL
    /// 
    /// This approach is not recommended due to performance implications.
    /// WhatsApp will need to download the audio from your server, which
    /// adds latency and potential failure points.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `audio_url` - URL to the hosted audio file
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::AudioMessage;
    /// let message = AudioMessage::from_url(
    ///     "+1234567890", 
    ///     "https://example.com/audio.mp3"
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn from_url(to: &str, audio_url: &str) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_url(audio_url)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "audio".to_string(),
            audio: AudioContent {
                id: None,
                link: Some(audio_url.to_string()),
            },
        })
    }
    
    /// Get the recipient phone number
    pub fn recipient(&self) -> &str {
        &self.to
    }
    
    /// Get the media ID if this message uses uploaded media
    pub fn media_id(&self) -> Option<&str> {
        self.audio.id.as_deref()
    }
    
    /// Get the URL if this message uses hosted media
    pub fn media_url(&self) -> Option<&str> {
        self.audio.link.as_deref()
    }
    
    /// Check if this message uses uploaded media (recommended)
    pub fn uses_uploaded_media(&self) -> bool {
        self.audio.id.is_some()
    }
    
    /// Validate audio file properties
    /// 
    /// This can be used to validate audio files before upload.
    /// Note: This validation is performed at the application level,
    /// WhatsApp will perform its own validation when receiving the message.
    pub fn validate_audio_file(
        mime_type: &str,
        file_size_bytes: u64,
    ) -> WhatsAppResult<()> {
        validate_mime_type(mime_type, MediaType::Audio)?;
        validate_file_size(file_size_bytes, MediaType::Audio)?;
        Ok(())
    }
    
    /// Get supported audio formats
    /// 
    /// Returns the list of MIME types supported by WhatsApp for audio messages.
    pub fn supported_formats() -> &'static [&'static str] {
        &[
            "audio/aac",    // AAC format
            "audio/amr",    // AMR format  
            "audio/mpeg",   // MP3 format
            "audio/mp4",    // MP4 Audio format
            "audio/ogg",    // OGG format (OPUS codecs only, mono input only)
        ]
    }
    
    /// Get maximum file size for audio messages
    /// 
    /// Returns the maximum file size in bytes (16 MB for audio).
    pub fn max_file_size() -> u64 {
        16 * 1024 * 1024 // 16 MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_audio_message_from_media_id() {
        let message = AudioMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_audio_message_from_url() {
        let message = AudioMessage::from_url(
            "+1234567890", 
            "https://example.com/audio.mp3"
        ).unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), None);
        assert_eq!(message.media_url(), Some("https://example.com/audio.mp3"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_audio_message_serialization_with_media_id() {
        let message = AudioMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["recipient_type"], "individual");
        assert_eq!(json["to"], "+1234567890");
        assert_eq!(json["type"], "audio");
        assert_eq!(json["audio"]["id"], "1013859600285441");
        assert!(json["audio"]["link"].is_null());
    }
    
    #[test]
    fn test_audio_message_serialization_with_url() {
        let message = AudioMessage::from_url(
            "+1234567890", 
            "https://example.com/audio.mp3"
        ).unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["audio"]["link"], "https://example.com/audio.mp3");
        assert!(json["audio"]["id"].is_null());
    }
    
    #[test]
    fn test_invalid_phone_number() {
        let result = AudioMessage::from_media_id("invalid", "123456");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_media_id() {
        let result = AudioMessage::from_media_id("+1234567890", "invalid_id");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_url() {
        let result = AudioMessage::from_url("+1234567890", "not-a-url");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_audio_file_validation() {
        // Valid audio formats
        assert!(AudioMessage::validate_audio_file("audio/mpeg", 1024 * 1024).is_ok());
        assert!(AudioMessage::validate_audio_file("audio/aac", 5 * 1024 * 1024).is_ok());
        
        // Invalid MIME type
        assert!(AudioMessage::validate_audio_file("audio/wav", 1024).is_err());
        
        // File too large (over 16MB)
        assert!(AudioMessage::validate_audio_file("audio/mpeg", 17 * 1024 * 1024).is_err());
    }
    
    #[test]
    fn test_supported_formats() {
        let formats = AudioMessage::supported_formats();
        assert!(formats.contains(&"audio/mpeg"));
        assert!(formats.contains(&"audio/aac"));
        assert!(formats.contains(&"audio/amr"));
        assert!(formats.contains(&"audio/mp4"));
        assert!(formats.contains(&"audio/ogg"));
    }
    
    #[test]
    fn test_max_file_size() {
        assert_eq!(AudioMessage::max_file_size(), 16 * 1024 * 1024);
    }
}
