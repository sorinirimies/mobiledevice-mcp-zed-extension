# Cross-Platform Support Guide

Complete guide for using Mobile Device MCP Server across macOS, Linux, and Windows platforms.

## Overview

The Mobile Device MCP Server is designed to work seamlessly across all major operating systems with platform-specific optimizations and feature availability.

## Platform Support Matrix

| Feature | macOS | Linux | Windows | Notes |
|---------|-------|-------|---------|-------|
| **Android Support** | ✅ | ✅ | ✅ | Full support on all platforms |
| Android Physical Devices | ✅ | ✅ | ✅ | Via ADB over USB |
| Android Emulators | ✅ | ✅ | ✅ | Android Studio compatible |
| **iOS Support** | ✅ | ❌ | ❌ | macOS exclusive |
| iOS Physical Devices | ⚠️ | ❌ | ❌ | Limited (requires libimobiledevice) |
| iOS Simulators | ✅ | ❌ | ❌ | Requires Xcode |
| **Build System** | ✅ | ✅ | ✅ | Native + WASM |
| Zed Extension | ✅ | ✅ | ✅ | WASM runs everywhere |
| Native Binary | ✅ | ✅ | ✅ | Platform-specific builds |

## Quick Start by Platform

### macOS

```bash
# Install prerequisites
brew install android-platform-tools      # For Android
xcode-select --install                   # For iOS
brew install libimobiledevice            # For iOS devices (optional)

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install
cargo install --path . --features native-binary,ios-support

# Run server
mobile-device-mcp-server
```

### Linux

```bash
# Install prerequisites (Ubuntu/Debian)
sudo apt-get update
sudo apt-get install -y android-tools-adb android-tools-fastboot

# Or Arch Linux
sudo pacman -S android-tools

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build and install (Android only on Linux)
cargo install --path . --features native-binary

# Run server
mobile-device-mcp-server
```

### Windows

```powershell
# Install prerequisites
choco install adb                        # Via Chocolatey
# Or: scoop install adb                  # Via Scoop

# Install Rust
winget install Rustlang.Rustup

# Build and install
cargo install --path . --features native-binary

# Run server
mobile-device-mcp-server.exe
```

## Platform-Specific Details

### macOS

**Advantages:**
- Full iOS and Android support
- Best developer experience for mobile
- Native Xcode integration
- Apple Silicon and Intel support

**System Requirements:**
- macOS 11.0 (Big Sur) or later
- Xcode 13.0+ for iOS simulators
- 8GB RAM minimum, 16GB recommended
- Apple Silicon (M1/M2/M3) or Intel processor

**Installation Methods:**
```bash
# Homebrew (Recommended)
brew install android-platform-tools
brew install libimobiledevice
xcode-select --install

# MacPorts
sudo port install android-platform-tools
sudo port install libimobiledevice

# Manual
# Download from developer.android.com and apple.com
```

**Platform-Specific Features:**
- iOS Simulator automation via `xcrun simctl`
- iOS device screenshots via `idevicescreenshot`
- Metal GPU acceleration for emulators
- Native Apple Silicon performance

**Known Issues:**
- Rosetta 2 required for some older tools on Apple Silicon
- iOS device support requires devices in Developer Mode (iOS 16+)
- Some Android emulators may need Intel x86 emulation on Apple Silicon

### Linux

**Advantages:**
- Excellent Android support
- Lower resource usage
- Great for CI/CD pipelines
- Free and open source

**System Requirements:**
- Ubuntu 20.04+, Debian 11+, Arch, Fedora 35+, or similar
- 4GB RAM minimum, 8GB recommended
- x86_64 or ARM64 architecture
- USB access permissions

**Distribution-Specific Setup:**

**Ubuntu/Debian:**
```bash
# Install ADB
sudo apt-get install android-tools-adb android-tools-fastboot

# Fix USB permissions
sudo usermod -aG plugdev $USER
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="plugdev"' | \
  sudo tee /etc/udev/rules.d/51-android.rules
sudo udevadm control --reload-rules
```

