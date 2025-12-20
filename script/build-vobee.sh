#!/usr/bin/env bash
set -euo pipefail

# VoBee AI - Production Build Script
# This script builds the VoBee AI application for production release

echo "🐝 VoBee AI - Production Build Script"
echo "======================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Detect OS
OS="unknown"
case "$(uname -s)" in
    Linux*)     OS="linux";;
    Darwin*)    OS="macos";;
    MINGW*|MSYS*|CYGWIN*) OS="windows";;
esac

echo -e "${YELLOW}Detected OS: ${OS}${NC}"

# Check for required tools
echo ""
echo "Checking for required tools..."

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo not found. Please install Rust.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ cargo found${NC}"

if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: rustc not found. Please install Rust.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ rustc found${NC}"

# Print versions
echo ""
echo "Tool versions:"
cargo --version
rustc --version

# Build configuration
BUILD_MODE="${BUILD_MODE:-release}"
TARGET_DIR="target/${BUILD_MODE}"
BINARY_NAME="vobee_assistant"

# OS-specific binary extension
BINARY_EXT=""
if [ "$OS" = "windows" ]; then
    BINARY_EXT=".exe"
fi

echo ""
echo -e "${YELLOW}Building VoBee AI in ${BUILD_MODE} mode...${NC}"

# Build the application
if [ "$BUILD_MODE" = "release" ]; then
    cargo build --release --example vobee_assistant
else
    cargo build --example vobee_assistant
fi

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Build successful!${NC}"
else
    echo -e "${RED}✗ Build failed!${NC}"
    exit 1
fi

# Find the binary
BINARY_PATH="${TARGET_DIR}/examples/${BINARY_NAME}${BINARY_EXT}"

if [ ! -f "$BINARY_PATH" ]; then
    echo -e "${RED}Error: Binary not found at ${BINARY_PATH}${NC}"
    exit 1
fi

# Get binary size
BINARY_SIZE=$(du -h "$BINARY_PATH" | cut -f1)
echo ""
echo -e "${GREEN}Binary information:${NC}"
echo "  Location: ${BINARY_PATH}"
echo "  Size: ${BINARY_SIZE}"

# Create dist directory
DIST_DIR="dist"
mkdir -p "$DIST_DIR"

# Copy binary to dist
cp "$BINARY_PATH" "$DIST_DIR/${BINARY_NAME}${BINARY_EXT}"
echo -e "${GREEN}✓ Binary copied to ${DIST_DIR}/${NC}"

# Create README for distribution
cat > "$DIST_DIR/README.txt" << 'EOF'
VoBee AI - Your Friendly AI Assistant
======================================

Thank you for downloading VoBee AI!

QUICK START
-----------
Run the application by double-clicking the executable or running:
  ./vobee_assistant  (Linux/macOS)
  vobee_assistant.exe  (Windows)

FEATURES
--------
- 🐝 Friendly AI Chatbot with engaging responses
- 💬 Pattern-based conversation system
- 📝 Persistent conversation history (saved automatically)
- 🎨 Light and Dark theme support
- 📄 Export conversations to JSON or Markdown
- 🧠 Learning capability from interactions

DATA STORAGE
------------
Your conversations are stored in:
- Linux: ~/.local/share/VoBee-AI/
- macOS: ~/Library/Application Support/VoBee-AI/
- Windows: %APPDATA%\VoBee-AI\

SETTINGS
--------
Settings are stored in:
- Linux: ~/.config/VoBee-AI/settings.json
- macOS: ~/Library/Application Support/VoBee-AI/settings.json
- Windows: %APPDATA%\VoBee-AI\settings.json

LICENSE
-------
Apache-2.0 License

For more information, visit:
https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J

EOF

echo -e "${GREEN}✓ README created${NC}"

# Create version info
# Try to get version, fallback to 0.1.0 if extraction fails
if command -v jq &> /dev/null; then
    VERSION=$(cargo metadata --format-version 1 --no-deps 2>/dev/null | jq -r '.packages[0].version' 2>/dev/null || echo "0.1.0")
else
    # Fallback method without jq
    VERSION=$(cargo metadata --format-version 1 --no-deps 2>/dev/null | grep -o '"version":"[^"]*"' | head -1 | cut -d'"' -f4 || echo "0.1.0")
fi
echo "$VERSION" > "$DIST_DIR/VERSION.txt"
echo -e "${GREEN}✓ Version file created (v${VERSION})${NC}"

# Create launcher script for Linux/macOS
if [ "$OS" != "windows" ]; then
    cat > "$DIST_DIR/launch-vobee.sh" << 'EOF'
#!/usr/bin/env bash
# VoBee AI Launcher
cd "$(dirname "$0")"
./vobee_assistant
EOF
    chmod +x "$DIST_DIR/launch-vobee.sh"
    echo -e "${GREEN}✓ Launcher script created${NC}"
fi

# Summary
echo ""
echo -e "${GREEN}======================================"
echo "Build Complete! 🎉"
echo "======================================${NC}"
echo ""
echo "Distribution files are in: ${DIST_DIR}/"
echo ""
echo "To test the application:"
echo "  cd ${DIST_DIR}"
echo "  ./${BINARY_NAME}${BINARY_EXT}"
echo ""
echo "Next steps:"
echo "  1. Test the application"
echo "  2. Create a release archive:"
echo "     tar -czf vobee-ai-${OS}.tar.gz -C ${DIST_DIR} ."
echo "  3. Upload to GitHub releases"
echo ""
