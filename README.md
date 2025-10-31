# Mobile Device MCP Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/sorinirimies/mobile-device-mcp/actions/workflows/ci.yml/badge.svg)](https://github.com/sorinirimies/mobile-device-mcp/actions/workflows/ci.yml)
[![Release](https://github.com/sorinirimies/mobile-device-mcp/actions/workflows/release.yml/badge.svg)](https://github.com/sorinirimies/mobile-device-mcp/actions/workflows/release.yml)
[![codecov](https://codecov.io/gh/sorinirimies/mobile-device-mcp/branch/main/graph/badge.svg)](https://codecov.io/gh/sorinirimies/mobile-device-mcp)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Android](https://img.shields.io/badge/Android-ADB-green.svg)](https://developer.android.com/tools/adb)
[![iOS](https://img.shields.io/badge/iOS-simctl-blue.svg)](https://developer.apple.com/documentation/xcode)

A comprehensive Model Context Protocol (MCP) server for mobile device automation. Control Android and iOS devices programmatically through a standardized interface, perfect for testing, automation, and AI-assisted mobile app development.

## 🚀 Features

- **19 Mobile Automation Tools** - Complete device control through MCP
- **Full Android Support** - 100% coverage for physical devices and emulators (19/19 tools)
- **Comprehensive iOS Support** - 95% coverage for simulators, 42% for physical devices (18/19 tools)
- **Screenshot Capture** - Visual device state with base64 PNG output (all platforms)
- **UI Element Inspection** - XML-based hierarchy parsing for Android
- **App Lifecycle Management** - Install, launch, terminate, and uninstall apps
- **Touch & Gesture Automation** - Tap, swipe, long press, and type
- **Cross-Platform Build** - Works on macOS, Linux, and Windows
- **Native Binary + WASM** - Standalone server and Zed extension support
- **Clean Architecture** - Modular, well-documented Rust codebase

## 📋 Available Tools

All tools use the `mobile_device_mcp_*` prefix for namespacing.

### Device Information (5 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_list_available_devices` | Discover connected devices and emulators | ✅ | ✅ |
| `mobile_device_mcp_get_screen_size` | Get device screen dimensions in pixels | ✅ | ✅ |
| `mobile_device_mcp_get_orientation` | Query portrait/landscape orientation | ✅ | ✅ |
| `mobile_device_mcp_list_apps` | List installed applications | ✅ | ✅* |
| `mobile_device_mcp_list_elements_on_screen` | UI element hierarchy with coordinates | ✅ | ⚠️** |

### Screen Interaction (6 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_take_screenshot` | Capture screen as base64 PNG | ✅ | ✅ |
| `mobile_device_mcp_save_screenshot` | Save screenshot to file | ✅ | ✅ |
| `mobile_device_mcp_click_on_screen_at_coordinates` | Tap at specific coordinates | ✅ | ✅* |
| `mobile_device_mcp_double_tap_on_screen` | Double-tap gesture | ✅ | ✅* |
| `mobile_device_mcp_long_press_on_screen_at_coordinates` | Long press/hold gesture | ✅ | ✅* |
| `mobile_device_mcp_swipe_on_screen` | Swipe with start/end coordinates | ✅ | ✅* |

### Input (2 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_type_keys` | Type text into focused field | ✅ | ✅* |
| `mobile_device_mcp_press_button` | Press hardware buttons (home, back, etc.) | ✅ | ✅* |

### App Management (4 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_launch_app` | Open an application | ✅ | ✅* |
| `mobile_device_mcp_terminate_app` | Force-stop an app | ✅ | ✅* |
| `mobile_device_mcp_install_app` | Install from APK/IPA file | ✅ | ✅* |
| `mobile_device_mcp_uninstall_app` | Remove an application | ✅ | ✅* |

### Navigation (2 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_open_url` | Open URL in default browser | ✅ | ✅* |
| `mobile_device_mcp_set_orientation` | Change portrait/landscape mode | ✅ | ✅* |

**Platform Summary:**
- **Android:** 19/19 tools (100% coverage - all devices)
- **iOS Simulator:** 18/19 tools (95% coverage - macOS only)
- **iOS Physical Device:** 8/19 tools (42% coverage - basic automation)

**Legend:**
- ✅ Full support
- ✅* Simulator support (iOS) or partial support
- ⚠️** Limited support (requires additional tools like WebDriverAgent)

## 🏗️ Architecture

```
mobile-mcp-zed-extension/
├── src/
│   ├── main.rs              # Native MCP server (JSON-RPC 2.0)
│   ├── lib.rs               # Zed WASM extension
│   ├── types.rs             # Shared type definitions
│   ├── devices/
│   │   ├── android.rs       # Android automation (adb_client)
│   │   └── ios.rs           # iOS automation (xcrun simctl)
│   ├── mcp/
│   │   └── protocol.rs      # MCP protocol implementation
│   └── tools/
│       ├── definitions.rs   # Tool schemas (19 tools)
│       └── handlers.rs      # Tool implementations
├── extension.toml           # Zed extension manifest
└── Cargo.toml              # Dependencies

Tech Stack:
- Protocol: JSON-RPC 2.0 via MCP
- Android: adb_client crate + shell commands
- iOS: xcrun simctl + libimobiledevice (macOS only)
- Cross-Platform: Runs on macOS, Linux, and Windows
- XML Parsing: quick-xml for UI hierarchy
- Build: Native binary + WASM extension
```

## 📦 Installation

### Prerequisites

**Android Support (All Platforms):**
```bash
# macOS
brew install android-platform-tools

# Linux (Ubuntu/Debian)
sudo apt-get install android-tools-adb

# Windows (PowerShell)
choco install adb

# Verify
adb version
```

**iOS Support (macOS only):**
```bash
# Install Xcode Command Line Tools (required)
xcode-select --install

# Install libimobiledevice (optional, for physical devices)
brew install libimobiledevice

# Verify
xcrun simctl list devices
idevice_id -l  # Lists connected physical devices
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/sorinirimies/mobile-device-mcp
cd mobile-device-mcp

# Build native binary
# macOS (with iOS support)
cargo build --release --features "native-binary,ios-support"

# Linux/Windows (Android only)
cargo build --release --features native-binary

# Binary location
./target/release/mobile-device-mcp-server

# Or build Zed extension (WASM)
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
```

### Install as Zed Extension

1. Open Zed editor
2. Go to Extensions (Cmd+Shift+X)
3. Search for "Mobile Device MCP"
4. Click Install

Or manually:
```bash
# Copy to Zed extensions directory
mkdir -p ~/.config/zed/extensions
cp -r . ~/.config/zed/extensions/mobile-device-mcp
```

## 🚦 Quick Start

### Using Just (Recommended)

[Just](https://github.com/casey/just) is a command runner that simplifies common tasks:

```bash
# Install just
cargo install just

# Show all available commands
just

# Quick development setup
just dev-setup

# Build and run
just build
just run

# Run tests
just test

# Install Zed extension
just install-zed

# Format, lint, and test
just pre-commit
```

See the [justfile](justfile) for all available commands.

### Start MCP Server (Standalone)

```bash
# Run the server
./target/release/mobile-device-mcp-server

# Set debug mode (optional)
export MOBILE_DEVICE_MCP_DEBUG=1
./target/release/mobile-device-mcp-server

# Or using just
just run
just run-debug
```

### Example MCP Requests

**List Devices:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "mobile_device_mcp_list_available_devices",
    "arguments": {}
  }
}
```

**Take Screenshot:**
```json
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "mobile_device_mcp_take_screenshot",
    "arguments": {
      "device_id": "emulator-5554",
      "platform": "android"
    }
  }
}
```

**List UI Elements:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "mobile_device_mcp_list_elements_on_screen",
    "arguments": {
      "device_id": "emulator-5554",
      "platform": "android",
      "filter": "Sign in"
    }
  }
}
```

## 🧪 Testing

```bash
# Run unit tests
just test

# Run tests with coverage
just test-coverage

# Run integration tests (requires connected device/emulator)
just test-integration

# Quick smoke test
just test-smoke

# Or manually
./scripts/test-all-tools.sh
./scripts/test-tools.sh

# Expected output:
# Total Tests:  18
# Passed:       18 ✅
# Failed:       0 ❌
```

## 📱 Device Setup

### Android Emulator
```bash
# Start emulator
emulator -avd Pixel_6_API_34

# Verify connection
adb devices
# Output: emulator-5554   device
```

### Android Physical Device
```bash
# Enable USB debugging on device
# Settings > Developer Options > USB Debugging

# Connect via USB and verify
adb devices
```

### iOS Simulator (macOS)
```bash
# List available simulators
xcrun simctl list devices

# Boot a simulator
xcrun simctl boot "iPhone 15 Pro"

# Verify
xcrun simctl list devices | grep Booted
```

## 🔧 Configuration

### Environment Variables

- `MOBILE_DEVICE_MCP_DEBUG=1` - Enable debug logging
- `MOBILE_PLATFORM=android|ios|auto` - Default platform (auto = both)

### Cargo Features

- `native-binary` - Build standalone MCP server (required for native builds)
- `default` - Build as Zed WASM extension

## 📚 Documentation

### Comprehensive Guides

- **[iOS Implementation Guide](docs/IOS_IMPLEMENTATION.md)** - Complete iOS automation documentation
- **[Android Implementation Guide](docs/ANDROID_IMPLEMENTATION.md)** - Complete Android automation documentation
- **[Windows Support Guide](docs/WINDOWS_SUPPORT.md)** - Windows-specific setup and usage
- **[Cross-Platform Guide](docs/CROSS_PLATFORM.md)** - Multi-OS deployment and testing
- **[Feature Parity](docs/FEATURE_PARITY.md)** - Detailed iOS vs Android comparison

## 📖 API Documentation

### Tool Response Format

All tools return MCP-compliant responses:

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": "Tool output here"
      }
    ]
  }
}
```

Screenshot responses use image type:
```json
{
  "result": {
    "content": [
      {
        "type": "image",
        "data": "<base64-png-data>",
        "mimeType": "image/png"
      }
    ]
  }
}
```

### Error Handling

Errors follow MCP error format:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": {
    "code": -1,
    "message": "Device not found: emulator-5554"
  }
}
```

