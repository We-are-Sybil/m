use common::{
    EventBus, EventBusError, MessageReceived, InteractionReceived, MessageFailed,
    MessageType, MessageContent, InteractionType, InteractionSelection, FailureType,
    WebhookMessageType, ContactMessage, LocationMessage, TextMessage, MediaMessage,
    ReactionMessage, InteractiveMessage, ReferralMessage, MessageError,
    KafkaEventBus,
};
use std::{
    collections::HashMap,
    sync::Arc,
};
use tracing::{debug, error, info, warn};

/// Handles transformation of WhatsApp webhook payloads into clean domain events
///
/// This service acts as the bridge between WhatsApp's complex webhook format
/// and our simplified event-driven architecture. It transforms raw webhook
/// data into business-focused events that other services can easily consume.
pub struct WebhookEventPublisher {
    
    /// Event bus for publishing events
    event_bus: Arc<KafkaEventBus>,
}

impl WebhookEventPublisher {
    /// Create a new webhook event publisher with enhanced event bus
    /// 
    /// Takes an enhanced event bus implementation that provides automatic
    /// retry logic, dead letter queue support, and reliable event delivery.
    pub fn new(event_bus: Arc<KafkaEventBus>) -> Self {
        info!("🔧 Initializing webhook event publisher with enhanced event bus");
        Self { event_bus }
    }
    
