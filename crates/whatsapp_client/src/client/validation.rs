use crate::errors::{WhatsAppError, WhatsAppResult};
use regex::Regex;
use std::sync::OnceLock;

/// Maximum file sizes for different media types (in bytes)
pub const MAX_AUDIO_SIZE: u64 = 16 * 1024 * 1024; // 16 MB
pub const MAX_DOCUMENT_SIZE: u64 = 100 * 1024 * 1024; // 100 MB
pub const MAX_IMAGE_SIZE: u64 = 5 * 1024 * 1024; // 5 MB
pub const MAX_VIDEO_SIZE: u64 = 16 * 1024 * 1024; // 16 MB

/// Maximum text lengths for various fields
pub const MAX_TEXT_MESSAGE_LENGTH: usize = 4096;
pub const MAX_CAPTION_LENGTH: usize = 1024;
pub const MAX_BUTTON_TITLE_LENGTH: usize = 20;
pub const MAX_BUTTON_ID_LENGTH: usize = 256;
pub const MAX_LIST_TITLE_LENGTH: usize = 24;
pub const MAX_LIST_DESCRIPTION_LENGTH: usize = 72;
pub const MAX_HEADER_TEXT_LENGTH: usize = 60;
pub const MAX_FOOTER_TEXT_LENGTH: usize = 60;
pub const MAX_URL_LENGTH: usize = 2048;

/// Validate phone number format (E.164)
/// 
/// WhatsApp requires phone numbers to be in E.164 format: +[country code][number]
/// Examples: +573212345432, +79823238746
pub fn validate_phone_number(phone: &str) -> WhatsAppResult<()> {
    static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();
    let regex = PHONE_REGEX.get_or_init(|| {
        Regex::new(r"^\+[1-9]\d{7,14}$").expect("Invalid phone regex")
    });
    
    if !regex.is_match(phone) {
        return Err(WhatsAppError::InvalidPhoneNumber(
            format!("Phone number must be in E.164 format (+1234567890): {}", phone)
        ));
    }
    
    Ok(())
}

/// Validate text message content
/// 
/// Checks message length and ensures it's not empty.
/// WhatsApp supports up to 4096 characters for text messages.
pub fn validate_text_message(message: &str) -> WhatsAppResult<()> {
    if message.is_empty() {
        return Err(WhatsAppError::InvalidMessageContent(
            "Message cannot be empty".to_string()
        ));
    }
    
    if message.len() > MAX_TEXT_MESSAGE_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Message too long: {} characters (max {})", 
                   message.len(), MAX_TEXT_MESSAGE_LENGTH)
        ));
    }
    
    Ok(())
}

/// Validate media caption
/// 
/// Captions are optional but when provided must be within WhatsApp's limits.
pub fn validate_caption(caption: &str) -> WhatsAppResult<()> {
    if caption.len() > MAX_CAPTION_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Caption too long: {} characters (max {})", 
                   caption.len(), MAX_CAPTION_LENGTH)
        ));
    }
    
    Ok(())
}

/// Validate interactive button
/// 
/// Buttons must have valid IDs and titles within WhatsApp's character limits.
pub fn validate_button(id: &str, title: &str) -> WhatsAppResult<()> {
    if id.is_empty() {
        return Err(WhatsAppError::InvalidMessageContent(
            "Button ID cannot be empty".to_string()
        ));
    }
    
    if id.len() > MAX_BUTTON_ID_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Button ID too long: {} characters (max {})", 
                   id.len(), MAX_BUTTON_ID_LENGTH)
        ));
    }
    
    if title.is_empty() {
        return Err(WhatsAppError::InvalidMessageContent(
            "Button title cannot be empty".to_string()
        ));
    }
    
    if title.len() > MAX_BUTTON_TITLE_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Button title too long: {} characters (max {})", 
                   title.len(), MAX_BUTTON_TITLE_LENGTH)
        ));
    }
    
    Ok(())
}

