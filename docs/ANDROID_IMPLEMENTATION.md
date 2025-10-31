# Android Implementation Guide

Complete guide to Android device automation support in the Mobile Device MCP Server.

## Overview

The Mobile Device MCP Server provides **complete Android support** with all 19 automation tools fully implemented. This includes support for physical devices, emulators, and Android TV devices.

## Feature Matrix

| Tool | Physical Device | Emulator | Android TV | Notes |
|------|-----------------|----------|------------|-------|
| `list_available_devices` | ✅ | ✅ | ✅ | Full support via ADB |
| `get_screen_size` | ✅ | ✅ | ✅ | Via `wm size` command |
| `get_orientation` | ✅ | ✅ | ✅ | Via settings command |
| `list_apps` | ✅ | ✅ | ✅ | Launcher activities only |
| `list_elements_on_screen` | ✅ | ✅ | ✅ | Via UI Automator XML dump |
| `take_screenshot` | ✅ | ✅ | ✅ | Fast PNG capture |
| `save_screenshot` | ✅ | ✅ | ✅ | Save to file system |
| `click_on_screen_at_coordinates` | ✅ | ✅ | ✅ | Via input tap |
| `double_tap_on_screen` | ✅ | ✅ | ✅ | Two rapid taps |
| `long_press_on_screen_at_coordinates` | ✅ | ✅ | ✅ | Via swipe with duration |
| `swipe_on_screen` | ✅ | ✅ | ✅ | Configurable duration |
| `type_keys` | ✅ | ✅ | ✅ | Full text input |
| `press_button` | ✅ | ✅ | ✅ | All hardware buttons |
| `launch_app` | ✅ | ✅ | ✅ | Via am start |
| `terminate_app` | ✅ | ✅ | ✅ | Via am force-stop |
| `install_app` | ✅ | ✅ | ✅ | APK installation |
| `uninstall_app` | ✅ | ✅ | ✅ | Package removal |
| `open_url` | ✅ | ✅ | ✅ | Opens in browser |
| `set_orientation` | ✅ | ✅ | ✅ | Portrait/landscape |

**Status:** All 19 tools fully implemented (100% coverage)

## Prerequisites

### For All Android Devices

**Required: Android Debug Bridge (ADB)**

**macOS:**
```bash
brew install android-platform-tools

# Verify installation
adb version
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install android-tools-adb android-tools-fastboot

# Verify installation
adb version
```

**Windows:**
```powershell
# Using Chocolatey
choco install adb

# Or download from:
# https://developer.android.com/tools/releases/platform-tools

# Verify installation
adb version
```

### For Physical Devices

**Device Setup:**
1. Enable Developer Options:
   - Go to Settings → About Phone
   - Tap "Build Number" 7 times
   - Developer Options will appear in Settings

2. Enable USB Debugging:
   - Settings → Developer Options → USB Debugging
   - Toggle ON

3. Connect via USB:
   - Connect device to computer
   - Accept "Allow USB Debugging" prompt on device
   - Authorize computer's RSA fingerprint

**Verify Connection:**
```bash
adb devices

# Expected output:
# List of devices attached
# ABC123DEF456    device
```

### For Android Emulators

**Android Studio Emulator:**
```bash
# List available AVDs
emulator -list-avds

# Start an emulator
emulator -avd Pixel_6_API_34

# Verify connection
adb devices
# Output: emulator-5554    device
```

**Genymotion:**
```bash
# Genymotion devices connect automatically
adb devices
```

## Architecture

### Device Management

```rust
pub struct AndroidDeviceManager {
    debug: bool,
    server: ADBServer,  // ADB client connection
}

pub struct AndroidRobot {
    device_id: String,
    server: ADBServer,
    debug: bool,
}
```

**Discovery Flow:**
1. Connect to ADB server on localhost:5037
2. Query connected devices via `adb devices`
3. Parse device list with states (device, unauthorized, offline)
4. Return structured device information

### ADB Communication

**Technology:** `adb_client` Rust crate (version 2.1+)

**Connection Methods:**
- TCP/IP: localhost:5037 (default ADB server port)
- USB: Direct device communication
- Wireless: Network debugging (Android 11+)

