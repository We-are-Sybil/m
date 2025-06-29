use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;
use std::future::Future;

pub trait Event: 
    Serialize 
    + for<'de> Deserialize<'de> 
    + Send 
    + Sync 
    + Clone 
    + fmt::Debug 
    + 'static 
{

    // Every event must declare which topic it belongs to.
    const TOPIC: &'static str;

    // Version helps evolve events over time without breaking
    // existing consumers.
    const VERSION: &'static str;

    fn partition_key(&self) -> Option<String> {
        None 
    }

    fn event_type(&self) -> &'static str {
        std::any::type_name::<Self>()
            .split("::")
            .last()
            .unwrap_or("UnknownEvent")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Event")]
pub struct EventEnvelope<T: Event> 
where 
    T: Event,
{
    pub event_id: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub event_type: String,
    pub version: String,
    pub data: T,
    pub metadata: std::collections::HashMap<String, String>,
    // Tracking for retry logic and dead letter queue decisions
    pub attempt_count: u32,
    pub max_attempts: u32,
}

impl<T> EventEnvelope<T>
where 
    T: Event,
{
    pub fn new(data: T) -> Self {
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            event_type: data.event_type().to_string(),
            version: T::VERSION.to_string(),
            data,
            metadata: std::collections::HashMap::new(),
            attempt_count: 0,
            max_attempts: 3, 
        }
    }

    pub fn increment_attempt(&mut self) {
        self.attempt_count += 1;
    }

    pub fn should_dead_letter(&self) -> bool {
        self.attempt_count >= self.max_attempts
    }
}

pub trait EventBus: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    /// Publishes an event to the event bus.
    fn publish<T>(&self, event: T) -> impl Future<Output = Result<(), Self::Error>> + Send + '_
        where 
            T: Event;

    /// Subscribes to events of a specific type with a handler function.
    fn subscribe<T, F>(&self, handler: F) -> impl Future<Output = Result<(), Self::Error>> + Send + '_
    where
        T: Event,
        F: Fn(EventEnvelope<T>) -> Result<(), Box<dyn Error + Send + Sync>> 
            + Send 
            + Sync 
            + 'static;

    fn health_check(&self) -> impl Future<Output = Result<(), Self::Error>> + Send + '_;
}

#[derive(Debug)]
pub enum EventBusError {
    PublishFailed(String),
    SubscriptionFailed(String),
    SerializationError(String),
    ConnectionError(String),
    TopicNotFound(String),
    ConfigError(String),
}

impl fmt::Display for EventBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventBusError::PublishFailed(msg) => write!(f, "Failed to publish event: {}", msg),
            EventBusError::SubscriptionFailed(msg) => write!(f, "Failed to subscribe: {}", msg),
            EventBusError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            EventBusError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            EventBusError::TopicNotFound(msg) => write!(f, "Topic not found: {}", msg),
            EventBusError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
        }
    }
}

impl Error for EventBusError {}