/// Validate list section and rows
/// 
/// List messages have specific limits on section titles and row content.
pub fn validate_list_section(title: &str, rows: &[(String, String, Option<String>)]) -> WhatsAppResult<()> {
    if title.is_empty() {
        return Err(WhatsAppError::InvalidMessageContent(
            "List section title cannot be empty".to_string()
        ));
    }
    
    if title.len() > MAX_LIST_TITLE_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("List section title too long: {} characters (max {})", 
                   title.len(), MAX_LIST_TITLE_LENGTH)
        ));
    }
    
    if rows.is_empty() {
        return Err(WhatsAppError::InvalidMessageContent(
            "List section must have at least one row".to_string()
        ));
    }
    
    if rows.len() > 10 {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("List section has too many rows: {} (max 10)", rows.len())
        ));
    }
    
    for (id, title, description) in rows {
        validate_button(id, title)?;
        
        if let Some(desc) = description {
            if desc.len() > MAX_LIST_DESCRIPTION_LENGTH {
                return Err(WhatsAppError::InvalidMessageContent(
                    format!("List row description too long: {} characters (max {})", 
                           desc.len(), MAX_LIST_DESCRIPTION_LENGTH)
                ));
            }
        }
    }
    
    Ok(())
}

/// Validate URL format
/// 
/// URLs must be properly formatted and within length limits.
pub fn validate_url(url: &str) -> WhatsAppResult<()> {
    if url.is_empty() {
        return Err(WhatsAppError::InvalidMessageContent(
            "URL cannot be empty".to_string()
        ));
    }

    if url.len() > MAX_URL_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("URL too long: {} characters (max {})", 
                   url.len(), MAX_URL_LENGTH)
        ));
    }
    
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(WhatsAppError::InvalidMessageContent(
            "URL must start with http:// or https://".to_string()
        ));
    }
    
    Ok(())
}

/// Validate location coordinates
/// 
/// Latitude must be between -90 and 90, longitude between -180 and 180.
pub fn validate_coordinates(latitude: f64, longitude: f64) -> WhatsAppResult<()> {
    if !(-90.0..=90.0).contains(&latitude) {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Invalid latitude: {} (must be between -90 and 90)", latitude)
        ));
    }
    
    if !(-180.0..=180.0).contains(&longitude) {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Invalid longitude: {} (must be between -180 and 180)", longitude)
        ));
    }
    
    Ok(())
}

/// Validate media ID format
/// 
/// Media IDs should be non-empty strings, typically numeric.
pub fn validate_media_id(media_id: &str) -> WhatsAppResult<()> {
    if media_id.is_empty() {
        return Err(WhatsAppError::InvalidMessageContent(
            "Media ID cannot be empty".to_string()
        ));
    }
    
    // WhatsApp media IDs are typically numeric strings
    if !media_id.chars().all(|c| c.is_ascii_digit()) {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Invalid media ID format: {} (should be numeric)", media_id)
        ));
    }
    
    Ok(())
}

/// Validate file size for media type
/// 
/// Different media types have different size limits.
pub fn validate_file_size(size_bytes: u64, media_type: MediaType) -> WhatsAppResult<()> {
    let max_size = match media_type {
        MediaType::Audio => MAX_AUDIO_SIZE,
        MediaType::Document => MAX_DOCUMENT_SIZE,
        MediaType::Image => MAX_IMAGE_SIZE,
        MediaType::Video => MAX_VIDEO_SIZE,
    };
    
    if size_bytes > max_size {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("File too large: {} bytes (max {} for {:?})", 
                   size_bytes, max_size, media_type)
        ));
    }
    
    Ok(())
}

/// Validate MIME type for media
/// 
/// WhatsApp only supports specific MIME types for each media category.
pub fn validate_mime_type(mime_type: &str, media_type: MediaType) -> WhatsAppResult<()> {
    let valid_mime_types: &[&str] = match media_type {
        MediaType::Audio => &[
            "audio/aac", "audio/amr", "audio/mpeg", "audio/mp4", "audio/ogg"
        ],
        MediaType::Document => &[
            "text/plain", 
            "application/pdf", 
            "application/vnd.ms-powerpoint",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "application/msword",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "application/vnd.ms-excel",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
        ],
        MediaType::Image => &[
            "image/jpeg", "image/png"
        ],
        MediaType::Video => &[
            "video/3gpp", "video/mp4"
        ],
    };
    
    if !valid_mime_types.contains(&mime_type) {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Unsupported MIME type '{}' for {:?}. Supported types: {:?}", 
                   mime_type, media_type, valid_mime_types)
        ));
    }
    
    Ok(())
}

/// Validate header text (for interactive messages)
pub fn validate_header_text(header: &str) -> WhatsAppResult<()> {
    if header.len() > MAX_HEADER_TEXT_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Header text too long: {} characters (max {})", 
                   header.len(), MAX_HEADER_TEXT_LENGTH)
        ));
    }
    
    Ok(())
}

