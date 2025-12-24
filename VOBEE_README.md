# VoBee AI Assistant - Multi-AI Integration

[![Build Status](https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/actions/workflows/ci.yml/badge.svg)](https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/actions/workflows/ci.yml)

VoBee AI Assistant is an advanced AI chatbot built on top of GPUI Component, featuring **multi-AI orchestration** across 6 major AI providers and a production-ready REST API.

## 🌟 Key Features

### Multi-AI Orchestration
- **6 AI Providers Integration**: OpenAI (GPT-4), Anthropic (Claude 4.5), Google (Gemini 3), DeepSeek, Microsoft Copilot, Perplexity Pro
- **Intelligent Task Routing**: Automatically selects the best AI provider based on task type
- **Fallback Mechanisms**: Seamless failover between providers
- **Rate Limiting**: Per-provider rate limits to prevent quota exhaustion
- **Cost Optimization**: Estimates and tracks API costs per request
- **Health Monitoring**: Real-time provider availability checking

### REST API Server
- **Production Ready**: Built with Axum for high-performance async HTTP
- **Cross-Origin Support**: CORS enabled for web and mobile integration
- **Thread-Safe**: Arc<Mutex<>> for concurrent access
- **Docker Ready**: Multi-stage Dockerfile with health checks
- **FlutterFlow Compatible**: Ready for no-code mobile app integration

### Desktop Application
- **Modern UI**: Beautiful GPUI-based interface with emoji avatars
- **Conversation History**: Persistent chat history
- **Real-time Updates**: Async message processing
- **Cross-Platform**: Linux, macOS, Windows support

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                  VoBee AI Assistant                     │
│                                                         │
│  ┌───────────────────┐       ┌──────────────────────┐  │
│  │ Desktop App       │       │  REST API Server     │  │
│  │  (GPUI)           │       │    (Axum)            │  │
│  │                   │       │                      │  │
│  │ • Chat UI         │       │ • /api/chat          │  │
│  │ • Avatar System   │       │ • /api/health        │  │
│  │ • History View    │       │ • /api/chat/history  │  │
│  └───────────────────┘       └──────────────────────┘  │
│           │                           │                 │
│           └───────────┬───────────────┘                 │
│                       │                                 │
│         ┌─────────────▼─────────────┐                   │
│         │   AI Orchestrator         │                   │
│         │                           │                   │
│         │ • Task Routing            │                   │
│         │ • Provider Selection      │                   │
│         │ • Rate Limiting           │                   │
│         │ • Fallback Logic          │                   │
│         │ • Cost Tracking           │                   │
│         └─────────────┬─────────────┘                   │
│                       │                                 │
│  ┌────────┬──────────┬┴────────┬──────────┬─────────┐  │
│  │OpenAI  │Anthropic │ Google  │DeepSeek  │Microsoft│  │
│  │(GPT-4) │(Claude)  │(Gemini) │          │Copilot  │  │
│  └────────┴──────────┴─────────┴──────────┴─────────┘  │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

## 🚀 Quick Start

### Prerequisites
- Rust 2024 Edition or later
- Docker (optional, for containerized deployment)
- API keys for at least one AI provider

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J.git
   cd VoBee-AI-by-Vobora-J
   ```

2. **Configure API Keys**
   ```bash
   cp .env.example .env
   # Edit .env and add your API keys
   ```

3. **Build the project**
   ```bash
   cargo build --release
   ```

### Running the Desktop App
```bash
cargo run -p vobee_assistant
```

### Running the API Server
```bash
cargo run -p vobee-api-server --bin server
```

Or with Docker:
```bash
docker-compose up
```

## 📚 Project Structure

```
.
├── crates/
│   ├── ai-orchestration/      # Multi-AI provider orchestration
│   │   ├── src/
│   │   │   ├── providers/     # AI provider implementations
│   │   │   ├── routing.rs     # Task routing logic
│   │   │   ├── rate_limiter.rs # Rate limiting
│   │   │   ├── orchestrator.rs # Main orchestrator
│   │   │   └── ...
│   │   └── Cargo.toml
│   │
│   ├── api-server/            # REST API server
│   │   ├── src/
│   │   │   ├── handlers.rs    # HTTP handlers
│   │   │   ├── state.rs       # Application state
│   │   │   └── ...
│   │   ├── Dockerfile
│   │   └── README.md
│   │
│   ├── ui/                    # GPUI component library
│   ├── assets/                # UI assets
│   └── ...
│
├── examples/
│   └── vobee_assistant/       # Desktop application
│       ├── src/
│       │   ├── chatbot.rs     # Chatbot logic
│       │   ├── main.rs        # Main app
│       │   └── ...
│       └── Cargo.toml
│
├── docker-compose.yml         # Docker Compose configuration
├── .env.example               # Environment variables template
└── README.md                  # Original GPUI Component README
```

## 🔌 API Documentation

### Endpoints

#### Health Check
```http
GET /api/health
```

Response:
```json
{
  "status": "healthy",
  "providers": {
    "OpenAI": true,
    "Anthropic": true,
    "Google": false
  }
}
```

#### Send Chat Message
```http
POST /api/chat
Content-Type: application/json