**Arch Linux:**
```bash
# Install ADB
sudo pacman -S android-tools

# USB permissions
sudo usermod -aG adbusers $USER
```

**Fedora/RHEL:**
```bash
# Install ADB
sudo dnf install android-tools

# USB permissions
sudo usermod -aG plugdev $USER
```

**Platform-Specific Features:**
- KVM acceleration for emulators
- Better Docker integration
- SSH/network debugging support
- Lower overhead for automated testing

**Known Issues:**
- No iOS support (Apple ecosystem only)
- Some USB chipsets may have compatibility issues
- Wayland may have issues with some emulator displays
- SELinux may require additional configuration

### Windows

**Advantages:**
- Full Android support
- Large developer base
- Good emulator performance with Hyper-V
- Native Windows tooling

**System Requirements:**
- Windows 10 (1903+) or Windows 11
- 8GB RAM minimum, 16GB recommended
- x86_64 architecture
- Hyper-V or WHPX for emulator acceleration

**Installation Methods:**
```powershell
# Chocolatey (Recommended)
choco install adb

# Scoop
scoop install adb

# Winget
winget install Google.AndroidStudio  # Includes ADB

# Manual
# Download from developer.android.com
```

**Platform-Specific Features:**
- Hyper-V acceleration for emulators
- Windows Subsystem for Android (WSA) support planned
- Native Windows Terminal integration
- PowerShell scripting support

**Known Issues:**
- No iOS support (Apple ecosystem only)
- Line ending differences (\r\n vs \n) handled automatically
- Path length limits (enable long paths in registry)
- Firewall may block ADB server
- Antivirus may flag ADB tools

## Build Configuration

### Universal Build (All Platforms)

```toml
# Cargo.toml features
[features]
default = []
native-binary = ["adb_client", "base64", "tokio", "uuid", "quick-xml"]
ios-support = ["idevice"]  # macOS only
```

### Platform-Specific Builds

**macOS (with iOS support):**
```bash
cargo build --release --features "native-binary,ios-support"
```

**Linux (Android only):**
```bash
cargo build --release --features native-binary
```

**Windows (Android only):**
```powershell
cargo build --release --features native-binary
```

**WASM (All platforms via Zed):**
```bash
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
```

## ADB Setup by Platform

### macOS ADB Setup

```bash
# Check ADB
which adb

# Start ADB server
adb start-server

# List devices
adb devices

# Wireless debugging (Android 11+)
adb pair <ip:port>  # Enter pairing code
adb connect <ip:port>
```

### Linux ADB Setup

```bash
# Check ADB
which adb

# Fix permissions if needed
sudo apt-get install adb
sudo usermod -aG plugdev $USER
newgrp plugdev

# Create udev rules
sudo tee /etc/udev/rules.d/51-android.rules << EOF
SUBSYSTEM=="usb", ATTR{idVendor}=="18d1", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTR{idVendor}=="04e8", MODE="0666", GROUP="plugdev"
SUBSYSTEM=="usb", ATTR{idVendor}=="22b8", MODE="0666", GROUP="plugdev"
EOF

sudo udevadm control --reload-rules
sudo udevadm trigger

# Start ADB server
adb start-server

# List devices
adb devices
```

### Windows ADB Setup

```powershell
# Check ADB
where.exe adb

# Start ADB server
adb start-server

# List devices
adb devices

# Troubleshoot USB driver issues
# Open Device Manager (devmgmt.msc)
# Look for "Android Device" or unknown devices
# Install manufacturer's USB driver if needed
```

## File Path Handling

### Cross-Platform Path Best Practices

```rust
use std::path::{Path, PathBuf};

// ✅ Good: Use PathBuf for cross-platform paths
let config_path = PathBuf::from(env::var("HOME").unwrap())
    .join(".config")
    .join("mobile-mcp")
    .join("settings.json");

// ✅ Good: Use Path::new and join
let temp_path = Path::new("/tmp")
    .join("screenshots")
    .join("capture.png");

// ❌ Bad: Hardcoded separators
let bad_path = "/tmp/screenshots/capture.png";  // Unix only
let bad_path = "C:\\temp\\screenshots\\capture.png";  // Windows only

// ✅ Good: Environment-specific temp directory
use std::env;
let temp_dir = env::temp_dir();
let screenshot_path = temp_dir.join("screenshot.png");
```

