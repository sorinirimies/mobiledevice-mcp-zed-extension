// mobile-mcp-zed-extension/src/tools/definitions.rs
// MCP Tool Definitions for Mobile Device Automation
//
// This module defines all available mobile device automation tools following
// the Model Context Protocol (MCP) specification. Each tool has:
// - A unique name (snake_case)
// - A human-readable description
// - A JSON schema defining its input parameters
//
// Tools are organized into categories:
// - Device Information: Query device state and capabilities
// - Screen Interaction: Visual actions (screenshot, tap, swipe)
// - Input: Text and button input
// - App Management: Install, launch, terminate apps
// - Navigation: URL opening, orientation control

use serde_json::{json, Value};

/// Represents an MCP tool with its schema
///
/// This structure encapsulates everything needed to define a tool in the
/// Model Context Protocol. The input_schema follows JSON Schema specification
/// and defines what parameters the tool accepts.
#[derive(Debug, Clone)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

impl ToolDefinition {
    /// Create a new tool definition
    ///
    /// # Arguments
    /// * `name` - Unique tool identifier (e.g., "mobile_device_mcp_take_screenshot")
    /// * `description` - Human-readable description of what the tool does
    /// * `input_schema` - JSON Schema object defining input parameters
    pub fn new(name: &str, description: &str, input_schema: Value) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            input_schema,
        }
    }

    /// Convert tool definition to MCP-compliant JSON format
    pub fn to_json(&self) -> Value {
        json!({
            "name": self.name,
            "description": self.description,
            "inputSchema": self.input_schema
        })
    }
}

/// Returns all available mobile device tools
///
/// This is the main entry point for tool discovery. Returns a complete list
/// of all 20+ tools available for mobile device automation. Tools are returned
/// in a logical order by category for better organization.
///
/// # Returns
/// A vector of all tool definitions ready to be sent to MCP clients
pub fn get_all_tools() -> Vec<ToolDefinition> {
    vec![
        // Device Information Tools
        tool_list_available_devices(),
        tool_get_screen_size(),
        tool_get_orientation(),
        tool_list_apps(),
        tool_list_elements_on_screen(),
        // Screen Interaction Tools
        tool_take_screenshot(),
        tool_save_screenshot(),
        tool_click_on_screen(),
        tool_double_tap_on_screen(),
        tool_long_press_on_screen(),
        tool_swipe_on_screen(),
        // Input Tools
        tool_type_keys(),
        tool_press_button(),
        // App Management Tools
        tool_launch_app(),
        tool_terminate_app(),
        tool_install_app(),
        tool_uninstall_app(),
        // Navigation Tools
        tool_open_url(),
        tool_set_orientation(),
    ]
}

// ============================================================================
// Device Information Tools
// ============================================================================
//
// These tools provide information about connected devices and their current state.
// They're typically the first tools to use when starting device automation,
// as they help you discover what devices are available and their capabilities.

/// List all connected mobile devices
///
/// This is typically the first tool to call - it discovers all Android devices
/// (physical + emulators) and iOS devices (simulators + physical on macOS).
/// No parameters required, making it perfect for device discovery.
fn tool_list_available_devices() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_list_available_devices",
        "List all available mobile devices and simulators. This includes both physical devices and emulators for Android and iOS platforms.",
        json!({
            "type": "object",
            "properties": {},
            "required": []
        })
    )
}

/// Get device screen dimensions
///
/// Returns the actual screen size in pixels. Useful for calculating tap coordinates
/// or determining if elements are visible on screen.
fn tool_get_screen_size() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_get_screen_size",
        "Get the screen size of the mobile device in pixels. Returns width and height.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier (e.g., 'emulator-5554' for Android or device UDID for iOS)"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                }
            },
            "required": ["device_id", "platform"]
        }),
    )
}

/// Query current device orientation
///
/// Determines if the device is in portrait or landscape mode.
/// Useful before performing UI actions that depend on screen layout.
fn tool_get_orientation() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_get_orientation",
        "Get the current screen orientation of the device. Returns 'portrait' or 'landscape'.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                }
            },
            "required": ["device_id", "platform"]
        }),
    )
}

