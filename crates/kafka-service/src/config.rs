use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct KafkaConfig {
    pub bootstrap_servers: String,

    pub consumer_group_id: String,
    pub input_topic: String,

    pub output_topic: String,

    pub batch_size: usize,
    pub processing_timeout_ms: u64,
}

impl KafkaConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        Self {
            bootstrap_servers: std::env::var("KAFKA_BOOTSTRAP_SERVERS")
                .unwrap_or_else(|_| "localhost:9092,localhost:9094,localhost:9096".to_string()),

            consumer_group_id: std::env::var("KAFKA_CONSUMER_GROUP_ID")
                .expect("KAFKA_CONSUMER_GROUP_ID must be set"),

            input_topic: std::env::var("KAFKA_INPUT_TOPIC")
                .expect("KAFKA_INPUT_TOPIC must be set"),

            output_topic: std::env::var("KAFKA_OUTPUT_TOPIC")
                .expect("KAFKA_OUTPUT_TOPIC must be set"),  

            batch_size: std::env::var("KAFKA_BATCH_SIZE")
                .unwrap_or_else(|_| "100".to_string())
                .parse()
                .expect("KAFKA_BATCH_SIZE must be a valid number"),

            processing_timeout_ms: std::env::var("KAFKA_TIMEOUT_MS")
                .unwrap_or_else(|_| "5000".to_string())
                .parse()
                .expect("KAFKA_TIMEOUT_MS must be a valid number"),
        }
    }

    pub fn bootstrap_servers(&self) -> Vec<&str> {
        self.bootstrap_servers
            .split(',')
            .map(|s| s.trim())
            .collect()
    }
}
