use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct WebhookVerifyQuery {
    #[serde(rename = "hub.mode")]
    pub mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub challenge: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct WebhookPayload {
    pub entry: Vec<Entry>,
}

#[derive(Deserialize, Debug)]
pub struct Entry {
    pub changes: Vec<Change>,
}

#[derive(Deserialize, Debug)]
pub struct Change {
    pub value: Value,
}

#[derive(Deserialize, Debug)]
pub struct Value {
    pub messages: Option<Vec<Message>>,
    pub metadata: Option<Metadata>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub id: String,
    pub from: String,
    #[serde(rename = "type")]
    pub message_type: String,
    pub text: Option<TextMessage>,
}

#[derive(Deserialize, Debug)]
pub struct TextMessage {
    pub body: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    pub phone_number_id: String,
}

#[derive(Serialize, Debug)]
pub struct WhatsAppMessage {
    pub messaging_product: String,
    pub to: String,
    pub text: TextBody,
    pub context: Option<MessageContext>,
}

#[derive(Serialize, Debug)]
pub struct TextBody {
    pub body: String,
}

#[derive(Serialize, Debug)]
pub struct MessageContext {
    pub message_id: String,
}

#[derive(Serialize, Debug)]
pub struct MessageStatus {
    pub messaging_product: String,
    pub status: String,
    pub message_id: String,
}
