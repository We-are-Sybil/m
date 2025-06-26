use crate::config::KafkaConfig;
use anyhow::Result;
use common::ProcessingError;
use rdkafka::{
    config::ClientConfig,
    producer::{FutureProducer, FutureRecord, Producer},
    util::Timeout,
};
use tracing::{info, error, warn};
use std::time::Duration;


/// Handles sending processed messages to Kafka
///
/// This producer is responsible for taking AI responses and reliably delivering
/// them to the output topic where the client service can pick them up. 
/// It handles serialization, delivery confirmation, and error recovery.
pub struct MessageProducer {
    /// The underlying rdkafka producer client
    producer: FutureProducer,

    /// Topic where we send processed messages
    output_topic: String,

    /// How long to wait for message delivery confirmation
    delivery_timeout: Duration,
}

impl MessageProducer {
    /// Create a new message producer
    ///
    /// This initializes rdkafka producer with settings optimized for
    /// microservices environments.
    pub async fn new(config: &KafkaConfig) -> Result<Self> {
        info!("🔧 Initializing Kafka producer with config: {:?}", config);

            let producer: FutureProducer = ClientConfig::new()
            // Basic connection settings
            .set("bootstrap.servers", &config.bootstrap_servers)
            
            // Reliability settings - these ensure messages are safely delivered
            .set("acks", "all")  // Wait for all replicas to acknowledge
            .set("enable.idempotence", "true")  // Prevent duplicate messages
            .set("retries", "10")  // Retry failed sends up to 10 times
            .set("retry.backoff.ms", "1000")  // Wait 1 second between retries
            
            // Performance optimization settings
            .set("compression.type", "zstd")  // Compress messages to save bandwidth
            .set("batch.size", "65536")  // Batch up to 64KB of messages
            .set("linger.ms", "5")  // Wait up to 5ms to batch messages
            .set("buffer.memory", "33554432")  // 32MB buffer for pending messages
            
            // Create the producer client
            .create()
            .map_err(|e| anyhow::anyhow!("Failed to create Kafka producer: {}", e))?;
        
        Ok(Self {
            producer,
            output_topic: config.output_topic.clone(),
            delivery_timeout: Duration::from_millis(config.processing_timeout_ms),
        })
    }

    /// Send a single AI response to the output topic
    ///
    /// This method handles the complete lifecycle of sending a message:
    /// serialization, delivery, and confirmation. It returns an error if 
    /// anything goes wrong so the caller can dacide how to handle failures.
    pub async fn send_message(&self, response: AIResponse) -> Result<(), ProcessingError> {
        // Serialize the response to json
        // original_event_id => message key for proper partitioning
        let message_key = response.original_event_id.clone();
        let message_payload = serde_json::to_string(&response)
            .map_err(ProcessingError::SerializationError)?;

        debug!("📤 Sending message to topic {}: {}", self.output_topic, message_key);

        // Create the record to send
        let record = FutureRecord::to(&self.output_topic)
            .key(&message_key)
            .payload(&message_payload);

        match self.producer.send(record, Timeout::After(self.delivery_timeout)).await {
            Ok(delivery) => {
                debug!("✅ Message sent successfully: {:?}", delivery);
                Ok(())
            }
            Err((kafka_error, _owned_message)) => {
                let error_msg = format!("❌ Failed to send message: {}", kafka_error)
                error!("{}", error_msg);
                Err(ProcessingError::KafkaError(error_msg))
            }
        }
    }

    /// Send multiple AI responses in a batch
    ///
    /// This is more efficient than sending messages one by one, because it 
    /// allows kafka to batch multiple messages together for better throughput.
    /// However, we still wait for individual delivery confirmations to ensure reliability.
    pub async fn send_batch(&self, responses: Vec<AIResponse>) -> Result<(), ProcessingError> {
        debug!("📦 Sending batch of {} messages to topic {}", responses.len(), self.output_topic);
        let send_futures: Vec<_> = responses
            .into_iter()
            .map(|response| self.send_message(response))
            .collect();

        // Wait for all messages to be sent
        let results = futures::future::join_all(send_futures).await;

        // Check for any errors in the batch
        let mut failed_count = 0;
        let mut last_error = None;

        for result in results {
            if let Err(e) = result {
                failed_count += 1;
                last_error = Some(e);
            }
        }

        if failed_count > 0 {
            let error_msg = format!("❌ Failed to send {} out of {} messages in batch", failed_count, responses.len());
            error!("{}", error_msg);
            Err(last_error.unwrap_or(ProcessingError::BatchError(error_msg, None)))
        } else {
            debug!("✅ All {} messages sent successfully", responses.len());
            Ok(())
        }

    }

    /// Flush any pending messages and wait for delivery
    ///
    /// This is importand during shutdown to ensure no messages are lost.
    /// It tells the producer to send any batched messages immediately
    /// and wait for all pending deliveries to complete.
    pub async fn flush(&self) -> Result<(), ProcessingError> {
        info!("🔄 Flushing Kafka producer to ensure all messages are sent");

        let flush_timeout = Duration::from_secs(30);
        let flush_result = tokio::time::timeout(
            flush_timeout,
            tokio::task::spawn_blocking({
                let producer = self.producer.clone();
                move || producer.flush(Duration::from_secs(10))
            })
        ).await;

        match flush_result {
            Ok(Ok(Ok(()))) => {
                info!("✅ All messages flushed successfully");
                Ok(())
            },
            Ok(Ok(Err(e))) => {
                let error_msg = format!("❌ Failed to flush messages: {}", e);
                error!("{}", error_msg);
                Err(ProcessingError::ProducerError(error_msg))
            },
            Ok(Err(e)) => {
                let error_msg = format!("❌ Flush task failed: {}", e);
                error!("{}", error_msg);
                Err(ProcessingError::ProducerError(error_msg))
            },
            Err(_) => {
                let error_msg = "❌ Flush operation timed out".to_string();
                error!("{}", error_msg);
                Err(ProcessingError::TimeoutError)
            }
        }
    }
}
