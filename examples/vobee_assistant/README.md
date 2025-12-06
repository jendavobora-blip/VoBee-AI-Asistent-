# VoBee AI Assistant

A Rust desktop application version of the VoBee AI chatbot, built using the GPUI component library.

## Features

- 🐝 **Friendly AI Chatbot** - Creative and engaging responses
- 💬 **Pattern-Based Responses** - Multiple response categories including greetings, jokes, fun facts, and more
- 📝 **Conversation History** - Persistent chat history during the session
- 🎨 **Modern UI** - Built with GPUI components for a native desktop experience
- 🧠 **Learning Capability** - Logs unrecognized queries for future improvements

## Response Categories

- Greetings & Farewells
- Identity & Capabilities
- Emotional responses (happy, sad, bored)
- Jokes
- Fun Facts
- Time-related responses
- Gratitude responses

## Running the Application

```bash
cargo run --example vobee_assistant
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

## Based On

This is a Rust desktop port of the original VoBee-AI-Assistant JavaScript/PWA chatbot, reimagined as a native desktop application using the GPUI framework.

## License

Follows the same license as the parent gpui-component project.
