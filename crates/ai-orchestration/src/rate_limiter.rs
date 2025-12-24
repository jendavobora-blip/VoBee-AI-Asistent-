use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

/// Rate limiter for AI providers
pub struct RateLimiter {
    limits: HashMap<String, u32>,
    state: Arc<Mutex<RateLimiterState>>,
}

struct RateLimiterState {
    requests: HashMap<String, Vec<Instant>>,
}

impl RateLimiter {
    /// Create a new rate limiter with specified limits (requests per minute)
    pub fn new(limits: HashMap<String, u32>) -> Self {
        Self {
            limits,
            state: Arc::new(Mutex::new(RateLimiterState {
                requests: HashMap::new(),
            })),
        }
    }
    
    /// Check if a request can be made to the provider
    pub async fn check_rate_limit(&self, provider: &str) -> bool {
        let mut state = self.state.lock().await;
        let now = Instant::now();
        let window = Duration::from_secs(60);
        
        // Get or create request history for provider
        let requests = state.requests.entry(provider.to_string()).or_insert_with(Vec::new);
        
        // Remove requests outside the time window
        requests.retain(|&time| now.duration_since(time) < window);
        
        // Check if limit exceeded
        let limit = self.limits.get(provider).copied().unwrap_or(60);
        if requests.len() >= limit as usize {
            return false;
        }
        
        // Record this request
        requests.push(now);
        true
    }
    
    /// Wait until rate limit allows a request
    pub async fn wait_for_rate_limit(&self, provider: &str) {
        while !self.check_rate_limit(provider).await {
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }
}
