#!/bin/bash

# Comprehensive test script for all mobile-device-mcp tools
# Tests all renamed tools with the mobile_device_mcp_ prefix

set -e

BINARY="./target/release/mobile-device-mcp-server"
DEVICE_ID="${DEVICE_ID:-emulator-5554}"
PLATFORM="${PLATFORM:-android}"

echo "=================================================="
echo "Mobile Device MCP - Comprehensive Tool Testing"
echo "=================================================="
echo ""
echo "Using device: $DEVICE_ID"
echo "Platform: $PLATFORM"
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "❌ Error: Binary not found at $BINARY"
    echo "Run: cargo build --release --features native-binary"
    exit 1
fi

# Counter for tests
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

# Function to run a test
run_test() {
    local test_name="$1"
    local test_cmd="$2"
    local expected_field="$3"

    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo "Test $TOTAL_TESTS: $test_name"
    echo "-------------------"

    result=$(echo "$test_cmd" | $BINARY 2>&1)

    if echo "$result" | grep -q "\"error\""; then
        echo "❌ FAILED - Error in response"
        echo "$result" | jq -r '.error.message' 2>/dev/null || echo "$result"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    elif [ -n "$expected_field" ] && echo "$result" | jq -e "$expected_field" >/dev/null 2>&1; then
        echo "✅ PASSED"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    elif [ -z "$expected_field" ]; then
        echo "✅ PASSED (no validation)"
        PASSED_TESTS=$((PASSED_TESTS + 1))
    else
        echo "❌ FAILED - Expected field not found: $expected_field"
        FAILED_TESTS=$((FAILED_TESTS + 1))
    fi
    echo ""
}

echo "=================================================="
echo "DEVICE INFORMATION TOOLS"
echo "=================================================="
echo ""

# Test 1: List available devices
run_test "mobile_device_mcp_list_available_devices" \
    '{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"mobile_device_mcp_list_available_devices","arguments":{}}}' \
    '.result.content[0].text'

# Test 2: Get screen size
run_test "mobile_device_mcp_get_screen_size" \
    "{\"jsonrpc\":\"2.0\",\"id\":2,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_get_screen_size\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" \
    '.result.content[0].text'

# Test 3: Get orientation
run_test "mobile_device_mcp_get_orientation" \
    "{\"jsonrpc\":\"2.0\",\"id\":3,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_get_orientation\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" \
    '.result.content[0].text'

# Test 4: List apps
run_test "mobile_device_mcp_list_apps" \
    "{\"jsonrpc\":\"2.0\",\"id\":4,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_list_apps\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" \
    '.result.content[0].text'

# Test 5: List elements on screen
run_test "mobile_device_mcp_list_elements_on_screen" \
    "{\"jsonrpc\":\"2.0\",\"id\":5,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_list_elements_on_screen\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" \
    '.result.content[0].text'

echo "=================================================="
echo "SCREEN INTERACTION TOOLS"
echo "=================================================="
echo ""

# Test 6: Take screenshot
run_test "mobile_device_mcp_take_screenshot" \
    "{\"jsonrpc\":\"2.0\",\"id\":6,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_take_screenshot\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\"}}}" \
    '.result.content[0].data'

# Test 7: Save screenshot
run_test "mobile_device_mcp_save_screenshot" \
    "{\"jsonrpc\":\"2.0\",\"id\":7,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_save_screenshot\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"output_path\":\"/tmp/test-mcp-screenshot.png\"}}}" \
    '.result.content[0].text'

# Verify screenshot file was created
if [ -f "/tmp/test-mcp-screenshot.png" ]; then
    echo "  ℹ️  Screenshot file verified: $(ls -lh /tmp/test-mcp-screenshot.png | awk '{print $5}')"
fi

# Test 8: Click on screen
run_test "mobile_device_mcp_click_on_screen_at_coordinates" \
    "{\"jsonrpc\":\"2.0\",\"id\":8,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_click_on_screen_at_coordinates\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"x\":640,\"y\":300}}}" \
    '.result.content[0].text'

