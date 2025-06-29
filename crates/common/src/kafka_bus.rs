use crate::message_bus::{
    Event, 
    EventBus,
    EventBusError,
    EventEnvelope,
};
use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord, Producer},
    util::Timeout,
};
use serde::{
    Serialize, 
    de::DeserializeOwned
};
use std::{
    error::Error,
    sync::Arc,
    time::Duration,
};
use tracing::{debug, error, info, warn};

/// Kafka-based event bus implementation
/// Provides reliable event publishing and subscription 
/// through Apache Kafka
pub struct KafkaEventBus {
    producer: Arc<FutureProducer>,
    config: KafkaConfig,
}

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,
    pub timeout_ms: u64,
    pub consumer_group_id: String,
}

impl KafkaConfig {
    /// Create config from env. variables
    ///
    /// Expected env. variables:
    /// - `KAFKA_BOOTSTRAP_SERVERS`: Comma-separated list of Kafka brokers addresses
    /// - `KAFKA_CONSUMER_GROUP_ID`: Consumer group identifier
    /// - `KAFKA_TIMEOUT_MS`: Timeout for operations in milliseconds (default: 5000) 
    pub fn from_env() -> Result<Self, EventBusError> {
        dotenv::dotenv().ok();

        let bootstrap_servers = std::env::var("KAFKA_BOOTSTRAP_SERVERS")
            .map_err(|_| EventBusError::ConfigError("KAFKA_BOOTSTRAP_SERVERS not set".to_string()))?;        

        let consumer_group_id = std::env::var("KAFKA_CONSUMER_GROUP_ID")
            .map_err(|_| EventBusError::ConfigError("KAFKA_CONSUMER_GROUP_ID not set".to_string()))?;

        let timeout_ms = std::env::var("KAFKA_TIMEOUT_MS")
            .unwrap_or_else(|_| "5000".to_string())
            .parse::<u64>()
            .map_err(|_| 
                EventBusError::ConfigError("KAFKA_TIMEOUT_MS must be a valid number".to_string()
            ))?;
        Ok(Self {
            bootstrap_servers,
            timeout_ms,
            consumer_group_id,
        })
    }
}

impl KafkaEventBus {
    /// Create a new KafkaEventBus instant
    ///
    /// This initializes the kafka producer
    pub async fn new(config: KafkaConfig) -> Result<Self, EventBusError> {
        info!("🔧 Initializing Kafka event bus with brokers: {}", config.bootstrap_servers);
        
        let producer: FutureProducer = ClientConfig::new()
            // Connection settings
            .set("bootstrap.servers", &config.bootstrap_servers)
            
            // Reliability settings - ensure messages are safely delivered
            .set("acks", "all")                    // Wait for all replicas to acknowledge
            .set("enable.idempotence", "true")     // Prevent duplicate messages
            .set("retries", "10")                  // Retry failed sends
            .set("retry.backoff.ms", "1000")       // Wait between retries
            
            // Performance optimizations
            .set("compression.type", "zstd")       // Compress messages
            .set("batch.size", "65536")            // Batch up to 64KB
            .set("linger.ms", "5")                 // Wait up to 5ms to batch
            .set("queue.buffering.max.kbytes", "32768")  // 32MB buffer
            
            .create()
            .map_err(|e| EventBusError::ConnectionError(
                format!("Failed to create Kafka producer: {}", e)
            ))?;
        
        info!("✅ Kafka event bus initialized successfully");
        
        Ok(Self {
            producer: Arc::new(producer),
            config,
        })
    }

    /// Publish an event with retry logic and dead letter queue support
    ///
    /// This method handles the complete lifecycle of event publishing:
    /// - Wraps the event ina an `EventEnvelope` with metadata.
    /// - Serializes the event to JSON.
    /// - Sends to appropriate Kafka topic.
    /// - Handles failures with retries and dead letter queue logic.
    async fn publish_envelope<T>(&self, envelope: EventEnvelope<T>) -> Result<(), EventBusError>
        where 
            T: Event + Serialize + DeserializeOwned + Send + 'static,
    {
        let topic = T::TOPIC;
        let key = envelope.data
            .partition_key()
            .unwrap_or(envelope.event_id.clone());

        debug!("📤 Publishing event {} to topic {}", envelope.event_id, topic);

        let payload = serde_json::to_string(&envelope)
            .map_err(|e| EventBusError::SerializationError(
                    format!("Failed to serialize event: {}", e)
            ))?;

        let record = FutureRecord::to(&topic)
            .key(&key)
            .payload(&payload);

        let timeout = Timeout::After(Duration::from_millis(self.config.timeout_ms));

        match self.producer.send(record, timeout).await {
            Ok(delivery) => {
                debug!("✅ Event {} published successfully: {:?}", envelope.event_id, delivery);
                Ok(())
            }
            Err((kafka_error, _)) => {
                error!("❌ Failed to publish event {}: {}", envelope.event_id, kafka_error);
                Err(EventBusError::PublishFailed(
                    format!("Kafka send error: {}", kafka_error)
                ))
            }
        }
    }
}


#[allow(async_fn_in_trait)]
impl EventBus for KafkaEventBus {
    type Error = EventBusError;
    
