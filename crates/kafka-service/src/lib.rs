pub mod config;
pub mod consumer;
pub mod producer;
pub mod processor;

pub use config::KafkaConfig;
pub use consumer::MessageConsumer;
pub use producer::MessageProducer;
pub use processor::MessageProcessor;

use anyhow::Result;
use tracing::{info, error};
use std::time::Duration;

/// Main service struct that orchestates the entire Kafka processing pipeline
/// This is analogous to the webhook service's main server, but instead of handling HTTP requests,
/// it processes kafka messages. It coordinates between consuming messages, processing
/// them, and producing results.
pub struct KafkaService {
    config: KafkaConfig,
    consumer: MessageConsumer,
    processor: MessageProcessor,
    producer: MessageProducer,
}
impl KafkaService { 
    /// Create a new kafka service instance
    /// 
    /// This initializes all the components needed for message processing.
    /// Each component has specific responsibilities in the pipeline.
    pub async fn new() -> Result<Self> {
       let config = KafkaConfig::from_env(); 
        info!("📋 Configuration loaded");
        info!("🔌 Bootstrap servers: {}", config.bootstrap_servers);
        info!("📥 Input topic: {}", config.input_topic);
        info!("📤 Output topic: {}", config.output_topic);
        info!("👥 Consumer group: {}", config.consumer_group_id);  

        let consumer = MessageConsumer::new(&config).await?;
        info!("📨 Kafka consumer initialized");

        let producer = MessageProducer::new(&config).await?;
        info!("📬 Kafka producer initialized");

        let processor = MessageProcessor::new(config.clone());
        info!("⚙️ Message processor initialized");

        Ok(Self {
            config,
            consumer,
            processor,
            producer,
        })
    }

    /// Start the main processing loop
    ///
    /// This is the heart of the kafka service. It continuously reads messages from
    /// the input topic, processes them, and sends the results to the output topic.
    pub async fn run(&mut self) -> Result<()> {
        info!("🎯 Starting message processing loop");
        
        loop {
            match self.process_next_batch().await {
                Ok(processed_count) => {
                    if processed_count > 0 {
                        info!("✅ Processed {} messages", processed_count);
                    }
                }
                Err(e) => {
                    error!("❌ Error processing messages: {}", e);
                    // In production, you might want to implement exponential backoff here
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// Process a single batch of messages
    ///
    /// This method assumes the core pattern: consume -> process -> produce.
    /// Reading a batch of messages, transoforms them through business logic,
    /// and sends the results downstream.
    async fn process_next_batch(&mut self) -> Result<usize> {
        info!("🔄 Processing next batch of messages...");
        let webhook_events = self.consumer
            .consume_batch(self.config.batch_size, Duration::from_millis(self.config.processing_timeout_ms))
            .await?;

        if webhook_events.is_empty() {
            return Ok(0);
        }

        let mut responses = Vec::new();
        for event in webhook_events {
            match self.processor.process_webhook_event(event).await {
                Ok(response) => responses.push(response),
                Err(e) => {
                    error!("❌ Error processing event: {:?}", e);
                    continue; // Skip this event and continue with the next
                }
            }
        }

        if !responses.is_empty() {
            self.producer.send_batch(responses.clone()).await?;
            info!("✅ Processed and sent {} responses", &responses.len());
        } else {
            info!("🔍 No valid responses to send in this batch");
        }
        Ok(responses.len())
    }

    /// Correct shutdown procedure
    ///
    /// This ensures that all messages are processed before shutting down,
    /// Important for preventing message data loss during deployments, restarts, or
    /// maintenance.
    pub async fn shutdown(&mut self) -> Result<()> {
        info!("🛑 Shutting down Kafka service gracefully...");

        self.producer.flush().await?;
        self.consumer.close().await?;

        info!("✅ Kafka service shutdown complete");
        Ok(())
    } 
}
