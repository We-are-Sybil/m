use common::{
    KafkaEventBus, KafkaConfig, EventBus, MessageReceived, 
    SubscriptionConfig, ProcessingResult, EventEnvelope,
};
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("🔍 Starting test consumer to verify event publishing...");

    // Create Kafka event bus
    let kafka_config = KafkaConfig::from_env()
        .map_err(|e| format!("Failed to load Kafka config: {}", e))?;
    
    let event_bus = Arc::new(KafkaEventBus::new(kafka_config).await
        .map_err(|e| format!("Failed to create event bus: {}", e))?);

    // Test connection
    event_bus.health_check().await
        .map_err(|e| format!("Event bus health check failed: {}", e))?;
    
    info!("✅ Connected to Kafka successfully");

    // Subscribe to message events
    let config = SubscriptionConfig {
        consumer_group: "test-consumer".to_string(),
        ..Default::default()
    };

    event_bus.subscribe::<MessageReceived, _>(
        config,
        |envelope: EventEnvelope<MessageReceived>| {
            let message = &envelope.data;
            info!("🎉 Received MessageReceived event:");
            info!("   Message ID: {}", message.message_id);
            info!("   From: {}", message.from_phone);
            info!("   Type: {:?}", message.message_type);
            info!("   Content: {:?}", message.content);
            info!("   Metadata: {:?}", message.metadata);
            
            Ok(ProcessingResult::Success)
        }
    ).await?;

    info!("🎯 Subscribed to conversation.messages topic");
    info!("💡 Send a test message to your webhook to see events appear here");
    info!("🛑 Press Ctrl+C to stop");

    // Keep the consumer running
    tokio::signal::ctrl_c().await?;
    info!("👋 Shutting down test consumer");

    Ok(())
}