/// List all installed applications
///
/// Returns a list of all user-installed apps with their package identifiers
/// and display names. Useful for discovering app package names before launching.
fn tool_list_apps() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_list_apps",
        "List all the installed apps on the device. Returns app package names and labels.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                }
            },
            "required": ["device_id", "platform"]
        }),
    )
}

/// List UI elements currently visible on screen
///
/// Performs UI hierarchy inspection to find all visible elements, their positions,
/// and properties. Optional filter allows searching for specific elements by text
/// or resource ID. This is powerful for finding exact tap coordinates.
fn tool_list_elements_on_screen() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_list_elements_on_screen",
        "List elements on screen and their coordinates, with optional filtering. Returns UI elements with their bounds and properties.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "filter": {
                    "type": "string",
                    "description": "Optional filter to search for specific elements (e.g., text content, resource ID)"
                }
            },
            "required": ["device_id", "platform"]
        })
    )
}

// ============================================================================
// Screen Interaction Tools
// ============================================================================
//
// These tools handle visual interactions with the device screen. They include
// screenshot capture and various touch gestures (tap, swipe, long press).
// These are the most commonly used tools for UI automation.

/// Capture current screen as image
///
/// Takes a screenshot and returns it as base64-encoded PNG data in the MCP response.
/// This is the primary way to "see" what's on the device screen. The image data
/// can be displayed in AI assistants that support image content.
fn tool_take_screenshot() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_take_screenshot",
        "Take a screenshot of the mobile device. Use this to understand the current state of the screen. Returns the screenshot as base64-encoded PNG image data.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                }
            },
            "required": ["device_id", "platform"]
        })
    )
}

/// Save screenshot to file system
///
/// Like take_screenshot, but saves directly to a file path instead of returning
/// base64 data. Useful for creating test artifacts, documentation, or when you
/// need to store many screenshots efficiently.
fn tool_save_screenshot() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_save_screenshot",
        "Save a screenshot of the mobile device to a file. Useful for creating test artifacts or documentation.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "output_path": {
                    "type": "string",
                    "description": "Path where the screenshot should be saved (e.g., '/tmp/screenshot.png')"
                }
            },
            "required": ["device_id", "platform", "output_path"]
        })
    )
}

/// Perform a single tap at coordinates
///
/// The most common interaction tool. Simulates a finger tap at the specified
/// pixel coordinates. Use list_elements_on_screen or take_screenshot first to
/// determine where to tap. Coordinates are absolute pixels from top-left (0,0).
fn tool_click_on_screen() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_click_on_screen_at_coordinates",
        "Click on the screen at given x,y coordinates. Use this to tap buttons, links, or any interactive elements.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "x": {
                    "type": "number",
                    "description": "X coordinate in pixels"
                },
                "y": {
                    "type": "number",
                    "description": "Y coordinate in pixels"
                }
            },
            "required": ["device_id", "platform", "x", "y"]
        })
    )
}

/// Perform a double-tap gesture
///
/// Executes two rapid taps at the same location. Common use cases include
/// zooming in on images, selecting text, or triggering double-tap gestures
/// in apps that support them.
fn tool_double_tap_on_screen() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_double_tap_on_screen",
        "Double-tap on the screen at given x,y coordinates. Useful for zoom or activation gestures.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "x": {
                    "type": "number",
                    "description": "X coordinate in pixels"
                },
                "y": {
                    "type": "number",
                    "description": "Y coordinate in pixels"
                }
            },
            "required": ["device_id", "platform", "x", "y"]
        })
    )
}

/// Perform a long press (press and hold)
///
/// Holds the touch at coordinates for a specified duration (default 1000ms).
/// Triggers long-press actions like context menus, drag-to-reorder, or
/// selection mode in list items.
fn tool_long_press_on_screen() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_long_press_on_screen_at_coordinates",
        "Long press on the screen at given x,y coordinates. Useful for context menus or drag operations.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "x": {
                    "type": "number",
                    "description": "X coordinate in pixels"
                },
                "y": {
                    "type": "number",
                    "description": "Y coordinate in pixels"
                },
                "duration": {
                    "type": "number",
                    "description": "Duration of long press in milliseconds (default: 1000)",
                    "default": 1000
                }
            },
            "required": ["device_id", "platform", "x", "y"]
        })
    )
}

