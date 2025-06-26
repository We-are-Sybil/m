pub mod config;
pub mod consumer;
pub mod producer;
pub mod processor;

pub use config::AppConfig;
pub use consumer::MessageConsumer;

use common::{WebhookEvent, AIResponse, ProcessingError};

use anyhow::Result;
use tracing::{info, error};


pub struct KafkaService {
    pub config: AppConfig,
}
