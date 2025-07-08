use crate::{
    errors::WhatsAppResult,
    client::validation::{
        validate_phone_number, validate_media_id, validate_url, 
        validate_mime_type, validate_file_size, validate_caption, MediaType
    },
};
use serde::{Serialize, Deserialize};

/// A document message that can be sent via WhatsApp
/// 
/// Document messages display a document icon and allow download within WhatsApp.
/// They can be sent using either uploaded media (recommended) or hosted media.
/// Documents support captions up to 1024 characters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentMessage {
    /// Always "whatsapp" for WhatsApp Business API
    messaging_product: String,
    /// Recipient type - always "individual" for direct messages
    recipient_type: String,
    /// Recipient's phone number in E.164 format
    to: String,
    /// Message type identifier
    #[serde(rename = "type")]
    message_type: String,
    /// Document content configuration
    document: DocumentContent,
}

/// Document message content structure
/// 
/// This contains either a media ID (for uploaded documents) or a URL (for hosted documents).
/// The media ID approach is recommended for better performance and reliability.
/// Documents can include an optional caption and filename.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct DocumentContent {
    /// Media ID for uploaded document (recommended approach)
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<String>,
    /// URL for hosted document (not recommended)
    #[serde(skip_serializing_if = "Option::is_none")]
    link: Option<String>,
    /// Optional caption text (max 1024 characters)
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    /// Optional filename for the document
    #[serde(skip_serializing_if = "Option::is_none")]
    filename: Option<String>,
}

