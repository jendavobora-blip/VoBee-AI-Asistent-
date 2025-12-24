# VoBee AI Assistant - Deployment Guide

## Prerequisites

### System Requirements
- **Operating System**: Windows 10+, macOS 11+, or Linux (Ubuntu 20.04+)
- **Rust**: 1.75+ (Edition 2024 support)
- **Memory**: 4GB RAM minimum, 8GB recommended
- **Storage**: 500MB for application and dependencies

### Platform-Specific Dependencies

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxkbcommon-dev \
    libxkbcommon-x11-dev \
    libfontconfig1-dev \
    libfreetype6-dev \
    libssl-dev \
    pkg-config
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Rust will be installed via rustup (see below)
```

#### Windows
```powershell
# Install Visual Studio C++ Build Tools or Visual Studio 2019+
# Download from: https://visualstudio.microsoft.com/downloads/

# Rust will be installed via rustup (see below)
```

## Installation

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Clone Repository

```bash
git clone https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J.git
cd VoBee-AI-by-Vobora-J
```

### 3. Build VoBee Assistant

#### Development Build
```bash
cd examples/vobee_assistant
cargo build
```

#### Release Build (Optimized)
```bash
cd examples/vobee_assistant
cargo build --release
```

The release binary will be at: `../../target/release/vobee_assistant`

## Running the Application

### Development Mode
```bash
cd examples/vobee_assistant
cargo run
```

### Release Mode
```bash
# After building in release mode
../../target/release/vobee_assistant
```

## Configuration

### Config File Location
The application looks for `config.toml` in the application directory:
```
examples/vobee_assistant/config.toml
```

### Basic Configuration
Edit `config.toml` to customize:

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

### Future: AI Model Configuration
When multi-AI features are implemented, add API keys:

```toml
[ai_models]
primary = "ollama"  # or "claude", "chatgpt", etc.
enable_multi_ai = false

[ai_models.ollama]
endpoint = "http://localhost:11434"
enabled = true

[ai_models.claude]
api_key = "your-api-key-here"
enabled = false
```

**⚠️ Security Note**: Never commit API keys to version control. Use environment variables or a separate `.env` file.

## Docker Deployment (Future)

### Dockerfile
```dockerfile
# Future implementation
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build release binary
RUN cd examples/vobee_assistant && cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    libxcb1 libxkbcommon0 libxkbcommon-x11-0 \
    libfontconfig1 libfreetype6 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/vobee_assistant /usr/local/bin/

CMD ["vobee_assistant"]
```

### Docker Compose
```yaml
# Future implementation
version: '3.8'

services:
  vobee-assistant:
    build: .
    volumes:
      - vobee-data:/data
    environment:
      - DISPLAY=${DISPLAY}
    network_mode: host

volumes:
  vobee-data:
```

## Troubleshooting

### Build Errors

#### Missing System Libraries (Linux)
```
error: could not find native static library `xcb`
```
**Solution**: Install required system dependencies (see Prerequisites)

#### Linker Errors (Windows)
```
error: linking with `link.exe` failed
```
**Solution**: Install Visual Studio C++ Build Tools

### Runtime Errors

#### Window Creation Failed
**Symptoms**: Application crashes on startup
**Solution**: 
- Ensure display server is running (Linux: X11 or Wayland)
- Check `config.toml` for valid window dimensions

#### Configuration File Not Found
**Symptoms**: Warning about missing config
**Solution**: Ensure `config.toml` exists in the application directory

### Performance Issues

#### Slow Response Times
**Current**: Not applicable (local pattern matching is fast)
**Future**: When using AI APIs, check:
- Network connectivity
- API rate limits
- Response caching configuration

## Testing

### Run Tests
```bash
# From repository root
cargo test --package vobee_assistant

# With output
cargo test --package vobee_assistant -- --nocapture
```

### Manual Testing Checklist
- [ ] Application launches successfully
- [ ] Welcome message displays
- [ ] User can send messages
- [ ] Bot responds appropriately
- [ ] Clear button works
- [ ] Window resizing works
- [ ] Application closes cleanly

## Production Deployment

### Best Practices

1. **Use Release Builds**
   - 50-100x faster than debug builds
   - Smaller binary size
   - `cargo build --release`

2. **Strip Debug Symbols** (Optional)
   ```bash
   strip target/release/vobee_assistant
   ```

3. **Package for Distribution**
   - Windows: Create installer (e.g., with WiX)
   - macOS: Create .app bundle and .dmg
   - Linux: Create .deb or .rpm package

4. **Include Dependencies**
   - Bundle required system libraries
   - Include default `config.toml`
   - Add README and LICENSE files

### Cross-Compilation

#### Build for Windows (from Linux)
```bash
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

#### Build for macOS (from macOS)
```bash
# For Apple Silicon
cargo build --release --target aarch64-apple-darwin

# For Intel Macs
cargo build --release --target x86_64-apple-darwin
```

#### Build for Linux (from any platform)
```bash
rustup target add x86_64-unknown-linux-gnu
cargo build --release --target x86_64-unknown-linux-gnu
```

## Continuous Integration

### GitHub Actions
The repository includes CI workflows that:
- Run tests on Windows, macOS, and Linux
- Check code formatting with `cargo fmt`
- Run linter with `cargo clippy`
- Build release binaries

### Local CI Checks
Before pushing:
```bash
# Format code
cargo fmt --all

# Run linter
cargo clippy --all -- -D warnings

# Run tests
cargo test --all

# Build release
cargo build --release
```

## Monitoring and Logs

### Current
- Console output for debugging
- No persistent logs

### Future
- Structured logging with `tracing` crate
- Log rotation
- Error reporting (opt-in telemetry)

## Backup and Recovery

### Current
- All data is in-memory only
- No backup needed
- Conversations lost on exit

### Future (with Persistence)
- Backup SQLite database regularly
- Export conversations to JSON
- Restore from backup file

```bash
# Future commands
vobee_assistant --export conversations.json
vobee_assistant --import conversations.json
vobee_assistant --backup /path/to/backup/
```

## Security Considerations

### Current
- No network access
- No data persistence
- No API keys required

### Future
- Store API keys in system keychain
- Encrypt database at rest
- Use HTTPS for all API calls
- Implement rate limiting
- Regular security audits

## Performance Tuning

### Build Optimizations

Add to `Cargo.toml`:
```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = 'abort'
```

### Runtime Optimizations
- Use release builds in production
- Configure appropriate window size
- Future: Enable response caching
- Future: Tune AI API timeouts

## Support and Updates

### Getting Help
- GitHub Issues: Report bugs and request features
- Documentation: Check README.md and ARCHITECTURE.md
- Community: (Future: Discord/Forum)

### Updates
```bash
cd VoBee-AI-by-Vobora-J
git pull
cd examples/vobee_assistant
cargo build --release
```

### Version Information
```bash
vobee_assistant --version  # Future implementation
```

## License

VoBee AI Assistant follows the license of the parent gpui-component project (Apache-2.0).

## Changelog

### v0.1.0 (Current)
- Initial release
- Basic pattern-based chatbot
- GPUI-based desktop UI
- Conversation history (in-memory)
- Unrecognized query tracking

### Future Versions
- v0.2.0: Storage & Persistence
- v0.3.0: Learning System
- v0.4.0: Multi-AI Integration
- v1.0.0: Full Avatar Consciousness System
