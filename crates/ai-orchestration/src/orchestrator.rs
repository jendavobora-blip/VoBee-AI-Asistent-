use crate::config::AiConfig;
use crate::error::{AiError, AiResult};
use crate::providers::*;
use crate::rate_limiter::RateLimiter;
use crate::routing::TaskRouter;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{error, info, warn};

/// Main orchestrator for coordinating multiple AI providers
pub struct AiOrchestrator {
    config: AiConfig,
    providers: HashMap<AiProviderType, Arc<dyn AiProvider>>,
    rate_limiter: Arc<RateLimiter>,
}

impl AiOrchestrator {
    /// Create a new orchestrator with the given configuration
    pub fn new(config: AiConfig) -> Self {
        let mut providers: HashMap<AiProviderType, Arc<dyn AiProvider>> = HashMap::new();
        
        // Initialize OpenAI provider
        if let Some(api_key) = config.api_keys.get("openai") {
            let provider = openai::OpenAiProvider::new(
                api_key.clone(),
                config.base_urls.get("openai").cloned(),
            );
            providers.insert(AiProviderType::OpenAI, Arc::new(provider));
        }
        
        // Initialize Anthropic provider
        if let Some(api_key) = config.api_keys.get("anthropic") {
            let provider = anthropic::AnthropicProvider::new(
                api_key.clone(),
                config.base_urls.get("anthropic").cloned(),
            );
            providers.insert(AiProviderType::Anthropic, Arc::new(provider));
        }
        
        // Initialize Google provider
        if let Some(api_key) = config.api_keys.get("google") {
            let provider = google::GoogleProvider::new(
                api_key.clone(),
                config.base_urls.get("google").cloned(),
            );
            providers.insert(AiProviderType::Google, Arc::new(provider));
        }
        
        // Initialize DeepSeek provider
        if let Some(api_key) = config.api_keys.get("deepseek") {
            let provider = deepseek::DeepSeekProvider::new(
                api_key.clone(),
                config.base_urls.get("deepseek").cloned(),
            );
            providers.insert(AiProviderType::DeepSeek, Arc::new(provider));
        }
        
        // Initialize Microsoft provider
        if let Some(api_key) = config.api_keys.get("microsoft") {
            let provider = microsoft::MicrosoftProvider::new(
                api_key.clone(),
                config.base_urls.get("microsoft").cloned(),
            );
            providers.insert(AiProviderType::Microsoft, Arc::new(provider));
        }
        
        // Initialize Perplexity provider
        if let Some(api_key) = config.api_keys.get("perplexity") {
            let provider = perplexity::PerplexityProvider::new(
                api_key.clone(),
                config.base_urls.get("perplexity").cloned(),
            );
            providers.insert(AiProviderType::Perplexity, Arc::new(provider));
        }
        
        let rate_limiter = Arc::new(RateLimiter::new(config.rate_limits.clone()));
        
        Self {
            config,
            providers,
            rate_limiter,
        }
    }
    
    /// Process a request using the most appropriate provider(s)
    pub async fn process_request(&self, request: AiRequest) -> AiResult<AiResponse> {
        // Route request to appropriate providers
        let provider_types = TaskRouter::route_task(&request.task_type);
        
        info!("Routing task {:?} to providers: {:?}", request.task_type, provider_types);
        
        // Try each provider in order
        for provider_type in provider_types {
            if let Some(provider) = self.providers.get(&provider_type) {
                // Check rate limit
                if !self.rate_limiter.check_rate_limit(provider_type.as_str()).await {
                    warn!("Rate limit exceeded for provider: {:?}", provider_type);
                    continue;
                }
                
                // Try to send request
                match provider.send_request(&request).await {
                    Ok(response) => {
                        info!("Successfully received response from {:?}", provider_type);
                        return Ok(response);
                    }
                    Err(e) => {
                        error!("Provider {:?} failed: {}", provider_type, e);
                        if !self.config.enable_fallback {
                            return Err(e);
                        }
                        // Continue to next provider
                        continue;
                    }
                }
            }
        }
        
        Err(AiError::AllProvidersFailed)
    }
    
    /// Get available providers
    pub fn available_providers(&self) -> Vec<AiProviderType> {
        self.providers.keys().copied().collect()
    }
    
    /// Check health of all providers
    pub async fn health_check(&self) -> HashMap<AiProviderType, bool> {
        let mut results = HashMap::new();
        
        for (provider_type, provider) in &self.providers {
            match provider.health_check().await {
                Ok(healthy) => {
                    results.insert(*provider_type, healthy);
                }
                Err(_) => {
                    results.insert(*provider_type, false);
                }
            }
        }
        
        results
    }
}
