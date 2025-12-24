use crate::providers::{AiProviderType, TaskType};

/// Router for selecting the best AI provider for a given task
pub struct TaskRouter;

impl TaskRouter {
    /// Route a task to the most appropriate AI provider(s)
    pub fn route_task(task_type: &TaskType) -> Vec<AiProviderType> {
        match task_type {
            TaskType::Conversation => vec![
                AiProviderType::OpenAI,
                AiProviderType::Anthropic,
            ],
            TaskType::CodeGeneration => vec![
                AiProviderType::DeepSeek,
                AiProviderType::OpenAI,
                AiProviderType::Anthropic,
            ],
            TaskType::Research => vec![
                AiProviderType::Perplexity,
                AiProviderType::OpenAI,
            ],
            TaskType::MultiModal => vec![
                AiProviderType::Google,
                AiProviderType::OpenAI,
            ],
            TaskType::Development => vec![
                AiProviderType::Microsoft,
                AiProviderType::DeepSeek,
                AiProviderType::OpenAI,
            ],
            TaskType::Reasoning => vec![
                AiProviderType::Anthropic,
                AiProviderType::OpenAI,
            ],
        }
    }
    
    /// Get fallback providers if primary fails
    pub fn get_fallback_providers(task_type: &TaskType) -> Vec<AiProviderType> {
        let mut primary = Self::route_task(task_type);
        if primary.len() > 1 {
            primary.remove(0);
            primary
        } else {
            // Generic fallbacks
            vec![
                AiProviderType::OpenAI,
                AiProviderType::Anthropic,
                AiProviderType::Google,
            ]
        }
    }
}
