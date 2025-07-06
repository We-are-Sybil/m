use serde::Deserialize;

/// Configuration for the WhatsApp client service
///
/// THis contains all the settings needed to both WhatsApp's Business API
/// and personal Kafka infrastructure. Each setting has sensible defaults
/// where possible to minimize configuration overhead.
#[derive(Deserialize, Debug, Clone)]
pub struct WhatsAppClientConfig {
    /// WhatsApp Business API access token
    pub access_token: String,
    /// WhatsApp Business version (e.g., "v23.0")
    pub api_version: String,
    /// WhatsApp Business phone number ID
    pub phone_number_id: String,
    /// Base URL for WhatsApp Graph API
    pub api_base_url: String,

    // Rate limiting configuration
    /// Maximum API calls per minute (WhatsApp allows 1000/min by default)
    pub max_api_calls_per_minute: u32,
    /// Burst capacity for rate limiter
    pub rate_limit_burst: u32,

    // HTTP client configuration
    /// Timeout for individual API calls in seconds
    pub request_timeout_seconds: u64,
    /// Maximum number of concurrent API calls
    pub max_concurrent_requests: usize,

    // Service configuration
    /// Host to bind the service to
    pub host: String,
    /// Port for health checks endpoint
    pub port: u16,
}
