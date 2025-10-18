#!/bin/bash

echo "🔍 Verifying Mobile Device MCP Extension Installation"
echo "=================================================="
echo

# Check binary
echo "1. Checking binary installation..."
if [ -x "$HOME/.cargo/bin/mobile-device-mcp-server" ]; then
    echo "✅ Binary found at: $HOME/.cargo/bin/mobile-device-mcp-server"
    SIZE=$(ls -lh "$HOME/.cargo/bin/mobile-device-mcp-server" | awk '{print $5}')
    echo "   Size: $SIZE"
else
    echo "❌ Binary NOT found at: $HOME/.cargo/bin/mobile-device-mcp-server"
    echo "   Run: cargo install --path . --bin mobile-device-mcp-server --features native-binary"
    exit 1
fi
echo

# Check WASM extension
echo "2. Checking WASM extension..."
WORK_DIR="$HOME/Library/Application Support/Zed/extensions/work/mcp-server-mobile-device"
if [ -f "$WORK_DIR/extension.wasm" ]; then
    echo "✅ WASM extension found"
    SIZE=$(ls -lh "$WORK_DIR/extension.wasm" | awk '{print $5}')
    echo "   Location: $WORK_DIR/extension.wasm"
    echo "   Size: $SIZE"
else
    echo "❌ WASM extension NOT found"
    echo "   Run: cargo build --target wasm32-wasip2 --release"
    echo "   Then: cp target/wasm32-wasip2/release/mobile_device_mcp.wasm \"$WORK_DIR/extension.wasm\""
    exit 1
fi
echo

# Check symlink
echo "3. Checking extension symlink..."
INSTALLED_DIR="$HOME/Library/Application Support/Zed/extensions/installed/mcp-server-mobile-device"
if [ -L "$INSTALLED_DIR" ]; then
    echo "✅ Symlink found"
    TARGET=$(readlink "$INSTALLED_DIR")
    echo "   Points to: $TARGET"
elif [ -d "$INSTALLED_DIR" ]; then
    echo "⚠️  Directory exists but is not a symlink"
    echo "   Location: $INSTALLED_DIR"
else
    echo "❌ Extension NOT installed"
    echo "   Run: ln -sf $(pwd) \"$INSTALLED_DIR\""
    exit 1
fi
echo

# Test binary
echo "4. Testing binary..."
if echo '{"jsonrpc":"2.0","method":"initialize","params":{},"id":1}' | "$HOME/.cargo/bin/mobile-device-mcp-server" 2>/dev/null | grep -q "result"; then
    echo "✅ Binary responds to MCP initialize request"
else
    echo "⚠️  Binary may not be responding correctly to MCP protocol"
fi
echo

# Check platform tools
echo "5. Checking platform tools..."
if command -v adb &> /dev/null; then
    echo "✅ Android Platform Tools (adb) found"
    adb version 2>&1 | head -1
else
    echo "⚠️  Android Platform Tools (adb) not found in PATH"
fi

if command -v xcrun &> /dev/null; then
    echo "✅ iOS Tools (xcrun) found"
else
    echo "⚠️  iOS Tools (xcrun) not found (macOS only)"
fi
echo

echo "=================================================="
echo "✅ Installation verification complete!"
echo
echo "Next steps:"
echo "1. Restart Zed completely (quit and reopen)"
echo "2. Open Assistant (Cmd+?)"
echo "3. Enable 'mcp-server-mobile-device' context server"
echo "4. Test with: 'List my mobile devices'"
