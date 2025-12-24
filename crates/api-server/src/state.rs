use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use vobee_ai_orchestration::AiOrchestrator;

/// Shared application state
#[derive(Clone)]
pub struct AppState {
    pub chatbot: Arc<Mutex<Chatbot>>,
    pub orchestrator: Arc<AiOrchestrator>,
}

impl AppState {
    pub fn new(orchestrator: AiOrchestrator) -> Self {
        Self {
            chatbot: Arc::new(Mutex::new(Chatbot::new())),
            orchestrator: Arc::new(orchestrator),
        }
    }
}

/// Simple chatbot with conversation history
pub struct Chatbot {
    conversation_history: Vec<Message>,
}

impl Chatbot {
    pub fn new() -> Self {
        Self {
            conversation_history: Vec::new(),
        }
    }

    pub fn add_message(&mut self, sender: MessageSender, text: String) {
        self.conversation_history.push(Message {
            sender,
            text,
            timestamp: Utc::now(),
        });
    }

    pub fn get_history(&self) -> &[Message] {
        &self.conversation_history
    }

    pub fn clear_history(&mut self) {
        self.conversation_history.clear();
    }
}

impl Default for Chatbot {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: MessageSender,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageSender {
    User,
    Bot,
}
