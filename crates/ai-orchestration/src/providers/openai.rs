use crate::error::{AiError, AiResult};
use crate::providers::{AiProvider, AiProviderType, AiRequest, AiResponse};
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// OpenAI API client (GPT-4, ChatGPT)
pub struct OpenAiProvider {
    api_key: String,
    base_url: String,
    client: Client,
}

#[derive(Serialize)]
struct OpenAiRequestBody {
    model: String,
    messages: Vec<Message>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiResponse {
    choices: Vec<Choice>,
    usage: Option<Usage>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

#[derive(Deserialize)]
struct Usage {
    total_tokens: u32,
}

impl OpenAiProvider {
    /// Create a new OpenAI provider
    pub fn new(api_key: String, base_url: Option<String>) -> Self {
        Self {
            api_key,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            client: Client::new(),
        }
    }
}

#[async_trait]
impl AiProvider for OpenAiProvider {
    fn provider_type(&self) -> AiProviderType {
        AiProviderType::OpenAI
    }
    
    async fn send_request(&self, request: &AiRequest) -> AiResult<AiResponse> {
        let mut messages = Vec::new();
        
        // Add system message if provided
        if let Some(system_prompt) = &request.system_prompt {
            messages.push(Message {
                role: "system".to_string(),
                content: system_prompt.clone(),
            });
        }
        
        // Add user message
        messages.push(Message {
            role: "user".to_string(),
            content: request.prompt.clone(),
        });
        
        let body = OpenAiRequestBody {
            model: "gpt-4".to_string(),
            messages,
            max_tokens: request.max_tokens,
            temperature: request.temperature,
        };
        
        let response = self.client
            .post(format!("{}/chat/completions", self.base_url))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AiError::Provider(format!("OpenAI API error: {}", error_text)));
        }
        
        let openai_response: OpenAiResponse = response.json().await?;
        
        let text = openai_response.choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| AiError::Provider("No response from OpenAI".to_string()))?;
        
        let tokens_used = openai_response.usage.map(|u| u.total_tokens);
        let cost_estimate = tokens_used.map(|t| t as f32 * 0.00003); // Rough estimate
        
        Ok(AiResponse {
            text,
            provider: AiProviderType::OpenAI,
            tokens_used,
            cost_estimate,
            metadata: std::collections::HashMap::new(),
        })
    }
    
    async fn health_check(&self) -> AiResult<bool> {
        // Simple health check - verify API key format
        Ok(!self.api_key.is_empty())
    }
    
    fn estimate_cost(&self, request: &AiRequest) -> f32 {
        // Rough estimate: $0.03 per 1K tokens
        let estimated_tokens = request.prompt.len() / 4 + request.max_tokens.unwrap_or(1000) as usize;
        (estimated_tokens as f32 / 1000.0) * 0.03
    }
}
