use kafka_service::KafkaService;
use anyhow::Result;
use tracing::{error, info};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("kafka_service=info, rdkafka=info")
        .init();

    let mut service = KafkaService::new().await?;

    let ctrl_c = tokio::signal::ctrl_c();

    tokio::select! {
        result = service.run() => {
            if let Err(e) = result {
                error!("❌ Kafka service encountered an error: {}", e);
            }
        }
        _ = ctrl_c => {
            error!("🔌 Received Ctrl+C signal, shutting down gracefully...");
            if let Err(e) = service.shutdown().await {
                error!("❌ Error during shutdown: {}", e);
            } else {
                info!("✅ Kafka service shutdown successfully");
            }
        }
    }

    Ok(())
}