/// Perform a swipe gesture
///
/// Simulates dragging a finger from start point to end point over a duration.
/// Essential for scrolling lists, switching between tabs/pages, pulling down
/// to refresh, or any drag gesture. Duration affects swipe speed.
fn tool_swipe_on_screen() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_swipe_on_screen",
        "Swipe on the screen from start coordinates to end coordinates. Useful for scrolling or gesture navigation.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "start_x": {
                    "type": "number",
                    "description": "Starting X coordinate in pixels"
                },
                "start_y": {
                    "type": "number",
                    "description": "Starting Y coordinate in pixels"
                },
                "end_x": {
                    "type": "number",
                    "description": "Ending X coordinate in pixels"
                },
                "end_y": {
                    "type": "number",
                    "description": "Ending Y coordinate in pixels"
                },
                "duration": {
                    "type": "number",
                    "description": "Duration of swipe in milliseconds (default: 300)",
                    "default": 300
                }
            },
            "required": ["device_id", "platform", "start_x", "start_y", "end_x", "end_y"]
        })
    )
}

// ============================================================================
// Input Tools
// ============================================================================
//
// Tools for text input and button presses. These simulate keyboard typing
// and hardware/software button presses (home, back, volume, etc.).

/// Type text into focused input field
///
/// Sends text input to whatever element currently has focus (must be an input
/// field, text area, or similar). Tap on an input field first before typing.
/// Special characters and emojis supported depending on platform.
fn tool_type_keys() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_type_keys",
        "Type text into the focused element. Use this to enter text in input fields, search boxes, etc.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "text": {
                    "type": "string",
                    "description": "Text to type"
                }
            },
            "required": ["device_id", "platform", "text"]
        })
    )
}

/// Press hardware or navigation button
///
/// Simulates pressing physical buttons (volume, power) or navigation buttons
/// (home, back). These are system-level actions that work regardless of what
/// app is currently running. Essential for navigation and system control.
fn tool_press_button() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_press_button",
        "Press a hardware or software button on device. Common buttons: home, back, menu, power, volume_up, volume_down, camera, enter.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "button": {
                    "type": "string",
                    "description": "Button name: home, back, menu, power, volume_up, volume_down, camera, enter, etc.",
                    "enum": ["home", "back", "menu", "power", "volume_up", "volume_down", "camera", "enter", "search", "app_switch"]
                }
            },
            "required": ["device_id", "platform", "button"]
        })
    )
}

// ============================================================================
// App Management Tools
// ============================================================================
//
// Tools for managing app lifecycle: launching, terminating, installing, and
// uninstalling applications. These require knowing app package names/bundle IDs,
// which can be discovered using the list_apps tool.

/// Launch an application
///
/// Starts an app by its package name (Android: com.example.app) or bundle ID
/// (iOS: com.example.app). Also accepts common app names ('chrome', 'youtube')
/// which are mapped to their package names. The app will open to its main activity.
fn tool_launch_app() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_launch_app",
        "Launch an app on mobile device. Use this to open a specific app. You can provide either the package name (Android) or bundle ID (iOS), or a common app name like 'chrome', 'youtube', etc.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "app_id": {
                    "type": "string",
                    "description": "App package name (Android: com.example.app) or bundle ID (iOS: com.example.app), or common name (chrome, youtube, settings, etc.)"
                }
            },
            "required": ["device_id", "platform", "app_id"]
        })
    )
}

/// Force-stop a running application
///
/// Forcefully terminates an app, killing all its processes. This is like
/// force-quit on desktop. The app will need to be relaunched to use again.
/// Use this to reset app state or stop misbehaving apps.
fn tool_terminate_app() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_terminate_app",
        "Stop and terminate an app on mobile device. Forces the app to close.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "app_id": {
                    "type": "string",
                    "description": "App package name (Android) or bundle ID (iOS)"
                }
            },
            "required": ["device_id", "platform", "app_id"]
        }),
    )
}

