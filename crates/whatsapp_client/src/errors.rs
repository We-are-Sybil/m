use thiserror::Error;

/// Comprehensive error types for WhatsApp API client operations
///
/// This herror hierarchy separates different failure modes so each of 
/// them can be handled appropriately.
#[derive(Error, Debug)]
pub enum WhatsAppError {
    /// HTTP request failed (network issues, timeouts, etc.)
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    /// WhatsApp API returned an error response
    #[error("WhatsApp API error: {code} - {message}")]
    ApiError {
        code: u32,
        message: String,
        error_data: Option<serde_json::Value>,
    },
    
    /// Rate limit exceeded - we hit WhatsApp's rate limits
    #[error("Rate limit exceeded: {message}")]
    RateLimitExceeded {
        message: String,
        retry_after_seconds: Option<u64>,
    },
    
    /// Authentication failed - invalid access token
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),
    
    /// Request serialization failed - our data couldn't be converted to JSON
    #[error("Failed to serialize request: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// Invalid phone number format
    #[error("Invalid phone number format: {0}")]
    InvalidPhoneNumber(String),
    
    /// Message content is invalid (too long, unsupported format, etc.)
    #[error("Invalid message content: {0}")]
    InvalidMessageContent(String),
    
    /// Service configuration is invalid
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    /// Operation timed out
    #[error("Operation timed out after {seconds} seconds")]
    TimeoutError { seconds: u64 },
    
    /// Too many retry attempts exhausted
    #[error("Maximum retry attempts ({attempts}) exceeded for operation: {operation}")]
    MaxRetriesExceeded { attempts: u32, operation: String },
    
    /// Generic internal error for unexpected situations
    #[error("Internal error: {0}")]
    InternalError(String),
}

/// WhatsApp API error response structure
///
/// This matches the error format that WhatsApp's Business API returns.
/// Understanding this structure helps extracting useful information
/// and respond appropriately to different error scenarios.
#[derive(serde::Deserialize, Debug)]
pub struct WhatsAppApiErrorResponse {
    pub error: WhatsAppApiError,
}

#[derive(serde::Deserialize, Debug)]
pub struct WhatsAppApiError {
    /// Error message from WhatsApp
    pub message: String,
    /// Error type identifier
    #[serde(rename = "type")]
    pub error_type: String,
    /// Numeric error code
    pub code: u32,
    /// Additional error details
    pub error_data: Option<serde_json::Value>,
    /// Facebook trace ID for debugging
    pub fbtrace_id: Option<String>,
}

impl WhatsAppError {
    /// Create an API error from a WhatsApp error response
    /// 
    /// This factory method helps us convert WhatsApp's error format
    /// into our internal error representation while preserving all
    /// the relevant debugging information.
    pub fn from_api_response(response: WhatsAppApiErrorResponse) -> Self {
        let api_error = response.error;
        
        // Check for specific error types that need special handling
        match api_error.code {
            // Authentication errors (4xx range)
            190 | 401 => WhatsAppError::AuthenticationError(api_error.message),
            
            // Rate limiting errors
            429 | 80007 => {
                // Try to extract retry-after from error_data if available
                let retry_after = api_error.error_data
                    .as_ref()
                    .and_then(|data| data.get("retry_after"))
                    .and_then(|val| val.as_u64());
                
                WhatsAppError::RateLimitExceeded {
                    message: api_error.message,
                    retry_after_seconds: retry_after,
                }
            },
            
            // Invalid phone number errors
            131051 | 131052 | 131053 => {
                WhatsAppError::InvalidPhoneNumber(api_error.message)
            },
            
            // Invalid message content errors
            131047 | 131048 | 131049 => {
                WhatsAppError::InvalidMessageContent(api_error.message)
            },
            
            // Generic API error for everything else
            _ => WhatsAppError::ApiError {
                code: api_error.code,
                message: api_error.message,
                error_data: api_error.error_data,
            },
        }
    }
    
    /// Check if this error is retryable
    /// 
    /// This is crucial for our retry logic. Some errors (like network timeouts)
    /// should be retried, while others (like authentication failures) should not.
    pub fn is_retryable(&self) -> bool {
        match self {
            // These errors might resolve themselves on retry
            WhatsAppError::HttpError(reqwest_error) => {
                // Retry timeouts and connection errors, but not client errors
                reqwest_error.is_timeout() 
                    || reqwest_error.is_connect()
                    || reqwest_error.is_request()
            },
            WhatsAppError::RateLimitExceeded { .. } => true,
            WhatsAppError::TimeoutError { .. } => true,
            WhatsAppError::ApiError { code, .. } => {
                // Only retry server errors (5xx), not client errors (4xx)
                *code >= 500 && *code < 600
            },
            
            // These errors are permanent and should not be retried
            WhatsAppError::AuthenticationError(_) => false,
            WhatsAppError::InvalidPhoneNumber(_) => false,
            WhatsAppError::InvalidMessageContent(_) => false,
            WhatsAppError::ConfigurationError(_) => false,
            WhatsAppError::SerializationError(_) => false,
            WhatsAppError::MaxRetriesExceeded { .. } => false,
            WhatsAppError::InternalError(_) => false,
        }
    }
    
    /// Get the suggested delay before retrying (in seconds)
    /// 
    /// This implements intelligent retry delays based on the error type.
    /// Rate limit errors get longer delays, while network errors get shorter ones.
    pub fn retry_delay_seconds(&self) -> Option<u64> {
        match self {
            WhatsAppError::RateLimitExceeded { retry_after_seconds, .. } => {
                // Use WhatsApp's suggested delay, or default to 60 seconds
                Some(retry_after_seconds.unwrap_or(60))
            },
            WhatsAppError::HttpError(_) => Some(5), // Quick retry for network issues
            WhatsAppError::TimeoutError { .. } => Some(10),
            WhatsAppError::ApiError { code, .. } => {
                if *code >= 500 { Some(30) } else { None } // Only retry server errors
            },
            _ => None, // Non-retryable errors
        }
    }
}

/// Result type alias for WhatsApp operations
/// 
/// This makes function signatures cleaner and provides a consistent
/// return type across all WhatsApp client operations.
pub type WhatsAppResult<T> = Result<T, WhatsAppError>;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_rate_limit_error_is_retryable() {
        let error = WhatsAppError::RateLimitExceeded {
            message: "Rate limit hit".to_string(),
            retry_after_seconds: Some(60),
        };
        
        assert!(error.is_retryable());
        assert_eq!(error.retry_delay_seconds(), Some(60));
    }
    
    #[test]
    fn test_auth_error_not_retryable() {
        let error = WhatsAppError::AuthenticationError("Invalid token".to_string());
        
        assert!(!error.is_retryable());
        assert_eq!(error.retry_delay_seconds(), None);
    }
    
    #[test]
    fn test_api_error_classification() {
        // Test server error (retryable)
        let server_error = WhatsAppError::ApiError {
            code: 500,
            message: "Internal server error".to_string(),
            error_data: None,
        };
        assert!(server_error.is_retryable());
        
        // Test client error (not retryable)
        let client_error = WhatsAppError::ApiError {
            code: 400,
            message: "Bad request".to_string(),
            error_data: None,
        };
        assert!(!client_error.is_retryable());
    }
}
