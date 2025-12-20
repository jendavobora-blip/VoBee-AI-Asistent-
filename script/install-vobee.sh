#!/usr/bin/env bash
set -euo pipefail

# VoBee AI - Installation Script
# This script installs VoBee AI on the system

echo "🐝 VoBee AI - Installation Script"
echo "=================================="

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Check if running as root (not needed for user install)
if [ "$EUID" -eq 0 ]; then 
    echo -e "${YELLOW}Warning: Running as root. Installing system-wide.${NC}"
    INSTALL_PREFIX="/usr/local"
    APPLICATIONS_DIR="/usr/share/applications"
else
    echo "Installing for current user..."
    INSTALL_PREFIX="$HOME/.local"
    APPLICATIONS_DIR="$HOME/.local/share/applications"
fi

BIN_DIR="$INSTALL_PREFIX/bin"

# Create directories
mkdir -p "$BIN_DIR"
mkdir -p "$APPLICATIONS_DIR"

# Check if binary exists in dist/
if [ ! -f "dist/vobee_assistant" ]; then
    echo -e "${RED}Error: vobee_assistant binary not found in dist/${NC}"
    echo "Please run: ./script/build-vobee.sh first"
    exit 1
fi

# Copy binary
echo "Installing binary to $BIN_DIR..."
cp dist/vobee_assistant "$BIN_DIR/"
chmod +x "$BIN_DIR/vobee_assistant"
echo -e "${GREEN}✓ Binary installed${NC}"

# Install desktop entry (Linux only)
if [ -f "script/vobee-ai.desktop" ]; then
    echo "Installing desktop entry to $APPLICATIONS_DIR..."
    cp script/vobee-ai.desktop "$APPLICATIONS_DIR/"
    
    # Update desktop database if available
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database "$APPLICATIONS_DIR" 2>/dev/null || true
    fi
    echo -e "${GREEN}✓ Desktop entry installed${NC}"
fi

# Create data directory
DATA_DIR="$HOME/.local/share/VoBee-AI"
mkdir -p "$DATA_DIR"
echo -e "${GREEN}✓ Data directory created: $DATA_DIR${NC}"

# Create config directory
CONFIG_DIR="$HOME/.config/VoBee-AI"
mkdir -p "$CONFIG_DIR"
echo -e "${GREEN}✓ Config directory created: $CONFIG_DIR${NC}"

echo ""
echo -e "${GREEN}=================================="
echo "Installation Complete! 🎉"
echo "==================================${NC}"
echo ""
echo "You can now run VoBee AI by:"
echo "  1. Running: vobee_assistant"
echo "  2. Or searching for 'VoBee AI' in your application menu"
echo ""
echo "Data is stored in: $DATA_DIR"
echo "Settings are in: $CONFIG_DIR"
echo ""
