pub mod config;
pub mod state;
pub mod types;
pub mod routes;
pub mod handlers;
pub mod event_publisher;

pub use routes::create_route;

use common::{KafkaEventBus, KafkaConfig, EventBus};
use std::sync::Arc;

/// Run the webhook server with enhanced event bus integration
/// 
/// This initializes the complete webhook service including:
/// - Configuration from environment variables
/// - Enhanced Kafka event bus with consumer and retry support
/// - HTTP server for handling WhatsApp webhooks
/// - Proper error handling and logging
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    // Load webhook configuration
    let config = config::AppConfig::from_env();
    tracing::info!("📓 Webhook configuration loaded successfully");
    tracing::info!("🔧 API Version: {}", config.api_version);
    tracing::info!("🔧 Phone Number ID: {}", config.phone_number_id);
    tracing::info!("💡 Verify Token: ...{}", &config.verify_token[config.verify_token.len().saturating_sub(4)..]);
    
    // Initialize enhanced Kafka event bus with full consumer support
    tracing::info!("🔧 Initializing Kafka event bus...");
    let kafka_config = KafkaConfig::from_env()
        .map_err(|e| format!("Failed to load Kafka configuration: {}", e))?;
    
    let event_bus = KafkaEventBus::new(kafka_config).await
        .map_err(|e| format!("Failed to initialize Kafka event bus: {}", e))?;
    
    // Wrap in the trait object that our application state expects
    let event_bus: Arc<dyn EventBus<Error = common::EventBusError> + Send + Sync> = Arc::new(event_bus);
    
    // Verify event bus connectivity before proceeding
    tracing::info!("🔍 Testing event bus connectivity...");
    event_bus.health_check().await
        .map_err(|e| format!("Event bus health check failed: {}", e))?;
    tracing::info!("✅ Event bus connected and healthy");
    
    // Create application state with the enhanced event bus
    let state = state::AppState::new(config.clone(), event_bus);
    
    // Create and configure the HTTP router with middleware
    let app = routes::create_route(state);
    
    // Start the HTTP server
    let addr = config.listen_address();
    tracing::info!("🌐 Webhook server starting on {}", addr);
    tracing::info!("📡 Ready to receive WhatsApp webhooks and publish events");
    
    let listener = tokio::net::TcpListener::bind(addr).await
        .map_err(|e| format!("Failed to bind to address {}: {}", addr, e))?;
    
    // Start serving requests
    axum::serve(listener, app.into_make_service()).await
        .map_err(|e| format!("Server error: {}", e))?;
    
    Ok(())
}
