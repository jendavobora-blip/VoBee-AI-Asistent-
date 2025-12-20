# VoBee AI User Guide

Welcome to VoBee AI - Your Friendly AI Assistant! 🐝

## Table of Contents

1. [Getting Started](#getting-started)
2. [Features Overview](#features-overview)
3. [Using VoBee AI](#using-vobee-ai)
4. [Customization](#customization)
5. [Data Management](#data-management)
6. [Troubleshooting](#troubleshooting)
7. [Advanced Usage](#advanced-usage)

## Getting Started

### First Launch

When you first launch VoBee AI, you'll be greeted with a friendly welcome message. The application automatically:
- Creates a data directory for storing conversations
- Creates a configuration directory for settings
- Initializes a new conversation

### Interface Overview

The VoBee AI interface consists of three main areas:

1. **Header Bar** (Top)
   - VoBee AI branding and title
   - Theme toggle button (🌙/☀️)
   - Export buttons (📄 JSON, 📝 MD)

2. **Chat Area** (Middle)
   - Displays all messages in the current conversation
   - User messages appear on the right in blue
   - VoBee responses appear on the left in gray/white

3. **Input Area** (Bottom)
   - Text input field for typing messages
   - Send button to submit messages
   - Clear button to start a new conversation

## Features Overview

### 🗨️ Conversational AI
VoBee AI responds to various types of queries including:
- **Greetings**: Say hello, hi, or good morning
- **Jokes**: Ask for a joke or something funny
- **Fun Facts**: Request interesting information
- **Emotional Support**: Share how you're feeling
- **General Chat**: Ask about VoBee's capabilities

### 💾 Persistent Storage
All conversations are automatically saved to a local database. Your chat history is:
- Stored securely on your device
- Never sent to external servers
- Easily exportable in multiple formats

### 🎨 Theme Support
Switch between light and dark themes:
- **Light Mode**: Bright, clean interface ideal for daytime use
- **Dark Mode**: Easy on the eyes for nighttime conversations
- Toggle anytime by clicking the theme button in the header

### 📤 Export Conversations
Save your conversations for reference:
- **JSON Format**: Structured data with timestamps
- **Markdown Format**: Human-readable format with formatting
- Exported files are saved to your Downloads folder

## Using VoBee AI

### Starting a Conversation

1. Type your message in the input field at the bottom
2. Press **Enter** or click the **Send** button
3. VoBee will respond within moments

### Example Conversations

**Greeting:**
```
You: Hello!
VoBee: Hi there! 👋 VoBee at your service! What's on your mind?
```

**Request a Joke:**
```
You: Tell me a joke
VoBee: Why don't scientists trust atoms? Because they make up everything! 😄
```

**Ask for Information:**
```
You: Fun fact
VoBee: Did you know? Honey never spoils! Archaeologists have found 3000-year-old honey that's still edible! 🍯
```

**Emotional Support:**
```
You: I'm feeling sad
VoBee: I'm sorry to hear that 😔 Remember, tough times don't last!
```

### Keyboard Shortcuts

- **Enter**: Send message
- **Clear Button**: Start a new conversation

## Customization

### Settings File

VoBee AI stores settings in a JSON file that you can edit:

**Location:**
- Linux: `~/.config/VoBee-AI/settings.json`
- macOS: `~/Library/Application Support/VoBee-AI/settings.json`
- Windows: `%APPDATA%\VoBee-AI\settings.json`

**Available Settings:**

```json
{
  "theme": "light",           // Options: "light", "dark", or "system"
  "enable_sounds": true,      // Enable sound effects (future feature)
  "save_history": true,       // Save conversation history
  "max_conversations": 100,   // Maximum number of conversations to keep
  "font_scale": 1.0,          // Font size multiplier (0.5 - 2.0)
  "show_timestamps": false,   // Show message numbers
  "auto_scroll": true         // Auto-scroll to new messages (future)
}
```

### Changing Themes

1. Click the theme toggle button in the header (🌙 or ☀️)
2. The interface immediately updates to the new theme
3. Your preference is saved automatically

## Data Management

### Where Is My Data Stored?

**Conversation Database:**
- Linux: `~/.local/share/VoBee-AI/conversations.db`
- macOS: `~/Library/Application Support/VoBee-AI/conversations.db`
- Windows: `%APPDATA%\VoBee-AI\conversations.db`

**Exported Files:**
- Default location: Your system's Downloads folder
- Format: `vobee_conversation_YYYYMMDD_HHMMSS.json` or `.md`

### Exporting Conversations

1. Click **📄 JSON** for JSON format or **📝 MD** for Markdown
2. The file is automatically saved to your Downloads folder
3. File includes all messages with timestamps

### Backing Up Your Data

To backup your conversations:

**Option 1: Manual Backup**
1. Navigate to your data directory
2. Copy the `conversations.db` file to a safe location

**Option 2: Export Regularly**
1. Use the export feature regularly
2. Store exported files in a backup location

### Starting Fresh

To clear all data and start fresh:

1. Close VoBee AI
2. Delete the data directory:
   - Linux: `rm -rf ~/.local/share/VoBee-AI`
   - macOS: `rm -rf ~/Library/Application\ Support/VoBee-AI`
   - Windows: Delete `%APPDATA%\VoBee-AI`
3. Restart VoBee AI

## Troubleshooting

### VoBee AI Won't Start

**Check System Requirements:**
- Rust application built for your platform
- Sufficient disk space (at least 100MB)
- Required system libraries installed

**Try:**
1. Check if the executable has proper permissions
2. Run from terminal to see error messages
3. Verify data directories can be created

### Messages Not Saving

**Check:**
1. Settings file has `save_history: true`
2. Data directory is writable
3. Sufficient disk space available

**Solution:**
1. Check settings: `cat ~/.config/VoBee-AI/settings.json`
2. Verify directory permissions
3. Try starting a new conversation

### Export Not Working

**Check:**
1. Downloads folder exists and is writable
2. Sufficient disk space
3. Application has file write permissions

**Solution:**
1. Manually create Downloads folder if missing
2. Check folder permissions
3. Try a different export format

### Theme Not Changing

**Try:**
1. Click the theme button multiple times
2. Restart the application
3. Check if settings file is writable

## Advanced Usage

### Database Structure

The SQLite database contains two tables:

**conversations:**
- `id`: Unique conversation identifier
- `title`: Conversation title
- `created_at`: Creation timestamp
- `updated_at`: Last update timestamp

**messages:**
- `id`: Unique message identifier
- `conversation_id`: Reference to conversation
- `sender`: "User" or "Bot"
- `text`: Message content
- `timestamp`: Message timestamp

### Manual Database Access

You can access the database directly using SQLite tools:

```bash
# Open database
sqlite3 ~/.local/share/VoBee-AI/conversations.db

# List all conversations
SELECT * FROM conversations;

# View messages from a conversation
SELECT * FROM messages WHERE conversation_id = 1;
```

### Customizing Responses

To add custom responses, you would need to modify the source code:

1. Edit `examples/vobee_assistant/src/response_patterns.rs`
2. Add new pattern categories or responses
3. Rebuild the application

### Integration with Other Tools

Export formats allow integration with:
- **JSON**: Parse with any programming language
- **Markdown**: Import into note-taking apps (Obsidian, Notion, etc.)
- **SQLite**: Query with database tools

## Tips and Best Practices

1. **Regular Exports**: Export important conversations regularly
2. **Clear Old Chats**: Use the Clear button to start fresh conversations
3. **Theme Usage**: Use dark mode in low-light conditions
4. **Backup**: Periodically backup your database file
5. **Explore**: Try different types of questions to discover all features

## Privacy and Security

- **Local Storage**: All data stays on your device
- **No Internet Required**: Works completely offline
- **No Tracking**: VoBee doesn't track or collect any data
- **Secure**: Conversations are stored in a local SQLite database

## Getting Help

For issues or questions:
1. Check this user guide
2. Review the project README
3. Report issues on GitHub
4. Check the project documentation

## Updates

VoBee AI is actively developed. Keep an eye out for:
- New response patterns
- Additional export formats
- Enhanced UI features
- Performance improvements

---

**Version**: 0.1.0  
**Last Updated**: December 2025  
**Project**: VoBee-AI-by-Vobora-J

Thank you for using VoBee AI! 🐝
