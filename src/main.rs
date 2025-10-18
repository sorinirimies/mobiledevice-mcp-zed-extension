// mobile-mcp-zed-extension/src/main.rs
// Mobile Device MCP Server - Clean and Modular Implementation

#![allow(unused_imports)]

use std::io::{self, BufRead, Write};

mod devices;
mod mcp;
mod tools;
mod types;

use crate::types::DeviceInfo;
use devices::{AndroidDeviceManager, IOSDeviceManager};
use mcp::{InitializeResult, McpErrorResponse, McpRequest, McpResponse, ToolCallParams};
use tools::{get_all_tools, handlers};
use types::MobileDeviceMcpSettings;

// ============================================================================
// MobileDeviceManager - Consolidated device management
// ============================================================================

pub struct MobileDeviceManager {
    android_manager: AndroidDeviceManager,
    ios_manager: IOSDeviceManager,
}
impl MobileDeviceManager {
    pub fn new(debug: bool) -> Self {
        Self {
            android_manager: AndroidDeviceManager::new(debug),
            ios_manager: IOSDeviceManager::new(debug),
        }
    }

    pub fn list_all_devices(&mut self, platform: &str) -> Vec<DeviceInfo> {
        let mut all_devices = Vec::new();

        match platform {
            "android" => {
                if let Ok(devices) = self.android_manager.list_devices() {
                    all_devices.extend(devices);
                }
            }
            "ios" => {
                if let Ok(devices) = self.ios_manager.list_devices() {
                    all_devices.extend(devices);
                }
            }
            _ => {
                if let Ok(devices) = self.android_manager.list_devices() {
                    all_devices.extend(devices);
                }
                if let Ok(devices) = self.ios_manager.list_devices() {
                    all_devices.extend(devices);
                }
            }
        }

        all_devices
    }

    // Delegate to handlers - these are just thin wrappers
    pub fn take_screenshot(&mut self, device_id: &str, platform: &str) -> Result<Vec<u8>, String> {
        match platform {
            "android" => self.android_manager.take_screenshot(device_id),
            "ios" => self.ios_manager.take_screenshot(device_id),
            _ => Err(format!("Unknown platform: {}", platform)),
        }
    }

    pub fn tap_screen(
        &mut self,
        device_id: &str,
        platform: &str,
        x: f64,
        y: f64,
    ) -> Result<String, String> {
        match platform {
            "android" => self.android_manager.tap_screen(device_id, x, y),
            "ios" => self.ios_manager.tap_screen(device_id, x, y),
            _ => Err(format!("Unknown platform: {}", platform)),
        }
    }

