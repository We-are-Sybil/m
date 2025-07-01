use crate::message_bus::{
    Event, 
    EventBus,
    EventBusError,
    EventEnvelope,
    ProcessingResult,
    SubscriptionConfig,
};
use rdkafka::{
    config::ClientConfig,
    consumer::{StreamConsumer, Consumer},
    producer::{FutureProducer, FutureRecord, Producer},
    util::Timeout,
    Message,
};
use futures::future::join_all;
use serde::{
    Serialize, 
    de::DeserializeOwned
};
use std::{
    collections::HashMap,
    error::Error,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// Kafka-based implementation of the EventBus trait
///
/// This provides a complete publish-subscribe system using Kafka
/// as underlying message broker. It handles producer operations for 
/// publishing events and consumer operations for subscribing to events.
pub struct KafkaEventBus {
    /// Kafka producer instance for publishing events
    producer: Arc<FutureProducer>,
    /// Coniguration for Kafka connections
    config: KafkaConfig,
    /// Active consumers (tracked for graceful shutdown)
    consumers : Arc<RwLock<HashMap<String, Arc<StreamConsumer>>>>,
    /// Shutdown signal for coordinating consumer shutdown
    shutdown_signal: Arc<tokio::sync::watch::Sender<bool>>,
    shutdown_receiver: tokio::sync::watch::Receiver<bool>,
}

/// Configuration for connecting to Kafka cluster
#[derive(Debug, Clone)]
pub struct KafkaConfig {
    /// Kafka broker addresses (comma-separated)
    pub bootstrap_servers: String,
    /// Default timeout for operations
    pub timeout_ms: u64,
    /// Base consumer group ID (will be suffixed for different subscriptions)
    pub consumer_group_id: String,
    /// Security configuration
    pub security_protocol: String,
}

impl KafkaConfig {
    /// Create configuration from environment variables
    /// 
    /// Expected environment variables:
    /// - KAFKA_BOOTSTRAP_SERVERS: Comma-separated list of broker addresses
    /// - KAFKA_TIMEOUT_MS: Operation timeout in milliseconds (optional, default: 5000)
    /// - KAFKA_CONSUMER_GROUP_ID: Base consumer group identifier
    /// - KAFKA_SECURITY_PROTOCOL: Security protocol (optional, default: PLAINTEXT)
    pub fn from_env() -> Result<Self, EventBusError> {
        dotenv::dotenv().ok();
        
        let bootstrap_servers = std::env::var("KAFKA_BOOTSTRAP_SERVERS")
            .map_err(|_| EventBusError::ConfigError(
                "KAFKA_BOOTSTRAP_SERVERS environment variable must be set".to_string()
            ))?;
            
        let consumer_group_id = std::env::var("KAFKA_CONSUMER_GROUP_ID")
            .map_err(|_| EventBusError::ConfigError(
                "KAFKA_CONSUMER_GROUP_ID environment variable must be set".to_string()
            ))?;
            
        let timeout_ms = std::env::var("KAFKA_TIMEOUT_MS")
            .unwrap_or_else(|_| "5000".to_string())
            .parse()
            .map_err(|_| EventBusError::ConfigError(
                "KAFKA_TIMEOUT_MS must be a valid number".to_string()
            ))?;
            
        let security_protocol = std::env::var("KAFKA_SECURITY_PROTOCOL")
            .unwrap_or_else(|_| "PLAINTEXT".to_string());
        
        Ok(Self {
            bootstrap_servers,
            timeout_ms,
            consumer_group_id,
            security_protocol,
        })
    }
}

impl KafkaEventBus {
    /// Create a new KafkaEventBus instant
    ///
    /// This initializes both the producer (for publishing) and the
    /// infraestructure needed to manage consumers (for subscribing). The
    /// producer is created immediately, while consumers are created 
    /// on-demand for each subscription.
    pub async fn new(config: KafkaConfig) -> Result<Self, EventBusError> {
        info!("🔧 Initializing Kafka event bus with brokers: {}", config.bootstrap_servers);
        
        // Create the producer with optimized settings
        let producer: FutureProducer = ClientConfig::new()
            // Connection settings
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("security.protocol", &config.security_protocol)
            
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
        
        // Create shutdown coordination
        let (shutdown_tx, shutdown_rx) = tokio::sync::watch::channel(false);
        
        info!("✅ Kafka event bus initialized successfully");
        
        Ok(Self {
            producer: Arc::new(producer),
            config,
            consumers: Arc::new(RwLock::new(HashMap::new())),
            shutdown_signal: Arc::new(shutdown_tx),
            shutdown_receiver: shutdown_rx,
        })
    }

    /// Create a new Kafka consumer with the specified configuration
    ///
    /// This sets up a consumer with optimized settings for reliable message
    /// processing in a microservices architecture.
    fn create_consumer(&self, consumer_group: &str) -> Result<StreamConsumer, EventBusError> {
        let consumer: StreamConsumer = ClientConfig::new()
            // Connection settings
            .set("bootstrap.servers", &self.config.bootstrap_servers)
            .set("security.protocol", &self.config.security_protocol)
            .set("group.id", consumer_group)

            // Consumer behavior settings
            .set("auto.offset.reset", "earliest")   // Start from the earliest message
            .set("enable.auto.commit", "false")     // Manual offset management
            .set("session.timeout.ms", "30000")     // 30 sec. ession timeout
            .set("heartbeat.intervals.ms", "3000") // 3 sec. heartbeat
            .set("max.poll.interval.ms", "300000")  // 5 min. max poll interval
            
            // Performance settings
            .set("fetch.min.bytes", "1024")         // Minimum bytes to fetch
            .set("fetch.max.wait.ms", "500")        // Wait up to 500ms for more data
            .set("max.partition.fetch.bytes", "1048576") // 1MB per partition

            .create()
            .map_err(|e|
                EventBusError::ConsumerError(
                    format!("Failed to create Kafka consumer: {}", e)
                )
            )?;
        Ok(consumer)            
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

    /// Process a single event envelope with the provided handler
    /// 
    /// This implements the core event processing logic including retry
    /// and dead letter queue handling based on the processing result.
    async fn process_event_envelope<T, F>(
        &self,
        envelope: EventEnvelope<T>,
        handler: &F,
    ) -> Result<bool, EventBusError>
    where
        T: Event,
        F: Fn(EventEnvelope<T>) -> Result<ProcessingResult, Box<dyn Error + Send + Sync>>,
    {
        let event_id = envelope.event_id.clone();
        let topic = T::TOPIC;
        
        debug!("🔄 Processing event {} from topic {}", event_id, topic);
        
        // Call the user's handler function
        match handler(envelope.clone()) {
            Ok(ProcessingResult::Success) => {
                debug!("✅ Event {} processed successfully", event_id);
                Ok(true) // Commit the offset
            }
            Ok(ProcessingResult::RetryableError(error_msg)) => {
                warn!("🔄 Event {} failed with retryable error: {}", event_id, error_msg);
                
                // Check if we should retry or send to DLQ
                if envelope.should_dead_letter() {
                    error!("💀 Event {} exceeded retry limit, sending to DLQ", event_id);
                    self.send_to_dead_letter_queue(envelope).await?;
                } else {
                    info!("⏰ Event {} will be retried (attempt {})", event_id, envelope.attempt_count + 1);
                    self.send_to_retry_queue(envelope).await?;
                }
                Ok(true) // Commit the offset (we've handled the error)
            }
            Ok(ProcessingResult::PermanentError(error_msg)) => {
                error!("💀 Event {} failed with permanent error: {}", event_id, error_msg);
                self.send_to_dead_letter_queue(envelope).await?;
                Ok(true) // Commit the offset
            }
            Err(handler_error) => {
                error!("❌ Handler threw exception for event {}: {}", event_id, handler_error);
                // Treat handler exceptions as retryable errors
                if envelope.should_dead_letter() {
                    self.send_to_dead_letter_queue(envelope).await?;
                } else {
                    self.send_to_retry_queue(envelope).await?;
                }
                Ok(true) // Commit the offset
            }
        }
    }

    /// Send a failed event to the retry queue for delayed reprocessing
    async fn send_to_retry_queue<T>(&self, mut envelope: EventEnvelope<T>) -> Result<(), EventBusError>
    where
        T: Event,
    {
        let retry_topic = format!("{}.retry", T::TOPIC);
        envelope.increment_attempt();
        
        // Add retry metadata
        envelope.add_metadata("retry_reason".to_string(), "retryable_error".to_string());
        envelope.add_metadata("original_topic".to_string(), T::TOPIC.to_string());
        envelope.add_metadata("retry_attempt".to_string(), envelope.attempt_count.to_string());
        
        let key = envelope.partition_key().unwrap_or(envelope.event_id.clone());
        let payload = serde_json::to_string(&envelope)
            .map_err(|e| EventBusError::SerializationError(format!("Failed to serialize retry event: {}", e)))?;
        
        let record = FutureRecord::to(&retry_topic)
            .key(&key)
            .payload(&payload);
        
        let timeout = Timeout::After(Duration::from_millis(self.config.timeout_ms));
        
        match self.producer.send(record, timeout).await {
            Ok(_) => {
                info!("📮 Event {} sent to retry queue {}", envelope.event_id, retry_topic);
                Ok(())
            }
            Err((kafka_error, _)) => {
                error!("❌ Failed to send event {} to retry queue: {}", envelope.event_id, kafka_error);
                Err(EventBusError::PublishFailed(format!("Retry queue send error: {}", kafka_error)))
            }
        }
    }
    
    /// Send a failed event to the dead letter queue for investigation
    async fn send_to_dead_letter_queue<T>(&self, mut envelope: EventEnvelope<T>) -> Result<(), EventBusError>
    where
        T: Event,
    {
        let dlq_topic = format!("{}.dlq", T::TOPIC);
        
        // Add DLQ metadata
        envelope.add_metadata("dlq_reason".to_string(), "max_retries_exceeded".to_string());
        envelope.add_metadata("original_topic".to_string(), T::TOPIC.to_string());
        envelope.add_metadata("final_attempt_count".to_string(), envelope.attempt_count.to_string());
        envelope.add_metadata("dlq_timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        
        let key = envelope.partition_key().unwrap_or(envelope.event_id.clone());
        let payload = serde_json::to_string(&envelope)
            .map_err(|e| EventBusError::SerializationError(format!("Failed to serialize DLQ event: {}", e)))?;
        
        let record = FutureRecord::to(&dlq_topic)
            .key(&key)
            .payload(&payload);
        
        let timeout = Timeout::After(Duration::from_millis(self.config.timeout_ms));
        
        match self.producer.send(record, timeout).await {
            Ok(_) => {
                warn!("💀 Event {} sent to dead letter queue {}", envelope.event_id, dlq_topic);
                Ok(())
            }
            Err((kafka_error, _)) => {
                error!("❌ Failed to send event {} to DLQ: {}", envelope.event_id, kafka_error);
                Err(EventBusError::PublishFailed(format!("DLQ send error: {}", kafka_error)))
            }
        }
    }

}


#[allow(async_fn_in_trait)]
impl EventBus for KafkaEventBus {
    type Error = EventBusError;
    
    /// Publish a single event to the appropriate Kafka topic
    async fn publish<T>(&self, event: T) -> Result<(), Self::Error>
    where
        T: Event,
    {
        let envelope = EventEnvelope::new(event);
        self.publish_envelope(envelope).await
    }
    
    /// Publish multiple events efficiently as a batch
    async fn publish_batch<T>(&self, events: Vec<T>) -> Result<(), Self::Error>
    where
        T: Event,
    {
        if events.is_empty() {
            return Ok(());
        }
        
        info!("📦 Publishing batch of {} events", events.len());
        
        // Convert all events to envelopes and publish them
        let mut publish_futures = Vec::new();
        for event in events {
            let envelope = EventEnvelope::new(event);
            publish_futures.push(self.publish_envelope(envelope));
        }
        
        // Wait for all publishes to complete
        let results = join_all(publish_futures).await;
        
        // Check if any failed
        for (i, result) in results.into_iter().enumerate() {
            if let Err(e) = result {
                error!("❌ Event {} in batch failed to publish: {}", i, e);
                return Err(e);
            }
        }
        
        info!("✅ Batch publishing completed successfully");
        Ok(())
    }
    
    /// Subscribe to events with a single-event handler
    async fn subscribe<T, F>(&self, config: SubscriptionConfig, handler: F) -> Result<(), Self::Error>
    where
        T: Event,
        F: Fn(EventEnvelope<T>) -> Result<ProcessingResult, Box<dyn Error + Send + Sync>> 
            + Send 
            + Sync 
            + 'static,
    {
        let topic = T::TOPIC;
        let consumer_group = format!("{}-{}", self.config.consumer_group_id, config.consumer_group);
        
        info!("🎯 Starting subscription to topic {} with consumer group {}", topic, consumer_group);
        
        // Create consumer
        let consumer = Arc::new(self.create_consumer(&consumer_group)?);
        
        // Subscribe to the topic
        consumer.subscribe(&[topic])
            .map_err(|e| EventBusError::SubscriptionFailed(format!("Failed to subscribe to topic {}: {}", topic, e)))?;
        
        // Store consumer reference for shutdown coordination
        {
            let mut consumers = self.consumers.write().await;
            consumers.insert(consumer_group.clone(), consumer.clone());
        }
        
        // Clone necessary references for the async task
        let event_bus = Arc::new(self.clone());
        let shutdown_rx = self.shutdown_receiver.clone();
        
        // Spawn the consumer loop
        tokio::spawn(async move {
            info!("🔄 Consumer loop starting for topic {}", topic);
            
            loop {
                // Check for shutdown signal
                if shutdown_rx.has_changed().unwrap_or(false) && *shutdown_rx.borrow() {
                    info!("🛑 Shutdown signal received for consumer {}", consumer_group);
                    break;
                }
                
                // Poll for messages
                match consumer.recv().await {
                    Ok(message) => {
                        // Extract message payload
                        let payload = match message.payload() {
                            Some(p) => p,
                            None => {
                                warn!("📭 Received empty message, skipping");
                                continue;
                            }
                        };
                        
                        // Deserialize event envelope
                        let envelope: EventEnvelope<T> = match serde_json::from_slice(payload) {
                            Ok(env) => env,
                            Err(e) => {
                                error!("❌ Failed to deserialize message: {}", e);
                                // Commit the offset to skip this bad message
                                if let Err(commit_err) = consumer.commit_message(&message, rdkafka::consumer::CommitMode::Async) {
                                    error!("❌ Failed to commit offset for bad message: {}", commit_err);
                                }
                                continue;
                            }
                        };
                        
                        // Process the event
                        match event_bus.process_event_envelope(envelope, &handler).await {
                            Ok(should_commit) => {
                                if should_commit {
                                    // Commit the offset to mark this message as processed
                                    if let Err(e) = consumer.commit_message(&message, rdkafka::consumer::CommitMode::Async) {
                                        error!("❌ Failed to commit offset: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                error!("❌ Failed to process event: {}", e);
                                // Still commit to avoid reprocessing the same message
                                if let Err(commit_err) = consumer.commit_message(&message, rdkafka::consumer::CommitMode::Async) {
                                    error!("❌ Failed to commit offset after processing error: {}", commit_err);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("❌ Error receiving message: {}", e);
                        // Sleep briefly to avoid tight loop on persistent errors
                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                }
            }
            
            info!("🏁 Consumer loop ended for topic {}", topic);
        });
        
        info!("✅ Subscription started successfully for topic {}", topic);
        Ok(())
    }
    
    /// Subscribe with batch processing (placeholder - would implement similar to single event)
    async fn subscribe_batch<T, F>(&self, _config: SubscriptionConfig, _handler: F) -> Result<(), Self::Error>
    where
        T: Event,
        F: Fn(Vec<EventEnvelope<T>>) -> Result<Vec<ProcessingResult>, Box<dyn Error + Send + Sync>> 
            + Send 
            + Sync 
            + 'static,
    {
        // TODO: Implement batch processing
        Err(EventBusError::SubscriptionFailed("Batch subscription not yet implemented".to_string()))
    }
    
    /// Check if the Kafka connection is healthy
    async fn health_check(&self) -> Result<(), Self::Error> {
        debug!("🏥 Performing Kafka health check");
        
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
    
    /// Gracefully shutdown the event bus
    async fn shutdown(&self) -> Result<(), Self::Error> {
        info!("🛑 Initiating graceful shutdown of Kafka event bus");
        
        // Signal all consumers to stop
        let _ = self.shutdown_signal.send(true);
        
        // Wait for consumers to finish processing
        tokio::time::sleep(Duration::from_secs(5)).await;
        
        // Clear consumer references
        {
            let mut consumers = self.consumers.write().await;
            consumers.clear();
        }
        
        info!("✅ Kafka event bus shutdown completed");
        Ok(())
    }
}

impl Clone for KafkaEventBus {
    fn clone(&self) -> Self {
        Self {
            producer: self.producer.clone(),
            config: self.config.clone(),
            consumers: self.consumers.clone(),
            shutdown_signal: self.shutdown_signal.clone(),
            shutdown_receiver: self.shutdown_receiver.clone(),
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

