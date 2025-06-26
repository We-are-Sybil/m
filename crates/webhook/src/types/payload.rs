use serde::Deserialize;
use common::{
    WebhookMessageType,
    TextMessage,
    ReactionMessage,
    MediaMessage,
    LocationMessage,
    ContactMessage,
    InteractiveMessage,
    ReferralMessage,
    MessageError,
};

#[derive(Deserialize, Debug)]
pub struct WebhookPayload {
    pub object: String,
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub id: String,
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Debug)]
pub struct Change {
    pub value: Value,
    pub field: String,
}

#[derive(Deserialize, Debug)]
pub struct Value {
    pub contacts: Option<Vec<Contact>>,
    pub messages: Option<Vec<Message>>,
    pub messaging_product: String,
    pub metadata: Option<Metadata>,
}

#[derive(Deserialize, Debug)]
pub struct Contact {
    pub profile: ContactProfile,
    pub wa_id: String,
}

#[derive(Deserialize, Debug)]
pub struct ContactProfile {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub timestamp: String,
    #[serde(rename = "type")]
    pub message_type: String,

    // Different message types (defined in message_types.rs)
    pub text: Option<TextMessage>,
    pub reaction: Option<ReactionMessage>,
    pub image: Option<MediaMessage>,
    pub sticker: Option<MediaMessage>,
    pub location: Option<LocationMessage>,
    pub contact: Option<Vec<ContactMessage>>,
    pub interactive: Option<InteractiveMessage>,
    pub referral: Option<ReferralMessage>,
    pub error: Option<Vec<MessageError>>,
    pub context: Option<MessageContext>,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub display_phone_number: Option<String>,
    pub phone_number_id: String,
}

// Message Context (used in incoming messages)
#[derive(Deserialize, Debug)]
pub struct MessageContext {
    pub message_id: String,
    pub from: Option<String>,
    pub id: Option<String>,
}




