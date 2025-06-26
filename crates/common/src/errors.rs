use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessingError {
    #[error("Failed to serialize message: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Kafka producer error: {0}")]
    ProducerError(String),
    
    #[error("Kafka consumer error: {0}")]
    ConsumerError(String),
    
    #[error("Message processing timeout")]
    TimeoutError,
    
    #[error("Invalid message format: {0}")]
    InvalidMessage(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Batch error: {0}")]
    BatchError(String, Option<Box<ProcessingError>>),
}
