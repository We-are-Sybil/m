use common::{KafkaEventBus, KafkaConfig, EventBus};
use whatsapp_client::{
    client::{
        message_types::{WhatsAppMessageSend, WhatsAppMessage, ResponsePriority},
        builders::InteractiveMessageBuilder,
    },
};
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    // Load environment variables
    dotenv::dotenv().ok();

    // Get phone number from environment variable or command line argument
    let phone_number = if let Some(arg_phone) = env::args().nth(1) {
        arg_phone
    } else {
        env::var("TEST_PHONE_NUMBER")
            .map_err(|_| "TEST_PHONE_NUMBER environment variable must be set or provide phone number as argument")?
    };

    info!("📱 Preparing to send interactive test message to {}", phone_number);

    // Create Kafka event bus
    let kafka_config = KafkaConfig::from_env()
        .map_err(|e| format!("Failed to load Kafka config: {}", e))?;
    
    let event_bus = KafkaEventBus::new(kafka_config).await
        .map_err(|e| format!("Failed to create event bus: {}", e))?;

    // Create an interactive message using the builder
    let message = InteractiveMessageBuilder::new()
        .to(&phone_number)
        .body("Choose a product category:")
        .header("Product Catalog")
        .footer("Free shipping on orders over $50")
        .list_button("Browse Products")
        .add_list_section("Electronics")
        .add_list_row("phones", "Smartphones", "Latest models")
        .add_list_row("laptops", "Laptops", "Business and gaming")
        .add_list_section("Clothing")
        .add_simple_list_row("mens", "Men's Clothing")
        .add_simple_list_row("women", "Women's Clothing")
        .build()?;

    let whatsapp_message = WhatsAppMessage::Interactive(message);

    // Create the WhatsAppMessageSend event
    let message_send = WhatsAppMessageSend::new(
        "wamid.HBgLNzk4MTAxMDk1ODIVAgASGCA2RDY3QjdFRkI5QkRDMTBENkEwRjhBQkI3RkRBNzIyMAA=".to_string(),
        whatsapp_message,
        ResponsePriority::Normal,
    );

    // Publish the event
    info!("🚀 Publishing interactive WhatsApp message send event to Kafka...");
    event_bus.publish(message_send).await?;

    info!("✅ Interactive test message published successfully!");
    info!("💡 Check your WhatsApp client logs to see the message being processed");
    info!("📱 The recipient should receive a message with 3 buttons: Get Help, Contact Support, More Info");

    Ok(())
}
