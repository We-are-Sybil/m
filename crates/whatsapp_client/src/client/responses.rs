use serde::Deserialize;

/// Standard response from WhatsApp when a message is sent successfully
/// 
/// This is the primary response structure returned by most message sending
/// operations. It contains contact information and message IDs that can
/// be used for tracking and correlation.
#[derive(Deserialize, Debug, Clone)]
pub struct WhatsAppMessageResponse {
    /// Always "whatsapp" for WhatsApp Business API
    pub messaging_product: String,
    /// Contact information for the recipient
    pub contacts: Vec<WhatsAppContact>,
    /// Information about the sent message(s)
    pub messages: Vec<WhatsAppMessage>,
}

impl WhatsAppMessageResponse {
    /// Get the primary message ID from the response
    /// 
    /// This is useful for tracking and logging purposes.
    /// Returns None if no messages are in the response (shouldn't happen normally).
    pub fn message_id(&self) -> Option<&str> {
        self.messages.first().map(|m| m.id.as_str())
    }
    
    /// Get the WhatsApp ID of the recipient
    /// 
    /// Returns the normalized WhatsApp user ID, which might be different
    /// from the phone number you sent to (due to WhatsApp's normalization).
    pub fn recipient_wa_id(&self) -> Option<&str> {
        self.contacts.first().map(|c| c.wa_id.as_str())
    }
    
    /// Check if the response indicates successful delivery
    /// 
    /// A successful response should have exactly one message and one contact.
    pub fn is_successful(&self) -> bool {
        !self.messages.is_empty() && !self.contacts.is_empty()
    }
}

/// Contact information in WhatsApp API response
/// 
/// This represents the recipient's contact information as WhatsApp sees it.
/// The wa_id is particularly important as it's WhatsApp's internal ID for the user.
#[derive(Deserialize, Debug, Clone)]
pub struct WhatsAppContact {
    /// The input phone number (as you sent it)
    pub input: String,
    /// WhatsApp's normalized ID for this user
    pub wa_id: String,
}

/// Message information in WhatsApp API response
/// 
/// This contains the unique identifier that WhatsApp assigns to your message.
/// This ID can be used for message status tracking and is included in delivery
/// receipts and read receipts if enabled.
#[derive(Deserialize, Debug, Clone)]
pub struct WhatsAppMessage {
    /// Unique message identifier assigned by WhatsApp
    pub id: String,
}

/// Response from media upload operations
/// 
/// When you upload media (images, documents, etc.) to WhatsApp, you get
/// this response containing the media ID that you can then use in messages.
#[derive(Deserialize, Debug, Clone)]
pub struct MediaUploadResponse {
    /// The ID of the uploaded media file
    pub id: String,
}

/// Response for webhook verification
/// 
/// This is used during the webhook setup process when WhatsApp verifies
/// that your webhook endpoint is valid and reachable.
#[derive(Deserialize, Debug, Clone)]
pub struct WebhookVerificationResponse {
    /// The challenge string that must be echoed back
    pub challenge: String,
}

/// Extended message response with additional metadata
/// 
/// Some API operations return additional information beyond the basic response.
/// This structure can accommodate those extended responses.
#[derive(Deserialize, Debug, Clone)]
pub struct ExtendedMessageResponse {
    /// Standard message response fields
    #[serde(flatten)]
    pub standard: WhatsAppMessageResponse,
    /// Additional metadata if present
    pub metadata: Option<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_message_response_parsing() {
        let response_json = json!({
            "messaging_product": "whatsapp",
            "contacts": [{
                "input": "+1234567890",
                "wa_id": "1234567890"
            }],
            "messages": [{
                "id": "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"
            }]
        });
        
        let response: WhatsAppMessageResponse = serde_json::from_value(response_json).unwrap();
        
        assert_eq!(response.messaging_product, "whatsapp");
        assert_eq!(response.contacts.len(), 1);
        assert_eq!(response.messages.len(), 1);
        assert_eq!(response.contacts[0].input, "+1234567890");
        assert_eq!(response.contacts[0].wa_id, "1234567890");
        assert_eq!(response.messages[0].id, "wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA");
        
        // Test convenience methods
        assert!(response.is_successful());
        assert_eq!(response.message_id(), Some("wamid.HBgLMTY0NjcwNDM1OTUVAgARGBI1RjQyNUE3NEYxMzAzMzQ5MkEA"));
        assert_eq!(response.recipient_wa_id(), Some("1234567890"));
    }
    
    #[test]
    fn test_media_upload_response_parsing() {
        let response_json = json!({
            "id": "1013859600285441"
        });
        
        let response: MediaUploadResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.id, "1013859600285441");
    }
    
    #[test]
    fn test_empty_response_handling() {
        let response_json = json!({
            "messaging_product": "whatsapp",
            "contacts": [],
            "messages": []
        });
        
        let response: WhatsAppMessageResponse = serde_json::from_value(response_json).unwrap();
        
        assert!(!response.is_successful());
        assert_eq!(response.message_id(), None);
        assert_eq!(response.recipient_wa_id(), None);
    }
}