/// Validate footer text (for interactive messages)
pub fn validate_footer_text(footer: &str) -> WhatsAppResult<()> {
    if footer.len() > MAX_FOOTER_TEXT_LENGTH {
        return Err(WhatsAppError::InvalidMessageContent(
            format!("Footer text too long: {} characters (max {})", 
                   footer.len(), MAX_FOOTER_TEXT_LENGTH)
        ));
    }
    
    Ok(())
}

/// Media types supported by WhatsApp
#[derive(Debug, Clone, Copy)]
pub enum MediaType {
    Audio,
    Document,
    Image,
    Video,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_phone_number_validation() {
        // Valid phone numbers
        assert!(validate_phone_number("+1234567890").is_ok());
        assert!(validate_phone_number("+441234567890").is_ok());
        assert!(validate_phone_number("+8613012345678").is_ok());
        
        // Invalid phone numbers
        assert!(validate_phone_number("1234567890").is_err()); // Missing +
        assert!(validate_phone_number("+0123456789").is_err()); // Starts with 0
        assert!(validate_phone_number("+123").is_err()); // Too short
        assert!(validate_phone_number("+1234567890123456").is_err()); // Too long
        assert!(validate_phone_number("+123abc456").is_err()); // Contains letters
        assert!(validate_phone_number("").is_err()); // Empty
    }
    
    #[test]
    fn test_text_message_validation() {
        // Valid messages
        assert!(validate_text_message("Hello world").is_ok());
        assert!(validate_text_message(&"x".repeat(4096)).is_ok()); // Max length
        
        // Invalid messages
        assert!(validate_text_message("").is_err()); // Empty
        assert!(validate_text_message(&"x".repeat(4097)).is_err()); // Too long
    }
    
    #[test]
    fn test_button_validation() {
        // Valid button
        assert!(validate_button("help", "Get Help").is_ok());
        assert!(validate_button("btn1", &"x".repeat(20)).is_ok()); // Max title length
        
        // Invalid buttons
        assert!(validate_button("", "title").is_err()); // Empty ID
        assert!(validate_button("id", "").is_err()); // Empty title
        assert!(validate_button("id", &"x".repeat(21)).is_err()); // Title too long
        assert!(validate_button(&"x".repeat(257), "title").is_err()); // ID too long
    }
    
    #[test]
    fn test_coordinate_validation() {
        // Valid coordinates
        assert!(validate_coordinates(37.7749, -122.4194).is_ok()); // San Francisco
        assert!(validate_coordinates(90.0, 180.0).is_ok()); // Extremes
        assert!(validate_coordinates(-90.0, -180.0).is_ok()); // Other extremes
        
        // Invalid coordinates
        assert!(validate_coordinates(91.0, 0.0).is_err()); // Latitude too high
        assert!(validate_coordinates(-91.0, 0.0).is_err()); // Latitude too low
        assert!(validate_coordinates(0.0, 181.0).is_err()); // Longitude too high
        assert!(validate_coordinates(0.0, -181.0).is_err()); // Longitude too low
    }
    
    #[test]
    fn test_url_validation() {
        // Valid URLs
        assert!(validate_url("https://example.com").is_ok());
        assert!(validate_url("http://example.com/path?query=1").is_ok());
        
        // Invalid URLs
        assert!(validate_url("").is_err()); // Empty
        assert!(validate_url("ftp://example.com").is_err()); // Wrong protocol
        assert!(validate_url("example.com").is_err()); // No protocol
        assert!(validate_url(&format!("https://{}.com", "x".repeat(3000))).is_err()); // Too long
    }
    
    #[test]
    fn test_mime_type_validation() {
        // Valid MIME types
        assert!(validate_mime_type("image/jpeg", MediaType::Image).is_ok());
        assert!(validate_mime_type("image/png", MediaType::Image).is_ok());
        assert!(validate_mime_type("audio/mpeg", MediaType::Audio).is_ok());
        assert!(validate_mime_type("video/mp4", MediaType::Video).is_ok());
        assert!(validate_mime_type("application/pdf", MediaType::Document).is_ok());
        
        // Invalid MIME types
        assert!(validate_mime_type("image/gif", MediaType::Image).is_err()); // Not supported
        assert!(validate_mime_type("audio/wav", MediaType::Audio).is_err()); // Not supported
        assert!(validate_mime_type("application/zip", MediaType::Document).is_err()); // Not supported
    }
}
