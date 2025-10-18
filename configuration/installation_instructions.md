# Mobile Device MCP Server - Setup Guide

Control your Android and iOS devices directly from Zed!

---

## 1. Install the Binary

The extension requires a native binary to function. Install it using:

```bash
cargo install mobile-device-mcp-server
```

Or download from [releases](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/releases).

## 2. Verify Installation

```bash
which mobile-device-mcp-server
# Should output: ~/.cargo/bin/mobile-device-mcp-server
```

If not found, add `~/.cargo/bin` to your PATH:
```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

## 3. Install Platform Tools

**For Android:**
```bash
brew install android-platform-tools  # macOS
# or: sudo apt install adb            # Linux
```

**For iOS (macOS only):**
```bash
xcode-select --install
```

## 4. Enable in Zed Settings

Add to your `settings.json`:

```json
{
  "context_servers": {
    "mcp-server-mobile-device": {
      "settings": {
        "debug": false,
        "platform": "auto"
      }
    }
  }
}
```

Then restart Zed. The extension is ready!

---

## 🧪 Test It

1. Connect an Android device or start an iOS simulator
2. Open Zed Assistant
3. Ask: **"List my mobile devices"**

**Settings Options:**
- `debug`: Enable verbose logging (default: `false`)
- `platform`: `"auto"`, `"android"`, or `"ios"` (default: `"auto"`)

---

## 📱 Available Tools

- **Device Management**: List devices, get screen size/orientation
- **Screen Interaction**: Screenshot, tap, swipe, long press
- **Input**: Type text, press buttons (home, back, etc.)
- **Apps**: List, launch, terminate, install, uninstall
- **Navigation**: Open URLs, change orientation

And more! See [full documentation](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension).

---

## 🐛 Troubleshooting

**Extension not starting?**
- Ensure binary is installed: `which mobile-device-mcp-server`
- Check logs: `~/Library/Logs/Zed/Zed.log` (macOS)

**No devices found?**
```bash
# Android
adb devices

# iOS
xcrun simctl list devices
```

**Need help?** [Open an issue](https://github.com/sorinirimies/mobiledevice-mcp-zed-extension/issues)

---

**Built with ❤️ for mobile automation in Zed**