# VoBee AI Assistant

A Rust desktop application version of the VoBee AI chatbot, built using the GPUI component library.

## Features

### Core Features
- 🐝 **Friendly AI Chatbot** - Creative and engaging responses
- 💬 **Pattern-Based Responses** - Multiple response categories including greetings, jokes, fun facts, and more
- 📝 **Persistent Conversation History** - Automatic saving of all conversations with SQLite database
- 🎨 **Modern UI** - Built with GPUI components for a native desktop experience
- 🧠 **Learning Capability** - Logs unrecognized queries for future improvements

### Enhanced Features (v0.1.0)
- 🌓 **Theme Support** - Toggle between Light and Dark modes
- 📤 **Export Conversations** - Save conversations as JSON or Markdown files
- 💾 **Auto-Save** - Conversations are automatically saved to local database
- ⚙️ **Settings System** - Customizable preferences stored locally
- 📊 **Message Tracking** - Optional timestamps on messages
- 🗂️ **Conversation Management** - Multiple conversation sessions

## Response Categories

- Greetings & Farewells
- Identity & Capabilities
- Emotional responses (happy, sad, bored)
- Jokes
- Fun Facts
- Time-related responses
- Gratitude responses

## Installation

### Prerequisites
- Rust toolchain (1.70 or later)
- System dependencies for GPUI (see parent project README)

### Building from Source

```bash
# Build the application
./script/build-vobee.sh

# Install system-wide (Linux/macOS)
sudo ./script/install-vobee.sh

# Or install for current user only
./script/install-vobee.sh
```

### Quick Start (Development)

```bash
cargo run --example vobee_assistant
```

## Usage

### Running the Application

After installation:
```bash
vobee_assistant
```

Or launch from your application menu (Linux with desktop environment).

### Keyboard Shortcuts
- **Enter** - Send message
- **Theme Toggle** - Click 🌙/☀️ button in header

### Exporting Conversations
Click the export buttons in the header:
- **📄 JSON** - Export as JSON format
- **📝 MD** - Export as Markdown format

Exported files are saved to your Downloads folder.

## Architecture

### Modules

- **`chatbot.rs`** - Core chatbot logic, message handling, and conversation management
- **`response_patterns.rs`** - Response templates and keyword matching system
- **`storage.rs`** - SQLite-based persistent storage for conversations
- **`settings.rs`** - Application settings and configuration management
- **`main.rs`** - GPUI UI implementation and application entry point

### Data Storage

#### Conversation Database
Location varies by platform:
- **Linux**: `~/.local/share/VoBee-AI/conversations.db`
- **macOS**: `~/Library/Application Support/VoBee-AI/conversations.db`
- **Windows**: `%APPDATA%\VoBee-AI\conversations.db`

#### Settings
- **Linux**: `~/.config/VoBee-AI/settings.json`
- **macOS**: `~/Library/Application Support/VoBee-AI/settings.json`
- **Windows**: `%APPDATA%\VoBee-AI\settings.json`

### Key Features

#### Pattern Matching
Keyword-based response system with multiple variations for each category

#### Random Responses
Provides variety by randomly selecting from available response templates

#### Message History
Maintains full conversation history with timestamps in SQLite database

#### Unrecognized Query Logging
Tracks queries that don't match any pattern for future learning

## Configuration

Edit `settings.json` in your config directory:

```json
{
  "theme": "light",           // "light", "dark", or "system"
  "enable_sounds": true,      // Enable sound effects (future)
  "save_history": true,       // Save conversation history
  "max_conversations": 100,   // Maximum conversations to keep
  "font_scale": 1.0,          // Font size multiplier
  "show_timestamps": false,   // Show message timestamps
  "auto_scroll": true         // Auto-scroll to new messages
}
```

## Usage Examples

Try these interactions:
- "Hello" or "Hi there" - Get a friendly greeting
- "Tell me a joke" - Hear a random joke
- "Fun fact" - Learn something interesting
- "I'm feeling sad" - Receive emotional support
- "What can you do?" - Learn about VoBee's capabilities
- "Bye" - Say goodbye

## Development

### Building

```bash
# Debug build
cargo build --example vobee_assistant

# Release build
cargo build --release --example vobee_assistant
```

### Dependencies

Key dependencies:
- `gpui` - UI framework
- `gpui-component` - UI components library
- `rusqlite` - SQLite database
- `chrono` - Date/time handling
- `serde` - Serialization
- `dirs` - Platform directories

## Deployment

### Creating a Release

```bash
# Build production version
./script/build-vobee.sh

# Create release archive
cd dist
tar -czf vobee-ai-linux.tar.gz *

# Or for a specific platform
tar -czf vobee-ai-$(uname -s | tr '[:upper:]' '[:lower:]').tar.gz *
```

### Package Structure

```
dist/
├── vobee_assistant          # Main executable
├── README.txt               # User documentation
├── VERSION.txt              # Version information
└── launch-vobee.sh         # Launcher script (Linux/macOS)
```

## Based On

This is a Rust desktop port of the original VoBee-AI-Assistant JavaScript/PWA chatbot, reimagined as a native desktop application using the GPUI framework.

## Contributing

See [CONTRIBUTING.md](../../CONTRIBUTING.md) in the parent project.

## License

Follows the same license as the parent gpui-component project (Apache-2.0).

## Roadmap

Future enhancements planned:
- [ ] Voice input/output
- [ ] Plugin system for custom responses
- [ ] Multi-language support
- [ ] Cloud sync for conversations
- [ ] Advanced AI model integration
- [ ] Custom themes and styling
- [ ] Notification system
- [ ] Keyboard shortcuts customization