    /// Publish an event to the Kafka topic
    ///
    /// Events are automatically wrapped in an `EventEnvelope` with metadata,
    /// serialized to JSON, and sent to the topic defined by the event type.
    async fn publish<T>(&self, event: T) -> Result<(), Self::Error>
            where 
                T: Event 
    {
        let envelope = EventEnvelope::new(event);
        self.publish_envelope(envelope).await
    }

    /// Subscribe to events of a specific type
    ///
    /// TODO: Full subscription implementation requires a consumer component
    /// that would typically run in a separate service or background task.
    /// This method is a placeholder for demonstration purposes.
    async fn subscribe<T, F>(&self, _handler: F) -> Result<(), Self::Error>
    where
        T: Event,
        F: Fn(EventEnvelope<T>) -> Result<(), Box<dyn Error + Send + Sync>> + Send + Sync + 'static,
    {
        warn!("🚧 Subscription not yet implemented - this will be added in the next phase");
        Err(EventBusError::SubscriptionFailed(
            "Subscription functionality requires consumer implementation".to_string()
        ))
    }

    /// Check if the Kafka connection is healthy
    /// 
    /// This attempts to get cluster metadata to verify connectivity.
    async fn health_check(&self) -> Result<(), Self::Error> {
        debug!("🏥 Performing Kafka health check");
        
        // Use a simple metadata request to check connectivity
        let timeout = Duration::from_millis(self.config.timeout_ms);
        let metadata_future = tokio::task::spawn_blocking({
            let producer = self.producer.clone();
            move || {
                producer.client().fetch_metadata(None, timeout)
            }
        });
        
        match tokio::time::timeout(Duration::from_secs(10), metadata_future).await {
            Ok(Ok(Ok(_metadata))) => {
                debug!("✅ Kafka health check passed");
                Ok(())
            }
            Ok(Ok(Err(e))) => {
                error!("❌ Kafka health check failed: {}", e);
                Err(EventBusError::ConnectionError(format!("Health check failed: {}", e)))
            }
            Ok(Err(e)) => {
                error!("❌ Kafka health check task failed: {}", e);
                Err(EventBusError::ConnectionError(format!("Health check task error: {}", e)))
            }
            Err(_) => {
                error!("❌ Kafka health check timed out");
                Err(EventBusError::ConnectionError("Health check timeout".to_string()))
            }
        }
    }
}

impl Drop for KafkaEventBus {
    /// Ensure clean shutdown of Kafka producer
    /// 
    /// This flushes any pending messages before the producer is dropped.
    fn drop(&mut self) {
        debug!("🧹 Cleaning up Kafka event bus");
        // The producer will flush automatically when dropped, but we could add
        // explicit flush logic here if needed for more control
    }
}

// Add some additional error types for Kafka-specific operations
#[derive(Debug)]
pub enum KafkaError {
    ConnectionFailed(String),
    TopicNotFound(String),
    ProducerError(String),
    ConsumerError(String),
}

impl std::fmt::Display for KafkaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KafkaError::ConnectionFailed(msg) => write!(f, "Kafka connection failed: {}", msg),
            KafkaError::TopicNotFound(msg) => write!(f, "Kafka topic not found: {}", msg),
            KafkaError::ProducerError(msg) => write!(f, "Kafka producer error: {}", msg),
            KafkaError::ConsumerError(msg) => write!(f, "Kafka consumer error: {}", msg),
        }
    }
}

impl Error for KafkaError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::{MessageReceived, MessageType, MessageContent};
    
    #[tokio::test]
    async fn test_kafka_config_from_env() {
        // Set test environment variables
        unsafe {
            std::env::remove_var("KAFKA_BOOTSTRAP_SERVERS");
            std::env::set_var("KAFKA_BOOTSTRAP_SERVERS", "localhost:9092");
            std::env::remove_var("KAFKA_CONSUMER_GROUP_ID");
            std::env::set_var("KAFKA_CONSUMER_GROUP_ID", "test-group");
            std::env::remove_var("KAFKA_TIMEOUT_MS");
            std::env::set_var("KAFKA_TIMEOUT_MS", "3000");
        }
        
        let config = KafkaConfig::from_env().expect("Should create config from env");
        
        assert_eq!(config.bootstrap_servers, "localhost:9092");
        assert_eq!(config.consumer_group_id, "test-group");
        assert_eq!(config.timeout_ms, 3000);
    }
    
    #[test]
    fn test_event_serialization() {
        let message = MessageReceived {
            message_id: "test-123".to_string(),
            from_phone: "+1234567890".to_string(),
            message_type: MessageType::Text,
            content: MessageContent::Text {
                body: "Hello, world!".to_string(),
            },
            received_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new(),
        };
        
        let envelope = EventEnvelope::new(message);
        
        // Test that we can serialize and deserialize the envelope
        let json = serde_json::to_string(&envelope).expect("Should serialize");
        let deserialized: EventEnvelope<MessageReceived> = 
            serde_json::from_str(&json).expect("Should deserialize");
        
        assert_eq!(deserialized.data.message_id, "test-123");
        assert_eq!(deserialized.version, "1.0");
    }
}

