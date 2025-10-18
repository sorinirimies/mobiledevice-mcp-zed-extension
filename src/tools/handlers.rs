// mobile-mcp-zed-extension/src/tools/handlers.rs
// Tool Handler Implementations for Mobile Device Automation
//
// This module implements the actual functionality for each MCP tool defined
// in definitions.rs. Each handler:
// - Takes a MobileDeviceManager reference and tool-specific parameters
// - Performs the device operation (via AndroidRobot or IOSDevice)
// - Returns an MCP-formatted JSON response or error
//
// Handlers are organized by category matching the tool definitions:
// - Device Information Handlers
// - Screen Interaction Handlers
// - Input Handlers
// - App Management Handlers
// - Navigation Handlers
//
// All handlers return HandlerResult which wraps either:
// - Ok(Value) - MCP response with content array
// - Err(String) - Error message to be sent to client

#[cfg(feature = "native-binary")]
use crate::MobileDeviceManager;
use serde_json::Value;

#[cfg(feature = "native-binary")]
use base64::{engine::general_purpose::STANDARD, Engine};

/// Result type for tool handlers
///
/// Success returns a JSON Value formatted for MCP protocol with a content array.
/// Error returns a String that will be wrapped in an MCP error response.
pub type HandlerResult = Result<Value, String>;

// ============================================================================
// Device Information Handlers
// ============================================================================
//
// These handlers query device state and capabilities. They provide essential
// information for device discovery and determining what operations are possible.

/// List all connected mobile devices
///
/// Discovers Android devices (via adb) and iOS devices (via xcrun simctl on macOS).
/// Returns a formatted list showing device name, ID, type, and connection state.
///
/// # Arguments
/// * `manager` - Mobile device manager with access to Android/iOS managers
/// * `platform` - Filter: "android", "ios", or "auto" for all devices
///
/// # Returns
/// MCP response with text content listing all devices, or error message
///
/// # Example Response
/// ```text
/// - Pixel 6 (emulator-5554) - android mobile [connected]
/// - iPhone 15 (UDID-123) - ios simulator [booted]
/// ```
pub fn handle_list_devices(manager: &mut MobileDeviceManager, platform: &str) -> HandlerResult {
    let devices = manager.list_all_devices(platform);

    let device_list = devices
        .iter()
        .map(|d| format!("- {} ({}) - {} [{}]", d.name, d.id, d.device_type, d.state))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(serde_json::json!({
        "content": [{
            "type": "text",
            "text": if device_list.is_empty() {
                "No devices found. Please ensure Android platform tools (adb) or iOS tools (xcrun) are installed.".to_string()
            } else {
                device_list
            }
        }]
    }))
}

/// Get device screen dimensions
///
/// Queries the device for its current screen resolution in pixels.
/// Useful for calculating tap coordinates or determining UI layout.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier (e.g., "emulator-5554")
/// * `platform` - Platform: "android" or "ios"
///
/// # Returns
/// MCP response with screen dimensions as text, or error if device not found
pub fn handle_get_screen_size(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.get_screen_size(device_id, platform) {
            Ok((width, height)) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Screen size: {}x{} pixels", width, height)
                }]
            })),
            Err(e) => Err(format!("Failed to get screen size: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform);
        Err("Not available in extension mode".to_string())
    }
}

/// Query current device orientation
///
/// Determines if the device screen is in portrait or landscape mode.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
///
/// # Returns
/// MCP response with orientation ("portrait" or "landscape"), or error
pub fn handle_get_orientation(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.get_orientation(device_id, platform) {
            Ok(orientation) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Current orientation: {}", orientation)
                }]
            })),
            Err(e) => Err(format!("Failed to get orientation: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform);
        Err("Not available in extension mode".to_string())
    }
}

