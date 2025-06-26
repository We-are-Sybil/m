use crate::{
    config::KafkaConfig,
}
use common::{
    WebhookEvent,
    ProcessingError,
};
use anyhow::Result;
use rdkafka::{
    config::ClientConfig,
    consumer::{Consumer, StreamConsumer},
    message::{BorrowedMessage, Message},
    ClientContext, ConsumerContext,
};
use tracing::{info, error, debug, warn};
use std::time::Duration;

/// Custom context for kafka consumer
///
/// This allows us to customize how the consumer handles various events
/// like rebalancing (when partitions get reassigned between consumers).
/// For now it will be simple, but work as a placeholder for future 
/// enhancements, like adding custom logic for monitoring comumer health 
/// handling partition reassignments, etc.
#[derive(Debug)]
pub struct MessageConsumerContext;

impl ClientContext for MessageConsumerContext {}
impl ConsumerContext for MessageConsumerContext {
    fn reblance(
        &self,
        native_ptr: *mut rdkafka::bindings::rd_kafka_t,
        err: rdkafka::error::KafkaResult<()>
        tpl: &rdkafka::TopicPartitionList
    ) {
        match err {
            Ok(()) => info!("✅ Consumer rebalance successful: {:?}", tpl),
            Err(e) => error!("❌ Consumer rebalance failed: {}: {:?}", e, tpl),
        }
    }
}

pub struct MessageConsumer {
    consumer: StreamConsumer<MessageConsumerContext>,
    input_topic: String,
}

impl MessageConsumer {
    pub async fn new(config: &KafkaConfig) -> Result<Self> {
        debug!("🔧 Initializing Kafka consumer with config: {:?}", config);

        // Config. the rdkafka consumer
        let consumer: StreamConsumer<MessageConsumerContext> = ClientConfig::new()
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("group.id", &config.group_id)

            // Offset management ~ controls where to start consuming messages
            .set("auto.offset.reset", "earliest")  // Start from beginning if no offset stored
            .set("enable.auto.commit", "false")   // We'll commit offsets manually for safety

            // Session and heartbeat settings - these control failure detection
            .set("session.timeout.ms", "30000")   // 30 seconds to detect consumer failure
            .set("heartbeat.interval.ms", "3000") // Send heartbeat every 3 seconds
            
            // Processing settings
            .set("max.poll.interval.ms", "300000") // 5 minutes max between polls
            .set("fetch.min.bytes", "1")          // Don't wait for large batches
            .set("fetch.wait.max.ms", "500")      // Max 500ms wait for messages

            .create_with_context(MessageConsumerContext)
            .map_err(|e| anyhow::anyhow!("Failed to create Kafka consumer: {}", e))?;

        consumer
            .subscribe(&[&config.input_topic])
            .map_err(|e| anyhow::anyhow!("Failed to subscribe to topic {}: {}", config.input_topic, e))?;

        info!("🔧 Kafka consumer initialized and subscribed to topic: {}", config.input_topic);

        Ok(Self {
            consumer,
            input_topic: config.input_topic.clone(),
        })
    }
    /// Consume a batch of webhook events 
    ///
    /// This method reads up to the `batch_size` messages from kafka within the
    /// specified timeout. It handles deserialization and error recovery,
    /// returning only successfully parsed events.
    pub async fn consume_batch(
        &self,
        batch_size: usize,
        timeout_ms: Duration,
    ) -> Result<Vec<WebhookEvent>, ProcessingError> {
        debug!("🔄 Consuming batch of up to {} messages from topic: {}", batch_size, self.input_topic);

        let timeout = Duration::from_millis(timeout_ms);
        let mut events = Vec::with_capacity(batch_size);
        let mut messages_to_commit = Vec::new();

        for _ in 0..batch_size {
            match tokio::time::timeout(timeout, self.consumer.recv()).await {
                Ok(Ok(message)) => {
                    match self.parse_message(&message) {
                        Ok(event) => {
                            events.push(event);
                            messages_to_commit.push(message);
                        }
                        Err(e) => {
                            warn!("❗ Failed to parse message: {}", e);
                            // Still commit to avoid reprocessing
                            messages_to_commit.push(message);
                        }
                    }
                }
                Ok(Err(e)) => {
                    error!("❌ Error receiving message: {}", e);
                    return Err(ProcessingError::ConsumerError(format!("Failed to receive message: {}", e)));
                }
                Err(_) => {
                    debug!("⏳ Timeout reached while waiting for messages");
                    break; // Exit loop if timeout occurs
                }
            }
        }
    }

    /// Parse a Kafka message into a WebhookEvent
    fn parse_message(&self, message: &BorrowedMessage) -> Result<WebhookEvent, ProcessingError> {
        let payload = message.payload()
            .ok_or_else(|| ProcessingError::InvalidMessage("Empty message payload".to_string()))?;
        
        serde_json::from_slice(payload)
            .map_err(|e| ProcessingError::InvalidMessage(format!("JSON parse error: {}", e)))
    }
    
    /// Commit message offsets
    async fn commit_messages(&self, messages: Vec<BorrowedMessage>) -> Result<(), ProcessingError> {
        for message in messages {
            self.consumer
                .commit_message(&message, rdkafka::consumer::CommitMode::Async)
                .map_err(|e| ProcessingError::ConsumerError(format!("Commit failed: {}", e)))?;
        }
        Ok(())
    }

    /// Close the consumer
    pub async fn close(&self) -> Result<(), ProcessingError> {
        // rdkafka handles cleanup automatically when dropped
        debug!("Consumer closed");
        Ok(())
    }

}

