use crate::error::{AiError, AiResult};
use crate::providers::{AiProvider, AiProviderType, AiRequest, AiResponse};
use async_trait::async_trait;

/// DeepSeek API client (Code understanding and search)
pub struct DeepSeekProvider {
    api_key: String,
    #[allow(dead_code)]
    base_url: String,
}

impl DeepSeekProvider {
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.deepseek.com/v1".to_string()),
        }
    }
}

#[async_trait]
impl AiProvider for DeepSeekProvider {
    fn provider_type(&self) -> AiProviderType {
        AiProviderType::DeepSeek
    }
    
    async fn send_request(&self, _request: &AiRequest) -> AiResult<AiResponse> {
        // TODO: Implement DeepSeek API integration
        Err(AiError::ProviderNotAvailable("DeepSeek integration not yet implemented".to_string()))
    }
    
    async fn health_check(&self) -> AiResult<bool> {
        Ok(!self.api_key.is_empty())
    }
    
    fn estimate_cost(&self, request: &AiRequest) -> f32 {
        let estimated_tokens = request.prompt.len() / 4 + request.max_tokens.unwrap_or(1000) as usize;
        (estimated_tokens as f32 / 1000.0) * 0.001
    }
}