### Platform-Specific Temporary Files

```rust
#[cfg(target_os = "macos")]
const TEMP_DIR: &str = "/tmp";

#[cfg(target_os = "linux")]
const TEMP_DIR: &str = "/tmp";

#[cfg(target_os = "windows")]
fn get_temp_dir() -> PathBuf {
    env::temp_dir()  // Usually C:\Users\<user>\AppData\Local\Temp
}
```

## Environment Variables

### Cross-Platform Environment Configuration

```bash
# macOS/Linux
export MOBILE_DEVICE_MCP_DEBUG=1
export MOBILE_PLATFORM=android

# Windows PowerShell
$env:MOBILE_DEVICE_MCP_DEBUG = "1"
$env:MOBILE_PLATFORM = "android"

# Windows CMD
set MOBILE_DEVICE_MCP_DEBUG=1
set MOBILE_PLATFORM=android
```

## Testing Across Platforms

### Automated Cross-Platform Testing

```bash
#!/usr/bin/env bash
# test-cross-platform.sh

set -e

PLATFORM=$(uname -s)

echo "Testing on $PLATFORM"

case "$PLATFORM" in
  Darwin)
    echo "Running macOS-specific tests..."
    cargo test --release --features "native-binary,ios-support"
    ./scripts/test-ios.sh
    ./scripts/test-android.sh
    ;;
  Linux)
    echo "Running Linux-specific tests..."
    cargo test --release --features native-binary
    ./scripts/test-android.sh
    ;;
  MINGW*|MSYS*|CYGWIN*)
    echo "Running Windows-specific tests..."
    cargo test --release --features native-binary
    ./scripts/test-android.sh
    ;;
  *)
    echo "Unknown platform: $PLATFORM"
    exit 1
    ;;
esac

echo "✅ All platform-specific tests passed!"
```

### GitHub Actions Matrix

```yaml
name: Cross-Platform Build

on: [push, pull_request]

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
        include:
          - os: ubuntu-latest
            features: native-binary
          - os: macos-latest
            features: native-binary,ios-support
          - os: windows-latest
            features: native-binary
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        override: true
    
    - name: Install ADB (Ubuntu)
      if: matrix.os == 'ubuntu-latest'
      run: sudo apt-get install -y android-tools-adb
    
    - name: Install ADB (macOS)
      if: matrix.os == 'macos-latest'
      run: brew install android-platform-tools
    
    - name: Install ADB (Windows)
      if: matrix.os == 'windows-latest'
      run: choco install adb
    
    - name: Build
      run: cargo build --release --features ${{ matrix.features }}
    
    - name: Test
      run: cargo test --release --features ${{ matrix.features }}
```

## Performance Considerations

### Platform-Specific Performance

**macOS:**
- Apple Silicon: 2-3x faster than Intel for native builds
- Screenshot capture: ~50-150ms (simulator), ~500-1000ms (device)
- UI inspection: Not available (use Accessibility Inspector)

**Linux:**
- KVM acceleration: Similar to bare metal
- Screenshot capture: ~50-200ms
- Lower memory overhead than other platforms
- Better for headless/server environments

**Windows:**
- Hyper-V acceleration: Near-native performance
- Screenshot capture: ~100-300ms
- Higher memory usage
- USB debugging may be slower on some systems

### Optimization Tips

```rust
// Use conditional compilation for platform-specific optimizations

#[cfg(target_os = "macos")]
fn optimize_screenshot() {
    // Use native APIs or faster methods on macOS
}

#[cfg(target_os = "linux")]
fn optimize_screenshot() {
    // Use V4L2 or faster capture methods
}

#[cfg(target_os = "windows")]
fn optimize_screenshot() {
    // Use Windows-specific APIs
}
```

## Troubleshooting by Platform

### macOS Issues

**"xcrun not found":**
```bash
xcode-select --install
sudo xcode-select --reset
```

