use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Application settings for VoBee AI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    /// Theme mode: "light", "dark", or "system"
    pub theme: String,
    /// Whether to enable sound effects
    pub enable_sounds: bool,
    /// Whether to save conversation history
    pub save_history: bool,
    /// Maximum number of conversations to keep
    pub max_conversations: usize,
    /// Font size multiplier (1.0 is default)
    pub font_scale: f32,
    /// Whether to show timestamps on messages
    pub show_timestamps: bool,
    /// Auto-scroll to new messages
    pub auto_scroll: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            enable_sounds: true,
            save_history: true,
            max_conversations: 100,
            font_scale: 1.0,
            show_timestamps: false,
            auto_scroll: true,
        }
    }
}

impl AppSettings {
    /// Load settings from file, or create default if file doesn't exist
    pub fn load(path: &PathBuf) -> Result<Self> {
        if path.exists() {
            let content = std::fs::read_to_string(path)
                .with_context(|| format!("Failed to read settings from {:?}", path))?;
            serde_json::from_str(&content).context("Failed to parse settings JSON")
        } else {
            let settings = Self::default();
            settings.save(path)?;
            Ok(settings)
        }
    }

    /// Save settings to file
    pub fn save(&self, path: &PathBuf) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create settings directory: {:?}", parent))?;
        }

        let content = serde_json::to_string_pretty(self).context("Failed to serialize settings")?;
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write settings to {:?}", path))
    }
}

/// Get the default settings path for VoBee AI
pub fn get_default_settings_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("Failed to get config directory")?;
    let vobee_dir = config_dir.join("VoBee-AI");
    std::fs::create_dir_all(&vobee_dir)
        .with_context(|| format!("Failed to create VoBee-AI config directory at {:?}", vobee_dir))?;
    Ok(vobee_dir.join("settings.json"))
}