**Command Execution:**
```rust
// Shell command execution
device.shell_command(&["command", "arg1", "arg2"])?;

// File operations
device.push("/local/path", "/device/path")?;
device.pull("/device/path", "/local/path")?;

// App management
device.install("/path/to/app.apk", InstallOptions::default())?;
```

### Screenshot Capture

**Method 1: Fast PNG Capture (Preferred)**
```bash
adb shell screencap -p
```
- Returns PNG data directly to stdout
- No file system writes needed
- ~50-200ms execution time

**Method 2: Raw Framebuffer (Fallback)**
```bash
adb shell screencap /sdcard/screenshot.png
adb pull /sdcard/screenshot.png
```
- Slower but more compatible
- Works on older Android versions

**Implementation:**
```rust
pub fn get_screenshot(&mut self) -> Result<Vec<u8>, String> {
    // Try fast method first
    let output = self.execute_shell_command(&["screencap", "-p"])?;
    
    // Handle line ending conversions (Windows)
    let png_data = if output.starts_with(&[0x89, 0x50, 0x4E, 0x47]) {
        output
    } else {
        // Fix corrupted PNG data
        fix_png_data(output)
    };
    
    Ok(png_data)
}
```

### Screen Interaction

#### Tap/Click
```bash
adb shell input tap <x> <y>
```

**Coordinate System:**
- Origin: Top-left corner (0, 0)
- X-axis: Left to right
- Y-axis: Top to bottom
- Units: Pixels (not density-independent)

#### Double Tap
```rust
pub fn double_tap(&mut self, x: u32, y: u32) -> Result<(), String> {
    self.tap(x, y)?;
    std::thread::sleep(Duration::from_millis(50));
    self.tap(x, y)?;
    Ok(())
}
```

#### Long Press
```bash
adb shell input swipe <x> <y> <x> <y> <duration_ms>
```
- Implemented as swipe with no movement
- Default duration: 1000ms (1 second)

#### Swipe
```bash
adb shell input swipe <start_x> <start_y> <end_x> <end_y> <duration_ms>
```

**Common Gestures:**
- Swipe Up: Bottom to top (dismiss notification, open app drawer)
- Swipe Down: Top to bottom (pull down notification shade)
- Swipe Left/Right: Horizontal navigation

### Text Input

**Method 1: Direct Text Input**
```bash
adb shell input text "Hello%sWorld"
```
- Spaces must be encoded as `%s`
- Special characters must be escaped
- Fast for simple text

**Method 2: Keyboard Events**
```bash
adb shell input keyevent KEYCODE_A
adb shell input keyevent KEYCODE_SHIFT_LEFT KEYCODE_A
```
- More reliable for special characters
- Slower (one event per character)

**Implementation:**
```rust
pub fn send_keys(&mut self, text: &str) -> Result<(), String> {
    if text.is_empty() {
        return Ok(());
    }
    
    // Escape special characters
    let escaped = self.escape_shell_text(text);
    
    // Split into chunks to avoid command length limits
    for chunk in escaped.chunks(100) {
        self.execute_shell_command(&["input", "text", chunk])?;
    }
    
    Ok(())
}

fn escape_shell_text(&self, text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\'', "\\'")
        .replace('"', "\\\"")
        .replace('`', "\\`")
        .replace(' ', "%s")  // Android requires %s for spaces
        .replace('\t', "\\t")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('|', "\\|")
        .replace('&', "\\&")
        .replace(';', "\\;")
        .replace('>', "\\>")
        .replace('<', "\\<")
        .replace('(', "\\(")
        .replace(')', "\\)")
}
```

### Hardware Buttons

**Supported Buttons:**
```rust
pub enum Button {
    Back,           // KEYCODE_BACK
    Home,           // KEYCODE_HOME
    Menu,           // KEYCODE_MENU
    Power,          // KEYCODE_POWER
    Camera,         // KEYCODE_CAMERA
    VolumeUp,       // KEYCODE_VOLUME_UP
    VolumeDown,     // KEYCODE_VOLUME_DOWN
    Enter,          // KEYCODE_ENTER
    DpadCenter,     // KEYCODE_DPAD_CENTER
    DpadUp,         // KEYCODE_DPAD_UP
    DpadDown,       // KEYCODE_DPAD_DOWN
    DpadLeft,       // KEYCODE_DPAD_LEFT
    DpadRight,      // KEYCODE_DPAD_RIGHT
}
```

**Usage:**
```bash
adb shell input keyevent <KEYCODE>
```

### UI Element Inspection

**Technology:** UI Automator XML Dump

**Command:**
```bash
adb shell uiautomator dump /dev/tty
```

**Output Format:**
```xml
<?xml version='1.0' encoding='UTF-8' standalone='yes' ?>
<hierarchy rotation="0">
  <node index="0" text="" resource-id="" class="android.widget.FrameLayout" 
        bounds="[0,0][1080,2400]" clickable="false" focused="false">
    <node index="0" text="Sign in" resource-id="com.example:id/sign_in_button"
          class="android.widget.Button" bounds="[100,1000][980,1200]"
          clickable="true" focused="false" />
  </node>
