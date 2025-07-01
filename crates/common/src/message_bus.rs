use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt,
};

/// Trait definin what makes a valid event in the system.
///
/// Wvents are the core communication mechanism between services.
/// Think of them as structured messages that carry business 
/// information from one system to another.
pub trait Event: 
    Serialize 
    + for<'de> Deserialize<'de> 
    + Send 
    + Sync 
    + Clone 
    + fmt::Debug 
    + 'static 
{

    /// Topic where the event should be published.
    /// Creating a clear mapping between events and Kafka topics.
    const TOPIC: &'static str;

    /// Version helps evolve events over time without breaking
    /// existing consumers.
    const VERSION: &'static str;

    /// Partition key determines which partition the event will be sent to.
    /// Events with the same key will be processed in order.
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

/// Envelope that wraps evets with metadata as they flow through the system.
/// 
/// It can be seen as a postal envelop and the event is the letter. In this case,
/// the envelope contains additional information such as the event ID, timestamp,
/// event type, version, and metadata that can be used for tracking and 
/// processing purposes.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Event")]
pub struct EventEnvelope<T: Event> 
where 
    T: Event,
{
    /// Unique identifier for tracking this specific event instance.
    pub event_id: String,
    /// Timestamp when the event was created.
    pub timestamp: chrono::DateTime<chrono::Utc>,
    /// Type of the event, useful for routing, processing, and debugging.
    pub event_type: String,
    /// Version of the event for schema evolution (backward compatibility).
    pub version: String,
    /// The actual event data, which implements the Event trait.
    pub data: T,
    /// Additional metadata (correlation IDs, tracing info, etc.).
    pub metadata: std::collections::HashMap<String, String>,
    /// How many times this event has been attempted to be processed.
    pub attempt_count: u32,
    /// Maximum attempts before sending to dead-letter queue.
    pub max_attempts: u32,
}

impl<T> EventEnvelope<T>
where 
    T: Event,
{
    /// Create a new event envelope with default retry settings.
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

    /// Create an envelope with custom retry settings.
    pub fn with_max_attempts(data: T, max_attempts: u32) -> Self {
        let mut envelope = Self::new(data);
        envelope.max_attempts = max_attempts;
        envelope
    }

    /// Record another processing attempt.
    pub fn increment_attempt(&mut self) {
        self.attempt_count += 1;
    }

    /// Should this event be sent to the dead-letter queue?
    pub fn should_dead_letter(&self) -> bool {
        self.attempt_count >= self.max_attempts
    }

    /// Add metadata to the envelope.
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Get the partition key for this envelope (from the wrapped event).
    pub fn partition_key(&self) -> Option<String> {
        self.data.partition_key()
    }

}

/// Result type for event processing handlers.
///
/// This allows handlers to indicate whether processing succeeded,
/// failed temporarily (should retry), or failed permanently.
#[derive(Debug)]
pub enum ProcessingResult {
    /// Event was processed successfully.
    Success,
    /// Processing failed but should be retried.
    RetryableError(String),
    /// Processing filed permanently (sent to dead-letter queue).
    PermanentError(String),
}

impl ProcessingResult {
    /// Create a retryable error result.
    pub fn retry(msg: impl Into<String>) -> Self {
        Self::RetryableError(msg.into())
    }

    /// Create a permanent error result.
    pub fn permanent_error(msg: impl Into<String>) -> Self {
        Self::PermanentError(msg.into())
    }
}

/// Configuration for event subscription behavior.
#[derive(Debug, Clone)]
pub struct SubscriptionConfig {
    /// Consumer group ID for this subscription.
    pub consumer_group: String,
    /// Maximum number of events to process in a single batch.
    pub max_batch_size: usize,
    /// Maximum time to wait for a batch to fill up
    pub batch_timeout_ms: u64,
    /// Whether to enable automatic offset commits
    pub auto_commit: bool,
    /// How often to commit offsets (if auto_commit is true)
    pub auto_commit_interval_ms: u64,

}

impl Default for SubscriptionConfig {
    fn default() -> Self {
        Self {
            consumer_group: "default-group".to_string(),
            max_batch_size: 100,
            batch_timeout_ms: 1000,
            auto_commit: true,
            auto_commit_interval_ms: 5000,
        }
    }
}

/// Main event bus abstraction for publishing and subscribing to events.
///
/// This trait defines the contract that all event bus implementations must follow.
/// It provides a clean interface that hides the complexity of the underlying
/// message system (Kafka, Redis, in-memory, etc...).
#[allow(async_fn_in_trait)]
pub trait EventBus: Send + Sync {
    type Error: Error + Send + Sync + 'static;

    /// Publishes an event to the event bus.
    async fn publish<T>(&self, event: T) -> Result<(), Self::Error>
    where
        T: Event;

    /// Publish a batch of events
    async fn publish_batch<T>(&self, events: Vec<T>) -> Result<(), Self::Error>
    where
        T: Event;

    /// Subscribes to events of a specific type with a handler function.
    async fn subscribe<T, F>(&self, config: SubscriptionConfig, handler: F) -> Result<(), Self::Error>
    where
        T: Event,
        F: Fn(EventEnvelope<T>) -> Result<ProcessingResult, Box<dyn Error + Send + Sync>> 
            + Send 
            + Sync 
            + 'static;

    /// Subscribe with batch processing for higher throughput
    ///
    /// This allows processing multiple events together, which can 
    /// be more efficient for scenarios like database batch updates 
    /// or API calls.
    async fn subscribe_batch<T, F>(&self, config: SubscriptionConfig, handler: F) -> Result<(), Self::Error>
    where
        T: Event,
        F: Fn(Vec<EventEnvelope<T>>) -> Result<Vec<ProcessingResult>, Box<dyn Error + Send + Sync>> 
            + Send 
            + Sync 
            + 'static;


    /// Checks if the event bus is healthy
    async fn health_check(&self) -> Result<(), Self::Error>;

    /// Gracefully yshut down the event bus
    ///
    /// This ensures that any pending messages are processed and 
    /// resources are cleanned up properly.
    async fn shutdown(&self) -> Result<(), Self::Error>;
}


#[derive(Debug)]
pub enum EventBusError {
    /// Failed to publish an event
    PublishFailed(String),
    /// Failed to start or manage a subscription
    SubscriptionFailed(String),
    /// Failed to serialize or deserialize an event
    SerializationError(String),
    /// Connection to the underlying messaging system failed
    ConnectionError(String),
    /// The requested topic does not exist
    TopicNotFound(String),
    /// Configuration is invalid or missing
    ConfigError(String),
    /// Consumer operation failed
    ConsumerError(String),
    /// Shutdown was requested or the system is shutting down
    ShutdownRequested,
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
            EventBusError::ConsumerError(msg) => write!(f, "Consumer error: {}", msg),
            EventBusError::ShutdownRequested => write!(f, "Shutdown was requested"),
        }
    }
}

impl Error for EventBusError {}
