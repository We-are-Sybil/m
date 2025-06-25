use axum::{
    routing::{get},
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    trace::TraceLayer,
};
use http::Method;

use crate::{
    handlers,
    state::AppState,
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/webhook", get(handlers::webhook::verify_webhook))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new().allow_origin(tower_http::cors::Any).allow_methods([Method::GET]))
        )
}
