# VoBee AI Assistant - Architecture

## Overview

VoBee AI Assistant is a Rust-based desktop chatbot application built with GPUI framework. This document outlines the current architecture and planned enhancements.

## Current Architecture (v0.1.0)

### Core Components

```
vobee_assistant/
├── src/
│   ├── main.rs              # UI and application entry point
│   ├── chatbot.rs           # Core chatbot logic and state
│   └── response_patterns.rs # Pattern matching and responses
├── config.toml              # Configuration file
└── Cargo.toml              # Dependencies
```

### Module Descriptions

#### `main.rs` - UI Layer (GPUI)
- Implements the `VoBeeApp` struct for UI rendering
- Handles user input and message display
- Manages the chat interface with GPUI components
- Features:
  - Message history display
  - Input field with send button
  - Clear conversation button

#### `chatbot.rs` - Core Logic
- `VoBeeChatbot` struct: Main chatbot state management
- `Message` struct: Individual message with sender, text, and timestamp
- `UnrecognizedQuery` struct: Tracks queries that don't match patterns
- Features:
  - Conversation history tracking
  - Unrecognized query logging (for future learning)
  - Pattern-based response generation

#### `response_patterns.rs` - Response System
- `ResponsePatterns` struct: Manages keyword → response mappings
- Pattern categories:
  - Greetings & Farewells
  - Identity questions
  - Capabilities queries
  - Emotional responses (happy, sad, bored)
  - Jokes
  - Fun facts
  - Thanks/gratitude
  - Time-related
  - Fallback responses

## Planned Architecture Enhancements

### Phase 1: Storage & Persistence

```rust
// Future: storage.rs
pub struct Storage {
    sqlite_db: SqliteConnection,
    // Future: vector_db for semantic search
}
```

**Features:**
- Persistent conversation history across sessions
- SQLite database for structured data
- Export/import conversation history
- Future: Vector database for semantic search

### Phase 2: Learning System

```rust
// Future: learning.rs
pub struct LearningSystem {
    unrecognized_queries: HashMap<String, QueryMetadata>,
    pattern_suggestions: Vec<PatternSuggestion>,
    learning_rate: f32,
}
```

**Features:**
- Analyze unrecognized queries
- Suggest new patterns to add
- Daily learning consolidation
- Adaptive response improvement

### Phase 3: Multi-AI Integration

```rust
// Future: ai_orchestrator.rs
pub struct AIOrchestrator {
    claude_client: Option<ClaudeClient>,
    chatgpt_client: Option<ChatGPTClient>,
    deepseek_client: Option<DeepSeekClient>,
    gemini_client: Option<GeminiClient>,
    perplexity_client: Option<PerplexityClient>,
    ollama_client: Option<OllamaClient>,
    routing_strategy: RoutingStrategy,
}

pub enum RoutingStrategy {
    BestForTask,      // Auto-select best AI for task type
    ConsensusVoting,  // Get responses from multiple AIs
    Sequential,       // Chain AIs for complex tasks
    Parallel,         // Aggregate multiple responses
    LocalFirst,       // Try local (Ollama) first, fallback to cloud
}
```

**Features:**
- Unified interface for multiple AI providers
- Intelligent routing based on query type
- Fallback mechanisms for reliability
- Privacy-first mode (Ollama for sensitive data)

### Phase 4: Avatar Consciousness System

```rust
// Future: avatar/consciousness.rs
pub struct AvatarConsciousness {
    mood_state: MoodState,
    awareness_level: f32,
    personality_traits: PersonalityTraits,
    emotional_intelligence: EmotionalIntelligence,
}

pub enum MoodState {
    Calm { confidence: f32 },
    Curious { interest_level: f32 },
    Stressed { anxiety: f32 },
    Excited { enthusiasm: f32 },
    Contemplative { depth: f32 },
}
```

**Features:**
- Mood-based response variation
- Personality consistency
- Emotional context awareness
- Daily self-reflection and improvement