## 🛠️ Development

### Quick Start with Just

```bash
# Complete development setup
just dev-setup

# Daily development workflow
just build        # Build debug binary
just test         # Run tests
just lint         # Run clippy
just fmt          # Format code
just pre-commit   # Run all checks before committing

# Watch for changes
just watch        # Auto-run tests on file changes
just watch-run    # Auto-restart server on file changes
```

### Project Structure

- **src/devices/** - Platform-specific device managers
  - `android.rs` - ADB client, UI automation, app management
  - `ios.rs` - simctl wrapper, screenshot, basic input
  
- **src/tools/** - MCP tool definitions
  - `definitions.rs` - JSON schemas for all 19 tools
  - `handlers.rs` - Implementation logic for each tool

- **src/mcp/** - Protocol layer
  - `protocol.rs` - JSON-RPC 2.0 structures

### Adding New Tools

1. Define tool schema in `src/tools/definitions.rs`
2. Implement handler in `src/tools/handlers.rs`
3. Add dispatch case in `src/main.rs`
4. Update tests in `scripts/test-all-tools.sh`
5. Run `just test` to verify

### Running Tests

```bash
# Using just (recommended)
just test                    # All tests
just test-integration        # Integration tests
just test-coverage          # With coverage report

# Or manually
cargo build --release --features native-binary
./scripts/test-all-tools.sh

# Run specific test
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"mobile_device_mcp_take_screenshot","arguments":{"device_id":"emulator-5554","platform":"android"}}}' | ./target/release/mobile-device-mcp-server
```

## 🐛 Troubleshooting

### Android Issues

**ADB not found:**
```bash
# Add to PATH
export PATH=$PATH:$ANDROID_HOME/platform-tools
```

**Device unauthorized:**
```bash
# Revoke and re-authorize
adb kill-server
adb start-server
# Accept prompt on device
```

**Emulator not detected:**
```bash
# Check ADB connection
adb devices
adb kill-server && adb start-server
```

### iOS Issues

**simctl not found:**
```bash
# Install Xcode Command Line Tools
xcode-select --install
```

**Simulator not booting:**
```bash
# Kill all simulators
killall Simulator
# Boot specific device
xcrun simctl boot "iPhone 15"
```

## 🌐 Cross-Platform Support

### Supported Platforms

| Platform | Android | iOS | Status |
|----------|---------|-----|--------|
| **macOS** | ✅ Full | ✅ Full | Production Ready |
| **Linux** | ✅ Full | ❌ N/A | Production Ready |
| **Windows** | ✅ Full | ❌ N/A | Production Ready |

**Quick Setup:**
- **macOS:** `brew install android-platform-tools && xcode-select --install`
- **Linux:** `sudo apt-get install android-tools-adb`
- **Windows:** `choco install adb`

See [Cross-Platform Guide](docs/CROSS_PLATFORM.md) for detailed instructions.

## 🔄 CI/CD

This project uses GitHub Actions for continuous integration and deployment:

### Workflows

- **CI** (`.github/workflows/ci.yml`) - Runs on every push and PR
  - ✅ Tests on Ubuntu, macOS, and Windows
  - ✅ Runs with stable and beta Rust
  - ✅ Code formatting checks (`cargo fmt`)
  - ✅ Linting with Clippy
  - ✅ Documentation checks
  - ✅ Security audits
  - ✅ Code coverage reports
  - ✅ MSRV (Minimum Supported Rust Version) checks

- **Release** (`.github/workflows/release.yml`) - Triggered on version tags
  - 📦 Builds binaries for all platforms (Linux, macOS, Windows)
  - 📦 Builds WASM extension for Zed
  - 📦 Creates GitHub releases with changelog
  - 📦 Publishes to crates.io
  - 📦 Generates release artifacts

- **Documentation** (`.github/workflows/docs.yml`) - Deploys docs to GitHub Pages
  - 📚 Builds and publishes API documentation
  - 🔗 Checks markdown links
  - ✏️ Spell checking

### Local CI Checks

Run the same checks as CI locally:

```bash
# Run all CI checks
just ci

# Individual checks
just fmt-check    # Check formatting
just lint         # Run clippy
just test         # Run tests
just doc-check    # Check documentation
just audit        # Security audit
```

### Creating a Release

```bash
# Using just (recommended)
just release 0.2.0

# Or manually
git-cliff --tag v0.2.0 --output CHANGELOG.md
git add CHANGELOG.md
git commit -m "chore: release v0.2.0"
git tag -a "v0.2.0" -m "Release v0.2.0"
git push origin main --tags
```

The release workflow will automatically:
1. Build binaries for all platforms
2. Create a GitHub release with changelog
3. Upload release artifacts
4. Publish to crates.io (if configured)

### Dependabot

Automated dependency updates are configured via `.github/dependabot.yml`:
- Weekly updates for Cargo dependencies
- Weekly updates for GitHub Actions
- Grouped minor and patch updates

## 🗂️ Changelog

See [CHANGELOG.md](CHANGELOG.md) for release history.

Generated using [git-cliff](https://git-cliff.org/).

## 🤝 Contributing

Contributions welcome! Areas of interest:

1. **iOS Feature Parity** - Implement missing iOS tools (see feature parity doc)
2. **Real Device Support** - iOS real device automation
3. **UI Testing** - Visual element matching, assertions
4. **Performance** - Optimize XML parsing, caching
5. **Platform Support** - Windows/Linux improvements

### Development Workflow

1. Fork and clone the repository
2. Run `just dev-setup` to configure your environment
3. Create a feature branch
4. Make your changes and add tests
5. Run `just pre-commit` to verify all checks pass
6. Submit a pull request

All PRs must pass CI checks before merging.

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **adb_client** - Rust ADB client library
- **quick-xml** - Fast XML parsing
- **MCP Protocol** - Model Context Protocol specification
- **Zed Editor** - Extension platform
- **just** - Command runner for development tasks
- **git-cliff** - Changelog generator

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/sorinirimies/mobile-device-mcp/issues)
- **Discussions:** [GitHub Discussions](https://github.com/sorinirimies/mobile-device-mcp/discussions)

---

**Version:** 0.1.0  
**Status:** Production Ready (Android Full, iOS Simulator), Beta (iOS Physical Devices)  
**Last Updated:** November 1, 2024  
**Platform Coverage:** macOS (Full), Linux (Android), Windows (Android)