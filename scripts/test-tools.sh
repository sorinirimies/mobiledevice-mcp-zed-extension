#!/bin/bash

# Test script for mobile device MCP tools
set -e

BINARY="./target/release/mobile-device-mcp-server"

echo "Testing Mobile Device MCP Tools..."
echo "=================================="

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "Error: Binary not found at $BINARY"
    echo "Run: cargo build --release --features native-binary"
    exit 1
fi

# Test 1: List devices
echo ""
echo "Test 1: List Available Devices"
echo "-------------------------------"
echo '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"mobile_device_mcp_list_available_devices","arguments":{}}}' | $BINARY

# Get device ID from environment or use default
DEVICE_ID="${DEVICE_ID:-emulator-5554}"
PLATFORM="${PLATFORM:-android}"

echo ""
echo "Using device: $DEVICE_ID (platform: $PLATFORM)"
echo ""

# Test 2: Get screen size
echo "Test 2: Get Screen Size"
echo "-----------------------"
echo "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_get_screen_size\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" | $BINARY

# Test 3: Take screenshot
echo ""
echo "Test 3: Take Screenshot"
echo "-----------------------"
echo "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_take_screenshot\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" | $BINARY | head -100

# Test 4: Save screenshot
echo ""
echo "Test 4: Save Screenshot to File"
echo "--------------------------------"
echo "{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_save_screenshot\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"output_path\":\"/tmp/test-screenshot.png\"}}}" | $BINARY

# Check if screenshot was saved
if [ -f "/tmp/test-screenshot.png" ]; then
    echo "✓ Screenshot saved successfully"
    ls -lh /tmp/test-screenshot.png
else
    echo "✗ Screenshot not saved"
fi

# Test 5: List elements on screen
echo ""
echo "Test 5: List Elements on Screen"
echo "--------------------------------"
echo "{\"jsonrpc\":\"2.0\",\"id\":5,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_list_elements_on_screen\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" | $BINARY

echo ""
echo "=================================="
echo "All tests completed!"
