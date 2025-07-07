use crate::{
    errors::WhatsAppResult,
    client::message_types::DocumentMessage,
};

/// Builder for creating document messages with fluent interface
/// 
/// This builder provides a discoverable way to create document messages
/// with rich metadata support. Documents are perfect for sharing files
/// like PDFs, Word documents, spreadsheets, and presentations with
/// descriptive context.
/// 
/// Documents support both captions (explaining the content) and filenames
/// (helping users understand the file type and save it appropriately).
/// 
/// # Example
/// ```
/// # use whatsapp_client::client::builders::DocumentMessageBuilder;
/// // Professional document sharing
/// let message = DocumentMessageBuilder::new()
///     .to("+1234567890")
///     .media_id("1013859600285441")
///     .caption("Q4 financial report with growth analysis")
///     .filename("Q4_2024_Financial_Report.pdf")
///     .build()?;
/// 
/// // Simple document without metadata
/// let message = DocumentMessageBuilder::new()
///     .to("+1234567890")
///     .media_url("https://example.com/manual.pdf")
///     .build()?;
/// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
/// ```
#[derive(Debug, Default)]
pub struct DocumentMessageBuilder {
    to: Option<String>,
    media_id: Option<String>,
    media_url: Option<String>,
    caption: Option<String>,
    filename: Option<String>,
}

impl DocumentMessageBuilder {
    /// Create a new document message builder
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
    
    /// Set the media ID for uploaded document (recommended approach)
    /// 
    /// Use this when you've uploaded the document to WhatsApp's media servers.
    /// This approach offers better performance, reliability, and preserves
    /// the original file quality without additional compression.
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
    
    /// Set the URL for hosted document (not recommended for production)
    /// 
    /// Use this for development/testing or when you can't upload to WhatsApp.
    /// The document must be publicly accessible via HTTPS. WhatsApp will
    /// download the file, which can be slow for large documents.
    /// 
    /// # Arguments
    /// * `url` - HTTPS URL to the document file
    /// 
    /// # Supported formats
    /// PDF, Word (.doc/.docx), Excel (.xls/.xlsx), PowerPoint (.ppt/.pptx), plain text
    pub fn media_url(mut self, url: &str) -> Self {
        // Only set URL if no media ID is already set
        if self.media_id.is_none() {
            self.media_url = Some(url.to_string());
        }
        self
    }
    