/// Install app from package file
///
/// Installs an app from a local APK (Android) or IPA (iOS) file. The file
/// must be accessible on the machine running the MCP server. For Android,
/// this uses 'adb install'. For iOS, requires developer provisioning.
fn tool_install_app() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_install_app",
        "Install an app on mobile device from a local APK file (Android) or IPA file (iOS).",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "app_path": {
                    "type": "string",
                    "description": "Path to APK file (Android) or IPA file (iOS)"
                }
            },
            "required": ["device_id", "platform", "app_path"]
        }),
    )
}

/// Remove an installed application
///
/// Completely uninstalls an app from the device, removing all its data.
/// Requires the app's package name (Android) or bundle ID (iOS). This action
/// is permanent - the app must be reinstalled to use again.
fn tool_uninstall_app() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_uninstall_app",
        "Uninstall an app from mobile device. Removes the app completely from the device.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "app_id": {
                    "type": "string",
                    "description": "App package name (Android) or bundle ID (iOS)"
                }
            },
            "required": ["device_id", "platform", "app_id"]
        }),
    )
}

// ============================================================================
// Navigation Tools
// ============================================================================
//
// Tools for URL navigation and device orientation control. These handle
// browser-based navigation and physical device rotation/orientation changes.

/// Open URL in default browser
///
/// Launches the device's default web browser and navigates to the URL.
/// Supports http://, https://, and app deep links. This is perfect for
/// web-based testing or opening web content from your automation.
fn tool_open_url() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_open_url",
        "Open a URL in browser on device. This will launch the default browser and navigate to the specified URL.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "url": {
                    "type": "string",
                    "description": "URL to open (must include http:// or https://)"
                }
            },
            "required": ["device_id", "platform", "url"]
        })
    )
}

