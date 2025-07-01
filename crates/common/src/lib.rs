pub mod errors;
pub mod events;
pub mod message_bus;
pub mod kafka_bus;
// Keep webhook_types for now - we need these to parse incoming WhatsApp webhooks
pub mod webhook_types;

// Re-export the core types that other crates will use
pub use errors::*;
pub use events::*;
pub use message_bus::*;
pub use webhook_types::*;
pub use kafka_bus::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    /// Test that our domain events can be properly created and serialized
    #[test]
    fn test_message_received_event() {
        let message = MessageReceived {
            message_id: "wamid.123".to_string(),
            from_phone: "+1234567890".to_string(),
            message_type: MessageType::Text,
            content: MessageContent::Text {
                body: "Hello, world!".to_string(),
            },
            received_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };
        
        let envelope = EventEnvelope::new(message.clone());
        
        assert_eq!(envelope.data.message_id, "wamid.123");
        assert_eq!(envelope.version, "1.0");
        assert_eq!(envelope.data.partition_key(), Some("+1234567890".to_string()));
        
        // Test serialization
        let json = serde_json::to_string(&envelope).expect("Should serialize");
        let deserialized: EventEnvelope<MessageReceived> = 
            serde_json::from_str(&json).expect("Should deserialize");
        
        assert_eq!(deserialized.data.message_id, "wamid.123");
    }
    
    /// Test that interaction events work properly
    #[test]
    fn test_interaction_event() {
        let interaction = InteractionReceived {
            original_message_id: "wamid.456".to_string(),
            from_phone: "+1234567890".to_string(),
            interaction_type: InteractionType::ButtonReply,
            selection: InteractionSelection::Button {
                id: "help_button".to_string(),
                title: "Get Help".to_string(),
            },
            received_at: chrono::Utc::now(),
        };
        
        let envelope = EventEnvelope::new(interaction.clone());
        
        assert_eq!(envelope.data.original_message_id, "wamid.456");
        assert_eq!(envelope.data.partition_key(), Some("+1234567890".to_string()));
    }
    
    /// Test response event creation
    #[test]
    fn test_response_ready_event() {
        let response = ResponseReady {
            original_message_id: "wamid.789".to_string(),
            to_phone: "+1234567890".to_string(),
            response_type: ResponseType::Interactive,
            content: ResponseContent::Interactive {
                body_text: "How can I help you?".to_string(),
                buttons: vec![
                    ResponseButton {
                        id: "option1".to_string(),
                        title: "Support".to_string(),
                    },
                    ResponseButton {
                        id: "option2".to_string(),
                        title: "Billing".to_string(),
                    },
                ],
            },
            generated_at: chrono::Utc::now(),
            priority: ResponsePriority::Normal,
        };
        
        let envelope = EventEnvelope::new(response.clone());
        
        assert_eq!(envelope.data.to_phone, "+1234567890");
        assert_eq!(envelope.data.partition_key(), Some("+1234567890".to_string()));
    }
    
    /// Test the dead letter queue logic
    #[test]
    fn test_dead_letter_logic() {
        let message = MessageReceived {
            message_id: "test-456".to_string(),
            from_phone: "+1234567890".to_string(),
            message_type: MessageType::Text,
            content: MessageContent::Text {
                body: "Test message".to_string(),
            },
            received_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };
        
        let mut envelope = EventEnvelope::new(message);
        
        // Should not dead letter initially
        assert!(!envelope.should_dead_letter());
        
        // Should not dead letter after 1-2 attempts
        envelope.increment_attempt();
        assert!(!envelope.should_dead_letter());
        envelope.increment_attempt();
        assert!(!envelope.should_dead_letter());
        
        // Should dead letter after 3rd attempt
        envelope.increment_attempt();
        assert!(envelope.should_dead_letter());
    }
}