impl DocumentMessage {
    /// Create a new document message using uploaded media ID
    /// 
    /// This is the recommended approach for sending document messages.
    /// The document must be uploaded to WhatsApp first using the media upload API.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `media_id` - ID of the uploaded document file from WhatsApp's media API
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::DocumentMessage;
    /// let message = DocumentMessage::from_media_id("+1234567890", "1013859600285441")?;
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
            message_type: "document".to_string(),
            document: DocumentContent {
                id: Some(media_id.to_string()),
                link: None,
                caption: None,
                filename: None,
            },
        })
    }
    
    /// Create a new document message using a hosted URL
    /// 
    /// This approach is not recommended due to performance implications.
    /// WhatsApp will need to download the document from your server, which
    /// adds latency and potential failure points.
    /// 
    /// # Arguments
    /// * `to` - Recipient phone number in E.164 format
    /// * `document_url` - URL to the hosted document file
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::DocumentMessage;
    /// let message = DocumentMessage::from_url(
    ///     "+1234567890", 
    ///     "https://example.com/document.pdf"
    /// )?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn from_url(to: &str, document_url: &str) -> WhatsAppResult<Self> {
        // Validate inputs
        validate_phone_number(to)?;
        validate_url(document_url)?;
        
        Ok(Self {
            messaging_product: "whatsapp".to_string(),
            recipient_type: "individual".to_string(),
            to: to.to_string(),
            message_type: "document".to_string(),
            document: DocumentContent {
                id: None,
                link: Some(document_url.to_string()),
                caption: None,
                filename: None,
            },
        })
    }
    
    /// Add a caption to the document message
    /// 
    /// Captions help explain what the document contains and are displayed
    /// below the document in WhatsApp. Maximum 1024 characters.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::DocumentMessage;
    /// let message = DocumentMessage::from_media_id("+1234567890", "123456")?
    ///     .with_caption("Here's the report you requested")?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_caption(mut self, caption: &str) -> WhatsAppResult<Self> {
        validate_caption(caption)?;
        self.document.caption = Some(caption.to_string());
        Ok(self)
    }
    
    /// Add a filename to the document message
    /// 
    /// The filename helps users understand what type of document it is
    /// and how to save it. This is particularly useful for documents
    /// sent via media ID where the original filename may not be preserved.
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::message_types::DocumentMessage;
    /// let message = DocumentMessage::from_media_id("+1234567890", "123456")?
    ///     .with_filename("quarterly_report.pdf");
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn with_filename(mut self, filename: &str) -> Self {
        self.document.filename = Some(filename.to_string());
        self
    }
    
    /// Get the recipient phone number
    pub fn recipient(&self) -> &str {
        &self.to
    }
    
    /// Get the media ID if this message uses uploaded media
    pub fn media_id(&self) -> Option<&str> {
        self.document.id.as_deref()
    }
    
    /// Get the URL if this message uses hosted media
    pub fn media_url(&self) -> Option<&str> {
        self.document.link.as_deref()
    }
    
    /// Get the caption text if set
    pub fn caption(&self) -> Option<&str> {
        self.document.caption.as_deref()
    }
    
    /// Get the filename if set
    pub fn filename(&self) -> Option<&str> {
        self.document.filename.as_deref()
    }
    
    /// Check if this message uses uploaded media (recommended)
    pub fn uses_uploaded_media(&self) -> bool {
        self.document.id.is_some()
    }
    
    /// Validate document file properties
    /// 
    /// This can be used to validate document files before upload.
    /// Note: This validation is performed at the application level,
    /// WhatsApp will perform its own validation when receiving the message.
    pub fn validate_document_file(
        mime_type: &str,
        file_size_bytes: u64,
    ) -> WhatsAppResult<()> {
        validate_mime_type(mime_type, MediaType::Document)?;
        validate_file_size(file_size_bytes, MediaType::Document)?;
        Ok(())
    }
    
    /// Get supported document formats
    /// 
    /// Returns the list of MIME types supported by WhatsApp for document messages.
    pub fn supported_formats() -> &'static [&'static str] {
        &[
            "text/plain",                                                               // Plain text files
            "application/pdf",                                                          // PDF documents
            "application/vnd.ms-powerpoint",                                           // PowerPoint presentations (legacy)
            "application/vnd.openxmlformats-officedocument.presentationml.presentation", // PowerPoint presentations (modern)
            "application/msword",                                                       // Word documents (legacy)
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document", // Word documents (modern)
            "application/vnd.ms-excel",                                                // Excel spreadsheets (legacy)
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",      // Excel spreadsheets (modern)
        ]
    }
    
    /// Get maximum file size for document messages
    /// 
    /// Returns the maximum file size in bytes (100 MB for documents).
    pub fn max_file_size() -> u64 {
        100 * 1024 * 1024 // 100 MB
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    
    #[test]
    fn test_document_message_from_media_id() {
        let message = DocumentMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert_eq!(message.caption(), None);
        assert_eq!(message.filename(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_document_message_from_url() {
        let message = DocumentMessage::from_url(
            "+1234567890", 
            "https://example.com/document.pdf"
        ).unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), None);
        assert_eq!(message.media_url(), Some("https://example.com/document.pdf"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_document_message_with_caption() {
        let message = DocumentMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_caption("Here's the report")
            .unwrap();
        
        assert_eq!(message.caption(), Some("Here's the report"));
    }
    
    #[test]
    fn test_document_message_with_filename() {
        let message = DocumentMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_filename("report.pdf");
        
        assert_eq!(message.filename(), Some("report.pdf"));
    }
    
    #[test]
    fn test_document_message_with_all_options() {
        let message = DocumentMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_caption("Quarterly report")
            .unwrap()
            .with_filename("q4_2024_report.pdf");
        
        assert_eq!(message.caption(), Some("Quarterly report"));
        assert_eq!(message.filename(), Some("q4_2024_report.pdf"));
    }
    
    #[test]
    fn test_document_message_serialization_with_media_id() {
        let message = DocumentMessage::from_media_id("+1234567890", "1013859600285441").unwrap();
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["recipient_type"], "individual");
        assert_eq!(json["to"], "+1234567890");
        assert_eq!(json["type"], "document");
        assert_eq!(json["document"]["id"], "1013859600285441");
        assert!(json["document"]["link"].is_null());
        assert!(json["document"]["caption"].is_null());
        assert!(json["document"]["filename"].is_null());
    }
    
    #[test]
    fn test_document_message_serialization_with_url_and_caption() {
        let message = DocumentMessage::from_url(
            "+1234567890", 
            "https://example.com/document.pdf"
        ).unwrap()
        .with_caption("Important document")
        .unwrap()
        .with_filename("document.pdf");
        
        let json = serde_json::to_value(&message).unwrap();
        
        assert_eq!(json["messaging_product"], "whatsapp");
        assert_eq!(json["document"]["link"], "https://example.com/document.pdf");
        assert_eq!(json["document"]["caption"], "Important document");
        assert_eq!(json["document"]["filename"], "document.pdf");
        assert!(json["document"]["id"].is_null());
    }
    
    #[test]
    fn test_invalid_phone_number() {
        let result = DocumentMessage::from_media_id("invalid", "123456");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_media_id() {
        let result = DocumentMessage::from_media_id("+1234567890", "invalid_id");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_url() {
        let result = DocumentMessage::from_url("+1234567890", "not-a-url");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_invalid_caption() {
        let long_caption = "x".repeat(1025); // Over 1024 character limit
        let result = DocumentMessage::from_media_id("+1234567890", "123456")
            .unwrap()
            .with_caption(&long_caption);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_document_file_validation() {
        // Valid document formats
        assert!(DocumentMessage::validate_document_file("application/pdf", 1024 * 1024).is_ok());
        assert!(DocumentMessage::validate_document_file("text/plain", 5 * 1024 * 1024).is_ok());
        assert!(DocumentMessage::validate_document_file("application/msword", 10 * 1024 * 1024).is_ok());
        
        // Invalid MIME type
        assert!(DocumentMessage::validate_document_file("application/zip", 1024).is_err());
        
        // File too large (over 100MB)
        assert!(DocumentMessage::validate_document_file("application/pdf", 101 * 1024 * 1024).is_err());
    }
    
    #[test]
    fn test_supported_formats() {
        let formats = DocumentMessage::supported_formats();
        assert!(formats.contains(&"application/pdf"));
        assert!(formats.contains(&"text/plain"));
        assert!(formats.contains(&"application/msword"));
        assert!(formats.contains(&"application/vnd.ms-excel"));
    }
    
    #[test]
    fn test_max_file_size() {
        assert_eq!(DocumentMessage::max_file_size(), 100 * 1024 * 1024);
    }
}
