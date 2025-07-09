use crate::{
    config::WhatsAppClientConfig,
    errors::{WhatsAppError, WhatsAppResult, WhatsAppApiErrorResponse},
    client::{
        responses::WhatsAppMessageResponse,
        
        message_types::{
            WhatsAppMessage,
            Message,
        },
    },
};
use reqwest::{
    Client, 
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}
};
use serde::Serialize;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};
use governor::{
    Quota, 
    clock::DefaultClock,
    RateLimiter, 
    state::{InMemoryState, NotKeyed}
};

/// Core WhatsApp Business API client focused on HTTP communication
/// 
/// This client handles the low-level HTTP communication with WhatsApp's API.
/// It provides a clean interface for sending any type of message payload
/// while handling rate limiting, retries, and error processing.
/// 
/// The client is designed to be message-type agnostic - it accepts any
/// serializable payload and handles the communication details.
pub struct WhatsAppClient {
    /// HTTP client for making API requests
    http_client: Client,
    /// Configuration containing credentials and settings
    config: WhatsAppClientConfig,
    /// Rate limiter to prevent hitting WhatsApp's API limits
    rate_limiter: RateLimiter<NotKeyed, InMemoryState, DefaultClock>,
    /// Base headers that are sent with every request
    default_headers: HeaderMap,
    /// Base URL for all WhatsApp API endpoints
    base_url: String,
}

