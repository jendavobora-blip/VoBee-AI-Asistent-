//! # VoBee API Server
//!
//! REST API server for VoBee AI Assistant providing HTTP endpoints
//! for chat functionality and system management.

pub mod routes;
pub mod handlers;
pub mod state;
pub mod error;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub use error::{ApiError, ApiResult};
pub use state::AppState;

/// Create the API router with all routes
pub fn create_router(state: AppState) -> Router {
    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(handlers::health_check))
        .route("/api/chat", post(handlers::chat))
        .route("/api/chat/history", get(handlers::get_history))
        .route("/api/chat/clear", post(handlers::clear_history))
        .with_state(state)
        .layer(cors)
}

/// Initialize logging
pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "vobee_api_server=info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}
