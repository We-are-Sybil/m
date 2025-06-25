use serde::Deserialize;
use super::message::*;

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