impl WhatsAppClient {
    /// Create a new WhatsApp API client
    /// 
    /// This initializes the HTTP client with optimized settings for WhatsApp's API,
    /// sets up rate limiting, and prepares authentication headers.
    pub fn new(config: WhatsAppClientConfig) -> WhatsAppResult<Self> {
        // Create HTTP client with optimized settings for WhatsApp API
        let http_client = Client::builder()
            .timeout(Duration::from_secs(config.request_timeout_seconds))
            .user_agent("rust-whatsapp-client/1.0")
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(config.max_concurrent_requests)
            .build()
            .map_err(|e| WhatsAppError::ConfigurationError(
                format!("Failed to create HTTP client: {}", e)
            ))?;
        
        // Set up rate limiter using token bucket algorithm
        let rate_limit_per_minute = std::num::NonZeroU32::new(config.rate_limit_per_minute)
            .ok_or_else(|| WhatsAppError::ConfigurationError(
                "Rate limit per minute must be greater than 0".to_string()
            ))?;
        let rate_limit_burst = std::num::NonZeroU32::new(config.rate_limit_burst)
            .ok_or_else(|| WhatsAppError::ConfigurationError(
                "Rate limit burst must be greater than 0".to_string()
            ))?;
        let quota = Quota::per_minute(rate_limit_per_minute)
            .allow_burst(rate_limit_burst);
        let rate_limiter = RateLimiter::direct(quota);
        
        // Prepare default headers for all requests
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&config.authorization_header())
                .map_err(|e| WhatsAppError::ConfigurationError(
                    format!("Invalid access token format: {}", e)
                ))?
        );
        default_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_static("application/json")
        );
        
        let base_url = config.messages_url();
        
        info!(
            "WhatsApp client initialized for phone number ID: {}, rate limit: {}/min",
            config.phone_number_id, config.rate_limit_per_minute
        );
        
        Ok(Self {
            http_client,
            config,
            rate_limiter,
            default_headers,
            base_url,
        })
    }
    
    /// Send any message payload to WhatsApp API
    /// 
    /// This is the core method that all message types use. It handles:
    /// - Rate limiting
    /// - Request serialization
    /// - HTTP communication
    /// - Retry logic with exponential backoff
    /// - Error handling and classification
    /// 
    /// The payload should be any struct that implements Serialize and
    /// matches WhatsApp's API format for the specific message type.
    pub async fn send_message(&self, payload: WhatsAppMessage) -> WhatsAppResult<WhatsAppMessageResponse> {
        match payload {
            WhatsAppMessage::Text(msg) => self.send_message_with_retry(&msg).await,
            WhatsAppMessage::Audio(msg) => self.send_message_with_retry(&msg).await,
            WhatsAppMessage::Contact(msg) => self.send_message_with_retry(&msg).await,
            WhatsAppMessage::Document(msg) => self.send_message_with_retry(&msg).await,
            WhatsAppMessage::Image(msg) => self.send_message_with_retry(&msg).await,
            WhatsAppMessage::Interactive(msg) => self.send_message_with_retry(&msg).await,
            WhatsAppMessage::Location(msg) => self.send_message_with_retry(&msg).await,
            WhatsAppMessage::Video(msg) => self.send_message_with_retry(&msg).await,
        }
    }
    
    /// Core retry logic for message sending
    /// 
    /// This implements intelligent retry with exponential backoff.
    /// Different error types get different retry treatments based on
    /// whether they're likely to succeed on retry.
    async fn send_message_with_retry<T>(&self, payload: &T) -> WhatsAppResult<WhatsAppMessageResponse> 
        where T: Message + Serialize
    {
        for attempt in 1..=self.config.max_retry_attempts {
            // Wait for rate limiter - this ensures we don't exceed WhatsApp's limits
            self.rate_limiter.until_ready().await;
            
            debug!("Attempt {} of {} for message send", attempt, self.config.max_retry_attempts);
            
            match self.send_message_once(payload).await {
                Ok(response) => {
                    debug!("Message sent successfully on attempt {}", attempt);
                    return Ok(response);
                }
                Err(error) => {
                    error!("Attempt {} failed: {}", attempt, error);
                    
                    // Check if we should retry this error
                    if !error.is_retryable() {
                        warn!("Error is not retryable, giving up: {}", error);
                        return Err(error);
                    }
                    
                    // Calculate delay for next attempt
                    if attempt < self.config.max_retry_attempts {
                        let delay = self.calculate_retry_delay(attempt, &error);
                        info!("Retrying in {} seconds (attempt {} of {})", 
                              delay.as_secs(), attempt + 1, self.config.max_retry_attempts);
                        sleep(delay).await;
                    }
                }
            }
        }
        
        // All retries exhausted
        Err(WhatsAppError::MaxRetriesExceeded {
            attempts: self.config.max_retry_attempts,
            operation: "send_message".to_string(),
        })
    }
    
    /// Send a single message attempt without retry logic
    /// 
    /// This method focuses purely on HTTP communication with WhatsApp's API.
    /// All retry logic is handled at a higher level.
    async fn send_message_once<T>(&self, payload: &T) -> WhatsAppResult<WhatsAppMessageResponse> 
        where T: Message + serde::Serialize
    {
        // Serialize the payload to JSON
        let json_payload = serde_json::to_value(&payload)
            .map_err(WhatsAppError::SerializationError)?;
        let response = self.http_client
            .post(&self.base_url)
            .headers(self.default_headers.clone())
            .json(&json_payload)
            .send()
            .await?;
        
        let status = response.status();
        let response_text = response.text().await?;
        
        if status.is_success() {
            // Parse successful response
            let message_response: WhatsAppMessageResponse = serde_json::from_str(&response_text)
                .map_err(WhatsAppError::SerializationError)?;
            
            info!("Message sent successfully: {}", message_response.messages[0].id);
            Ok(message_response)
        } else {
            // Parse error response
            match serde_json::from_str::<WhatsAppApiErrorResponse>(&response_text) {
                Ok(error_response) => {
                    Err(WhatsAppError::from_api_response(error_response))
                }
                Err(_) => {
                    // Couldn't parse error response, create a generic error
                    Err(WhatsAppError::ApiError {
                        code: status.as_u16() as u32,
                        message: format!("HTTP {} error: {}", status, response_text),
                        error_data: None,
                    })
                }
            }
        }
    }
    
    /// Calculate exponential backoff delay for retries
    fn calculate_retry_delay(&self, attempt: u32, error: &WhatsAppError) -> Duration {
        // Start with error-specific delay if available
        let base_delay = error.retry_delay_seconds()
            .unwrap_or_else(|| {
                // Fallback to exponential backoff
                let delay_ms = self.config.initial_retry_delay_ms * (2_u64.pow(attempt - 1));
                std::cmp::min(delay_ms, self.config.max_retry_delay_ms) / 1000
            });
        
        Duration::from_secs(base_delay)
    }
    
    /// Get client configuration (useful for debugging and monitoring)
    pub fn config(&self) -> &WhatsAppClientConfig {
        &self.config
    }
    
    /// Check if the rate limiter has capacity for immediate requests
    /// 
    /// This can be useful for monitoring and deciding whether to
    /// queue messages or wait before sending more.
    pub async fn has_rate_capacity(&self) -> bool {
        // Check if we can make a request without waiting
        self.rate_limiter.check().is_ok()
    }
    
    /// Get current rate limiter state for monitoring
    /// 
    /// Returns the number of tokens currently available in the rate limiter.
    /// This is useful for metrics and monitoring systems.
    pub fn rate_limiter_tokens_available(&self) -> u32 {
        // This is an approximation since governor doesn't expose exact token count
        if self.rate_limiter.check().is_ok() {
            self.config.rate_limit_burst // Assume we have burst capacity available
        } else {
            0 // Rate limited
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    fn create_test_config() -> WhatsAppClientConfig {
        WhatsAppClientConfig {
            access_token: "test_token".to_string(),
            api_version: "v23.0".to_string(),
            phone_number_id: "123456789".to_string(),
            api_base_url: "https://graph.facebook.com".to_string(),
            rate_limit_per_minute: 800,
            rate_limit_burst: 50,
            request_timeout_seconds: 30,
            max_concurrent_requests: 20,
            max_retry_attempts: 3,
            initial_retry_delay_ms: 1000,
            max_retry_delay_ms: 30000,
            host: "0.0.0.0".to_string(),
            port: 8001,
        }
    }
    
    #[tokio::test]
    async fn test_client_initialization() {
        let config = create_test_config();
        let client = WhatsAppClient::new(config.clone());
        
        assert!(client.is_ok());
        
        let client = client.unwrap();
        assert_eq!(client.config().phone_number_id, "123456789");
        assert_eq!(client.config().api_version, "v23.0");
    }
    
    #[tokio::test]
    async fn test_rate_limiter_functionality() {
        let config = create_test_config();
        let client = WhatsAppClient::new(config).unwrap();
        
        // Should have capacity initially
        assert!(client.has_rate_capacity().await);
        
        // Token count should be non-zero initially
        assert!(client.rate_limiter_tokens_available() > 0);
    }
    
    #[tokio::test]
    async fn test_payload_serialization() {
        // Test that we can serialize a simple payload
        let test_payload = json!({
            "messaging_product": "whatsapp",
            "to": "+1234567890",
            "type": "text",
            "text": {
                "body": "Test message"
            }
        });
        
        // This should serialize without error (actual sending would fail without valid credentials)
        let serialized = serde_json::to_value(&test_payload);
        assert!(serialized.is_ok());
    }
}
