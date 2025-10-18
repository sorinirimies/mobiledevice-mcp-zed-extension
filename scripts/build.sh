#!/bin/bash
set -e

echo "🔨 Building Mobile Device MCP Server..."
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Build the native MCP server binary
echo -e "${BLUE}Building native MCP server binary...${NC}"
cargo build --release --bin mobile-device-mcp-server --features native-binary

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Native binary built successfully${NC}"
    echo "  Location: target/release/mobile-device-mcp-server"
else
    echo "❌ Failed to build native binary"
    exit 1
fi

echo ""

# Build the Zed extension (WASM)
echo -e "${BLUE}Building Zed extension (WASM)...${NC}"
cargo build --release --target wasm32-wasip1 --lib

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✓ Zed extension built successfully${NC}"
    echo "  Location: target/wasm32-wasip1/release/mobile_device_mcp_server.wasm"
else
    echo "❌ Failed to build Zed extension"
    exit 1
fi

echo ""
echo -e "${GREEN}🎉 Build complete!${NC}"
echo ""
echo "Next steps:"
echo "  1. Install the native binary:"
echo "     cargo install --path . --bin mobile-device-mcp-server --features native-binary"
echo ""
echo "  2. Install the Zed extension:"
echo "     - Copy target/wasm32-wasip1/release/mobile_device_mcp_server.wasm to your Zed extensions directory"
echo "     - Or publish to the Zed extension marketplace"
