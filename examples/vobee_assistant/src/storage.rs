use crate::chatbot::{Message, MessageSender};
use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::PathBuf;

/// Manages persistent storage for VoBee AI conversations
pub struct ConversationStorage {
    conn: Connection,
}

impl ConversationStorage {
    /// Create a new storage instance with the database at the given path
    pub fn new(db_path: PathBuf) -> Result<Self> {
        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open database at {:?}", db_path))?;

        let storage = Self { conn };
        storage.initialize_schema()?;
        Ok(storage)
    }

    /// Initialize the database schema
    fn initialize_schema(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                conversation_id INTEGER NOT NULL,
                sender TEXT NOT NULL,
                text TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                FOREIGN KEY(conversation_id) REFERENCES conversations(id) ON DELETE CASCADE
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_conversation 
             ON messages(conversation_id)",
            [],
        )?;

        Ok(())
    }

    /// Create a new conversation
    pub fn create_conversation(&self, title: &str) -> Result<i64> {
        let now = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO conversations (title, created_at, updated_at) VALUES (?1, ?2, ?3)",
            params![title, now, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    /// Save a message to a conversation
    pub fn save_message(&self, conversation_id: i64, message: &Message) -> Result<()> {
        let sender = match message.sender {
            MessageSender::User => "User",
            MessageSender::Bot => "Bot",
        };

        self.conn.execute(
            "INSERT INTO messages (conversation_id, sender, text, timestamp) 
             VALUES (?1, ?2, ?3, ?4)",
            params![
                conversation_id,
                sender,
                message.text,
                message.timestamp.to_rfc3339()
            ],
        )?;

        // Update conversation's updated_at timestamp
        self.conn.execute(
            "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
            params![Utc::now().to_rfc3339(), conversation_id],
        )?;

        Ok(())
    }

    /// Load all messages from a conversation
    pub fn load_messages(&self, conversation_id: i64) -> Result<Vec<Message>> {
        let mut stmt = self.conn.prepare(
            "SELECT sender, text, timestamp FROM messages 
             WHERE conversation_id = ?1 
             ORDER BY id ASC",
        )?;

        let messages = stmt
            .query_map([conversation_id], |row| {
                let sender_str: String = row.get(0)?;
                let sender = if sender_str == "User" {
                    MessageSender::User
                } else {
                    MessageSender::Bot
                };
                let text: String = row.get(1)?;
                let timestamp_str: String = row.get(2)?;
                let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|e| {
                        eprintln!("Warning: Failed to parse timestamp '{}': {}", timestamp_str, e);
                        Utc::now()
                    });

                Ok(Message {
                    sender,
                    text,
                    timestamp,
                })
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(messages)
    }

    /// List all conversations
    #[allow(dead_code)]
    pub fn list_conversations(&self) -> Result<Vec<(i64, String, DateTime<Utc>)>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, updated_at FROM conversations 
             ORDER BY updated_at DESC",
        )?;

        let conversations = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let title: String = row.get(1)?;
                let updated_str: String = row.get(2)?;
                let updated = DateTime::parse_from_rfc3339(&updated_str)
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(|e| {
                        eprintln!("Warning: Failed to parse timestamp '{}': {}", updated_str, e);
                        Utc::now()
                    });

                Ok((id, title, updated))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(conversations)
    }

    /// Delete a conversation and all its messages
    #[allow(dead_code)]
    pub fn delete_conversation(&self, conversation_id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM conversations WHERE id = ?1",
            params![conversation_id],
        )?;
        Ok(())
    }

    /// Export conversation to JSON
    pub fn export_conversation_json(&self, conversation_id: i64) -> Result<String> {
        let messages = self.load_messages(conversation_id)?;
        serde_json::to_string_pretty(&messages).context("Failed to serialize messages")
    }

    /// Export conversation to Markdown
    pub fn export_conversation_markdown(&self, conversation_id: i64) -> Result<String> {
        let messages = self.load_messages(conversation_id)?;
        let mut markdown = String::from("# VoBee AI Conversation\n\n");

        for msg in messages {
            let sender_name = match msg.sender {
                MessageSender::User => "**You**",
                MessageSender::Bot => "**VoBee**",
            };
            markdown.push_str(&format!(
                "{} ({})\n\n{}\n\n---\n\n",
                sender_name,
                msg.timestamp.format("%Y-%m-%d %H:%M:%S"),
                msg.text
            ));
        }

        Ok(markdown)
    }
}

/// Get the default database path for VoBee AI
pub fn get_default_db_path() -> Result<PathBuf> {
    let data_dir = dirs::data_dir().context("Failed to get data directory")?;
    let vobee_dir = data_dir.join("VoBee-AI");
    std::fs::create_dir_all(&vobee_dir)
        .with_context(|| format!("Failed to create VoBee-AI directory at {:?}", vobee_dir))?;
    Ok(vobee_dir.join("conversations.db"))
}
