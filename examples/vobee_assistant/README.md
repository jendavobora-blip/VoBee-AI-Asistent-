# VoBee AI Assistant

A Rust desktop application version of the VoBee AI chatbot, built using the GPUI component library.

## Features

- 🐝 **Friendly AI Chatbot** - Creative and engaging responses
- 💬 **Pattern-Based Responses** - Multiple response categories including greetings, jokes, fun facts, and more
- 📝 **Conversation History** - Persistent chat history during the session
- 🎨 **Modern UI** - Built with GPUI components for a native desktop experience
- 🧠 **Learning Capability** - Logs unrecognized queries for future improvements
- 🌐 **REST API** - FlutterFlow integration via REST API server
- 🔌 **Cross-Platform Integration** - Easily integrate with Flutter and FlutterFlow apps

## Response Categories

- Greetings & Farewells
- Identity & Capabilities
- Emotional responses (happy, sad, bored)
- Jokes
- Fun Facts
- Time-related responses
- Gratitude responses

## Running the Application

### Desktop GUI Application

Run the desktop application with GPUI interface:

```bash
cargo run --example vobee_assistant
```

### API Server for FlutterFlow Integration

Run the REST API server for integration with FlutterFlow apps:

```bash
cargo run --bin vobee_api
```

The API server will start on `http://localhost:3000` by default. 

**Documentation:**
- [Quick Start Guide](QUICKSTART.md) - Get started in minutes
- [FlutterFlow Integration Guide](FLUTTERFLOW_INTEGRATION.md) - Detailed integration instructions
- [Security Summary](SECURITY_SUMMARY.md) - Security considerations and best practices

## Architecture

### Modules

- **`chatbot.rs`** - Core chatbot logic, message handling, and conversation management
- **`response_patterns.rs`** - Response templates and keyword matching system
- **`main.rs`** - GPUI UI implementation and application entry point
- **`api_server.rs`** - REST API server for FlutterFlow integration
- **`api_main.rs`** - API server entry point

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

## Based On

This is a Rust desktop port of the original VoBee-AI-Assistant JavaScript/PWA chatbot, reimagined as a native desktop application using the GPUI framework.

## License

Follows the same license as the parent gpui-component project.
