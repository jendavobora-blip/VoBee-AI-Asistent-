# Changelog

All notable changes to VoBee AI will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-12-20

### Added

#### Core Features
- **Persistent Storage System**: SQLite-based conversation history storage
  - Automatic saving of all conversations
  - Message history with timestamps
  - Conversation management (create, load, delete)
  - Data integrity with foreign key constraints
  
- **Settings Management**: JSON-based configuration system
  - Theme selection (light/dark/system)
  - Conversation history toggle
  - Maximum conversation limit
  - Font scaling support
  - Timestamp display option
  - Auto-scroll functionality

- **UI Enhancements**:
  - Light and Dark theme support with toggle button
  - Export functionality for conversations (JSON and Markdown formats)
  - Improved message display with better styling
  - Theme-aware color schemes
  - Optional message numbering/timestamps
  - Better visual hierarchy

- **Backend Integration**:
  - Automatic conversation persistence to SQLite database
  - Settings persistence across sessions
  - Data export to multiple formats
  - Platform-specific data directories (Linux, macOS, Windows)

#### Deployment Tools
- **Build Script** (`script/build-vobee.sh`):
  - Automated production builds
  - Platform detection (Linux, macOS, Windows)
  - Binary size reporting
  - Distribution directory creation
  - README and version file generation
  - Launcher script creation for Unix systems

- **Installation Script** (`script/install-vobee.sh`):
  - System-wide and user-local installation options
  - Desktop entry file installation (Linux)
  - Automatic directory creation
  - Desktop database updates

- **Desktop Integration**:
  - Linux desktop entry file (`.desktop`)
  - Application menu integration
  - Proper categorization (Utility, Chat, Education)

#### Documentation
- **Enhanced README**: Comprehensive feature documentation
  - Installation instructions
  - Usage examples
  - Configuration guide
  - Data storage locations
  - Deployment instructions

- **User Guide** (`USER_GUIDE.md`):
  - Getting started guide
  - Feature overview
  - Usage examples
  - Customization options
  - Troubleshooting section
  - Data management guide
  - Privacy and security information

- **Deployment Guide** (`DEPLOYMENT.md`):
  - Build instructions for all platforms
  - Package creation guides (DEB, RPM, DMG, NSIS)
  - CI/CD integration examples
  - Release management procedures
  - Version management guidelines

### Changed
- **VoBeeApp Structure**: Extended with new fields
  - Added storage integration
  - Added settings management
  - Added theme mode tracking
  - Added conversation ID tracking

- **Message Handling**: Enhanced message processing
  - Integrated with storage system
  - Added conditional persistence based on settings
  - Improved timestamp handling

- **UI Layout**: Improved visual design
  - Better header organization
  - Added control buttons in header
  - Theme-aware colors throughout interface
  - Improved message styling

### Technical Details

#### Dependencies Added
- `rusqlite` (0.32): SQLite database integration with bundled SQLite
- `dirs` (5.0): Platform-specific directory management

#### Module Structure
```
examples/vobee_assistant/src/
├── main.rs              # UI and application logic (enhanced)
├── chatbot.rs           # Core chatbot logic (enhanced with attributes)
├── response_patterns.rs # Response templates (unchanged)
├── storage.rs           # NEW: Persistent storage module
└── settings.rs          # NEW: Settings management module
```

#### Database Schema
```sql
conversations
  - id (INTEGER PRIMARY KEY)
  - title (TEXT)
  - created_at (TEXT)
  - updated_at (TEXT)

messages
  - id (INTEGER PRIMARY KEY)
  - conversation_id (INTEGER FK)
  - sender (TEXT)
  - text (TEXT)
  - timestamp (TEXT)
```

#### File Structure
```
dist/
├── vobee_assistant      # Main executable
├── README.txt           # User documentation
├── VERSION.txt          # Version information
└── launch-vobee.sh     # Launcher script (Unix)

script/
├── build-vobee.sh       # Build script
├── install-vobee.sh     # Installation script
└── vobee-ai.desktop    # Desktop entry file
```

### Security
- All data stored locally on user's device
- No external network communication
- SQLite database with proper foreign key constraints
- Settings file with readable JSON format for transparency

### Performance
- Efficient SQLite queries with indexes
- Minimal memory footprint
- Fast message rendering
- Responsive UI with GPUI framework

### Platform Support
- **Linux**: Full support with desktop integration
- **macOS**: Full support with app bundle capability
- **Windows**: Full support with installer options

### Known Limitations
- No cloud sync (by design - privacy-first)
- No voice input/output (planned for future)
- No plugin system yet (planned for future)
- No multi-language UI (planned for future)
- System theme detection not implemented (defaults to light)

### Breaking Changes
None - This is the first enhanced release.

### Deprecations
None

### Future Enhancements (Roadmap)
- Voice input/output support
- Plugin system for custom responses
- Multi-language UI support
- Cloud sync option (optional)
- Advanced AI model integration
- Custom themes and styling
- Notification system
- Keyboard shortcuts customization
- Conversation search and filtering
- Analytics dashboard
- Auto-update mechanism

## [0.0.1] - 2025-12-03 (Initial Fork)

### Initial Features
- Basic chatbot functionality
- Pattern-based response system
- Simple message history (session only)
- Basic GPUI interface
- Response categories:
  - Greetings
  - Farewells
  - Identity
  - Capabilities
  - Emotional responses
  - Jokes
  - Fun facts
  - Thanks
  - Time
  - Bored responses

---

[0.1.0]: https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/releases/tag/v0.1.0
[0.0.1]: https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/commit/initial