# Test 9: Double tap
run_test "mobile_device_mcp_double_tap_on_screen" \
    "{\"jsonrpc\":\"2.0\",\"id\":9,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_double_tap_on_screen\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"x\":640,\"y\":300}}}" \
    '.result.content[0].text'

# Test 10: Long press
run_test "mobile_device_mcp_long_press_on_screen_at_coordinates" \
    "{\"jsonrpc\":\"2.0\",\"id\":10,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_long_press_on_screen_at_coordinates\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"x\":640,\"y\":300,\"duration\":1000}}}" \
    '.result.content[0].text'

# Test 11: Swipe
run_test "mobile_device_mcp_swipe_on_screen" \
    "{\"jsonrpc\":\"2.0\",\"id\":11,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_swipe_on_screen\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"start_x\":640,\"start_y\":1500,\"end_x\":640,\"end_y\":500,\"duration\":300}}}" \
    '.result.content[0].text'

echo "=================================================="
echo "INPUT TOOLS"
echo "=================================================="
echo ""

# Test 12: Type keys
run_test "mobile_device_mcp_type_keys" \
    "{\"jsonrpc\":\"2.0\",\"id\":12,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_type_keys\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"text\":\"test input\"}}}" \
    '.result.content[0].text'

# Test 13: Press button
run_test "mobile_device_mcp_press_button" \
    "{\"jsonrpc\":\"2.0\",\"id\":13,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_press_button\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"button\":\"back\"}}}" \
    '.result.content[0].text'

echo "=================================================="
echo "APP MANAGEMENT TOOLS"
echo "=================================================="
echo ""

# Test 14: Launch app (YouTube)
run_test "mobile_device_mcp_launch_app" \
    "{\"jsonrpc\":\"2.0\",\"id\":14,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_launch_app\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"app_id\":\"youtube\"}}}" \
    '.result.content[0].text'

# Wait a bit for app to launch
sleep 2

# Test 15: Terminate app
run_test "mobile_device_mcp_terminate_app" \
    "{\"jsonrpc\":\"2.0\",\"id\":15,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_terminate_app\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"app_id\":\"com.google.android.youtube\"}}}" \
    '.result.content[0].text'

# Test 16: Install app (skip - requires APK path)
echo "Test 16: mobile_device_mcp_install_app"
echo "-------------------"
echo "⏭️  SKIPPED (requires APK file)"
echo ""

# Test 17: Uninstall app (skip - don't want to uninstall test apps)
echo "Test 17: mobile_device_mcp_uninstall_app"
echo "-------------------"
echo "⏭️  SKIPPED (destructive test)"
echo ""

echo "=================================================="
echo "NAVIGATION TOOLS"
echo "=================================================="
echo ""

# Test 18: Open URL
run_test "mobile_device_mcp_open_url" \
    "{\"jsonrpc\":\"2.0\",\"id\":18,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_open_url\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"url\":\"https://example.com\"}}}" \
    '.result.content[0].text'

# Wait for browser to load
sleep 2

# Test 19: Set orientation to landscape
run_test "mobile_device_mcp_set_orientation" \
    "{\"jsonrpc\":\"2.0\",\"id\":19,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_set_orientation\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"orientation\":\"landscape\"}}}" \
    '.result.content[0].text'

# Wait a bit
sleep 1

# Test 20: Set orientation back to portrait
run_test "mobile_device_mcp_set_orientation (back to portrait)" \
    "{\"jsonrpc\":\"2.0\",\"id\":20,\"method\":\"tools/call\",\"params\":{\"name\":\"mobile_device_mcp_set_orientation\",\"arguments\":{\"device_id\":\"$DEVICE_ID\",\"platform\":\"$PLATFORM\",\"orientation\":\"portrait\"}}}" \
    '.result.content[0].text'

echo "=================================================="
echo "TEST SUMMARY"
echo "=================================================="
echo ""
echo "Total Tests:  $TOTAL_TESTS"
echo "Passed:       $PASSED_TESTS ✅"
echo "Failed:       $FAILED_TESTS ❌"
echo ""

if [ $FAILED_TESTS -eq 0 ]; then
    echo "🎉 All tests passed!"
    exit 0
else
    echo "⚠️  Some tests failed. Please review the output above."
    exit 1
fi
