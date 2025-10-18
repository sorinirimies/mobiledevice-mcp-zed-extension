# Mobile Device MCP Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Android](https://img.shields.io/badge/Android-ADB-green.svg)](https://developer.android.com/tools/adb)
[![iOS](https://img.shields.io/badge/iOS-simctl-blue.svg)](https://developer.apple.com/documentation/xcode)

A comprehensive Model Context Protocol (MCP) server for mobile device automation. Control Android and iOS devices programmatically through a standardized interface, perfect for testing, automation, and AI-assisted mobile app development.

## 🚀 Features

- **19 Mobile Automation Tools** - Complete device control through MCP
- **Android Support** - Full automation for physical devices and emulators
- **iOS Support** - Simulator control on macOS (partial feature set)
- **Screenshot Capture** - Visual device state with base64 PNG output
- **UI Element Inspection** - XML-based hierarchy parsing for Android
- **App Lifecycle Management** - Install, launch, terminate, and uninstall apps
- **Touch & Gesture Automation** - Tap, swipe, long press, and type
- **Cross-Platform Build** - Native binary + Zed extension (WASM)
- **Clean Architecture** - Modular, well-documented Rust codebase

## 📋 Available Tools

All tools use the `mobile_device_mcp_*` prefix for namespacing.

### Device Information (5 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_list_available_devices` | Discover connected devices and emulators | ✅ | ✅ |
| `mobile_device_mcp_get_screen_size` | Get device screen dimensions in pixels | ✅ | ❌ |
| `mobile_device_mcp_get_orientation` | Query portrait/landscape orientation | ✅ | ❌ |
| `mobile_device_mcp_list_apps` | List installed applications | ✅ | ❌ |
| `mobile_device_mcp_list_elements_on_screen` | UI element hierarchy with coordinates | ✅ | ❌ |

### Screen Interaction (6 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_take_screenshot` | Capture screen as base64 PNG | ✅ | ✅ |
| `mobile_device_mcp_save_screenshot` | Save screenshot to file | ✅ | ✅ |
| `mobile_device_mcp_click_on_screen_at_coordinates` | Tap at specific coordinates | ✅ | ✅* |
| `mobile_device_mcp_double_tap_on_screen` | Double-tap gesture | ✅ | ✅* |
| `mobile_device_mcp_long_press_on_screen_at_coordinates` | Long press/hold gesture | ✅ | ❌ |
| `mobile_device_mcp_swipe_on_screen` | Swipe with start/end coordinates | ✅ | ❌ |

*iOS: Simulator only

### Input (2 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_type_keys` | Type text into focused field | ✅ | ✅* |
| `mobile_device_mcp_press_button` | Press hardware buttons (home, back, etc.) | ✅ | ❌ |

*iOS: Simulator only

### App Management (4 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_launch_app` | Open an application | ✅ | ❌ |
| `mobile_device_mcp_terminate_app` | Force-stop an app | ✅ | ❌ |
| `mobile_device_mcp_install_app` | Install from APK/IPA file | ✅ | ❌ |
| `mobile_device_mcp_uninstall_app` | Remove an application | ✅ | ❌ |

### Navigation (2 tools)

| Tool | Description | Android | iOS |
|------|-------------|---------|-----|
| `mobile_device_mcp_open_url` | Open URL in default browser | ✅ | ❌ |
| `mobile_device_mcp_set_orientation` | Change portrait/landscape mode | ✅ | ❌ |

**Platform Summary:**
- **Android:** 19/19 tools (100% coverage)
- **iOS:** 3/19 tools (simulator only, macOS only)

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
- iOS: xcrun simctl (macOS only)
- XML Parsing: quick-xml for UI hierarchy
- Build: Native binary + WASM extension
```

## 📦 Installation

### Prerequisites

**Android Support:**
```bash
# macOS
brew install android-platform-tools

# Ubuntu/Debian
sudo apt-get install android-tools-adb

# Verify
adb version
```

**iOS Support (macOS only):**
```bash
# Install Xcode Command Line Tools
xcode-select --install

# Verify
xcrun simctl list devices
```

### Build from Source

```bash
# Clone repository
git clone https://github.com/sorinirimies/mobile-device-mcp
cd mobile-device-mcp

# Build native binary (for standalone use)
cargo build --release --features native-binary

# Binary location
./target/release/mobile-device-mcp-server

# Or build Zed extension (WASM)
cargo build --release --target wasm32-wasi
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

### Start MCP Server (Standalone)

```bash
# Run the server
./target/release/mobile-device-mcp-server

# Set debug mode (optional)
export MOBILE_DEVICE_MCP_DEBUG=1
./target/release/mobile-device-mcp-server
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
# Run comprehensive test suite (requires connected device/emulator)
./test-all-tools.sh

# Quick smoke test
./test-tools.sh

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
4. Update tests in `test-all-tools.sh`

### Running Tests

```bash
# Build with native features
cargo build --release --features native-binary

# Run all tests
./test-all-tools.sh

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

## 📚 Documentation

- [Architecture Guide](ARCHITECTURE.md)
- [iOS vs Android Feature Parity](IOS_ANDROID_FEATURE_PARITY.md)
- [Tool Fix & Rename Summary](TOOLS_FIX_AND_RENAME_SUMMARY.md)
- [Verification Complete](VERIFICATION_COMPLETE.md)

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

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **adb_client** - Rust ADB client library
- **quick-xml** - Fast XML parsing
- **MCP Protocol** - Model Context Protocol specification
- **Zed Editor** - Extension platform

## 📞 Support

- **Issues:** [GitHub Issues](https://github.com/sorinirimies/mobile-device-mcp/issues)
- **Discussions:** [GitHub Discussions](https://github.com/sorinirimies/mobile-device-mcp/discussions)

---

**Version:** 0.1.0  
**Status:** Production Ready (Android), Beta (iOS)  
**Last Updated:** October 18, 2024