/// List all installed applications
///
/// Returns user-installed apps with their package names and display labels.
/// System apps are typically excluded. This helps discover app identifiers
/// needed for launch_app and terminate_app operations.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
///
/// # Returns
/// MCP response with formatted app list, or error if query fails
pub fn handle_list_apps(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.list_apps(device_id, platform) {
            Ok(apps) => {
                let app_list = apps
                    .iter()
                    .map(|app| format!("- {} ({})", app.app_name, app.package_name))
                    .collect::<Vec<_>>()
                    .join("\n");

                Ok(serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": if app_list.is_empty() {
                            "No apps found".to_string()
                        } else {
                            format!("Installed apps:\n{}", app_list)
                        }
                    }]
                }))
            }
            Err(e) => Err(format!("Failed to list apps: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform);
        Err("Not available in extension mode".to_string())
    }
}

/// List UI elements currently on screen
///
/// Performs UI hierarchy inspection to find visible elements, their bounds,
/// text content, and properties. Optional filter allows searching for specific
/// elements. This is powerful for automating UI interactions without hardcoded
/// coordinates.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `filter` - Optional text filter to search for specific elements
///
/// # Returns
/// MCP response with element list (position, size, text, resource ID), or error
pub fn handle_list_elements(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    filter: Option<&str>,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.list_elements_on_screen(device_id, platform, filter) {
            Ok(elements) => {
                let element_list = elements
                    .iter()
                    .map(|el| {
                        format!(
                            "- {} at ({},{}) size {}x{} [type: {}]{}",
                            el.label,
                            el.rect.x,
                            el.rect.y,
                            el.rect.width,
                            el.rect.height,
                            el.element_type,
                            if let Some(res_id) = &el.identifier {
                                format!(" [id: {}]", res_id)
                            } else {
                                String::new()
                            }
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                Ok(serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": if element_list.is_empty() {
                            format!("No elements found (parsed {} elements, all filtered out)", elements.len())
                        } else {
                            format!("Screen elements:\n{}", element_list)
                        }
                    }]
                }))
            }
            Err(e) => Err(format!("Failed to list elements: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, filter);
        Err("Not available in extension mode".to_string())
    }
}

// ============================================================================
// Screen Interaction Handlers
// ============================================================================
//
// These handlers perform visual interactions with the device screen, including
// screenshot capture and touch gestures (tap, swipe, long press, etc.).

/// Capture current screen as image
///
/// Takes a screenshot of the device screen and returns it as base64-encoded
/// PNG data in the MCP response. This is the primary way to "see" what's
/// currently displayed on the device. AI assistants that support image content
/// can display these screenshots directly.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
///
/// # Returns
/// MCP response with base64-encoded PNG image data, or error if capture fails
pub fn handle_take_screenshot(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.take_screenshot(device_id, platform) {
            Ok(screenshot_data) => {
                let base64_data = STANDARD.encode(&screenshot_data);
                Ok(serde_json::json!({
                    "content": [{
                        "type": "image",
                        "data": base64_data,
                        "mimeType": "image/png"
                    }]
                }))
            }
            Err(e) => Err(format!("Failed to take screenshot: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform);
        Err("Not available in extension mode".to_string())
    }
}

/// Save screenshot to file system
///
/// Like take_screenshot, but saves directly to a file path instead of returning
/// base64 data. More efficient for batch operations or when creating test artifacts.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `output_path` - File path where screenshot should be saved (e.g., "/tmp/screen.png")
///
/// # Returns
/// MCP response confirming save location, or error if capture/save fails
pub fn handle_save_screenshot(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    output_path: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.take_screenshot(device_id, platform) {
            Ok(screenshot_data) => match std::fs::write(output_path, screenshot_data) {
                Ok(_) => Ok(serde_json::json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Screenshot saved to: {}", output_path)
                    }]
                })),
                Err(e) => Err(format!("Failed to save screenshot: {}", e)),
            },
            Err(e) => Err(format!("Failed to take screenshot: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, output_path);
        Err("Not available in extension mode".to_string())
    }
}

/// Perform a single tap at coordinates
///
/// Simulates a finger tap at the specified pixel coordinates. This is the most
/// common interaction - use list_elements or take_screenshot first to find
/// coordinates. Coordinates are absolute pixels from top-left corner (0,0).
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `x` - X coordinate in pixels
/// * `y` - Y coordinate in pixels
///
/// # Returns
/// MCP response confirming tap action, or error if device communication fails
pub fn handle_click_screen(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    x: f64,
    y: f64,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.tap_screen(device_id, platform, x, y) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Clicked at ({}, {}): {}", x, y, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to click: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, x, y);
        Err("Not available in extension mode".to_string())
    }
}

