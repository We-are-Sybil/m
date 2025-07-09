use common::{
    KafkaEventBus, KafkaConfig, EventBus, 
    SubscriptionConfig, ProcessingResult, EventEnvelope,
};
use whatsapp_client::{
    client::{
        core::WhatsAppClient,
        message_types::{
            WhatsAppMessageSend, 
            WhatsAppMessage,
            Message,
        },
    },
    config::WhatsAppClientConfig,
    errors::WhatsAppResult,
};
use std::sync::Arc;
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("📱 Starting WhatsApp message sender service...");

    // Initialize WhatsApp client
    let whatsapp_config = WhatsAppClientConfig::from_env();
    let whatsapp_client = Arc::new(WhatsAppClient::new(whatsapp_config)
        .map_err(|e| format!("Failed to create WhatsApp client: {}", e))?);

    info!("✅ WhatsApp client initialized successfully");

    // Create Kafka event bus
    let kafka_config = KafkaConfig::from_env()
        .map_err(|e| format!("Failed to load Kafka config: {}", e))?;
    
    let event_bus = Arc::new(KafkaEventBus::new(kafka_config).await
        .map_err(|e| format!("Failed to create event bus: {}", e))?);

    // Test connection
    event_bus.health_check().await
        .map_err(|e| format!("Event bus health check failed: {}", e))?;
    
    info!("✅ Connected to Kafka successfully");

    // Subscribe to WhatsApp message send events
    let config = SubscriptionConfig {
        consumer_group: "whatsapp-sender".to_string(),
        ..Default::default()
    };

    let client_clone = whatsapp_client.clone();
    event_bus.subscribe::<WhatsAppMessageSend, _>(
        config,
        move |envelope: EventEnvelope<WhatsAppMessageSend>| {
            let client = client_clone.clone();
            let message_send = &envelope.data;
            
            info!("📨 Processing WhatsApp message send event (original: {})", 
                  message_send.original_message_id);
            
            // Handle the message sending in a blocking context
            let result = tokio::task::block_in_place(|| {
                let rt = tokio::runtime::Handle::current();
                rt.block_on(async {
                    process_whatsapp_message_send(client, message_send).await
                })
            });
            
            match result {
                Ok(response) => {
                    info!("✅ Message sent successfully. WhatsApp ID: {}", 
                          response.messages.first().map(|m| &m.id).unwrap_or(&"unknown".to_string()));
                    Ok(ProcessingResult::Success)
                }
                Err(e) => {
                    error!("❌ Failed to send WhatsApp message: {}", e);
                    if e.is_retryable() {
                        Ok(ProcessingResult::RetryableError(e.to_string()))
                    } else {
                        Ok(ProcessingResult::PermanentError(e.to_string()))
                    }
                }
            }
        }
    ).await?;

    info!("🎯 Subscribed to conversation.responses topic");
    info!("📞 Waiting for WhatsApp message send events...");
    info!("🛑 Press Ctrl+C to stop");

    // Keep the service running
    tokio::signal::ctrl_c().await?;
    info!("👋 Shutting down WhatsApp sender service");

    Ok(())
}

async fn process_whatsapp_message_send(
    client: Arc<WhatsAppClient>,
    message_send: &WhatsAppMessageSend,
) -> WhatsAppResult<whatsapp_client::client::responses::WhatsAppMessageResponse> {
    let recipient = get_recipient_from_message(&message_send.message);
    
    info!("🚀 Sending {} message to {} (priority: {:?})",
          get_message_type_name(&message_send.message),
          recipient,
          message_send.priority);

    // Send the message using the WhatsApp client
    // The message is already in the correct format for the WhatsApp API
    client.send_message(message_send.message.clone()).await
}

fn get_recipient_from_message(message: &WhatsAppMessage) -> &str {
    match message {
        WhatsAppMessage::Text(msg) => msg.recipient(),
        WhatsAppMessage::Audio(msg) => msg.recipient(),
        WhatsAppMessage::Contact(msg) => msg.recipient(),
        WhatsAppMessage::Document(msg) => msg.recipient(),
        WhatsAppMessage::Image(msg) => msg.recipient(),
        WhatsAppMessage::Interactive(msg) => msg.recipient(),
        WhatsAppMessage::Location(msg) => msg.recipient(),
        WhatsAppMessage::Video(msg) => msg.recipient(),
    }
}

fn get_message_type_name(message: &WhatsAppMessage) -> &'static str {
    match message {
        WhatsAppMessage::Text(_) => "text",
        WhatsAppMessage::Audio(_) => "audio",
        WhatsAppMessage::Contact(_) => "contact",
        WhatsAppMessage::Document(_) => "document",
        WhatsAppMessage::Image(_) => "image",
        WhatsAppMessage::Interactive(_) => "interactive",
        WhatsAppMessage::Location(_) => "location",
        WhatsAppMessage::Video(_) => "video",
    }
}
