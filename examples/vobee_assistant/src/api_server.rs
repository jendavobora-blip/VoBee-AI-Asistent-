use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

use crate::chatbot::{MessageSender, VoBeeChatbot};

// API Request/Response types
#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub messages: Vec<MessageItem>,
}

#[derive(Debug, Serialize)]
pub struct MessageItem {
    pub sender: String,
    pub text: String,
    pub timestamp: String,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

// Application state
#[derive(Clone)]
pub struct AppState {
    pub chatbot: Arc<Mutex<VoBeeChatbot>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            chatbot: Arc::new(Mutex::new(VoBeeChatbot::new())),
        }
    }
}

// API Handlers
async fn health() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn chat(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, (StatusCode, Json<ErrorResponse>)> {
    if payload.message.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "Message cannot be empty".to_string(),
            }),
        ));
    }

    let mut chatbot = state.chatbot.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to acquire lock: {}", e),
            }),
        )
    })?;

    let response = chatbot.process_message(payload.message);
    let timestamp = chrono::Utc::now().to_rfc3339();

    Ok(Json(ChatResponse {
        response,
        timestamp,
    }))
}

async fn get_history(
    State(state): State<AppState>,
) -> Result<Json<HistoryResponse>, (StatusCode, Json<ErrorResponse>)> {
    let chatbot = state.chatbot.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to acquire lock: {}", e),
            }),
        )
    })?;

    let messages: Vec<MessageItem> = chatbot
        .get_conversation_history()
        .iter()
        .map(|msg| MessageItem {
            sender: match msg.sender {
                MessageSender::User => "user".to_string(),
                MessageSender::Bot => "bot".to_string(),
            },
            text: msg.text.clone(),
            timestamp: msg.timestamp.to_rfc3339(),
        })
        .collect();

    Ok(Json(HistoryResponse { messages }))
}

async fn clear_history(
    State(state): State<AppState>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let mut chatbot = state.chatbot.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: format!("Failed to acquire lock: {}", e),
            }),
        )
    })?;

    chatbot.clear_conversation_history();
    Ok(StatusCode::NO_CONTENT)
}

// Create and configure the router
pub fn create_router() -> Router {
    let state = AppState::new();

    // Configure CORS for FlutterFlow web apps
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/api/health", get(health))
        .route("/api/chat", post(chat))
        .route("/api/chat/history", get(get_history))
        .route("/api/chat/history", delete(clear_history))
        .layer(cors)
        .with_state(state)
}

// Start the API server
pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let app = create_router();
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("🐝 VoBee API Server running on http://{}", addr);
    println!("📝 API Documentation:");
    println!("  GET  /api/health         - Health check");
    println!("  POST /api/chat           - Send message (body: {{\"message\": \"text\"}} )");
    println!("  GET  /api/chat/history   - Get conversation history");
    println!("  DELETE /api/chat/history - Clear conversation history");
    
    axum::serve(listener, app).await?;
    Ok(())
}