impl Message {
    pub fn get_message_type(&self) -> Option<WebhookMessageType> {
        match self.message_type.as_str() {
            "text" => self.text.as_ref().map(|t| WebhookMessageType::Text(t.clone())),
            "reaction" => self.reaction.as_ref().map(|r| WebhookMessageType::Reaction(r.clone())),
            "image" => self.image.as_ref().map(|i| WebhookMessageType::Image(i.clone())),
            "sticker" => self.sticker.as_ref().map(|s| WebhookMessageType::Sticker(s.clone())),
            "location" => self.location.as_ref().map(|l| WebhookMessageType::Location(l.clone())),
            "contact" => self.contact.clone().map(WebhookMessageType::Contact),
            "interactive" => self.interactive.clone().map(WebhookMessageType::Interactive),
            "referral" => self.referral.clone().map(WebhookMessageType::Referral),
            _ => self.error.clone().map(WebhookMessageType::Unknown).or_else(|| Some(WebhookMessageType::Unknown(vec![]))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{
        ContactPhone,
        ButtonReply,
    };

    fn create_base_message() -> Message {
        Message {
            id: "12345".to_string(),
            from: "1234567890".to_string(),
            timestamp: "2023-10-01T12:00:00Z".to_string(),
            message_type: "text".to_string(),
            text: None,
            reaction: None,
            image: None,
            sticker: None,
            location: None,
            contact: None,
            interactive: None,
            referral: None,
            error: None,
            context: None,
        }
    }

    #[test]
    fn test_webhook_get_message_type_text() {
        let mut message = create_base_message();
        message.message_type = "text".to_string();
        message.text = Some(TextMessage {
            body: "Hello, World!".to_string(),
        });

        let result = message.get_message_type();
        assert!(result.is_some());

        match result.unwrap() {
            WebhookMessageType::Text(text) => {
                assert_eq!(text.body, "Hello, World!");
            }
            _ => panic!("Expected a Text message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_text_missing_data() {
        let mut message = create_base_message();
        message.message_type = "text".to_string();
        message.text = None;

        let result = message.get_message_type();
        assert!(result.is_none());
    }

    #[test]
    fn test_webhook_get_message_type_reaction() {
        let mut message = create_base_message();
        message.message_type = "reaction".to_string();
        message.reaction = Some(ReactionMessage {
            message_id: "msg123".to_string(),
            emoji: "👍".to_string(),
        });

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Reaction(reaction) => {
                assert_eq!(reaction.message_id, "msg123");
                assert_eq!(reaction.emoji, "👍");
            }
            _ => panic!("Expected Reaction message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_image() {
        let mut message = create_base_message();
        message.message_type = "image".to_string();
        message.image = Some(MediaMessage {
            id: Some("img123".to_string()),
            mime_type: "image/jpeg".to_string(),
            sha256: "abc123".to_string(),
            caption: Some("Test image".to_string()),
        });

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Image(media) => {
                assert_eq!(media.id, Some("img123".to_string()));
                assert_eq!(media.mime_type, "image/jpeg");
                assert_eq!(media.sha256, "abc123");
                assert_eq!(media.caption, Some("Test image".to_string()));
            }
            _ => panic!("Expected Image message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_sticker() {
        let mut message = create_base_message();
        message.message_type = "sticker".to_string();
        message.sticker = Some(MediaMessage {
            id: Some("sticker123".to_string()),
            mime_type: "image/webp".to_string(),
            sha256: "def456".to_string(),
            caption: None,
        });

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Sticker(media) => {
                assert_eq!(media.id, Some("sticker123".to_string()));
                assert_eq!(media.mime_type, "image/webp");
                assert_eq!(media.sha256, "def456");
                assert_eq!(media.caption, None);
            }
            _ => panic!("Expected Sticker message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_location() {
        let mut message = create_base_message();
        message.message_type = "location".to_string();
        message.location = Some(LocationMessage {
            latitude: 37.7749,
            longitude: -122.4194,
            name: Some("San Francisco".to_string()),
            address: Some("San Francisco, CA".to_string()),
        });

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Location(location) => {
                assert_eq!(location.latitude, 37.7749);
                assert_eq!(location.longitude, -122.4194);
                assert_eq!(location.name, Some("San Francisco".to_string()));
                assert_eq!(location.address, Some("San Francisco, CA".to_string()));
            }
            _ => panic!("Expected Location message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_contact() {
        let mut message = create_base_message();
        message.message_type = "contact".to_string();
        message.contact = Some(vec![ContactMessage {
            addresses: None,
            birthday: None,
            emails: None,
            name: common::ContactName {
                formatted_name: Some("John Doe".to_string()),
                first_name: Some("John".to_string()),
                last_name: Some("Doe".to_string()),
                middle_name: None,
                suffix: None,
                prefix: None,
            },
            org: None,
            phones: Some(vec![ContactPhone {
                phone: "+1234567890".to_string(),
                wa_id: Some("1234567890".to_string()),
                phone_type: Some("MOBILE".to_string()),
            }]),
            urls: None,
        }]);

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Contact(contacts) => {
                assert_eq!(contacts.len(), 1);
                assert_eq!(contacts[0].name.formatted_name, Some("John Doe".to_string()));
                assert_eq!(contacts[0].name.first_name, Some("John".to_string()));
                assert_eq!(contacts[0].phones.as_ref().unwrap()[0].phone, "+1234567890");
            }
            _ => panic!("Expected Contact message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_interactive() {
        let mut message = create_base_message();
        message.message_type = "interactive".to_string();
        message.interactive = Some(InteractiveMessage {
            interactive_type: "button_reply".to_string(),
            button_reply: Some(ButtonReply {
                id: "btn1".to_string(),
                title: "Yes".to_string(),
            }),
            list_reply: None,
        });

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Interactive(interactive) => {
                assert_eq!(interactive.interactive_type, "button_reply");
                assert!(interactive.button_reply.is_some());
                assert_eq!(interactive.button_reply.unwrap().id, "btn1");
            }
            _ => panic!("Expected Interactive message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_referral() {
        let mut message = create_base_message();
        message.message_type = "referral".to_string();
        message.referral = Some(ReferralMessage {
            source_url: "https://example.com".to_string(),
            source_id: "ref123".to_string(),
            source_type: "ad".to_string(),
            headline: Some("Great Product".to_string()),
            body: Some("Check this out".to_string()),
            media_type: Some("image".to_string()),
            image_url: Some("https://example.com/image.jpg".to_string()),
            video_url: None,
            thumbnail_url: None,
            ctwa_clid: None,
        });

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Referral(referral) => {
                assert_eq!(referral.source_url, "https://example.com");
                assert_eq!(referral.source_id, "ref123");
                assert_eq!(referral.source_type, "ad");
                assert_eq!(referral.headline, Some("Great Product".to_string()));
            }
            _ => panic!("Expected Referral message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_unknown_with_error() {
        let mut message = create_base_message();
        message.message_type = "unknown_type".to_string();
        message.error = Some(vec![MessageError {
            code: 400,
            title: "Bad Request".to_string(),
            description: "Unknown message type".to_string(),
        }]);

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Unknown(errors) => {
                assert_eq!(errors.len(), 1);
                assert_eq!(errors[0].code, 400);
                assert_eq!(errors[0].title, "Bad Request");
            }
            _ => panic!("Expected Unknown message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_unknown_without_error() {
        let mut message = create_base_message();
        message.message_type = "unknown_type".to_string();
        message.error = None;

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Unknown(errors) => {
                assert_eq!(errors.len(), 0);
            }
            _ => panic!("Expected Unknown message type"),
        }
    }

    #[test]
    fn test_webhook_get_message_type_empty_string() {
        let mut message = create_base_message();
        message.message_type = "".to_string();

        let result = message.get_message_type();
        assert!(result.is_some());
        match result.unwrap() {
            WebhookMessageType::Unknown(errors) => {
                assert_eq!(errors.len(), 0);
            }
            _ => panic!("Expected Unknown message type"),
        }
    }
}