**"idevice not available":**
```bash
brew install libimobiledevice
brew link --overwrite libimobiledevice
```

**Apple Silicon compatibility:**
```bash
# Install Rosetta 2 for x86 apps
softwareupdate --install-rosetta
```

### Linux Issues

**"Permission denied" for USB:**
```bash
sudo usermod -aG plugdev $USER
sudo udevadm control --reload-rules
# Log out and back in
```

**"ADB not found":**
```bash
export PATH="$PATH:$HOME/Android/Sdk/platform-tools"
echo 'export PATH="$PATH:$HOME/Android/Sdk/platform-tools"' >> ~/.bashrc
```

**KVM acceleration not available:**
```bash
# Check if KVM is available
kvm-ok

# Install if missing
sudo apt-get install qemu-kvm
sudo usermod -aG kvm $USER
```

### Windows Issues

**"ADB is not recognized":**
```powershell
# Add to PATH
$env:Path += ";C:\platform-tools"
[Environment]::SetEnvironmentVariable("Path", $env:Path, "User")
```

**Hyper-V not available:**
```powershell
# Enable Hyper-V
Enable-WindowsOptionalFeature -Online -FeatureName Microsoft-Hyper-V -All
```

**Firewall blocking ADB:**
```powershell
New-NetFirewallRule -DisplayName "ADB" -Direction Inbound -Program "C:\platform-tools\adb.exe" -Action Allow
```

## Docker Support

### Cross-Platform Docker Image

```dockerfile
FROM rust:1.70-slim as builder

# Install platform tools
RUN apt-get update && apt-get install -y \
    android-tools-adb \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build for Linux
RUN cargo build --release --features native-binary

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    android-tools-adb \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mobile-device-mcp-server /usr/local/bin/

# Expose ADB port
EXPOSE 5037

CMD ["mobile-device-mcp-server"]
```

### Running in Docker

```bash
# Build image
docker build -t mobile-mcp-server .

# Run with host network (for ADB access)
docker run --rm --network host mobile-mcp-server

# Run with USB device passthrough
docker run --rm --privileged -v /dev/bus/usb:/dev/bus/usb mobile-mcp-server
```

## CI/CD Best Practices

### Multi-Platform Release

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: mobile-device-mcp-server-linux-x86_64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: mobile-device-mcp-server-macos-x86_64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: mobile-device-mcp-server-macos-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: mobile-device-mcp-server-windows-x86_64.exe
    
    runs-on: ${{ matrix.os }}
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: ${{ matrix.target }}
        override: true
    
    - name: Build
      run: cargo build --release --target ${{ matrix.target }} --features native-binary
    
    - name: Upload artifact
      uses: actions/upload-artifact@v3
      with:
        name: ${{ matrix.artifact }}
        path: target/${{ matrix.target }}/release/mobile-device-mcp-server*
```

## Additional Resources

### Documentation by Platform

- **macOS:** See `docs/IOS_IMPLEMENTATION.md`
- **Linux:** See `docs/ANDROID_IMPLEMENTATION.md`
- **Windows:** See `docs/WINDOWS_SUPPORT.md`

### External Resources

- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Android ADB Documentation](https://developer.android.com/tools/adb)
- [Xcode Command Line Tools](https://developer.apple.com/xcode/)
- [Cross-Platform Rust](https://doc.rust-lang.org/rustc/platform-support.html)

## Contributing

When adding platform-specific features:

1. Test on all supported platforms
2. Use conditional compilation (`#[cfg(...)]`)
3. Document platform limitations
4. Update this guide
5. Add platform-specific tests

## Support

For platform-specific issues:

- **macOS/iOS:** [GitHub Issues - iOS label](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/issues?q=label:ios)
- **Android:** [GitHub Issues - Android label](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/issues?q=label:android)
- **Windows:** [GitHub Issues - Windows label](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/issues?q=label:windows)
- **Linux:** [GitHub Issues - Linux label](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/issues?q=label:linux)

---

**Last Updated:** 2024-11-01  
**Supported Platforms:** macOS (11+), Linux (Ubuntu 20.04+), Windows (10/11)  
**Status:** Production Ready (All Platforms)