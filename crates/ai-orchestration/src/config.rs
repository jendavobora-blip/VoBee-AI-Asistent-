use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Configuration for AI providers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiConfig {
    /// API keys for each provider
    pub api_keys: HashMap<String, String>,
    
    /// Base URLs for each provider
    pub base_urls: HashMap<String, String>,
    
    /// Rate limits (requests per minute) for each provider
    pub rate_limits: HashMap<String, u32>,
    
    /// Timeout in seconds for API calls
    pub timeout_seconds: u64,
    
    /// Enable fallback to other providers on failure
    pub enable_fallback: bool,
    
    /// Maximum retry attempts per provider
    pub max_retries: u32,
}

impl Default for AiConfig {
    fn default() -> Self {
        let mut base_urls = HashMap::new();
        base_urls.insert("openai".to_string(), "https://api.openai.com/v1".to_string());
        base_urls.insert("anthropic".to_string(), "https://api.anthropic.com/v1".to_string());
        base_urls.insert("google".to_string(), "https://generativelanguage.googleapis.com/v1".to_string());
        
        let mut rate_limits = HashMap::new();
        rate_limits.insert("openai".to_string(), 60);
        rate_limits.insert("anthropic".to_string(), 50);
        rate_limits.insert("google".to_string(), 60);
        rate_limits.insert("deepseek".to_string(), 30);
        rate_limits.insert("microsoft".to_string(), 60);
        rate_limits.insert("perplexity".to_string(), 40);
        
        Self {
            api_keys: HashMap::new(),
            base_urls,
            rate_limits,
            timeout_seconds: 30,
            enable_fallback: true,
            max_retries: 3,
        }
    }
}

impl AiConfig {
    /// Create configuration from environment variables
    pub fn from_env() -> Self {
        let mut config = Self::default();
        
        // Load API keys from environment
        if let Ok(key) = std::env::var("OPENAI_API_KEY") {
            config.api_keys.insert("openai".to_string(), key);
        }
        if let Ok(key) = std::env::var("ANTHROPIC_API_KEY") {
            config.api_keys.insert("anthropic".to_string(), key);
        }
        if let Ok(key) = std::env::var("GOOGLE_API_KEY") {
            config.api_keys.insert("google".to_string(), key);
        }
        if let Ok(key) = std::env::var("DEEPSEEK_API_KEY") {
            config.api_keys.insert("deepseek".to_string(), key);
        }
        if let Ok(key) = std::env::var("MICROSOFT_API_KEY") {
            config.api_keys.insert("microsoft".to_string(), key);
        }
        if let Ok(key) = std::env::var("PERPLEXITY_API_KEY") {
            config.api_keys.insert("perplexity".to_string(), key);
        }
        
        config
    }
    
    /// Check if a provider is configured
    pub fn is_provider_configured(&self, provider: &str) -> bool {
        self.api_keys.contains_key(provider)
    }
}
