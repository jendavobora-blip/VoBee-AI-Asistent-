use crate::error::AiResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

pub mod openai;
pub mod anthropic;
pub mod google;
pub mod deepseek;
pub mod microsoft;
pub mod perplexity;

/// Supported AI providers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AiProviderType {
    OpenAI,
    Anthropic,
    Google,
    DeepSeek,
    Microsoft,
    Perplexity,
}

impl AiProviderType {
    /// Get the string identifier for the provider
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OpenAI => "openai",
            Self::Anthropic => "anthropic",
            Self::Google => "google",
            Self::DeepSeek => "deepseek",
            Self::Microsoft => "microsoft",
            Self::Perplexity => "perplexity",
        }
    }
}

/// Task type for routing decisions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskType {
    /// General conversational AI
    Conversation,
    /// Code generation and understanding
    CodeGeneration,
    /// Research and knowledge retrieval
    Research,
    /// Multi-modal processing (images, audio, etc.)
    MultiModal,
    /// Development assistance
    Development,
    /// Advanced reasoning
    Reasoning,
}

/// Request to an AI provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiRequest {
    /// The prompt or query
    pub prompt: String,
    
    /// Task type for routing
    pub task_type: TaskType,
    
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    
    /// Temperature for response generation
    pub temperature: Option<f32>,
    
    /// System prompt/instructions
    pub system_prompt: Option<String>,
    
    /// Additional metadata
    pub metadata: std::collections::HashMap<String, String>,
}

impl AiRequest {
    /// Create a new request
    pub fn new(prompt: impl Into<String>, task_type: TaskType) -> Self {
        Self {
            prompt: prompt.into(),
            task_type,
            max_tokens: None,
            temperature: None,
            system_prompt: None,
            metadata: std::collections::HashMap::new(),
        }
    }
    
    /// Set max tokens
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
    
    /// Set temperature
    pub fn with_temperature(mut self, temperature: f32) -> Self {
        self.temperature = Some(temperature);
        self
    }
    
    /// Set system prompt
    pub fn with_system_prompt(mut self, system_prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(system_prompt.into());
        self
    }
}

/// Response from an AI provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiResponse {
    /// The generated response text
    pub text: String,
    
    /// Provider that generated the response
    pub provider: AiProviderType,
    
    /// Tokens used in the request
    pub tokens_used: Option<u32>,
    
    /// Cost estimate in USD
    pub cost_estimate: Option<f32>,
    
    /// Response metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Trait for AI providers
#[async_trait]
pub trait AiProvider: Send + Sync {
    /// Get the provider type
    fn provider_type(&self) -> AiProviderType;
    
    /// Send a request to the provider
    async fn send_request(&self, request: &AiRequest) -> AiResult<AiResponse>;
    
    /// Check if the provider is available
    async fn health_check(&self) -> AiResult<bool>;
    
    /// Get the estimated cost for a request
    fn estimate_cost(&self, request: &AiRequest) -> f32;
}
