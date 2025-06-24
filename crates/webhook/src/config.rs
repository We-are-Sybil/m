use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub verify_token: String,
    pub access_token: String,
    pub api_version: String,
    pub phone_number_id: String,
    pub max_file_size_mb: u64,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();
        Self {
            verify_token: std::env::var("WEBHOOK_VERIFY_TOKEN").expect("VERIFY_TOKEN must be set"),
            access_token: std::env::var("WEBHOOK_ACCESS_TOKEN").expect("ACCESS_TOKEN must be set"),
            api_version: std::env::var("WEBHOOK_API_VERSION").unwrap_or_else(|_| "v23.0".to_string()),
            phone_number_id: std::env::var("WEBHOOK_PHONE_NUMBER_ID").expect("PHONE_NUMBER_ID must be set"),
            max_file_size_mb: std::env::var("WEBHOOK_MAX_FILE_SIZE_MB")
                .unwrap_or_else(|_| "25".to_string())
                .parse()
                .expect("WEBHOOK_MAX_FILE_SIZE_MB must be a valid number"),
            host: std::env::var("WEBHOOK_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("WEBHOOK_PORT")
                .unwrap_or_else(|_| "8000".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            }
    }

    pub fn listen_address(&self) -> std::net::SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid host or port")
    }
}


