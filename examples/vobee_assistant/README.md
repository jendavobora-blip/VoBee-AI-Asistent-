# VoBee AI Assistant 🐝

A production-ready, self-learning Rust desktop application built using the GPUI framework. VoBee is designed to evolve from a pattern-based chatbot into an advanced multi-AI powered assistant with consciousness-like behavior.

## Current Features (v0.1.0)

- 🐝 **Friendly AI Chatbot** - Creative and engaging responses
- 💬 **Pattern-Based Responses** - Multiple response categories including greetings, jokes, fun facts, and more
- 📝 **Conversation History** - In-memory chat history during the session
- 🎨 **Modern UI** - Built with GPUI components for a native desktop experience
- 🧠 **Learning Foundation** - Tracks unrecognized queries for future improvements
- ⚙️ **Configurable** - TOML-based configuration system

## Response Categories

- Greetings & Farewells
- Identity & Capabilities
- Emotional responses (happy, sad, bored)
- Jokes
- Fun Facts
- Time-related responses
- Gratitude responses

## Quick Start

### Prerequisites
- Rust 1.75+ with 2024 edition support
- Platform-specific dependencies (see [DEPLOYMENT.md](DEPLOYMENT.md))

### Running the Application

```bash
cd examples/vobee_assistant
cargo run
```

For production builds:
```bash
cargo build --release
../../target/release/vobee_assistant
```

## Architecture

### Modules

- **`chatbot.rs`** - Core chatbot logic, message handling, and conversation management
- **`response_patterns.rs`** - Response templates and keyword matching system
- **`main.rs`** - GPUI UI implementation and application entry point

### Key Features

- **Pattern Matching**: Keyword-based response system with multiple variations for each category
- **Random Responses**: Provides variety by randomly selecting from available response templates
- **Message History**: Maintains full conversation history with timestamps
- **Unrecognized Query Logging**: Tracks queries that don't match any pattern for future learning

## Usage Examples

Try these interactions:
- "Hello" or "Hi there" - Get a friendly greeting
- "Tell me a joke" - Hear a random joke
- "Fun fact" - Learn something interesting
- "I'm feeling sad" - Receive emotional support
- "What can you do?" - Learn about VoBee's capabilities
- "Bye" - Say goodbye

## Planned Features (Roadmap)

### Phase 1: Storage & Persistence (v0.2.0)
- ✨ SQLite database for conversation history
- ✨ Persistent storage across sessions
- ✨ Export/import conversations
- ✨ Vector database for semantic search

### Phase 2: Learning System (v0.3.0)
- 🧠 Pattern learning from interactions
- 🧠 Daily self-improvement routines
- 🧠 Adaptive response generation
- 🧠 User preference learning

### Phase 3: Multi-AI Integration (v0.4.0)
- 🤖 Claude 4.5 Sonnet - Complex reasoning
- 🤖 ChatGPT-4 - Natural conversation
- 🤖 DeepSeek - Code generation
- 🤖 Gemini 3 - Multimodal understanding
- 🤖 Perplexity Pro - Real-time research
- 🤖 Ollama - Privacy-first local inference
- 🔀 Intelligent AI routing strategies

### Phase 4: Avatar Consciousness (v1.0.0)
- 🧘 Mood states and emotional intelligence
- 🧘 Personality consistency
- 🧘 Neural memory architecture (episodic, semantic, procedural)
- 🧘 Daily reflection and growth
- 🧘 Context-aware conversations

### Phase 5: Advanced Features (v1.x)
- 📊 Market integration (crypto/stocks)
- 🚨 FOMO prevention and calming guidance
- 🔐 Enhanced privacy and security
- 🌐 REST API and mobile integration
- 🐳 Docker deployment
- 📱 FlutterFlow mobile app integration

## Architecture

VoBee follows a modular architecture designed for extensibility. See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed information on:
- Current component structure
- Data flow and state management
- Planned enhancements and integrations
- Technology stack

## Configuration

Configuration is managed via `config.toml`. See example configuration:

```toml
[assistant]
name = "VoBee"
version = "0.1.0"

[learning]
enable_learning = true
track_unrecognized = true

[ui]
window_width = 800
window_height = 600
```

Future versions will support AI model configuration with API keys.

## Deployment

For production deployment instructions, see [DEPLOYMENT.md](DEPLOYMENT.md), which covers:
- Platform-specific requirements
- Building release binaries
- Cross-compilation
- Docker containerization (future)
- Performance tuning

## Development

### Project Structure

```
vobee_assistant/
├── src/
│   ├── main.rs              # UI and app entry point
│   ├── chatbot.rs           # Core chatbot logic
│   └── response_patterns.rs # Pattern matching
├── config.toml              # Configuration
├── ARCHITECTURE.md          # Architecture documentation
├── DEPLOYMENT.md            # Deployment guide
└── README.md               # This file
```

### Running Tests

```bash
cargo test --package vobee_assistant
```

### Code Quality

```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all -- -D warnings

# Check for typos
typos
```

## Contributing

Contributions are welcome! When contributing:
1. Follow Rust 2024 idioms and best practices
2. Add tests for new functionality
3. Update documentation
4. Ensure CI passes

## Based On

This is a Rust desktop port of the original VoBee-AI-Assistant JavaScript/PWA chatbot, reimagined as a native desktop application using the GPUI framework. The project aims to create a production-ready, self-learning AI assistant with advanced capabilities.

## License

Apache-2.0 (follows the parent gpui-component project license)

## Acknowledgments

- Built with [GPUI](https://gpui.rs) framework
- UI components from [gpui-component](https://github.com/longbridge/gpui-component)
- Inspired by modern AI assistants and conversational AI research
