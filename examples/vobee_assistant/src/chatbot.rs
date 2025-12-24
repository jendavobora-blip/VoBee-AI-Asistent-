use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::response_patterns::ResponsePatterns;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub sender: MessageSender,
    pub text: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MessageSender {
    User,
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrecognizedQuery {
    pub query: String,
    pub count: usize,
    pub first_seen: DateTime<Utc>,
    pub last_seen: DateTime<Utc>,
}

pub struct VoBeeChatbot {
    patterns: ResponsePatterns,
    conversation_history: Vec<Message>,
    unrecognized_queries: HashMap<String, UnrecognizedQuery>,
}

impl VoBeeChatbot {
    pub fn new() -> Self {
        Self {
            patterns: ResponsePatterns::new(),
            conversation_history: Vec::new(),
            unrecognized_queries: HashMap::new(),
        }
    }

    pub fn process_message(&mut self, user_input: String) -> String {
        // Add user message to history
        let user_message = Message {
            sender: MessageSender::User,
            text: user_input.clone(),
            timestamp: Utc::now(),
        };
        self.conversation_history.push(user_message);

        // Get bot response
        let response = self.patterns.get_response(&user_input);
        
        // Check if it's a fallback response (unrecognized)
        if response.contains("still learning") || response.contains("new one for me") {
            self.log_unrecognized_query(user_input);
        }

        // Add bot response to history
        let bot_message = Message {
            sender: MessageSender::Bot,
            text: response.clone(),
            timestamp: Utc::now(),
        };
        self.conversation_history.push(bot_message);

        response
    }

    fn log_unrecognized_query(&mut self, query: String) {
        let now = Utc::now();
        self.unrecognized_queries
            .entry(query.clone())
            .and_modify(|e| {
                e.count += 1;
                e.last_seen = now;
            })
            .or_insert(UnrecognizedQuery {
                query,
                count: 1,
                first_seen: now,
                last_seen: now,
            });
    }

    #[allow(dead_code)]
    pub fn get_conversation_history(&self) -> &[Message] {
        &self.conversation_history
    }

    pub fn clear_conversation_history(&mut self) {
        self.conversation_history.clear();
    }

    #[allow(dead_code)]
    pub fn get_unrecognized_queries(&self) -> Vec<&UnrecognizedQuery> {
        let mut queries: Vec<&UnrecognizedQuery> = self.unrecognized_queries.values().collect();
        queries.sort_by(|a, b| b.count.cmp(&a.count));
        queries
    }

    pub fn welcome_message() -> String {
        "Hello! 🐝 I'm VoBee, your friendly AI assistant! I'm here to chat, share jokes, and help brighten your day! Feel free to ask me anything!".to_string()
    }
}

impl Default for VoBeeChatbot {
    fn default() -> Self {
        Self::new()
    }
}
