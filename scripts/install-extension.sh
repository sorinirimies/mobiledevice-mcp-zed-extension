#!/bin/bash
set -e

echo "🔨 Installing Mobile Device MCP Extension for Zed..."
echo ""

# Build the WASM extension
echo "Building WASM extension..."
cargo build --release --target wasm32-wasip1 --lib --features zed-extension

if [ $? -ne 0 ]; then
    echo "❌ Failed to build extension"
    exit 1
fi

# Determine Zed extensions directory
if [[ "$OSTYPE" == "darwin"* ]]; then
    ZED_EXT_DIR="$HOME/Library/Application Support/Zed/extensions/installed/mcp-server-mobile-device"
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
    ZED_EXT_DIR="$HOME/.config/zed/extensions/installed/mcp-server-mobile-device"
else
    echo "❌ Unsupported OS: $OSTYPE"
    exit 1
fi

# Create extension directory
echo "Creating extension directory: $ZED_EXT_DIR"
mkdir -p "$ZED_EXT_DIR"

# Copy files
echo "Copying extension files..."
cp target/wasm32-wasip1/release/mobile_device_mcp_server.wasm "$ZED_EXT_DIR/extension.wasm"
cp extension.toml "$ZED_EXT_DIR/"
cp -r configuration "$ZED_EXT_DIR/"

echo ""
echo "✅ Extension installed successfully!"
echo ""
echo "Location: $ZED_EXT_DIR"
echo ""
echo "Next steps:"
echo "  1. Restart Zed"
echo "  2. The extension will be available in Extensions panel"
echo "  3. Make sure you have installed the binary:"
echo "     cargo install --path . --bin mobile-device-mcp-server --features native-binary"
