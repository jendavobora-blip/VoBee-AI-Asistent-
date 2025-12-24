use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vobee_ai_orchestration::{AiRequest, TaskType};

use crate::{
    error::{ApiError, ApiResult},
    state::{AppState, Message, MessageSender},
};

/// Health check endpoint
pub async fn health_check(State(state): State<AppState>) -> ApiResult<Json<HealthResponse>> {
    let health = state.orchestrator.health_check().await;
    
    let all_healthy = health.values().all(|&v| v);
    let status = if all_healthy { "healthy" } else { "degraded" };
    
    Ok(Json(HealthResponse {
        status: status.to_string(),
        providers: health.into_iter().map(|(k, v)| (format!("{:?}", k), v)).collect(),
    }))
}

/// Chat endpoint
pub async fn chat(
    State(state): State<AppState>,
    Json(payload): Json<ChatRequest>,
) -> ApiResult<Json<ChatResponse>> {
    if payload.message.trim().is_empty() {
        return Err(ApiError::BadRequest("Message cannot be empty".to_string()));
    }

    // Add user message to history
    {
        let mut chatbot = state.chatbot.lock().await;
        chatbot.add_message(MessageSender::User, payload.message.clone());
    }

    // Create AI request
    let ai_request = AiRequest::new(payload.message, TaskType::Conversation)
        .with_max_tokens(payload.max_tokens.unwrap_or(1000))
        .with_temperature(payload.temperature.unwrap_or(0.7));

    // Process request through orchestrator
    let response = state.orchestrator.process_request(ai_request).await?;

    // Add bot response to history
    {
        let mut chatbot = state.chatbot.lock().await;
        chatbot.add_message(MessageSender::Bot, response.text.clone());
    }

    Ok(Json(ChatResponse {
        message: response.text,
        provider: format!("{:?}", response.provider),
        tokens_used: response.tokens_used,
        cost_estimate: response.cost_estimate,
    }))
}

/// Get conversation history
pub async fn get_history(
    State(state): State<AppState>,
) -> ApiResult<Json<HistoryResponse>> {
    let chatbot = state.chatbot.lock().await;
    let history = chatbot.get_history().to_vec();

    Ok(Json(HistoryResponse { messages: history }))
}

/// Clear conversation history
pub async fn clear_history(State(state): State<AppState>) -> ApiResult<StatusCode> {
    let mut chatbot = state.chatbot.lock().await;
    chatbot.clear_history();

    Ok(StatusCode::OK)
}

// Request/Response types

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub message: String,
    pub provider: String,
    pub tokens_used: Option<u32>,
    pub cost_estimate: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub providers: HashMap<String, bool>,
}

#[derive(Debug, Serialize)]
pub struct HistoryResponse {
    pub messages: Vec<Message>,
}
