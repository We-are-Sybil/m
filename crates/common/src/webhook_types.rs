use serde::{Deserialize, Serialize};

// Basic message types from webhook
#[derive(Deserialize, Debug, Clone)]
pub struct TextMessage {
    pub body: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReactionMessage {
    pub message_id: String,
    pub emoji: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaMessage {
    pub id: Option<String>,
    pub mime_type: String,
    pub sha256: String,
    pub caption: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LocationMessage {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

// Contact message types
#[derive(Deserialize, Debug, Clone)]
pub struct ContactMessage {
    pub addresses: Option<Vec<ContactAddress>>,
    pub birthday: Option<String>,
    pub emails: Option<Vec<ContactEmail>>,
    pub name: ContactName,
    pub org: Option<ContactOrg>,
    pub phones: Option<Vec<ContactPhone>>,
    pub urls: Option<Vec<ContactUrl>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactAddress {
    pub city: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub state: Option<String>,
    pub street: Option<String>,
    #[serde(rename = "type")]
    pub address_type: Option<String>,
    pub zip: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactEmail {
    pub email: String,
    #[serde(rename = "type")]
    pub email_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactName {
    pub formatted_name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub middle_name: Option<String>,
    pub suffix: Option<String>,
    pub prefix: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactOrg {
    pub company: Option<String>,
    pub department: Option<String>,
    pub title: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactPhone {
    pub phone: String,
    pub wa_id: Option<String>,
    #[serde(rename = "type")]
    pub phone_type: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ContactUrl {
    pub url: String,
    #[serde(rename = "type")]
    pub url_type: Option<String>,
}

// Interactive message types
#[derive(Deserialize, Debug, Clone)]
pub struct InteractiveMessage {
    #[serde(rename = "type")]
    pub interactive_type: String,
    pub button_reply: Option<ButtonReply>,
    pub list_reply: Option<ListReply>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListReply {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
}

// Referral and error types
#[derive(Deserialize, Debug, Clone)]
pub struct ReferralMessage {
    pub source_url: String,
    pub source_id: String,
    pub source_type: String,
    pub headline: Option<String>,
    pub body: Option<String>,
    pub media_type: Option<String>,
    pub image_url: Option<String>,
    pub video_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub ctwa_clid: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MessageError {
    pub code: u32,
    pub title: String,
    pub description: String,
}

// Message type enum
#[derive(Debug)]
pub enum WebhookMessageType {
    Text(TextMessage),
    Reaction(ReactionMessage),
    Image(MediaMessage),
    Sticker(MediaMessage),
    Location(LocationMessage),
    Contact(Vec<ContactMessage>),
    Interactive(InteractiveMessage),
    Referral(ReferralMessage),
    Unknown(Vec<MessageError>),
}