/// Perform a double-tap gesture
///
/// Executes two rapid taps at the same location with 50ms delay between them.
/// Common for zoom gestures, text selection, or double-tap-specific actions.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `x` - X coordinate in pixels
/// * `y` - Y coordinate in pixels
///
/// # Returns
/// MCP response confirming double tap, or error
pub fn handle_double_tap(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    x: f64,
    y: f64,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.double_tap_screen(device_id, platform, x, y) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Double tapped at ({}, {}): {}", x, y, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to double tap: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, x, y);
        Err("Not available in extension mode".to_string())
    }
}

/// Perform a long press (press and hold)
///
/// Holds touch at coordinates for specified duration. Triggers context menus,
/// drag operations, or selection mode in many apps.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `x` - X coordinate in pixels
/// * `y` - Y coordinate in pixels
/// * `duration` - Hold duration in milliseconds (default 1000ms if None)
///
/// # Returns
/// MCP response confirming long press with duration, or error
pub fn handle_long_press(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    x: f64,
    y: f64,
    duration: Option<u32>,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        let duration_ms = duration.unwrap_or(1000);
        match manager.long_press_screen(device_id, platform, x, y, duration_ms) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Long pressed at ({}, {}) for {}ms: {}", x, y, duration_ms, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to long press: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, x, y, duration);
        Err("Not available in extension mode".to_string())
    }
}

/// Perform a swipe gesture
///
/// Simulates dragging a finger from start to end point over specified duration.
/// Essential for scrolling, switching pages, pull-to-refresh, or drag gestures.
/// Duration affects swipe speed - shorter is faster.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `start_x` - Starting X coordinate in pixels
/// * `start_y` - Starting Y coordinate in pixels
/// * `end_x` - Ending X coordinate in pixels
/// * `end_y` - Ending Y coordinate in pixels
/// * `duration` - Swipe duration in milliseconds (default 300ms if None)
///
/// # Returns
/// MCP response confirming swipe action, or error
#[allow(clippy::too_many_arguments)]
pub fn handle_swipe(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    start_x: f64,
    start_y: f64,
    end_x: f64,
    end_y: f64,
    duration: Option<u32>,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        let duration_ms = duration.unwrap_or(300);
        match manager.swipe_screen(
            device_id,
            platform,
            start_x,
            start_y,
            end_x,
            end_y,
            duration_ms,
        ) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Swiped from ({}, {}) to ({}, {}): {}", start_x, start_y, end_x, end_y, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to swipe: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (
            manager, device_id, platform, start_x, start_y, end_x, end_y, duration,
        );
        Err("Not available in extension mode".to_string())
    }
}

// ============================================================================
// Input Handlers
// ============================================================================
//
// These handlers simulate text input and hardware button presses.
// Text input requires an input field to have focus first (tap on it).

/// Type text into focused input field
///
/// Sends text to the currently focused element (must be a text input).
/// Make sure to tap on an input field first before calling this.
/// Supports alphanumeric, special characters, and emojis (platform-dependent).
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `text` - Text to type
///
/// # Returns
/// MCP response confirming text input, or error if no field is focused
pub fn handle_type_keys(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    text: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.type_text(device_id, platform, text) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Typed text: {}", msg)
                }]
            })),
            Err(e) => Err(format!("Failed to type text: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, text);
        Err("Not available in extension mode".to_string())
    }
}

/// Press hardware or navigation button
///
/// Simulates pressing physical buttons (volume, power) or navigation buttons
/// (home, back, menu). These are system-level actions that work regardless
/// of the current app state. Essential for navigation and device control.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `button` - Button name: "home", "back", "menu", "power", "volume_up", etc.
///
/// # Returns
/// MCP response confirming button press, or error if button not recognized
pub fn handle_press_button(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    button: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.press_button(device_id, platform, button) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Pressed button '{}': {}", button, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to press button: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, button);
        Err("Not available in extension mode".to_string())
    }
}

// ============================================================================
// App Management Handlers
// ============================================================================
//
// These handlers manage app lifecycle: launching, terminating, installing,
// and uninstalling applications. They require app package names/bundle IDs
// which can be discovered using handle_list_apps.

