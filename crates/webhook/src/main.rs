use webhook::run_server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter("webhook=info,tower_http=debug")
        .init();

    run_server().await
}

