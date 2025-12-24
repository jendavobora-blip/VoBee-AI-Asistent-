# VoBee API Server

REST API server for VoBee AI Assistant with multi-AI provider orchestration.

## Features

- **Multi-AI Integration**: Orchestrates 6 AI providers (OpenAI, Anthropic, Google, DeepSeek, Microsoft, Perplexity)
- **RESTful API**: Clean HTTP endpoints for chat functionality
- **Thread-safe**: Async Rust with Tokio and Arc<Mutex<>> for state management
- **CORS enabled**: Ready for cross-origin requests (FlutterFlow integration)
- **Health monitoring**: Provider health check endpoint
- **Docker ready**: Includes Dockerfile with health checks

## API Endpoints

### Health Check
```
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

### Chat
```
POST /api/chat
Content-Type: application/json

{
  "message": "Hello, VoBee!",
  "max_tokens": 1000,
  "temperature": 0.7
}
```

Response:
```json
{
  "message": "Hello! How can I help you today?",
  "provider": "OpenAI",
  "tokens_used": 45,
  "cost_estimate": 0.00135
}
```

### Get History
```
GET /api/chat/history
```

Response:
```json
{
  "messages": [
    {
      "sender": "User",
      "text": "Hello, VoBee!",
      "timestamp": "2025-12-24T06:00:00Z"
    },
    {
      "sender": "Bot",
      "text": "Hello! How can I help you today?",
      "timestamp": "2025-12-24T06:00:01Z"
    }
  ]
}
```

### Clear History
```
POST /api/chat/clear
```

Response: 200 OK

## Configuration

Set environment variables for AI providers:

```bash
export OPENAI_API_KEY="sk-..."
export ANTHROPIC_API_KEY="..."
export GOOGLE_API_KEY="..."
export DEEPSEEK_API_KEY="..."
export MICROSOFT_API_KEY="..."
export PERPLEXITY_API_KEY="..."
export PORT=3000  # Optional, defaults to 3000
```

## Running

### Development
```bash
cargo run -p vobee-api-server --bin server
```

### Production
```bash
cargo build --release -p vobee-api-server
./target/release/server
```

### Docker
```bash
docker build -t vobee-api-server -f crates/api-server/Dockerfile .
docker run -p 3000:3000 \
  -e OPENAI_API_KEY="sk-..." \
  vobee-api-server
```

## Architecture

```
┌─────────────────────────────────────────────┐
│           VoBee API Server                  │
│                                             │
│  ┌──────────────────────────────────────┐  │
│  │         Axum HTTP Server             │  │
│  │  /api/health  /api/chat  /api/...   │  │
│  └──────────────────────────────────────┘  │
│                    │                        │
│  ┌──────────────────────────────────────┐  │
│  │      AI Orchestrator                 │  │
│  │  • Task Routing                      │  │
│  │  • Provider Selection                │  │
│  │  • Fallback Logic                    │  │
│  │  • Rate Limiting                     │  │
│  └──────────────────────────────────────┘  │
│                    │                        │
│  ┌─────────┬─────────┬─────────┬────────┐  │
│  │ OpenAI  │Anthropic│  Google │  ...   │  │
│  └─────────┴─────────┴─────────┴────────┘  │
└─────────────────────────────────────────────┘
```

## FlutterFlow Integration

The API server is designed for easy FlutterFlow integration:

1. Add API base URL in FlutterFlow: `http://localhost:3000/api` or `https://your-domain.com/api`
2. Create API calls for each endpoint
3. Use the chat endpoint in your Flutter widgets
4. Display conversation history from the history endpoint

## License

See main repository LICENSE
