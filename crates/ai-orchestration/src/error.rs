use thiserror::Error;

/// Result type for AI operations
pub type AiResult<T> = Result<T, AiError>;

/// Errors that can occur during AI operations
#[derive(Error, Debug)]
pub enum AiError {
    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Rate limit exceeded for provider {provider}")]
    RateLimit { provider: String },

    #[error("Authentication failed: {0}")]
    Authentication(String),

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("All providers failed")]
    AllProvidersFailed,

    #[error("Provider not available: {0}")]
    ProviderNotAvailable(String),

    #[error("Timeout error: {0}")]
    Timeout(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