### Phase 5: Advanced Memory System

```rust
// Future: avatar/memory.rs
pub struct NeuralMemory {
    episodic_memory: EpisodicMemory,    // Conversation history with context
    semantic_memory: SemanticMemory,     // Learned knowledge
    procedural_memory: ProceduralMemory, // Learned behaviors
    working_memory: WorkingMemory,       // Active conversation context
}
```

**Features:**
- Multi-tiered memory architecture
- Context-aware responses based on conversation history
- Knowledge consolidation
- Memory prioritization and forgetting

## Technology Stack

### Current
- **Language**: Rust (Edition 2024)
- **UI Framework**: GPUI 0.2.2
- **Components**: gpui-component 0.5.0-preview2
- **Serialization**: serde + serde_json
- **Time**: chrono 0.4
- **Random**: rand 0.8

### Planned Additions
- **Database**: SQLite (rusqlite)
- **Vector DB**: qdrant-client or similar
- **HTTP Client**: reqwest (already in workspace)
- **AI SDKs**: anthropic-sdk, openai-api, etc.
- **Config**: toml crate for config parsing
- **Async**: Already available via gpui's async runtime

## Data Flow

### Current Flow
```
User Input → VoBeeApp → VoBeeChatbot → ResponsePatterns → Response
                ↓
        Conversation History
                ↓
    Unrecognized Query Tracking
```

### Future Flow (Multi-AI)
```
User Input → VoBeeApp → VoBeeChatbot → AIOrchestrator
                                             ↓
                                    Routing Decision
                                             ↓
                        ┌────────────────────┼────────────────────┐
                        ↓                    ↓                    ↓
                  Claude/GPT            Ollama (Local)      DeepSeek
                        ↓                    ↓                    ↓
                        └────────────────────┼────────────────────┘
                                             ↓
                                    Response Aggregation
                                             ↓
                                    Learning System
                                             ↓
                                    Storage Layer
```

## Security Considerations

### Current
- All data stored in memory only (ephemeral)
- No external network calls
- No sensitive data handling

### Planned
- Encrypted storage for conversation history
- API key management (secure storage)
- Local-first processing for sensitive queries (Ollama)
- Data retention policies
- User privacy controls

## Performance Considerations

### Current
- Lightweight pattern matching (HashMap lookups)
- In-memory storage only
- Synchronous response generation

### Planned
- Async API calls to AI providers
- Database query optimization
- Response caching
- Rate limiting for API calls
- Background learning tasks

## Testing Strategy

### Current
- No automated tests (manual testing only)

### Planned
- Unit tests for core logic
- Integration tests for AI clients
- UI tests with GPUI test utilities
- Mock AI responses for testing
- Performance benchmarks

## Deployment

### Current
- Local development build only
- Manual cargo run

### Planned
- Release builds with optimizations
- Cross-platform binaries (Windows, macOS, Linux)
- Docker containerization
- Auto-update mechanism
- Telemetry (opt-in)

## Configuration Management

Configuration is managed through `config.toml`:
- Assistant settings (name, version)
- Learning parameters
- Storage options
- UI preferences
- AI model credentials (future)

## Future Roadmap

1. **Q1 2025**: Storage & Persistence Layer
2. **Q2 2025**: Basic Learning System
3. **Q3 2025**: Multi-AI Integration (Ollama first)
4. **Q4 2025**: Avatar Consciousness & Advanced Features
5. **2026**: Market Integration, API Server, Mobile Apps

## Contributing

When contributing to VoBee AI Assistant:
1. Follow Rust 2024 idioms and best practices
2. Add documentation for new modules
3. Include unit tests for business logic
4. Update this ARCHITECTURE.md with new components
5. Ensure backward compatibility when possible

## References

- [GPUI Framework](https://gpui.rs)
- [gpui-component Library](https://github.com/longbridge/gpui-component)
- Original VoBee-AI-Assistant (JavaScript/PWA)
