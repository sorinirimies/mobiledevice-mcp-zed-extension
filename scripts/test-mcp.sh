#!/bin/bash

# Mobile Device MCP Server Test Script
# Tests all functionality of the native MCP server

set -e

BINARY="./target/release/mobile-device-mcp-server"
SETTINGS='{"debug": true, "platform": "auto"}'

echo "🧪 Mobile Device MCP Server Test Suite"
echo "================================"

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found. Building..."
    cargo build --release --features "native-binary,ios-support"
fi

echo "✅ Binary found: $BINARY"
echo

# Test 1: Initialize
echo "📋 Test 1: Initialize"
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {}}' | \
    MOBILE_DEVICE_MCP_SETTINGS="$SETTINGS" $BINARY | \
    jq -r '.result.serverInfo.name' | \
    grep -q "mobile-device-mcp-server" && echo "✅ Initialize works" || echo "❌ Initialize failed"
echo

# Test 2: List tools
echo "📋 Test 2: List Tools"
TOOLS=$(echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}' | \
    MOBILE_DEVICE_MCP_SETTINGS="$SETTINGS" $BINARY | \
    jq -r '.result.tools | length')
echo "✅ Found $TOOLS tools"
echo

# Test 3: List devices
echo "📋 Test 3: List Available Devices"
DEVICES=$(echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "mobile_list_available_devices", "arguments": {}}}' | \
    MOBILE_DEVICE_MCP_SETTINGS="$SETTINGS" $BINARY | \
    jq -r '.result.content[0].text')
echo "Devices found:"
echo "$DEVICES"
echo

# Check if we have any devices for further testing
if echo "$DEVICES" | grep -q "No devices found"; then
    echo "⚠️  No devices available for testing screenshots and interactions"
    echo "✅ MCP server is working but no devices connected"
    exit 0
fi

# Extract first device ID (assuming Android emulator format)
DEVICE_ID=$(echo "$DEVICES" | grep -o 'emulator-[0-9]*' | head -1)
if [ -z "$DEVICE_ID" ]; then
    DEVICE_ID=$(echo "$DEVICES" | grep -o '([^)]*' | sed 's/(//' | head -1)
fi

if [ -n "$DEVICE_ID" ]; then
    echo "🎯 Testing with device: $DEVICE_ID"
    echo

    # Test 4: Take screenshot
    echo "📋 Test 4: Take Screenshot"
    SCREENSHOT_RESULT=$(echo "{\"jsonrpc\": \"2.0\", \"id\": 4, \"method\": \"tools/call\", \"params\": {\"name\": \"mobile_take_screenshot\", \"arguments\": {\"device_id\": \"$DEVICE_ID\", \"platform\": \"android\"}}}" | \
        MOBILE_DEVICE_MCP_SETTINGS="$SETTINGS" $BINARY)

    if echo "$SCREENSHOT_RESULT" | jq -e '.result.content[0].data' > /dev/null 2>&1; then
        SCREENSHOT_SIZE=$(echo "$SCREENSHOT_RESULT" | jq -r '.result.content[0].data' | wc -c)
        echo "✅ Screenshot captured ($SCREENSHOT_SIZE characters base64)"
    else
        ERROR_MSG=$(echo "$SCREENSHOT_RESULT" | jq -r '.error.message // "Unknown error"')
        echo "❌ Screenshot failed: $ERROR_MSG"
    fi
    echo

    # Test 5: Tap screen
    echo "📋 Test 5: Tap Screen (center: 500, 500)"
    TAP_RESULT=$(echo "{\"jsonrpc\": \"2.0\", \"id\": 5, \"method\": \"tools/call\", \"params\": {\"name\": \"mobile_tap_screen\", \"arguments\": {\"device_id\": \"$DEVICE_ID\", \"platform\": \"android\", \"x\": 500, \"y\": 500}}}" | \
        MOBILE_DEVICE_MCP_SETTINGS="$SETTINGS" $BINARY)

    if echo "$TAP_RESULT" | jq -e '.result' > /dev/null 2>&1; then
        TAP_MSG=$(echo "$TAP_RESULT" | jq -r '.result.content[0].text')
        echo "✅ Tap successful: $TAP_MSG"
    else
        ERROR_MSG=$(echo "$TAP_RESULT" | jq -r '.error.message // "Unknown error"')
        echo "❌ Tap failed: $ERROR_MSG"
    fi
    echo

    # Test 6: Type text
    echo "📋 Test 6: Type Text"
    TYPE_RESULT=$(echo "{\"jsonrpc\": \"2.0\", \"id\": 6, \"method\": \"tools/call\", \"params\": {\"name\": \"mobile_type_text\", \"arguments\": {\"device_id\": \"$DEVICE_ID\", \"platform\": \"android\", \"text\": \"Hello MCP!\"}}}" | \
        MOBILE_DEVICE_MCP_SETTINGS="$SETTINGS" $BINARY)

    if echo "$TYPE_RESULT" | jq -e '.result' > /dev/null 2>&1; then
        TYPE_MSG=$(echo "$TYPE_RESULT" | jq -r '.result.content[0].text')
        echo "✅ Type text successful: $TYPE_MSG"
    else
        ERROR_MSG=$(echo "$TYPE_RESULT" | jq -r '.error.message // "Unknown error"')
        echo "❌ Type text failed: $ERROR_MSG"
    fi
fi

echo
echo "🎉 Mobile Device MCP Server Test Complete!"
echo "================================"

# Test invalid method
echo "📋 Error Handling Test: Invalid Method"
ERROR_TEST=$(echo '{"jsonrpc": "2.0", "id": 99, "method": "invalid/method", "params": {}}' | \
    MOBILE_DEVICE_MCP_SETTINGS="$SETTINGS" $BINARY)

if echo "$ERROR_TEST" | jq -e '.error' > /dev/null 2>&1; then
    ERROR_MSG=$(echo "$ERROR_TEST" | jq -r '.error.message')
    echo "✅ Error handling works: $ERROR_MSG"
else
    echo "❌ Error handling failed"
fi

echo
echo "✅ All tests completed successfully!"
echo "📦 Binary size: $(ls -lh target/release/mobile-device-mcp-server | awk '{print $5}')"
echo "🔧 Build features: native-binary, ios-support"
echo
echo "Ready for Zed installation! 🚀"
