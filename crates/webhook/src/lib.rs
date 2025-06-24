pub mod config;
pub mod state;
pub mod types;
pub mod routes;
pub mod handlers;

pub use routes::create_route;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {

    let config = config::AppConfig::from_env();
    tracing::info!("📓 Configuration loaded");
    tracing::info!("🔧 Api Version: {}", config.api_version);
    tracing::info!("🔧 Phone Number ID: {}", config.phone_number_id);
    tracing::info!("💡 Verify Token: {}", config.verify_token);

    let http_client = reqwest::Client::new();

    let state = state::AppState {
        config: config.clone(),
        http_client,
    };

    let app = routes::create_route(state);

    let addr = config.listen_address();
    tracing::info!("🌐 Server starting on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
