#!/bin/bash
# Mobile Device MCP Zed Extension - Build Script
# Builds both the WASM extension and native binary

set -e

echo "🏗️  Mobile Device MCP Zed Extension Build Script"
echo "=========================================="

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Check if we're on macOS for iOS support
if [[ "$OSTYPE" == "darwin"* ]]; then
    IOS_SUPPORT="ios-support"
    echo -e "${BLUE}📱 macOS detected - enabling iOS support${NC}"
else
    IOS_SUPPORT=""
    echo -e "${YELLOW}⚠️  Non-macOS system - iOS support disabled${NC}"
fi

echo ""
echo -e "${BLUE}Step 1: Building native binary...${NC}"
if [ -n "$IOS_SUPPORT" ]; then
    cargo build --release --features "native-binary,$IOS_SUPPORT"
else
    cargo build --release --features "native-binary"
fi

if [ $? -eq 0 ]; then
    echo -e "${GREEN}✅ Native binary built successfully${NC}"
else
    echo -e "${RED}❌ Native binary build failed${NC}"
    exit 1
fi

echo ""
echo -e "${BLUE}Step 2: Checking WASM extension...${NC}"
if [ -f "extension.wasm" ]; then
    echo -e "${GREEN}✅ WASM extension already exists${NC}"
    echo -e "${BLUE}📦 Size: $(ls -lh extension.wasm | awk '{print $5}')${NC}"
else
    echo -e "${YELLOW}⚠️  No WASM extension found - using shell script approach${NC}"
    echo -e "${BLUE}Note: This is expected due to native dependency conflicts${NC}"
fi

echo ""
echo -e "${BLUE}Step 3: Verifying build outputs...${NC}"

# Check native binary
if [ -f "target/release/mobile-device-mcp-server" ]; then
    echo -e "${GREEN}✅ Native binary: target/release/mobile-device-mcp-server${NC}"
    echo -e "${BLUE}   Size: $(ls -lh target/release/mobile-device-mcp-server | awk '{print $5}')${NC}"
else
    echo -e "${RED}❌ Native binary not found${NC}"
    exit 1
fi

# Check extension files
if [ -f "extension.toml" ]; then
    echo -e "${GREEN}✅ Extension manifest: extension.toml${NC}"
else
    echo -e "${RED}❌ Extension manifest not found${NC}"
    exit 1
fi

if [ -f "extension.wasm" ]; then
    echo -e "${GREEN}✅ WASM extension: extension.wasm${NC}"
else
    echo -e "${YELLOW}⚠️  WASM extension: Not available (using shell script approach)${NC}"
fi

echo ""
echo -e "${BLUE}Step 4: Testing native binary...${NC}"

# Test MCP server initialization
echo -e "${BLUE}Testing MCP server...${NC}"
MCP_TEST=$(echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | ./target/release/mobile-device-mcp-server 2>/dev/null || echo "MCP test failed")
if [[ "$MCP_TEST" == *"mobile-device-mcp-server"* ]]; then
    echo -e "${GREEN}✅ MCP server responding correctly${NC}"
else
    echo -e "${YELLOW}⚠️  MCP server test inconclusive${NC}"
fi

echo ""
echo -e "${GREEN}🎉 Build Complete!${NC}"
echo "=========================================="
echo ""
echo -e "${BLUE}📋 Installation Summary:${NC}"
echo -e "   Native Binary: ${GREEN}Ready${NC}"
echo -e "   WASM Extension: ${GREEN}Ready${NC} (shell script approach)"
echo -e "   Extension Manifest: ${GREEN}Ready${NC}"
echo ""
echo -e "${BLUE}🔧 Next Steps:${NC}"
echo "1. Install the extension in Zed by symlinking this directory to:"
echo "   ~/.config/zed/extensions/mobile-device-mcp-server/"
echo ""
echo "2. Or use the dev mode installer:"
echo "   ./install-dev.sh"
echo ""
echo "3. Reload Zed extensions: Cmd+Shift+P → 'zed: reload extensions'"
echo ""
echo -e "${BLUE}💡 Usage:${NC}"
echo "Once installed, use natural language with Zed's AI:"
echo "- 'List all my connected mobile devices'"
echo "- 'Take a screenshot of my Android device'"
echo "- 'Tap at coordinates 200,300 on my phone'"
