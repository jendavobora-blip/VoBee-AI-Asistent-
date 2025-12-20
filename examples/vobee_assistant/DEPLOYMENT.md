# VoBee AI Deployment Guide

This guide covers building, packaging, and deploying VoBee AI for different platforms.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Building for Production](#building-for-production)
3. [Platform-Specific Instructions](#platform-specific-instructions)
4. [Creating Release Packages](#creating-release-packages)
5. [Installation Methods](#installation-methods)
6. [CI/CD Integration](#cicd-integration)

## Prerequisites

### Development Tools

- **Rust Toolchain**: 1.70 or later
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Cargo**: Included with Rust
  ```bash
  cargo --version
  ```

### System Dependencies

#### Linux (Ubuntu/Debian)
```bash
sudo apt-get update
sudo apt-get install -y \
    build-essential \
    libfontconfig-dev \
    libwayland-dev \
    libxkbcommon-x11-dev \
    libx11-xcb-dev \
    libssl-dev \
    libzstd-dev \
    pkg-config
```

#### macOS
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Rust will handle most dependencies
```

#### Windows
- Install Visual Studio Build Tools 2019 or later
- Rust toolchain with MSVC target
- Windows 10 SDK

## Building for Production

### Using the Build Script

The easiest way to build VoBee AI:

```bash
# Navigate to project root
cd VoBee-AI-by-Vobora-J

# Run build script
./script/build-vobee.sh
```

This script:
1. Checks for required tools
2. Builds in release mode
3. Creates a `dist/` directory
4. Copies the binary
5. Generates documentation files
6. Creates launcher scripts

### Manual Build

For manual control over the build process:

```bash
# Release build
cargo build --release --example vobee_assistant

# Find the binary
ls -lh target/release/examples/vobee_assistant

# Test the binary
./target/release/examples/vobee_assistant
```

### Build Configurations

#### Optimized Release Build
```bash
# Maximum optimization
RUSTFLAGS="-C target-cpu=native" cargo build --release --example vobee_assistant
```

#### Debug Build
```bash
# For development and debugging
cargo build --example vobee_assistant
```

#### Cross-Compilation
```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Build for target
cargo build --release --target x86_64-unknown-linux-gnu --example vobee_assistant
```

## Platform-Specific Instructions

### Linux

#### Building
```bash
./script/build-vobee.sh
```

#### Installing
```bash
# System-wide installation (requires sudo)
sudo ./script/install-vobee.sh

# User-local installation
./script/install-vobee.sh
```

#### Package Structure
```
dist/
├── vobee_assistant          # Executable
├── README.txt               # User documentation
├── VERSION.txt              # Version info
└── launch-vobee.sh         # Launcher script
```

#### Creating DEB Package
```bash
# Install packaging tools
sudo apt-get install dpkg-deb

# Create package structure
mkdir -p vobee-ai_0.1.0/DEBIAN
mkdir -p vobee-ai_0.1.0/usr/local/bin
mkdir -p vobee-ai_0.1.0/usr/share/applications

# Create control file
cat > vobee-ai_0.1.0/DEBIAN/control << EOF
Package: vobee-ai
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: VoBee AI Team
Description: Your Friendly AI Assistant
 VoBee AI is a desktop chatbot application built with Rust and GPUI.
EOF

# Copy files
cp dist/vobee_assistant vobee-ai_0.1.0/usr/local/bin/
cp script/vobee-ai.desktop vobee-ai_0.1.0/usr/share/applications/

# Build package
dpkg-deb --build vobee-ai_0.1.0
```

#### Creating RPM Package
```bash
# Install rpmbuild
sudo yum install rpm-build

# Create RPM structure
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}

# Create spec file (vobee-ai.spec)
# Build RPM
rpmbuild -ba vobee-ai.spec
```

### macOS

#### Building
```bash
./script/build-vobee.sh
```

#### Creating App Bundle
```bash
# Create bundle structure
mkdir -p "VoBee AI.app/Contents/MacOS"
mkdir -p "VoBee AI.app/Contents/Resources"

# Copy executable
cp dist/vobee_assistant "VoBee AI.app/Contents/MacOS/"

# Create Info.plist
cat > "VoBee AI.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>vobee_assistant</string>
    <key>CFBundleIdentifier</key>
    <string>com.vobee.ai</string>
    <key>CFBundleName</key>
    <string>VoBee AI</string>
    <key>CFBundleVersion</key>
    <string>0.1.0</string>
    <key>CFBundleShortVersionString</key>
    <string>0.1.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
</dict>
</plist>
EOF

# Create DMG
hdiutil create -volname "VoBee AI" -srcfolder "VoBee AI.app" -ov -format UDZO vobee-ai-macos.dmg
```

### Windows

#### Building
```powershell
# Run from PowerShell
$env:BUILD_MODE="release"
cargo build --release --example vobee_assistant
```

#### Creating Installer (NSIS)
```nsis
; vobee-ai-installer.nsi
!define APPNAME "VoBee AI"
!define COMPANYNAME "VoBee Team"
!define DESCRIPTION "Your Friendly AI Assistant"
!define VERSIONMAJOR 0
!define VERSIONMINOR 1
!define VERSIONBUILD 0

Name "${APPNAME}"
OutFile "vobee-ai-installer.exe"
InstallDir "$PROGRAMFILES\${APPNAME}"

Section "Install"
    SetOutPath "$INSTDIR"
    File "dist\vobee_assistant.exe"
    File "dist\README.txt"
    
    CreateShortCut "$DESKTOP\${APPNAME}.lnk" "$INSTDIR\vobee_assistant.exe"
    CreateShortCut "$SMPROGRAMS\${APPNAME}.lnk" "$INSTDIR\vobee_assistant.exe"
SectionEnd
```

## Creating Release Packages

### Archive Creation

#### Linux/macOS
```bash
# Build first
./script/build-vobee.sh

# Create tarball
cd dist
tar -czf vobee-ai-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).tar.gz *

# Or zip
zip -r vobee-ai-$(uname -s | tr '[:upper:]' '[:lower:]')-$(uname -m).zip *
```

#### Windows
```powershell
# Use PowerShell
Compress-Archive -Path dist\* -DestinationPath vobee-ai-windows-x64.zip
```

### GitHub Release

1. **Create Release on GitHub:**
   ```bash
   # Tag the release
   git tag -a v0.1.0 -m "Release v0.1.0"
   git push origin v0.1.0
   ```

2. **Upload Artifacts:**
   - Navigate to GitHub repository
   - Click "Releases" → "Create new release"
   - Upload platform-specific archives
   - Add release notes

3. **Automated via GitHub Actions:**
   Create `.github/workflows/release.yml`:
   ```yaml
   name: Release
   
   on:
     push:
       tags:
         - 'v*'
   
   jobs:
     build:
       strategy:
         matrix:
           os: [ubuntu-latest, macos-latest, windows-latest]
       
       runs-on: ${{ matrix.os }}
       
       steps:
         - uses: actions/checkout@v4
         
         - name: Install Rust
           uses: actions-rs/toolchain@v1
           with:
             toolchain: stable
         
         - name: Build
           run: cargo build --release --example vobee_assistant
         
         - name: Create Archive
           run: |
             # Platform-specific archiving
         
         - name: Upload Release Asset
           uses: actions/upload-release-asset@v1
           with:
             upload_url: ${{ github.event.release.upload_url }}
             asset_path: ./release.tar.gz
   ```

## Installation Methods

### Method 1: Direct Binary

```bash
# Download
wget https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/releases/download/v0.1.0/vobee-ai-linux-x64.tar.gz

# Extract
tar -xzf vobee-ai-linux-x64.tar.gz

# Run
./vobee_assistant
```

### Method 2: Install Script

```bash
# Download and run install script
curl -sSL https://raw.githubusercontent.com/jendavobora-blip/VoBee-AI-by-Vobora-J/main/script/install-vobee.sh | bash
```

### Method 3: Package Manager

#### Homebrew (macOS)
```ruby
# Create formula
class VobeeAi < Formula
  desc "Your Friendly AI Assistant"
  homepage "https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J"
  url "https://github.com/jendavobora-blip/VoBee-AI-by-Vobora-J/releases/download/v0.1.0/vobee-ai-macos.tar.gz"
  sha256 "..."
  
  def install
    bin.install "vobee_assistant"
  end
end
```

#### APT (Debian/Ubuntu)
```bash
# Add repository
echo "deb [trusted=yes] https://apt.vobee.ai/ stable main" | sudo tee /etc/apt/sources.list.d/vobee-ai.list

# Install
sudo apt update
sudo apt install vobee-ai
```

## CI/CD Integration

### GitHub Actions Example

```yaml
name: Build and Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Cache cargo
        uses: actions/cache@v3
        with:
          path: ~/.cargo
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Build
        run: cargo build --release --example vobee_assistant
      
      - name: Test
        run: cargo test --example vobee_assistant
      
      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: vobee-${{ matrix.os }}
          path: target/release/examples/vobee_assistant*
```

## Version Management

### Semantic Versioning

VoBee AI follows semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Version File

Update version in:
1. `examples/vobee_assistant/Cargo.toml`
2. Release notes
3. Documentation

## Troubleshooting Build Issues

### Missing Dependencies

**Linux:**
```bash
# Install all dependencies
sudo apt-get install -y build-essential pkg-config libfontconfig-dev
```

**macOS:**
```bash
# Ensure Xcode tools are installed
xcode-select --install
```

**Windows:**
```powershell
# Ensure Visual Studio Build Tools are installed
```

### Build Errors

**Clear cache:**
```bash
cargo clean
rm -rf target/
cargo build --release --example vobee_assistant
```

**Update dependencies:**
```bash
cargo update
cargo build --release --example vobee_assistant
```

## Best Practices

1. **Always test before release**: Run the application on each target platform
2. **Sign binaries**: Use code signing for distribution
3. **Include documentation**: Package README and license files
4. **Version clearly**: Tag releases with semantic versioning
5. **Automate**: Use CI/CD for consistent builds
6. **Test installation**: Verify install scripts on clean systems

## Support

For deployment issues:
1. Check this guide
2. Review build logs
3. Test on target platform
4. Report issues on GitHub

---

**Last Updated**: December 2025  
**Version**: 0.1.0
