pub mod errors;
pub mod events;
pub mod message_types;
pub mod webhook_types;
pub mod message_bus;


pub use errors::*;
pub use events::*;
pub use message_types::*;
pub use webhook_types::*;
pub use message_bus::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_event_envelope_creation() {
        let message = MessageReceived {
            message_id: "test-123".to_string(),
            from_phone: "+1234567890".to_string(),
            message_type: "text".to_string(),
            content: "Hello world".to_string(),
            received_at: chrono::Utc::now(),
        };
        
        let envelope = EventEnvelope::new(message.clone());
        
        assert_eq!(envelope.data.message_id, "test-123");
        assert_eq!(envelope.attempt_count, 0);
        assert_eq!(envelope.max_attempts, 3);
        assert!(!envelope.should_dead_letter());
        assert_eq!(envelope.version, "1.0");
    }
    
    #[test]
    fn test_dead_letter_logic() {
        let message = MessageReceived {
            message_id: "test-456".to_string(),
            from_phone: "+1234567890".to_string(),
            message_type: "text".to_string(),
            content: "Hello world".to_string(),
            received_at: chrono::Utc::now(),
        };
        
        let mut envelope = EventEnvelope::new(message);
        
        // Let's add debugging to see what's actually happening
        println!("Initial state: attempt_count={}, max_attempts={}", envelope.attempt_count, envelope.max_attempts);
        
        // First attempt
        envelope.increment_attempt();
        println!("After 1st attempt: attempt_count={}, should_dead_letter={}", envelope.attempt_count, envelope.should_dead_letter());
        assert!(!envelope.should_dead_letter());
        
        // Second attempt
        envelope.increment_attempt();
        println!("After 2nd attempt: attempt_count={}, should_dead_letter={}", envelope.attempt_count, envelope.should_dead_letter());
        assert!(!envelope.should_dead_letter());
        
        // Third attempt - this should trigger dead lettering
        envelope.increment_attempt();
        println!("After 3rd attempt: attempt_count={}, should_dead_letter={}", envelope.attempt_count, envelope.should_dead_letter());
        assert!(envelope.should_dead_letter(), 
            "Expected dead letter after 3 attempts, but got attempt_count={}, max_attempts={}", 
            envelope.attempt_count, envelope.max_attempts);
    }
    
    // Let's also add a test to verify our understanding of the logic
    #[test]
    fn test_dead_letter_edge_cases() {
        let message = MessageReceived {
            message_id: "test-edge".to_string(),
            from_phone: "+1234567890".to_string(),
            message_type: "text".to_string(),
            content: "Edge case test".to_string(),
            received_at: chrono::Utc::now(),
        };
        
        let mut envelope = EventEnvelope::new(message);
        
        // Should not dead letter initially
        assert!(!envelope.should_dead_letter());
        
        // Should not dead letter after 1 attempt
        envelope.increment_attempt();
        assert_eq!(envelope.attempt_count, 1);
        assert!(!envelope.should_dead_letter());
        
        // Should not dead letter after 2 attempts  
        envelope.increment_attempt();
        assert_eq!(envelope.attempt_count, 2);
        assert!(!envelope.should_dead_letter());
        
        // Should dead letter after 3 attempts (equals max_attempts)
        envelope.increment_attempt();
        assert_eq!(envelope.attempt_count, 3);
        assert_eq!(envelope.max_attempts, 3);
        assert!(envelope.should_dead_letter());
        
        // Should still dead letter after more attempts
        envelope.increment_attempt();
        assert_eq!(envelope.attempt_count, 4);
        assert!(envelope.should_dead_letter());
    }
}
