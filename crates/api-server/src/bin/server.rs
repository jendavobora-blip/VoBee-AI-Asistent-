use std::net::SocketAddr;
use vobee_ai_orchestration::{AiConfig, AiOrchestrator};
use vobee_api_server::{create_router, init_logging, AppState};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_logging();

    // Load AI configuration from environment
    let ai_config = AiConfig::from_env();

    // Create orchestrator
    let orchestrator = AiOrchestrator::new(ai_config);

    // Create application state
    let state = AppState::new(orchestrator);

    // Create router
    let app = create_router(state);

    // Get port from environment or use default
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("VoBee API Server listening on {}", addr);

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