/// Change device screen orientation
///
/// Rotates the device display between portrait and landscape modes. This
/// physically changes how the screen is oriented, useful for testing
/// responsive layouts or apps with orientation-specific features.
fn tool_set_orientation() -> ToolDefinition {
    ToolDefinition::new(
        "mobile_device_mcp_set_orientation",
        "Change the screen orientation of the device. Sets the device to portrait or landscape mode.",
        json!({
            "type": "object",
            "properties": {
                "device_id": {
                    "type": "string",
                    "description": "Device identifier"
                },
                "platform": {
                    "type": "string",
                    "description": "Platform: 'android' or 'ios'",
                    "enum": ["android", "ios"]
                },
                "orientation": {
                    "type": "string",
                    "description": "Target orientation",
                    "enum": ["portrait", "landscape"]
                }
            },
            "required": ["device_id", "platform", "orientation"]
        })
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_tools_count() {
        let tools = get_all_tools();
        assert_eq!(tools.len(), 19, "Should have exactly 19 tools");
    }

    #[test]
    fn test_all_tools_have_names() {
        let tools = get_all_tools();
        for tool in tools {
            assert!(!tool.name.is_empty(), "Tool name should not be empty");
            assert!(
                tool.name.starts_with("mobile_device_mcp_"),
                "Tool name should start with mobile_device_mcp_ prefix"
            );
        }
    }

    #[test]
    fn test_all_tools_have_descriptions() {
        let tools = get_all_tools();
        for tool in tools {
            assert!(
                !tool.description.is_empty(),
                "Tool {} should have a description",
                tool.name
            );
        }
    }

    #[test]
    fn test_all_tools_have_input_schemas() {
        let tools = get_all_tools();
        for tool in tools {
            assert!(
                tool.input_schema.is_object(),
                "Tool {} should have an object input schema",
                tool.name
            );
        }
    }

    #[test]
    fn test_tool_names_unique() {
        let tools = get_all_tools();
        let mut names = std::collections::HashSet::new();
        for tool in tools {
            assert!(
                names.insert(tool.name.clone()),
                "Tool name {} should be unique",
                tool.name
            );
        }
    }

    #[test]
    fn test_tool_definition_to_json() {
        let tool = ToolDefinition::new(
            "test_tool",
            "Test description",
            serde_json::json!({"type": "object"}),
        );

        let json = tool.to_json();
        assert_eq!(json["name"], "test_tool");
        assert_eq!(json["description"], "Test description");
        assert!(json["inputSchema"].is_object());
    }

    #[test]
    fn test_list_devices_tool() {
        let tools = get_all_tools();
        let list_devices = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_list_available_devices")
            .expect("Should have list_available_devices tool");

        assert!(list_devices.description.contains("available"));
    }

    #[test]
    fn test_screenshot_tool() {
        let tools = get_all_tools();
        let screenshot = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_take_screenshot")
            .expect("Should have take_screenshot tool");

        let schema = &screenshot.input_schema;
        assert!(schema["properties"]["device_id"].is_object());
        assert!(schema["properties"]["platform"].is_object());
    }

    #[test]
    fn test_tap_tool() {
        let tools = get_all_tools();
        let tap = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_click_on_screen_at_coordinates")
            .expect("Should have tap tool");

        let schema = &tap.input_schema;
        assert!(schema["properties"]["x"].is_object());
        assert!(schema["properties"]["y"].is_object());
        assert_eq!(schema["properties"]["x"]["type"], "number");
        assert_eq!(schema["properties"]["y"]["type"], "number");
    }

    #[test]
    fn test_type_text_tool() {
        let tools = get_all_tools();
        let type_text = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_type_keys")
            .expect("Should have type_keys tool");

        let schema = &type_text.input_schema;
        assert!(schema["properties"]["text"].is_object());
        assert_eq!(schema["properties"]["text"]["type"], "string");
    }

    #[test]
    fn test_swipe_tool() {
        let tools = get_all_tools();
        let swipe = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_swipe_on_screen")
            .expect("Should have swipe tool");

        let schema = &swipe.input_schema;
        assert!(schema["properties"]["start_x"].is_object());
        assert!(schema["properties"]["start_y"].is_object());
        assert!(schema["properties"]["end_x"].is_object());
        assert!(schema["properties"]["end_y"].is_object());
    }

    #[test]
    fn test_launch_app_tool() {
        let tools = get_all_tools();
        let launch_app = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_launch_app")
            .expect("Should have launch_app tool");

        let schema = &launch_app.input_schema;
        assert!(schema["properties"]["app_id"].is_object());
    }

    #[test]
    fn test_required_fields() {
        let tools = get_all_tools();
        for tool in tools {
            if tool.input_schema["required"].is_array() {
                let required = tool.input_schema["required"].as_array().unwrap();
                for field in required {
                    let field_name = field.as_str().unwrap();
                    assert!(
                        tool.input_schema["properties"][field_name].is_object(),
                        "Required field {} should be in properties for tool {}",
                        field_name,
                        tool.name
                    );
                }
            }
        }
    }

    #[test]
    fn test_platform_enum_values() {
        let tools = get_all_tools();
        for tool in tools {
            if let Some(platform_prop) = tool.input_schema["properties"]["platform"].as_object() {
                if let Some(enum_values) = platform_prop.get("enum") {
                    let enums = enum_values.as_array().unwrap();
                    assert!(enums.contains(&serde_json::json!("android")));
                    assert!(enums.contains(&serde_json::json!("ios")));
                }
            }
        }
    }

    #[test]
    fn test_orientation_enum_values() {
        let tools = get_all_tools();
        let set_orientation = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_set_orientation")
            .expect("Should have set_orientation tool");

        let enum_values = set_orientation.input_schema["properties"]["orientation"]["enum"]
            .as_array()
            .unwrap();
        assert!(enum_values.contains(&serde_json::json!("portrait")));
        assert!(enum_values.contains(&serde_json::json!("landscape")));
    }

    #[test]
    fn test_button_enum_values() {
        let tools = get_all_tools();
        let press_button = tools
            .iter()
            .find(|t| t.name == "mobile_device_mcp_press_button")
            .expect("Should have press_button tool");

        let enum_values = press_button.input_schema["properties"]["button"]["enum"]
            .as_array()
            .unwrap();
        assert!(enum_values.contains(&serde_json::json!("home")));
        assert!(enum_values.contains(&serde_json::json!("back")));
        assert!(enum_values.contains(&serde_json::json!("power")));
    }

    #[test]
    fn test_all_tools_serializable() {
        let tools = get_all_tools();
        for tool in tools {
            let json = serde_json::to_string(&tool.to_json());
            assert!(json.is_ok(), "Tool {} should be serializable", tool.name);
        }
    }
}