{
  "message": "What is Rust?",
  "max_tokens": 1000,
  "temperature": 0.7
}
```

Response:
```json
{
  "message": "Rust is a systems programming language...",
  "provider": "OpenAI",
  "tokens_used": 150,
  "cost_estimate": 0.0045
}
```

#### Get Conversation History
```http
GET /api/chat/history
```

#### Clear History
```http
POST /api/chat/clear
```

Full API documentation: [crates/api-server/README.md](crates/api-server/README.md)

## 🎯 Task Routing

VoBee intelligently routes tasks to the most appropriate AI provider:

| Task Type | Primary Provider(s) | Fallback |
|-----------|---------------------|----------|
| Conversation | OpenAI, Anthropic | Google |
| Code Generation | DeepSeek, OpenAI | Anthropic |
| Research | Perplexity, OpenAI | - |
| Multi-modal | Google, OpenAI | - |
| Development | Microsoft, DeepSeek | OpenAI |
| Reasoning | Anthropic, OpenAI | - |

## 🔧 Configuration

### Environment Variables

```bash
# AI Provider API Keys
OPENAI_API_KEY=sk-...
ANTHROPIC_API_KEY=...
GOOGLE_API_KEY=...
DEEPSEEK_API_KEY=...
MICROSOFT_API_KEY=...
PERPLEXITY_API_KEY=...

# Server Configuration
PORT=3000
RUST_LOG=info
```

### Rate Limits (default)
- OpenAI: 60 requests/minute
- Anthropic: 50 requests/minute
- Google: 60 requests/minute
- DeepSeek: 30 requests/minute
- Microsoft: 60 requests/minute
- Perplexity: 40 requests/minute

## 🐳 Docker Deployment

### Build and Run
```bash
docker-compose up -d
```

### Custom Configuration
```bash
docker-compose up -d \
  -e OPENAI_API_KEY=sk-... \
  -e PORT=8080
```

### Health Check
```bash
curl http://localhost:3000/api/health
```

## 🛠️ Development

### Build All Crates
```bash
cargo build --workspace
```

### Run Tests
```bash
cargo test --workspace
```

### Check Code
```bash
cargo check --workspace
```

### Format Code
```bash
cargo fmt --all
```

### Run Linter
```bash
cargo clippy --all -- -D warnings
```

## 🤝 Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📝 License

This project extends the original GPUI Component library. See [LICENSE](LICENSE-APACHE) for details.

## 🙏 Acknowledgments

- Built on [GPUI](https://gpui.rs) - GPU-accelerated UI framework
- Original component library: [longbridge/gpui-component](https://github.com/longbridge/gpui-component)
- AI provider integrations: OpenAI, Anthropic, Google, DeepSeek, Microsoft, Perplexity

## 📞 Support

For issues and questions:
- GitHub Issues: [https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/issues](https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/issues)
- Pull Requests: [https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/pulls](https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/pulls)

---

**Note**: This is an enhanced version of the VoBee AI Assistant with enterprise-grade multi-AI orchestration and production-ready deployment capabilities.