/// Launch an application
///
/// Starts an app by its package name (Android: com.android.chrome) or
/// bundle ID (iOS: com.apple.mobilesafari). Also accepts common names
/// like "chrome", "youtube" which are mapped to package names internally.
/// The app opens to its main activity/view.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `app_id` - App package name, bundle ID, or common name
///
/// # Returns
/// MCP response confirming app launch, or error if app not found
pub fn handle_launch_app(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    app_id: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.launch_app(device_id, platform, app_id) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Launched app '{}': {}", app_id, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to launch app: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, app_id);
        Err("Not available in extension mode".to_string())
    }
}

/// Force-stop a running application
///
/// Forcefully terminates an app, killing all its processes. Like force-quit
/// on desktop. The app must be relaunched to use again. Useful for resetting
/// app state or stopping misbehaving apps.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `app_id` - App package name or bundle ID
///
/// # Returns
/// MCP response confirming termination, or error if app not running
pub fn handle_terminate_app(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    app_id: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.terminate_app(device_id, platform, app_id) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Terminated app '{}': {}", app_id, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to terminate app: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, app_id);
        Err("Not available in extension mode".to_string())
    }
}

/// Install app from package file
///
/// Installs an app from local APK (Android) or IPA (iOS) file. The file
/// must be accessible on the machine running the MCP server. For Android,
/// uses 'adb install'. For iOS, requires developer provisioning/signing.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `app_path` - Local file path to APK or IPA file
///
/// # Returns
/// MCP response confirming installation, or error if file not found/invalid
pub fn handle_install_app(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    app_path: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.install_app(device_id, platform, app_path) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Installed app from '{}': {}", app_path, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to install app: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, app_path);
        Err("Not available in extension mode".to_string())
    }
}

/// Remove an installed application
///
/// Completely uninstalls an app from device, removing all its data.
/// This action is permanent - the app must be reinstalled to use again.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `app_id` - App package name or bundle ID
///
/// # Returns
/// MCP response confirming uninstallation, or error if app not installed
pub fn handle_uninstall_app(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    app_id: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.uninstall_app(device_id, platform, app_id) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Uninstalled app '{}': {}", app_id, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to uninstall app: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, app_id);
        Err("Not available in extension mode".to_string())
    }
}

// ============================================================================
// Navigation Handlers
// ============================================================================
//
// These handlers manage URL navigation and device orientation control.

/// Open URL in default browser
///
/// Launches the device's default web browser and navigates to the URL.
/// Supports http://, https://, and app deep links. Perfect for web-based
/// testing or opening web content during automation.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `url` - URL to open (must include protocol: http:// or https://)
///
/// # Returns
/// MCP response confirming URL opened, or error if browser launch fails
pub fn handle_open_url(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    url: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.open_url(device_id, platform, url) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Opened URL '{}': {}", url, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to open URL: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, url);
        Err("Not available in extension mode".to_string())
    }
}

/// Change device screen orientation
///
/// Rotates the device display between portrait and landscape modes.
/// This physically changes how the screen is oriented (not just CSS).
/// Useful for testing responsive layouts or orientation-specific features.
///
/// # Arguments
/// * `manager` - Mobile device manager
/// * `device_id` - Device identifier
/// * `platform` - Platform: "android" or "ios"
/// * `orientation` - Target orientation: "portrait" or "landscape"
///
/// # Returns
/// MCP response confirming orientation change, or error if not supported
pub fn handle_set_orientation(
    manager: &mut MobileDeviceManager,
    device_id: &str,
    platform: &str,
    orientation: &str,
) -> HandlerResult {
    #[cfg(feature = "native-binary")]
    {
        match manager.set_orientation(device_id, platform, orientation) {
            Ok(msg) => Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!("Set orientation to '{}': {}", orientation, msg)
                }]
            })),
            Err(e) => Err(format!("Failed to set orientation: {}", e)),
        }
    }
    #[cfg(not(feature = "native-binary"))]
    {
        let _ = (manager, device_id, platform, orientation);
        Err("Not available in extension mode".to_string())
    }
}
