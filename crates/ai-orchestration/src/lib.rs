//! # VoBee AI Orchestration
//!
//! Multi-AI orchestration system for integrating and coordinating multiple AI providers:
//! - Claude 4.5 (Anthropic) - Advanced reasoning and code generation
//! - ChatGPT/GPT-4 (OpenAI) - Conversational AI and general intelligence
//! - DeepSeek - Code understanding and search
//! - Gemini 3 (Google) - Multi-modal processing
//! - Microsoft Copilot - Development assistance
//! - Perplexity Pro - Research and knowledge retrieval

pub mod providers;
pub mod routing;
pub mod error;
pub mod config;
pub mod rate_limiter;
pub mod orchestrator;

pub use error::{AiError, AiResult};
pub use orchestrator::AiOrchestrator;
pub use providers::{AiProvider, AiRequest, AiResponse, TaskType};
pub use config::AiConfig;
