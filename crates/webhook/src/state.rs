// crates/webhook/src/state.rs - Updated state with enhanced event bus integration
use crate::config::AppConfig;
use common::{EventBus, EventBusError};

use reqwest::Client;
use std::sync::Arc;

/// Application state shared across all webhook handlers
/// 
/// This contains the configuration and shared resources that handlers
/// need to process webhook requests and publish events. The state is designed
/// to be cloned efficiently across handler invocations while sharing the
/// underlying resources like the HTTP client and event bus connection.
#[derive(Clone)]
pub struct AppState {
    /// Configuration loaded from environment variables
    pub config: AppConfig,
    /// HTTP client for making external requests (if needed for webhook validation)
    pub http_client: Client,
    /// Event bus for publishing domain events to the Kafka cluster
    /// This uses the enhanced EventBus trait with full consumer and retry support
    pub event_bus: Arc<dyn EventBus<Error = EventBusError> + Send + Sync>,
}

impl AppState {
    /// Create new application state with the enhanced event bus
    /// 
    /// This initializes the shared state that will be passed to all
    /// webhook handlers. The event bus provides reliable event publishing
    /// with automatic retry logic and dead letter queue support.
    pub fn new(
        config: AppConfig,
        event_bus: Arc<dyn EventBus<Error = EventBusError> + Send + Sync>,
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
    pub fn event_bus(&self) -> &Arc<dyn EventBus<Error = EventBusError> + Send + Sync> {
        &self.event_bus
    }
}