    pub fn type_text(
        &mut self,
        device_id: &str,
        platform: &str,
        text: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => self.android_manager.type_text(device_id, text),
            "ios" => self.ios_manager.type_text(device_id, text),
            _ => Err(format!("Unknown platform: {}", platform)),
        }
    }

    // Additional methods using AndroidRobot
    pub fn get_screen_size(
        &mut self,
        device_id: &str,
        platform: &str,
    ) -> Result<(u32, u32), String> {
        match platform {
            "android" => {
                use devices::android::{AndroidRobot, ScreenSize};
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.get_screen_size().map(|s| (s.width, s.height))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn get_orientation(&mut self, device_id: &str, platform: &str) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot
                    .get_orientation()
                    .map(|o| format!("{:?}", o).to_lowercase())
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn set_orientation(
        &mut self,
        device_id: &str,
        platform: &str,
        orientation: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                use devices::android::Orientation;
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                let orient = match orientation {
                    "portrait" => Orientation::Portrait,
                    "landscape" => Orientation::Landscape,
                    _ => return Err(format!("Invalid orientation: {}", orientation)),
                };
                robot.set_orientation(orient)?;
                Ok(format!("Set orientation to {}", orientation))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn double_tap_screen(
        &mut self,
        device_id: &str,
        platform: &str,
        x: f64,
        y: f64,
    ) -> Result<String, String> {
        self.tap_screen(device_id, platform, x, y)?;
        std::thread::sleep(std::time::Duration::from_millis(50));
        self.tap_screen(device_id, platform, x, y)?;
        Ok("Double tap executed".to_string())
    }

    pub fn long_press_screen(
        &mut self,
        device_id: &str,
        platform: &str,
        x: f64,
        y: f64,
        _duration: u32,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.long_press(x as u32, y as u32)?;
                Ok(format!("Long pressed at ({}, {})", x, y))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn swipe_screen(
        &mut self,
        device_id: &str,
        platform: &str,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        duration: u32,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.swipe_coordinates(
                    start_x as u32,
                    start_y as u32,
                    end_x as u32,
                    end_y as u32,
                    duration,
                )?;
                Ok("Swipe executed".to_string())
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn press_button(
        &mut self,
        device_id: &str,
        platform: &str,
        button: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                use devices::android::Button;
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                let btn = match button.to_lowercase().as_str() {
                    "home" => Button::Home,
                    "back" => Button::Back,
                    "menu" => Button::Menu,
                    "power" => Button::Power,
                    "volume_up" => Button::VolumeUp,
                    "volume_down" => Button::VolumeDown,
                    "camera" => Button::Camera,
                    "enter" => Button::Enter,
                    "search" => Button::Back,     // Fallback
                    "app_switch" => Button::Menu, // Fallback
                    _ => return Err(format!("Unknown button: {}", button)),
                };
                robot.press_button(btn)?;
                Ok(format!("Pressed button: {}", button))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn list_apps(
        &mut self,
        device_id: &str,
        platform: &str,
    ) -> Result<Vec<devices::android::InstalledApp>, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.list_installed_apps()
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn list_elements_on_screen(
        &mut self,
        device_id: &str,
        platform: &str,
        filter: Option<&str>,
    ) -> Result<Vec<devices::android::ScreenElement>, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.list_screen_elements(filter)
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn launch_app(
        &mut self,
        device_id: &str,
        platform: &str,
        app_id: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.launch_app(app_id)?;
                Ok(format!("Launched app: {}", app_id))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn terminate_app(
        &mut self,
        device_id: &str,
        platform: &str,
        app_id: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.terminate_app(app_id)?;
                Ok(format!("Terminated app: {}", app_id))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn install_app(
        &mut self,
        device_id: &str,
        platform: &str,
        app_path: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.install_app(app_path)?;
                Ok(format!("Installed app from: {}", app_path))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn uninstall_app(
        &mut self,
        device_id: &str,
        platform: &str,
        app_id: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.uninstall_app(app_id)?;
                Ok(format!("Uninstalled app: {}", app_id))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }

    pub fn open_url(
        &mut self,
        device_id: &str,
        platform: &str,
        url: &str,
    ) -> Result<String, String> {
        match platform {
            "android" => {
                let mut robot = self.android_manager.create_robot(device_id.to_string());
                robot.open_url(url)?;
                Ok(format!("Opened URL: {}", url))
            }
            _ => Err("Not implemented for this platform".to_string()),
        }
    }
}

// ============================================================================
// MCP Server Implementation
// ============================================================================

struct McpServer {
    manager: MobileDeviceManager,
    settings: MobileDeviceMcpSettings,
}

impl McpServer {
    fn new(settings: MobileDeviceMcpSettings) -> Self {
        Self {
            manager: MobileDeviceManager::new(settings.debug),
            settings,
        }
    }

    fn send_response(&self, id: serde_json::Value, result: serde_json::Value) {
        let response = McpResponse::success(id, result);
        if let Ok(json) = response.to_json() {
            println!("{}", json);
        }
    }

    fn send_error(&self, id: serde_json::Value, message: &str) {
        let error = McpErrorResponse::error(id, -1, message.to_string());
        if let Ok(json) = error.to_json() {
            println!("{}", json);
        }
    }

    fn handle_initialize(&self, id: serde_json::Value) {
        let result = InitializeResult::new();
        self.send_response(id, serde_json::to_value(result).unwrap());
    }

    fn handle_tools_list(&self, id: serde_json::Value) {
        let tools: Vec<_> = get_all_tools().iter().map(|t| t.to_json()).collect();
        self.send_response(id, serde_json::json!({ "tools": tools }));
    }

    fn handle_tool_call(&mut self, id: serde_json::Value, params: ToolCallParams) {
        let result = self.dispatch_tool(&params.name, params.arguments);

        match result {
            Ok(response) => self.send_response(id, response),
            Err(e) => self.send_error(id, &e),
        }
    }

    fn dispatch_tool(
        &mut self,
        tool_name: &str,
        args: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        // Extract common parameters
        let device_id = args.get("device_id").and_then(|v| v.as_str()).unwrap_or("");
        let platform = args
            .get("platform")
            .and_then(|v| v.as_str())
            .unwrap_or(&self.settings.platform);

        match tool_name {
            // Device Info
            "mobile_device_mcp_list_available_devices" => {
                handlers::handle_list_devices(&mut self.manager, platform)
            }
            "mobile_device_mcp_get_screen_size" => {
                handlers::handle_get_screen_size(&mut self.manager, device_id, platform)
            }
            "mobile_device_mcp_get_orientation" => {
                handlers::handle_get_orientation(&mut self.manager, device_id, platform)
            }
            "mobile_device_mcp_list_apps" => {
                handlers::handle_list_apps(&mut self.manager, device_id, platform)
            }
            "mobile_device_mcp_list_elements_on_screen" => {
                let filter = args.get("filter").and_then(|v| v.as_str());
                handlers::handle_list_elements(&mut self.manager, device_id, platform, filter)
            }

            // Screen Interaction
            "mobile_device_mcp_take_screenshot" => {
                handlers::handle_take_screenshot(&mut self.manager, device_id, platform)
            }
            "mobile_device_mcp_save_screenshot" => {
                let output = args
                    .get("output_path")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing output_path")?;
                handlers::handle_save_screenshot(&mut self.manager, device_id, platform, output)
            }
            "mobile_device_mcp_click_on_screen_at_coordinates" => {
                let x = args
                    .get("x")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing x coordinate")?;
                let y = args
                    .get("y")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing y coordinate")?;
                handlers::handle_click_screen(&mut self.manager, device_id, platform, x, y)
            }
            "mobile_device_mcp_double_tap_on_screen" => {
                let x = args
                    .get("x")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing x coordinate")?;
                let y = args
                    .get("y")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing y coordinate")?;
                handlers::handle_double_tap(&mut self.manager, device_id, platform, x, y)
            }
            "mobile_device_mcp_long_press_on_screen_at_coordinates" => {
                let x = args
                    .get("x")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing x coordinate")?;
                let y = args
                    .get("y")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing y coordinate")?;
                let duration = args
                    .get("duration")
                    .and_then(|v| v.as_u64())
                    .map(|d| d as u32);
                handlers::handle_long_press(&mut self.manager, device_id, platform, x, y, duration)
            }
            "mobile_device_mcp_swipe_on_screen" => {
                let start_x = args
                    .get("start_x")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing start_x")?;
                let start_y = args
                    .get("start_y")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing start_y")?;
                let end_x = args
                    .get("end_x")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing end_x")?;
                let end_y = args
                    .get("end_y")
                    .and_then(|v| v.as_f64())
                    .ok_or("Missing end_y")?;
                let duration = args
                    .get("duration")
                    .and_then(|v| v.as_u64())
                    .map(|d| d as u32);
                handlers::handle_swipe(
                    &mut self.manager,
                    device_id,
                    platform,
                    start_x,
                    start_y,
                    end_x,
                    end_y,
                    duration,
                )
            }

            // Input
            "mobile_device_mcp_type_keys" => {
                let text = args
                    .get("text")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing text")?;
                handlers::handle_type_keys(&mut self.manager, device_id, platform, text)
            }
            "mobile_device_mcp_press_button" => {
                let button = args
                    .get("button")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing button")?;
                handlers::handle_press_button(&mut self.manager, device_id, platform, button)
            }

            // App Management
            "mobile_device_mcp_launch_app" => {
                let app_id = args
                    .get("app_id")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing app_id")?;
                handlers::handle_launch_app(&mut self.manager, device_id, platform, app_id)
            }
            "mobile_device_mcp_terminate_app" => {
                let app_id = args
                    .get("app_id")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing app_id")?;
                handlers::handle_terminate_app(&mut self.manager, device_id, platform, app_id)
            }
            "mobile_device_mcp_install_app" => {
                let app_path = args
                    .get("app_path")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing app_path")?;
                handlers::handle_install_app(&mut self.manager, device_id, platform, app_path)
            }
            "mobile_device_mcp_uninstall_app" => {
                let app_id = args
                    .get("app_id")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing app_id")?;
                handlers::handle_uninstall_app(&mut self.manager, device_id, platform, app_id)
            }

            // Navigation
            "mobile_device_mcp_open_url" => {
                let url = args
                    .get("url")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing url")?;
                handlers::handle_open_url(&mut self.manager, device_id, platform, url)
            }
            "mobile_device_mcp_set_orientation" => {
                let orientation = args
                    .get("orientation")
                    .and_then(|v| v.as_str())
                    .ok_or("Missing orientation")?;
                handlers::handle_set_orientation(
                    &mut self.manager,
                    device_id,
                    platform,
                    orientation,
                )
            }

            _ => Err(format!("Unknown tool: {}", tool_name)),
        }
    }

    fn run(&mut self) {
        let stdin = io::stdin();
        let reader = stdin.lock();

        for line in reader.lines() {
            let line = match line {
                Ok(l) => l,
                Err(_) => continue,
            };

            if line.trim().is_empty() {
                continue;
            }

            let request = match McpRequest::from_json(&line) {
                Ok(req) => req,
                Err(e) => {
                    eprintln!("Failed to parse request: {}", e);
                    continue;
                }
            };

            let id = request.id.unwrap_or(serde_json::Value::Null);

            match request.method.as_str() {
                "initialize" => self.handle_initialize(id),
                "tools/list" => self.handle_tools_list(id),
                "tools/call" => {
                    if let Some(params) = request.params {
                        match serde_json::from_value::<ToolCallParams>(params) {
                            Ok(tool_call) => self.handle_tool_call(id, tool_call),
                            Err(e) => self.send_error(id, &format!("Invalid params: {}", e)),
                        }
                    } else {
                        self.send_error(id, "Missing params for tools/call");
                    }
                }
                _ => self.send_error(id, &format!("Unknown method: {}", request.method)),
            }
        }
    }
}

// ============================================================================
// Main Entry Point
// ============================================================================

fn main() {
    let settings = MobileDeviceMcpSettings {
        debug: std::env::var("MOBILE_DEVICE_MCP_DEBUG").is_ok(),
        platform: std::env::var("MOBILE_PLATFORM").unwrap_or_else(|_| "auto".to_string()),
    };

    let mut server = McpServer::new(settings);
    server.run();
}