    /// Add a caption explaining the document content
    /// 
    /// Captions help recipients understand what the document contains
    /// before downloading. They're especially valuable for business
    /// communications where context matters.
    /// 
    /// # Arguments
    /// * `text` - Caption text (up to 1024 characters)
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::DocumentMessageBuilder;
    /// let message = DocumentMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .media_id("123456")
    ///     .caption("Updated pricing guide for Q1 2025 - includes new product tiers")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn caption(mut self, text: &str) -> Self {
        self.caption = Some(text.to_string());
        self
    }
    
    /// Set the filename for the document
    /// 
    /// Filenames help users understand what type of file they're receiving
    /// and how to save it. This is especially important when using media IDs
    /// where the original filename might not be preserved.
    /// 
    /// # Arguments
    /// * `name` - Filename with appropriate extension
    /// 
    /// # Best Practices
    /// - Include the file extension (.pdf, .docx, etc.)
    /// - Use descriptive names that indicate content
    /// - Avoid special characters that might cause issues
    /// 
    /// # Example
    /// ```
    /// # use whatsapp_client::client::builders::DocumentMessageBuilder;
    /// let message = DocumentMessageBuilder::new()
    ///     .to("+1234567890")
    ///     .media_id("123456")
    ///     .filename("Employee_Handbook_2025.pdf")
    ///     .build()?;
    /// # Ok::<(), whatsapp_client::errors::WhatsAppError>(())
    /// ```
    pub fn filename(mut self, name: &str) -> Self {
        self.filename = Some(name.to_string());
        self
    }
    
    /// Remove any previously set caption
    /// 
    /// Useful for conditional caption logic where you might want to
    /// clear a previously set caption based on business rules.
    pub fn without_caption(mut self) -> Self {
        self.caption = None;
        self
    }
    
    /// Remove any previously set filename
    /// 
    /// Useful for conditional filename logic or when you want to
    /// let WhatsApp handle the filename automatically.
    pub fn without_filename(mut self) -> Self {
        self.filename = None;
        self
    }
    
    /// Build the document message
    /// 
    /// This validates all the configuration and creates the final DocumentMessage.
    /// Returns an error if required fields are missing or invalid.
    /// 
    /// # Validation Process
    /// 1. Recipient phone number must be set and valid E.164 format
    /// 2. Either media_id OR media_url must be set (media_id preferred)
    /// 3. Caption (if provided) must be 1024 characters or less
    /// 4. All WhatsApp validation rules are applied (file size ≤100MB, supported formats)
    /// 
    /// # Error Handling
    /// Returns detailed error messages to guide developers toward solutions
    pub fn build(self) -> WhatsAppResult<DocumentMessage> {
        let to = self.to.ok_or_else(|| {
            crate::errors::WhatsAppError::InvalidMessageContent(
                "Recipient phone number is required for document messages".to_string()
            )
        })?;
        
        // Create the base message using the appropriate method
        let mut message = match (self.media_id, self.media_url) {
            (Some(id), _) => {
                // Media ID takes precedence (recommended approach)
                DocumentMessage::from_media_id(&to, &id)?
            },
            (None, Some(url)) => {
                // Fall back to URL approach
                DocumentMessage::from_url(&to, &url)?
            },
            (None, None) => {
                return Err(crate::errors::WhatsAppError::InvalidMessageContent(
                    "Either media_id or media_url must be provided for document messages".to_string()
                ));
            }
        };
        
        // Add caption if provided (this can fail validation)
        if let Some(caption_text) = self.caption {
            message = message.with_caption(&caption_text)?;
        }
        
        // Add filename if provided (this doesn't fail, just sets metadata)
        if let Some(filename_text) = self.filename {
            message = message.with_filename(&filename_text);
        }
        
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_document_message_minimal() {
        let message = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_id("1013859600285441")
            .build()
            .unwrap();
        
        assert_eq!(message.recipient(), "+1234567890");
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.caption(), None);
        assert_eq!(message.filename(), None);
        assert!(message.uses_uploaded_media());
    }
    
    #[test]
    fn test_document_message_with_full_metadata() {
        let message = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_id("1013859600285441")
            .caption("Q4 financial report with detailed analysis")
            .filename("Q4_2024_Financial_Report.pdf")
            .build()
            .unwrap();
        
        assert_eq!(message.caption(), Some("Q4 financial report with detailed analysis"));
        assert_eq!(message.filename(), Some("Q4_2024_Financial_Report.pdf"));
    }
    
    #[test]
    fn test_document_message_with_url() {
        let message = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/document.pdf")
            .caption("External document")
            .filename("external_doc.pdf")
            .build()
            .unwrap();
        
        assert_eq!(message.media_url(), Some("https://example.com/document.pdf"));
        assert!(!message.uses_uploaded_media());
    }
    
    #[test]
    fn test_metadata_removal() {
        let message = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption("First caption")
            .filename("first.pdf")
            .without_caption()
            .without_filename()
            .build()
            .unwrap();
        
        assert_eq!(message.caption(), None);
        assert_eq!(message.filename(), None);
    }
    
    #[test]
    fn test_media_id_precedence_with_metadata() {
        let message = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_url("https://example.com/doc.pdf")
            .caption("Test document")
            .filename("test.pdf")
            .media_id("1013859600285441") // This should override the URL
            .build()
            .unwrap();
        
        assert_eq!(message.media_id(), Some("1013859600285441"));
        assert_eq!(message.media_url(), None);
        assert_eq!(message.caption(), Some("Test document"));
        assert_eq!(message.filename(), Some("test.pdf"));
    }
    
    #[test]
    fn test_builder_method_chaining_order() {
        // Test various chaining orders to ensure flexibility
        let message1 = DocumentMessageBuilder::new()
            .filename("report.pdf")
            .caption("Financial report")
            .media_id("123456")
            .to("+1234567890")
            .build()
            .unwrap();
        
        let message2 = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption("Financial report")
            .filename("report.pdf")
            .build()
            .unwrap();
        
        // Both should produce equivalent results
        assert_eq!(message1.recipient(), message2.recipient());
        assert_eq!(message1.media_id(), message2.media_id());
        assert_eq!(message1.caption(), message2.caption());
        assert_eq!(message1.filename(), message2.filename());
    }
    
    #[test]
    fn test_conditional_metadata_building() {
        // Simulate business logic that conditionally adds metadata
        let is_important = true;
        let has_filename = false;
        
        let mut builder = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456");
        
        if is_important {
            builder = builder.caption("IMPORTANT: Please review urgently");
        }
        
        if has_filename {
            builder = builder.filename("important.pdf");
        }
        
        let message = builder.build().unwrap();
        assert_eq!(message.caption(), Some("IMPORTANT: Please review urgently"));
        assert_eq!(message.filename(), None);
    }
    
    #[test]
    fn test_missing_recipient_error() {
        let result = DocumentMessageBuilder::new()
            .media_id("123456")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Recipient phone number is required"));
    }
    
    #[test]
    fn test_missing_media_error() {
        let result = DocumentMessageBuilder::new()
            .to("+1234567890")
            .caption("Test document")
            .build();
        
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Either media_id or media_url must be provided"));
    }
    
    #[test]
    fn test_invalid_caption_length() {
        let long_caption = "x".repeat(1025); // Over 1024 character limit
        let result = DocumentMessageBuilder::new()
            .to("+1234567890")
            .media_id("123456")
            .caption(&long_caption)
            .build();
        
        assert!(result.is_err());
    }
}
