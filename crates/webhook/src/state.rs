use crate::config::AppConfig;
use common::KafkaEventBus;

use reqwest::Client;
use std::sync::Arc;

/// Application state shared across all webhook handlers
/// 
/// This contains the configuration and shared resources that handlers
/// need to process webhook requests and publish events to the Kafka.
/// Designed to be cloned efficiently across handler invocations.
#[derive(Clone)]
pub struct AppState {
    /// Configuration loaded from environment variables.
    pub config: AppConfig,
    /// HTTP client for making external requests (if needed for webhook validation).
    pub http_client: Client,
    /// Kafka event bus for publishing domain events to the cluster.
    pub event_bus: Arc<KafkaEventBus>,
}

impl AppState {
    /// Create new application state with the enhanced event bus
    ///
    /// # Arguments
    /// * `config` - Application configuration including WhatsApp API creds.
    /// * `event_bus` - Arc-wrapped Kafka event bus for publishing events.
    ///
    /// # Panics
    /// Panics if the HTTP client cannot be created, which should not happen
    pub fn new(
        config: AppConfig,
        event_bus: Arc<KafkaEventBus>,
    ) -> Self {
        // Create HTTP client with reasonable timeouts for any external requests
        let http_client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
        
        Self {
            config,
            http_client,
            event_bus,
        }
    }
    
    /// Get a reference to the event bus for publishing events
    /// 
    /// This provides access to the event bus while maintaining the Arc wrapper
    /// for efficient cloning across async contexts.
    pub fn event_bus(&self) -> &Arc<KafkaEventBus> {
        &self.event_bus
    }
}