    /// Process a WhatsApp message and publish appropriate domain events
    /// 
    /// This is the main entry point for webhook processing. It takes the
    /// raw message data from WhatsApp and transforms it into one or more
    /// domain events that represent what actually happened from a business
    /// perspective. The enhanced event bus handles all retry logic and
    /// failure scenarios automatically.
    pub async fn process_message(
        &self,
        message_id: String,
        from_phone: String,
        timestamp: String,
        webhook_message_type: Option<WebhookMessageType>,
        context_message_id: Option<String>,
    ) -> Result<(), EventBusError> {
        debug!("📨 Processing message {} from {} with enhanced event publishing", message_id, from_phone);
        
        // Parse the timestamp from WhatsApp format
        let received_at = self.parse_timestamp(&timestamp)?;
        
        // Create metadata for additional context
        let mut metadata = HashMap::new();
        if let Some(context_id) = context_message_id {
            metadata.insert("context_message_id".to_string(), context_id);
        }
        // Add processing metadata for tracing
        metadata.insert("processed_by".to_string(), "webhook_event_publisher".to_string());
        metadata.insert("processing_timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        
        match webhook_message_type {
            Some(msg_type) => {
                match msg_type {
                    // Handle regular messages (text, media, location, etc.)
                    WebhookMessageType::Text(text) => {
                        self.publish_text_message(message_id, from_phone, text, received_at, metadata).await
                    }
                    WebhookMessageType::Image(media) => {
                        self.publish_media_message(message_id, from_phone, media, MessageType::Image, received_at, metadata).await
                    }
                    WebhookMessageType::Sticker(media) => {
                        self.publish_media_message(message_id, from_phone, media, MessageType::Sticker, received_at, metadata).await
                    }
                    WebhookMessageType::Location(location) => {
                        self.publish_location_message(message_id, from_phone, location, received_at, metadata).await
                    }
                    WebhookMessageType::Contact(contacts) => {
                        self.publish_contact_message(message_id, from_phone, contacts, received_at, metadata).await
                    }
                    
                    // Handle interactive responses (buttons, lists)
                    WebhookMessageType::Interactive(interactive) => {
                        self.publish_interaction(message_id, from_phone, interactive, received_at).await
                    }
                    
                    // Handle other message types
                    WebhookMessageType::Reaction(reaction) => {
                        self.publish_reaction_message(message_id, from_phone, reaction, received_at, metadata).await
                    }
                    WebhookMessageType::Referral(referral) => {
                        self.publish_referral_message(message_id, from_phone, referral, received_at, metadata).await
                    }
                    
                    // Handle errors and unknown message types
                    WebhookMessageType::Unknown(errors) => {
                        self.publish_failure_message(message_id, from_phone, errors, received_at).await
                    }
                }
            }
            None => {
                warn!("🤷 Received message {} with no recognizable type", message_id);
                self.publish_unknown_message_failure(message_id, from_phone, received_at).await
            }
        }
    }
    
    /// Publish a text message event using the enhanced event bus
    async fn publish_text_message(
        &self,
        message_id: String,
        from_phone: String,
        text: TextMessage,
        received_at: chrono::DateTime<chrono::Utc>,
        metadata: HashMap<String, String>,
    ) -> Result<(), EventBusError> {
        let event = MessageReceived {
            message_id: message_id.clone(),
            from_phone,
            message_type: MessageType::Text,
            content: MessageContent::Text {
                body: text.body,
            },
            received_at,
            metadata,
        };
        
        debug!("📤 Publishing text message event for message {}", message_id);
        // The enhanced event bus automatically handles retries and dead letter queues
        self.event_bus.publish(event).await
    }
    
    /// Publish a media message event (image, audio, video, document) using enhanced event bus
    async fn publish_media_message(
        &self,
        message_id: String,
        from_phone: String,
        media: MediaMessage,
        message_type: MessageType,
        received_at: chrono::DateTime<chrono::Utc>,
        metadata: HashMap<String, String>,
    ) -> Result<(), EventBusError> {
        let event = MessageReceived {
            message_id: message_id.clone(),
            from_phone,
            message_type,
            content: MessageContent::Media {
                media_id: media.id.unwrap_or_else(|| "unknown".to_string()),
                caption: media.caption,
                mime_type: media.mime_type,
            },
            received_at,
            metadata,
        };
        
        debug!("📤 Publishing media message event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish a location message event using enhanced event bus
    async fn publish_location_message(
        &self,
        message_id: String,
        from_phone: String,
        location: LocationMessage,
        received_at: chrono::DateTime<chrono::Utc>,
        metadata: HashMap<String, String>,
    ) -> Result<(), EventBusError> {
        let event = MessageReceived {
            message_id: message_id.clone(),
            from_phone,
            message_type: MessageType::Location,
            content: MessageContent::Location {
                latitude: location.latitude,
                longitude: location.longitude,
                name: location.name,
                address: location.address,
            },
            received_at,
            metadata,
        };
        
        debug!("📤 Publishing location message event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish a contact message event using enhanced event bus
    async fn publish_contact_message(
        &self,
        message_id: String,
        from_phone: String,
        contacts: Vec<ContactMessage>,
        received_at: chrono::DateTime<chrono::Utc>,
        metadata: HashMap<String, String>,
    ) -> Result<(), EventBusError> {
        // For simplicity, we'll take the first contact if multiple are provided
        let contact = contacts.into_iter().next().unwrap_or_else(|| ContactMessage {
            name: common::ContactName {
                formatted_name: Some("Unknown Contact".to_string()),
                first_name: None,
                last_name: None,
                middle_name: None,
                suffix: None,
                prefix: None,
            },
            phones: None,
            emails: None,
            addresses: None,
            birthday: None,
            org: None,
            urls: None,
        });
        
        let name = contact.name.formatted_name
            .or_else(|| {
                let first = contact.name.first_name.unwrap_or_default();
                let last = contact.name.last_name.unwrap_or_default();
                if first.is_empty() && last.is_empty() {
                    None
                } else {
                    Some(format!("{} {}", first, last).trim().to_string())
                }
            })
            .unwrap_or_else(|| "Unknown Contact".to_string());
        
        let phone_number = contact.phones
            .and_then(|phones| phones.into_iter().next())
            .map(|p| p.phone)
            .unwrap_or_else(|| "Unknown Phone".to_string());
        
        let email = contact.emails
            .and_then(|emails| emails.into_iter().next())
            .map(|e| e.email);
        
        let event = MessageReceived {
            message_id: message_id.clone(),
            from_phone,
            message_type: MessageType::Contact,
            content: MessageContent::Contact {
                name,
                phone_number,
                email,
            },
            received_at,
            metadata,
        };
        
        debug!("📤 Publishing contact message event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish an interaction event (button click, list selection) using enhanced event bus
    async fn publish_interaction(
        &self,
        message_id: String,
        from_phone: String,
        interactive: InteractiveMessage,
        received_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), EventBusError> {
        let (interaction_type, selection) = match interactive.interactive_type.as_str() {
            "button_reply" => {
                if let Some(button_reply) = interactive.button_reply {
                    (
                        InteractionType::ButtonReply,
                        InteractionSelection::Button {
                            id: button_reply.id,
                            title: button_reply.title,
                        }
                    )
                } else {
                    warn!("🚨 Button reply without button data for message {}", message_id);
                    return self.publish_interaction_failure(message_id, from_phone, received_at).await;
                }
            }
            "list_reply" => {
                if let Some(list_reply) = interactive.list_reply {
                    (
                        InteractionType::ListReply,
                        InteractionSelection::List {
                            id: list_reply.id,
                            title: list_reply.title,
                            description: list_reply.description,
                        }
                    )
                } else {
                    warn!("🚨 List reply without list data for message {}", message_id);
                    return self.publish_interaction_failure(message_id, from_phone, received_at).await;
                }
            }
            _ => {
                warn!("🚨 Unknown interaction type: {} for message {}", interactive.interactive_type, message_id);
                return self.publish_interaction_failure(message_id, from_phone, received_at).await;
            }
        };
        
        let event = InteractionReceived {
            original_message_id: message_id.clone(), // Note: this should be the ID of the message with buttons
            from_phone,
            interaction_type,
            selection,
            received_at,
        };
        
        debug!("📤 Publishing interaction event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish a reaction message (for now, treat as a special text message)
    async fn publish_reaction_message(
        &self,
        message_id: String,
        from_phone: String,
        reaction: ReactionMessage,
        received_at: chrono::DateTime<chrono::Utc>,
        mut metadata: HashMap<String, String>,
    ) -> Result<(), EventBusError> {
        metadata.insert("reaction_to_message".to_string(), reaction.message_id);
        metadata.insert("message_type".to_string(), "reaction".to_string());
        
        let event = MessageReceived {
            message_id: message_id.clone(),
            from_phone,
            message_type: MessageType::Text,
            content: MessageContent::Text {
                body: format!("Reacted with: {}", reaction.emoji),
            },
            received_at,
            metadata,
        };
        
        debug!("📤 Publishing reaction as text message event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish a referral message (from ads, etc.)
    async fn publish_referral_message(
        &self,
        message_id: String,
        from_phone: String,
        referral: ReferralMessage,
        received_at: chrono::DateTime<chrono::Utc>,
        mut metadata: HashMap<String, String>,
    ) -> Result<(), EventBusError> {
        metadata.insert("referral_source_url".to_string(), referral.source_url);
        metadata.insert("referral_source_type".to_string(), referral.source_type);
        metadata.insert("message_type".to_string(), "referral".to_string());
        if let Some(headline) = referral.headline {
            metadata.insert("referral_headline".to_string(), headline);
        }
        
        let body = referral.body.unwrap_or_else(|| "User came from referral".to_string());
        
        let event = MessageReceived {
            message_id: message_id.clone(),
            from_phone,
            message_type: MessageType::Text,
            content: MessageContent::Text { body },
            received_at,
            metadata,
        };
        
        debug!("📤 Publishing referral as text message event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish a failure event when message processing fails
    async fn publish_failure_message(
        &self,
        message_id: String,
        from_phone: String,
        errors: Vec<MessageError>,
        received_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), EventBusError> {
        let error_details = if errors.is_empty() {
            "Unknown error occurred".to_string()
        } else {
            errors.into_iter()
                .map(|e| format!("{}: {}", e.title, e.description))
                .collect::<Vec<_>>()
                .join("; ")
        };
        
        let event = MessageFailed {
            message_id: message_id.clone(),
            phone: from_phone,
            failure_type: FailureType::ValidationError,
            error_details,
            attempt_count: 1,
            failed_at: received_at,
        };
        
        error!("📤 Publishing message failure event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish failure when interaction processing fails
    async fn publish_interaction_failure(
        &self,
        message_id: String,
        from_phone: String,
        received_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), EventBusError> {
        let event = MessageFailed {
            message_id: message_id.clone(),
            phone: from_phone,
            failure_type: FailureType::ValidationError,
            error_details: "Failed to parse interaction data".to_string(),
            attempt_count: 1,
            failed_at: received_at,
        };
        
        error!("📤 Publishing interaction failure event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Publish failure when message type is unknown
    async fn publish_unknown_message_failure(
        &self,
        message_id: String,
        from_phone: String,
        received_at: chrono::DateTime<chrono::Utc>,
    ) -> Result<(), EventBusError> {
        let event = MessageFailed {
            message_id: message_id.clone(),
            phone: from_phone,
            failure_type: FailureType::UnknownError,
            error_details: "Unknown or unsupported message type".to_string(),
            attempt_count: 1,
            failed_at: received_at,
        };
        
        warn!("📤 Publishing unknown message failure event for message {}", message_id);
        self.event_bus.publish(event).await
    }
    
    /// Parse WhatsApp timestamp format into chrono DateTime
    fn parse_timestamp(&self, timestamp: &str) -> Result<chrono::DateTime<chrono::Utc>, EventBusError> {
        // WhatsApp sends Unix timestamps as strings
        let unix_timestamp = timestamp.parse::<i64>()
            .map_err(|_| EventBusError::SerializationError(
                format!("Invalid timestamp format: {}", timestamp)
            ))?;
        
        chrono::DateTime::from_timestamp(unix_timestamp, 0)
            .ok_or_else(|| EventBusError::SerializationError(
                format!("Invalid Unix timestamp: {}", unix_timestamp)
            ))
    }
}