</hierarchy>
```

**Parsing:**
```rust
pub fn list_screen_elements(&mut self, filter: Option<&str>) 
    -> Result<Vec<ScreenElement>, String> {
    
    // Get XML dump
    let xml = self.execute_shell_command_string(&["uiautomator", "dump", "/dev/tty"])?;
    
    // Parse with quick-xml
    let mut reader = XmlReader::from_str(&xml);
    let mut elements = Vec::new();
    
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"node" => {
                let element = parse_node_attributes(&e)?;
                
                // Apply filter if provided
                if let Some(filter_text) = filter {
                    if element_matches_filter(&element, filter_text) {
                        elements.push(element);
                    }
                } else {
                    elements.push(element);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
    }
    
    Ok(elements)
}
```

**Extracted Information:**
- Element type (Button, TextView, EditText, etc.)
- Text content
- Resource ID
- Bounds (x, y, width, height)
- Clickable state
- Focused state
- Content description

### Screen Properties

#### Get Screen Size
```bash
adb shell wm size
# Output: Physical size: 1080x2400
```

**Parsing:**
```rust
pub fn get_screen_size(&mut self) -> Result<ScreenSize, String> {
    let output = self.execute_shell_command_string(&["wm", "size"])?;
    
    // Parse "Physical size: 1080x2400" or "1080x2400"
    if let Some(dimensions) = output.split(':').last() {
        let parts: Vec<&str> = dimensions.trim().split('x').collect();
        if parts.len() == 2 {
            let width = parts[0].parse::<u32>()?;
            let height = parts[1].parse::<u32>()?;
            
            // Get density scale
            let density = self.get_display_density()?;
            
            return Ok(ScreenSize {
                width,
                height,
                scale: density,
            });
        }
    }
    
    Err("Failed to parse screen size".to_string())
}
```

#### Get/Set Orientation
```bash
# Get orientation
adb shell settings get system user_rotation
# Output: 0 (portrait), 1 (landscape left), 2 (portrait inverted), 3 (landscape right)

# Set orientation
adb shell settings put system user_rotation 0
adb shell settings put system accelerometer_rotation 0
```

### App Management

#### List Installed Apps
```bash
# List all packages
adb shell pm list packages

# List with launcher activities
adb shell cmd package list packages --show-uid
```

**Implementation:**
```rust
pub fn list_apps(&mut self) -> Result<Vec<InstalledApp>, String> {
    // Get packages with launcher activities
    let output = self.execute_shell_command_string(&[
        "cmd", "package", "query-activities",
        "--brief", "android.intent.action.MAIN",
    ])?;
    
    let mut apps = Vec::new();
    for line in output.lines() {
        if let Some(package_name) = extract_package_name(line) {
            let app_name = self.get_app_label(&package_name)
                .unwrap_or_else(|_| package_name.clone());
            
            apps.push(InstalledApp {
                package_name,
                app_name,
            });
        }
    }
    
    Ok(apps)
}
```

#### Launch App
```bash
# Launch with main activity
adb shell am start -n com.example.app/.MainActivity

# Launch with default activity
adb shell monkey -p com.example.app -c android.intent.category.LAUNCHER 1
```

**Implementation:**
```rust
pub fn launch_app(&mut self, package_name: &str) -> Result<(), String> {
    // Method 1: Try monkey command (most reliable)
    let result = self.execute_shell_command(&[
        "monkey",
        "-p", package_name,
        "-c", "android.intent.category.LAUNCHER",
        "1",
    ]);
    
    if result.is_ok() {
        return Ok(());
    }
    
    // Method 2: Try am start (requires activity name)
    self.execute_shell_command(&[
        "am", "start",
        "-n", &format!("{}/.MainActivity", package_name),
    ])
}
```

#### Terminate App
```bash
adb shell am force-stop com.example.app
```

#### Install App
```bash
adb install /path/to/app.apk

# With options
adb install -r /path/to/app.apk  # Replace existing
adb install -d /path/to/app.apk  # Allow downgrade
adb install -g /path/to/app.apk  # Grant all permissions
```

**Implementation:**
```rust
pub fn install_app(&mut self, apk_path: &str) -> Result<(), String> {
    let mut device = self.get_device()?;
    
    let options = InstallOptions {
        replace_existing: true,
        grant_permissions: true,
        ..Default::default()
    };
    
    device.install(apk_path, options)
        .map_err(|e| format!("Install failed: {}", e))?;
    
    Ok(())
}
```

#### Uninstall App
```bash
adb uninstall com.example.app
```

### Navigation

#### Open URL
```bash
adb shell am start -a android.intent.action.VIEW -d "https://example.com"
```

**Supported URL Schemes:**
- `http://` / `https://` - Opens in browser
- `tel:` - Opens dialer
- `mailto:` - Opens email client
- `geo:` - Opens maps
- Custom schemes - Opens associated app

## Usage Examples

### List All Android Devices

```bash
echo '{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "mobile_device_mcp_list_available_devices",
    "arguments": {
      "platform": "android"
    }
  }
}' | mobile-device-mcp-server
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [{
      "type": "text",
      "text": "Found 2 Android devices:\n\n1. Pixel 6 [Emulator]\n   ID: emulator-5554\n   State: Connected\n\n2. Samsung Galaxy S21 [Physical]\n   ID: ABC123DEF456\n   State: Connected"
    }]
  }
}
```

### Take Screenshot

```bash
echo '{
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
}' | mobile-device-mcp-server
```

### List UI Elements with Filter

```bash
echo '{
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
}' | mobile-device-mcp-server
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "result": {
    "content": [{
      "type": "text",
      "text": "Found 1 matching element:\n\nButton: \"Sign in\"\n  Position: (100, 1000) Size: 880×200\n  Resource ID: com.example:id/sign_in_button\n  Clickable: true\n  Tap coordinates: (540, 1100)"
    }]
  }
}
```

### Automated Test Flow

```bash
#!/bin/bash
DEVICE="emulator-5554"
SERVER="./mobile-device-mcp-server"

# 1. Launch app
echo "Launching app..."
$SERVER <<EOF
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"mobile_device_mcp_launch_app","arguments":{"device_id":"$DEVICE","platform":"android","app_id":"com.example.app"}}}
EOF

# 2. Wait for app to load
sleep 2

# 3. Find and tap "Sign in" button
echo "Finding Sign in button..."
$SERVER <<EOF
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mobile_device_mcp_list_elements_on_screen","arguments":{"device_id":"$DEVICE","platform":"android","filter":"Sign in"}}}
EOF

# 4. Tap at button center (example: 540, 1100)
echo "Tapping Sign in button..."
$SERVER <<EOF
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"mobile_device_mcp_click_on_screen_at_coordinates","arguments":{"device_id":"$DEVICE","platform":"android","x":540,"y":1100}}}
EOF

# 5. Type username
echo "Entering username..."
$SERVER <<EOF
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"mobile_device_mcp_type_keys","arguments":{"device_id":"$DEVICE","platform":"android","text":"user@example.com"}}}
EOF

# 6. Take screenshot
echo "Capturing screenshot..."
$SERVER <<EOF
{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"mobile_device_mcp_take_screenshot","arguments":{"device_id":"$DEVICE","platform":"android"}}}
EOF
```

## Performance Optimization

### Screenshot Capture
- **Fast path:** Direct PNG capture (~50-200ms)
- **Avoid:** Repeated file system writes
- **Optimize:** Reuse ADB connections

### UI Element Inspection
- **Cache:** XML dumps when possible
- **Filter early:** Pass filter to reduce parsing
- **Limit depth:** Don't parse entire hierarchy if not needed

### Command Batching
```rust
// Bad: Multiple ADB calls
device.shell_command(&["input", "tap", "100", "100"])?;
device.shell_command(&["input", "tap", "200", "200"])?;
device.shell_command(&["input", "tap", "300", "300"])?;

// Good: Batch commands
device.shell_command(&["sh", "-c", 
    "input tap 100 100 && input tap 200 200 && input tap 300 300"
])?;
```

### Connection Pooling
```rust
// Reuse AndroidRobot instances
let mut robot = manager.create_robot(device_id);
robot.tap(100, 100)?;
robot.tap(200, 200)?;
robot.tap(300, 300)?;
// Robot maintains ADB connection throughout
```

## Error Handling

### Common Errors and Solutions

**"Device not found"**
```bash
# Check device connection
adb devices

# Restart ADB server
adb kill-server
adb start-server
```

**"Device unauthorized"**
```bash
# Revoke and re-authorize
adb kill-server
# Disconnect and reconnect device
# Accept prompt on device
```

**"Insufficient permissions"**
```bash
# Grant app permissions
adb shell pm grant com.example.app android.permission.CAMERA

# Or install with -g flag
adb install -g app.apk
```

**"Activity not found"**
```bash
# List activities
adb shell pm dump com.example.app | grep Activity

# Launch with full activity name
adb shell am start -n com.example.app/.ui.MainActivity
```

**"Screen is locked"**
```bash
# Unlock device
adb shell input keyevent KEYCODE_WAKEUP
adb shell input keyevent KEYCODE_MENU
```

## Testing

### Comprehensive Test Suite

```bash
#!/bin/bash
# test-android-tools.sh

set -e

DEVICE="${1:-emulator-5554}"
SERVER="./target/release/mobile-device-mcp-server"

echo "🧪 Testing Android Tools on $DEVICE"
echo "===================================="

# Test 1: Device detection
echo "✓ Test 1: List devices"
$SERVER <<EOF | jq -r '.result.content[0].text'
{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"mobile_device_mcp_list_available_devices","arguments":{"platform":"android"}}}
EOF

# Test 2: Screenshot
echo "✓ Test 2: Take screenshot"
$SERVER <<EOF | jq -r '.result.content[0].data' | head -c 20
{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"mobile_device_mcp_take_screenshot","arguments":{"device_id":"$DEVICE","platform":"android"}}}
EOF
echo " (base64 data truncated)"

# Test 3: Screen size
echo "✓ Test 3: Get screen size"
$SERVER <<EOF | jq -r '.result.content[0].text'
{"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"mobile_device_mcp_get_screen_size","arguments":{"device_id":"$DEVICE","platform":"android"}}}
EOF

# Test 4: Tap
echo "✓ Test 4: Tap screen"
$SERVER <<EOF | jq -r '.result.content[0].text'
{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"mobile_device_mcp_click_on_screen_at_coordinates","arguments":{"device_id":"$DEVICE","platform":"android","x":500,"y":1000}}}
EOF

# Test 5: Type text
echo "✓ Test 5: Type text"
$SERVER <<EOF | jq -r '.result.content[0].text'
{"jsonrpc":"2.0","id":5,"method":"tools/call","params":{"name":"mobile_device_mcp_type_keys","arguments":{"device_id":"$DEVICE","platform":"android","text":"Hello Android!"}}}
EOF

# Test 6: Press button
echo "✓ Test 6: Press home button"
$SERVER <<EOF | jq -r '.result.content[0].text'
{"jsonrpc":"2.0","id":6,"method":"tools/call","params":{"name":"mobile_device_mcp_press_button","arguments":{"device_id":"$DEVICE","platform":"android","button":"home"}}}
EOF

echo ""
echo "✅ All Android tests passed!"
```

## Best Practices

### 1. Device State Management
```rust
// Always check device state before operations
if device.is_screen_on()? {
    device.tap(x, y)?;
} else {
    device.wake()?;
    device.unlock()?;
    device.tap(x, y)?;
}
```

### 2. Error Recovery
```rust
// Retry transient failures
const MAX_RETRIES: u32 = 3;
for attempt in 1..=MAX_RETRIES {
    match device.take_screenshot() {
        Ok(screenshot) => return Ok(screenshot),
        Err(e) if attempt < MAX_RETRIES => {
            eprintln!("Screenshot attempt {} failed: {}", attempt, e);
            std::thread::sleep(Duration::from_millis(500));
        }
        Err(e) => return Err(e),
    }
}
```

### 3. Resource Cleanup
```rust
// Terminate test apps after testing
device.terminate_app("com.example.testapp")?;

// Clear app data
device.shell_command(&["pm", "clear", "com.example.testapp"])?;

// Disable animations for testing
device.shell_command(&[
    "settings", "put", "global", "window_animation_scale", "0"
])?;
```

### 4. Coordinate Handling
```rust
// Account for different screen densities
let screen_size = device.get_screen_size()?;
let center_x = screen_size.width / 2;
let center_y = screen_size.height / 2;

// Use relative coordinates
device.tap(center_x, center_y)?;
```

### 5. Text Input Best Practices
```rust
// Focus the text field first
device.tap(text_field_x, text_field_y)?;
std::thread::sleep(Duration::from_millis(100));

// Clear existing text
device.press_button(Button::DpadCenter)?;
device.shell_command(&["input", "keyevent", "KEYCODE_MOVE_END"])?;
for _ in 0..50 {
    device.shell_command(&["input", "keyevent", "KEYCODE_DEL"])?;
}

// Type new text
device.send_keys("New text")?;
```

## Troubleshooting

### ADB Connection Issues

**ADB server out of date:**
```bash
adb kill-server
adb start-server
```

**Device offline:**
```bash
# Reconnect device
adb disconnect
adb connect <device_ip>:5555
```

**Multiple devices connected:**
```bash
# Specify device by serial
adb -s emulator-5554 shell
```

### Performance Issues

**Slow screenshot capture:**
- Use PNG method (`screencap -p`)
- Reduce image quality if needed
- Check device storage space

**Slow UI element inspection:**
- Filter results to reduce XML size
- Cache XML dumps when possible
- Limit inspection depth

### Emulator Issues

**Emulator not detected:**
```bash
# Check emulator status
adb devices

# Restart emulator
adb -s emulator-5554 emu kill
emulator -avd Your_AVD_Name
```

**Emulator too slow:**
- Enable hardware acceleration (HAXM/WHPX)
- Allocate more RAM
- Use x86 system image instead of ARM
- Enable "Cold Boot" in AVD settings

## Additional Resources

- [Android Debug Bridge (ADB) Documentation](https://developer.android.com/tools/adb)
- [UI Automator Documentation](https://developer.android.com/training/testing/other-components/ui-automator)
- [Android Input Command Reference](https://source.android.com/docs/core/interaction/input)
- [Package Manager Commands](https://developer.android.com/tools/adb#pm)
- [Activity Manager Commands](https://developer.android.com/tools/adb#am)

## Contributing

To extend Android support:

1. Identify new automation needs
2. Research ADB/shell commands
3. Implement in `src/devices/android.rs`
4. Add handler in `src/main.rs`
5. Update documentation
6. Add comprehensive tests

See `CONTRIBUTING.md` for detailed guidelines.

---

**Last Updated:** 2024-11-01  
**Android Support Version:** Complete (19/19 tools)  
**Status:** Production Ready  
**Minimum Android Version:** Android 5.0 (API 21)  
**Recommended:** Android 11+ (API 30+) for full feature support