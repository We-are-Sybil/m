use serde::Deserialize;

/// Configuration for the WhatsApp client service
///
/// THis contains all the settings needed to both WhatsApp's Business API
/// and personal Kafka infrastructure. Each setting has sensible defaults
/// where possible to minimize configuration overhead.
#[derive(Deserialize, Debug, Clone)]
pub struct WhatsAppClientConfig {
    // WhatsApp Business API configuration
    /// Your WhatsApp Business API access token
    pub access_token: String,
    /// WhatsApp Business API version (e.g., "v23.0")
    pub api_version: String,
    /// Your WhatsApp Business phone number ID
    pub phone_number_id: String,
    /// Base URL for WhatsApp Graph API
    pub api_base_url: String,
    
    // Rate limiting configuration
    /// Maximum API calls per minute (WhatsApp allows 1000/min by default)
    pub rate_limit_per_minute: u32,
    /// Burst capacity for rate limiter
    pub rate_limit_burst: u32,
    
    // HTTP client configuration
    /// Timeout for individual API calls in seconds
    pub request_timeout_seconds: u64,
    /// Maximum number of concurrent API calls
    pub max_concurrent_requests: usize,
    
    // Retry configuration
    /// Maximum retry attempts for failed API calls
    pub max_retry_attempts: u32,
    /// Initial retry delay in milliseconds
    pub initial_retry_delay_ms: u64,
    /// Maximum retry delay in milliseconds (for exponential backoff)
    pub max_retry_delay_ms: u64,
    
    // Service configuration
    /// Host to bind the service to
    pub host: String,
    /// Port for health check endpoint
    pub port: u16,}

impl WhatsAppClientConfig {
    /// Load configuration from environment variables
    ///
    /// This follows the same pattern as your webhook service, making it
    /// easy to configure consistently across your infrastructure.
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            // WhatsApp API credentials - these must be set
            access_token: std::env::var("WHATSAPP_ACCESS_TOKEN")
                .expect("WHATSAPP_ACCESS_TOKEN environment variable must be set"),
            api_version: std::env::var("WHATSAPP_API_VERSION")
                .unwrap_or_else(|_| "v23.0".to_string()),
            phone_number_id: std::env::var("WHATSAPP_PHONE_NUMBER_ID")
                .expect("WHATSAPP_PHONE_NUMBER_ID environment variable must be set"),
            api_base_url: std::env::var("WHATSAPP_API_BASE_URL")
                .unwrap_or_else(|_| "https://graph.facebook.com".to_string()),
            
            // Rate limiting - conservative defaults to avoid hitting WhatsApp limits
            rate_limit_per_minute: std::env::var("WHATSAPP_RATE_LIMIT_PER_MINUTE")
                .unwrap_or_else(|_| "800".to_string()) // 80% of WhatsApp's 1000/min limit
                .parse()
                .expect("WHATSAPP_RATE_LIMIT_PER_MINUTE must be a valid number"),
            rate_limit_burst: std::env::var("WHATSAPP_RATE_LIMIT_BURST")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .expect("WHATSAPP_RATE_LIMIT_BURST must be a valid number"),
            
            // HTTP client settings - optimized for reliability
            request_timeout_seconds: std::env::var("WHATSAPP_REQUEST_TIMEOUT_SECONDS")
                .unwrap_or_else(|_| "30".to_string())
                .parse()
                .expect("WHATSAPP_REQUEST_TIMEOUT_SECONDS must be a valid number"),
            max_concurrent_requests: std::env::var("WHATSAPP_MAX_CONCURRENT_REQUESTS")
                .unwrap_or_else(|_| "20".to_string())
                .parse()
                .expect("WHATSAPP_MAX_CONCURRENT_REQUESTS must be a valid number"),
            
            // Retry configuration - aggressive retries for reliability
            max_retry_attempts: std::env::var("WHATSAPP_MAX_RETRY_ATTEMPTS")
                .unwrap_or_else(|_| "3".to_string())
                .parse()
                .expect("WHATSAPP_MAX_RETRY_ATTEMPTS must be a valid number"),
            initial_retry_delay_ms: std::env::var("WHATSAPP_INITIAL_RETRY_DELAY_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()
                .expect("WHATSAPP_INITIAL_RETRY_DELAY_MS must be a valid number"),
            max_retry_delay_ms: std::env::var("WHATSAPP_MAX_RETRY_DELAY_MS")
                .unwrap_or_else(|_| "30000".to_string())
                .parse()
                .expect("WHATSAPP_MAX_RETRY_DELAY_MS must be a valid number"),
            
            // Service configuration
            host: std::env::var("WHATSAPP_CLIENT_HOST")
                .unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("WHATSAPP_CLIENT_PORT")
                .unwrap_or_else(|_| "8001".to_string())
                .parse()
                .expect("WHATSAPP_CLIENT_PORT must be a valid number"),
        }
    }

    /// Get the complete URL for sending messages via WhatsApp API
    ///
    /// This constructs the full endpoint URL that will be used to send
    /// messages. The formaat follows the WhatsApp API's requirements.
    pub fn messages_url(&self) -> String {
        format!(
            "{}/{}/{}/messages",
            self.api_base_url,
            self.api_version,
            self.phone_number_id,
        )
    }

    /// Get the authorization header value for WhatsApp API requests
    pub fn authorization_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messages_url_construction() {
        let config = WhatsAppClientConfig {
            api_base_url: "https://graph.facebook.com".to_string(),
            api_version: "v23.0".to_string(),
            phone_number_id: "123456789".to_string(),
            // ... other fields would be filled with test values
            access_token: "test_token".to_string(),
            rate_limit_per_minute: 800,
            rate_limit_burst: 50,
            request_timeout_seconds: 30,
            max_concurrent_requests: 20,
            max_retry_attempts: 3,
            initial_retry_delay_ms: 1000,
            max_retry_delay_ms: 30000,
            host: "0.0.0.0".to_string(),
            port: 8001,
        };
        
        let expected_url = "https://graph.facebook.com/v23.0/123456789/messages";
        assert_eq!(config.messages_url(), expected_url);
    }
    
    #[test]
    fn test_authorization_header() {
        let config = WhatsAppClientConfig {
            access_token: "test_token_123".to_string(),
            // ... other fields would be filled with test values
            api_base_url: "https://graph.facebook.com".to_string(),
            api_version: "v23.0".to_string(),
            phone_number_id: "123456789".to_string(),
            rate_limit_per_minute: 800,
            rate_limit_burst: 50,
            request_timeout_seconds: 30,
            max_concurrent_requests: 20,
            max_retry_attempts: 3,
            initial_retry_delay_ms: 1000,
            max_retry_delay_ms: 30000,
            host: "0.0.0.0".to_string(),
            port: 8001,
        };
        
        assert_eq!(config.authorization_header(), "Bearer test_token_123");
    }
}
