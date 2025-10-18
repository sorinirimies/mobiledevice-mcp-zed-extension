// mobile-mcp-zed-extension/src/devices/android.rs
// Comprehensive Android Device Management Module with mobile-mcp features

use crate::types::DeviceInfo;
use adb_client::{ADBDeviceExt, ADBServer, DeviceState};
use quick_xml::events::Event;
use quick_xml::Reader as XmlReader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct AndroidDevice {
    pub device_id: String,
    pub device_type: AndroidDeviceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum AndroidDeviceType {
    Mobile,
    TV,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct InstalledApp {
    pub package_name: String,
    pub app_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ScreenElement {
    pub element_type: String,
    pub text: Option<String>,
    pub label: String,
    pub rect: ScreenElementRect,
    pub focused: Option<bool>,
    pub identifier: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ScreenElementRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ScreenSize {
    pub width: u32,
    pub height: u32,
    pub scale: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Button {
    Back,
    Home,
    Menu,
    Power,
    Camera,
    VolumeUp,
    VolumeDown,
    Enter,
    DpadCenter,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Orientation {
    Portrait,
    Landscape,
}

#[allow(dead_code)]
pub struct AndroidRobot {
    device_id: String,
    server: ADBServer,
    debug: bool,
}

#[allow(dead_code)]
impl AndroidRobot {
    pub fn new(device_id: String, debug: bool) -> Self {
        Self {
            device_id,
            server: ADBServer::default(),
            debug,
        }
    }

    fn log_debug(&self, message: &str) {
        if self.debug {
            eprintln!("[DEBUG] Android Robot ({}): {}", self.device_id, message);
        }
    }

    fn get_device(&mut self) -> Result<adb_client::ADBServerDevice, String> {
        self.server
            .get_device_by_name(&self.device_id)
            .map_err(|e| format!("Failed to get device {}: {:?}", self.device_id, e))
    }

    fn execute_shell_command(&mut self, args: &[&str]) -> Result<Vec<u8>, String> {
        let mut device = self.get_device()?;
        let mut output = Vec::new();
        device
            .shell_command(args, &mut output)
            .map_err(|e| format!("Shell command failed: {:?}", e))?;
        Ok(output)
    }

    fn execute_shell_command_string(&mut self, args: &[&str]) -> Result<String, String> {
        let output = self.execute_shell_command(args)?;
        Ok(String::from_utf8_lossy(&output).to_string())
    }

    pub fn get_system_features(&mut self) -> Result<Vec<String>, String> {
        self.log_debug("Getting system features");
        let output = self.execute_shell_command_string(&["pm", "list", "features"])?;

        Ok(output
            .lines()
            .map(|line| line.trim())
            .filter(|line| line.starts_with("feature:"))
            .map(|line| line["feature:".len()..].to_string())
            .collect())
    }

    pub fn list_installed_apps(&mut self) -> Result<Vec<InstalledApp>, String> {
        self.log_debug("Listing installed apps");

        let output = self.execute_shell_command(&["pm", "list", "packages", "-3"])?;
        let output_str = String::from_utf8_lossy(&output);

        let mut apps = Vec::new();
        for line in output_str.lines() {
            if let Some(package) = line.strip_prefix("package:") {
                apps.push(InstalledApp {
                    package_name: package.to_string(),
                    app_name: package.to_string(), // Would need additional query for actual name
                });
            }
        }

        Ok(apps)
    }

    pub fn list_screen_elements(
        &mut self,
        filter: Option<&str>,
    ) -> Result<Vec<ScreenElement>, String> {
        self.log_debug("Listing screen elements");

        // Use uiautomator dump to get UI hierarchy
        // First dump to default location, then read the file
        let _ = self.execute_shell_command(&["uiautomator", "dump"])?;
        let output = self.execute_shell_command(&["cat", "/sdcard/window_dump.xml"])?;
        let output_str = String::from_utf8_lossy(&output);

        self.log_debug(&format!(
            "XML content length: {} bytes, first 200 chars: {}",
            output_str.len(),
            &output_str.chars().take(200).collect::<String>()
        ));

        let mut elements = Vec::new();

        // Parse XML using quick-xml Reader
        self.parse_xml_with_quick_xml(&output_str, filter, &mut elements);

        self.log_debug(&format!("Found {} elements", elements.len()));
        Ok(elements)
    }

    fn parse_xml_with_quick_xml(
        &self,
        xml_content: &str,
        filter: Option<&str>,
        elements: &mut Vec<ScreenElement>,
    ) {
        let mut reader = XmlReader::from_str(xml_content);
        reader.trim_text(true);

        let mut buf = Vec::new();
        let mut total_nodes = 0;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if e.name().as_ref() == b"node" {
                        total_nodes += 1;
                        // Parse attributes into a HashMap
                        let mut attrs = HashMap::new();
                        for attr in e.attributes().flatten() {
                            let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                            let value = String::from_utf8_lossy(&attr.value).to_string();
                            attrs.insert(key, value);
                        }

                        // Extract relevant attributes
                        let text = attrs.get("text").and_then(|t| {
                            if t.is_empty() {
                                None
                            } else {
                                Some(t.clone())
                            }
                        });

                        let content_desc = attrs.get("content-desc").and_then(|c| {
                            if c.is_empty() {
                                None
                            } else {
                                Some(c.clone())
                            }
                        });

                        let resource_id = attrs.get("resource-id").and_then(|r| {
                            if r.is_empty() {
                                None
                            } else {
                                Some(r.clone())
                            }
                        });

                        let class = attrs.get("class").cloned().unwrap_or_default();

                        let bounds_str = attrs.get("bounds").cloned().unwrap_or_default();

                        let focused = attrs.get("focused").map(|f| f == "true");

                        // Parse bounds to get rect
                        let rect = self.parse_bounds(&bounds_str).unwrap_or(ScreenElementRect {
                            x: 0,
                            y: 0,
                            width: 0,
                            height: 0,
                        });

                        // Determine label - prefer text, then content-desc, then resource-id
                        let label = text
                            .clone()
                            .or_else(|| content_desc.clone())
                            .or_else(|| resource_id.clone())
                            .unwrap_or_else(|| {
                                if class.is_empty() {
                                    "Unknown".to_string()
                                } else {
                                    class.clone()
                                }
                            });

                        // Apply filter if provided
                        let should_include = if let Some(filter_text) = filter {
                            text.as_ref()
                                .map(|t| t.contains(filter_text))
                                .unwrap_or(false)
                                || content_desc
                                    .as_ref()
                                    .map(|c| c.contains(filter_text))
                                    .unwrap_or(false)
                                || resource_id
                                    .as_ref()
                                    .map(|r| r.contains(filter_text))
                                    .unwrap_or(false)
                        } else {
                            true
                        };

                        // Only include elements that are visible (have non-zero size)
                        if should_include && (rect.width > 0 && rect.height > 0) {
                            elements.push(ScreenElement {
                                element_type: class.clone(),
                                text: text.clone(),
                                label: label.clone(),
                                rect,
                                focused,
                                identifier: resource_id,
                            });
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    self.log_debug(&format!(
                        "Error parsing XML at position {}: {:?}",
                        reader.buffer_position(),
                        e
                    ));
                    break;
                }
                _ => {}
            }
            buf.clear();
        }
        self.log_debug(&format!(
            "Parsed {} total nodes, added {} elements to list",
            total_nodes,
            elements.len()
        ));
    }

    fn parse_bounds(&self, bounds_str: &str) -> Option<ScreenElementRect> {
        if bounds_str.is_empty() {
            return None;
        }

        // bounds format: [x1,y1][x2,y2]
        let parts: Vec<&str> = bounds_str.split("][").collect();
        if parts.len() != 2 {
            return None;
        }

        let start = parts[0].trim_start_matches('[');
        let end = parts[1].trim_end_matches(']');

        let start_coords: Vec<&str> = start.split(',').collect();
        let end_coords: Vec<&str> = end.split(',').collect();

        if start_coords.len() != 2 || end_coords.len() != 2 {
            return None;
        }

        let x1 = start_coords[0].parse::<i32>().ok()?;
        let y1 = start_coords[1].parse::<i32>().ok()?;
        let x2 = end_coords[0].parse::<i32>().ok()?;
        let y2 = end_coords[1].parse::<i32>().ok()?;

        Some(ScreenElementRect {
            x: x1,
            y: y1,
            width: x2 - x1,
            height: y2 - y1,
        })
    }

    pub fn get_screen_size(&mut self) -> Result<ScreenSize, String> {
        self.log_debug("Getting screen size");
        let output = self.execute_shell_command_string(&["wm", "size"])?;

        let screen_size_str = output
            .split_whitespace()
            .last()
            .ok_or("Failed to parse screen size output")?;

        let parts: Vec<&str> = screen_size_str.split('x').collect();
        if parts.len() != 2 {
            return Err("Invalid screen size format".to_string());
        }

        let width = parts[0]
            .parse::<u32>()
            .map_err(|_| "Failed to parse screen width")?;
        let height = parts[1]
            .parse::<u32>()
            .map_err(|_| "Failed to parse screen height")?;

        Ok(ScreenSize {
            width,
            height,
            scale: 1.0,
        })
    }

    pub fn list_apps(&mut self) -> Result<Vec<InstalledApp>, String> {
        self.log_debug("Listing installed apps with launcher activities");
        let output = self.execute_shell_command_string(&[
            "cmd",
            "package",
            "query-activities",
            "-a",
            "android.intent.action.MAIN",
            "-c",
            "android.intent.category.LAUNCHER",
        ])?;

        let mut apps = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for line in output.lines() {
            let line = line.trim();
            if let Some(stripped) = line.strip_prefix("packageName=") {
                let package_name = stripped.to_string();
                if seen.insert(package_name.clone()) {
                    apps.push(InstalledApp {
                        app_name: package_name.clone(),
                        package_name,
                    });
                }
            }
        }

        Ok(apps)
    }

    pub fn launch_app(&mut self, package_name: &str) -> Result<(), String> {
        self.log_debug(&format!("Launching app: {}", package_name));

        // First, check if the app is installed
        let installed_apps = self.list_apps()?;
        let app_exists = installed_apps
            .iter()
            .any(|app| app.package_name == package_name);

        if !app_exists {
            // Get list of installed app names for better error message
            let available_apps: Vec<String> = installed_apps
                .iter()
                .map(|app| format!("  - {} ({})", app.app_name, app.package_name))
                .collect();

            return Err(format!(
                "App '{}' is not installed on this device.\n\nAvailable apps:\n{}",
                package_name,
                available_apps.join("\n")
            ));
        }

        self.execute_shell_command(&[
            "monkey",
            "-p",
            package_name,
            "-c",
            "android.intent.category.LAUNCHER",
            "1",
        ])?;
        Ok(())
    }

    pub fn terminate_app(&mut self, package_name: &str) -> Result<(), String> {
        self.log_debug(&format!("Terminating app: {}", package_name));
        self.execute_shell_command(&["am", "force-stop", package_name])?;
        Ok(())
    }

    pub fn list_running_processes(&mut self) -> Result<Vec<String>, String> {
        self.log_debug("Listing running processes");
        let output = self.execute_shell_command_string(&["ps", "-e"])?;

        Ok(output
            .lines()
            .map(|line| line.trim())
            .filter(|line| line.starts_with('u')) // non-system processes
            .filter_map(|line| line.split_whitespace().nth(8).map(|s| s.to_string()))
            .collect())
    }

    pub fn swipe(&mut self, direction: SwipeDirection) -> Result<(), String> {
        self.log_debug(&format!("Swiping: {:?}", direction));
        let screen_size = self.get_screen_size()?;
        let center_x = screen_size.width / 2;

        let (x0, y0, x1, y1) = match direction {
            SwipeDirection::Up => {
                let x = center_x;
                let y0 = (screen_size.height as f32 * 0.80) as u32;
                let y1 = (screen_size.height as f32 * 0.20) as u32;
                (x, y0, x, y1)
            }
            SwipeDirection::Down => {
                let x = center_x;
                let y0 = (screen_size.height as f32 * 0.20) as u32;
                let y1 = (screen_size.height as f32 * 0.80) as u32;
                (x, y0, x, y1)
            }
            SwipeDirection::Left => {
                let x0 = (screen_size.width as f32 * 0.80) as u32;
                let x1 = (screen_size.width as f32 * 0.20) as u32;
                let y = screen_size.height / 2;
                (x0, y, x1, y)
            }
            SwipeDirection::Right => {
                let x0 = (screen_size.width as f32 * 0.20) as u32;
                let x1 = (screen_size.width as f32 * 0.80) as u32;
                let y = screen_size.height / 2;
                (x0, y, x1, y)
            }
        };

        self.execute_shell_command(&[
            "input",
            "swipe",
            &x0.to_string(),
            &y0.to_string(),
            &x1.to_string(),
            &y1.to_string(),
            "1000",
        ])?;
        Ok(())
    }

    pub fn swipe_from_coordinate(
        &mut self,
        x: u32,
        y: u32,
        direction: SwipeDirection,
        distance: Option<u32>,
    ) -> Result<(), String> {
        self.log_debug(&format!(
            "Swiping from ({}, {}) direction: {:?}",
            x, y, direction
        ));
        let screen_size = self.get_screen_size()?;

        let default_distance_y = (screen_size.height as f32 * 0.3) as u32;
        let default_distance_x = (screen_size.width as f32 * 0.3) as u32;

        let (x0, y0, x1, y1) = match direction {
            SwipeDirection::Up => {
                let distance = distance.unwrap_or(default_distance_y);
                (x, y, x, y.saturating_sub(distance))
            }
            SwipeDirection::Down => {
                let distance = distance.unwrap_or(default_distance_y);
                (x, y, x, std::cmp::min(screen_size.height, y + distance))
            }
            SwipeDirection::Left => {
                let distance = distance.unwrap_or(default_distance_x);
                (x, y, x.saturating_sub(distance), y)
            }
            SwipeDirection::Right => {
                let distance = distance.unwrap_or(default_distance_x);
                (x, y, std::cmp::min(screen_size.width, x + distance), y)
            }
        };

        self.execute_shell_command(&[
            "input",
            "swipe",
            &x0.to_string(),
            &y0.to_string(),
            &x1.to_string(),
            &y1.to_string(),
            "1000",
        ])?;
        Ok(())
    }

    pub fn swipe_coordinates(
        &mut self,
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        duration_ms: u32,
    ) -> Result<(), String> {
        self.log_debug(&format!(
            "Swiping from ({}, {}) to ({}, {}) duration: {}ms",
            start_x, start_y, end_x, end_y, duration_ms
        ));

        self.execute_shell_command(&[
            "input",
            "swipe",
            &start_x.to_string(),
            &start_y.to_string(),
            &end_x.to_string(),
            &end_y.to_string(),
            &duration_ms.to_string(),
        ])?;
        Ok(())
    }

    pub fn get_screenshot(&mut self) -> Result<Vec<u8>, String> {
        self.log_debug("Taking screenshot");

        // Try to determine if we need display ID for multi-display devices
        let display_count = self.get_display_count()?;

        if display_count <= 1 {
            // Single display device - use simple screencap
            return self.execute_screencap_simple();
        }

        // Multi-display device - try to find the active display
        if let Ok(display_id) = self.get_first_display_id() {
            return self.execute_screencap_with_display(&display_id);
        }

        // Fallback to simple screencap
        self.execute_screencap_simple()
    }

    fn execute_screencap_simple(&mut self) -> Result<Vec<u8>, String> {
        let mut device = self.get_device()?;
        let mut output = Vec::new();
        device
            .shell_command(&["screencap", "-p"], &mut output)
            .map_err(|e| format!("Screenshot command failed: {:?}", e))?;

        // Validate PNG data
        if output.len() >= 8
            && output[0] == 0x89
            && output[1] == 0x50
            && output[2] == 0x4E
            && output[3] == 0x47
        {
            Ok(output)
        } else {
            Err("Invalid PNG data received from screencap".to_string())
        }
    }

    fn execute_screencap_with_display(&mut self, display_id: &str) -> Result<Vec<u8>, String> {
        let mut device = self.get_device()?;
        let mut output = Vec::new();
        device
            .shell_command(&["screencap", "-p", "-d", display_id], &mut output)
            .map_err(|e| format!("Screenshot with display ID failed: {:?}", e))?;
        Ok(output)
    }

    fn get_display_count(&mut self) -> Result<u32, String> {
        let output =
            self.execute_shell_command_string(&["dumpsys", "SurfaceFlinger", "--display-id"])?;
        let count = output
            .lines()
            .filter(|line| line.starts_with("Display "))
            .count() as u32;
        Ok(count)
    }

    fn get_first_display_id(&mut self) -> Result<String, String> {
        // Try modern approach first (Android 11+)
        if let Ok(display_id) = self.get_display_id_modern() {
            return Ok(display_id);
        }

        // Fallback to legacy dumpsys approach
        self.get_display_id_legacy()
    }

    fn get_display_id_modern(&mut self) -> Result<String, String> {
        let output = self.execute_shell_command_string(&["cmd", "display", "get-displays"])?;

        for line in output.lines() {
            if line.starts_with("Display id ") && line.contains(", state ON,") {
                if let Some(captures) = line.split("uniqueId \"").nth(1) {
                    if let Some(unique_id) = captures.split('"').next() {
                        let display_id = if let Some(stripped) = unique_id.strip_prefix("local:") {
                            stripped.to_string()
                        } else {
                            unique_id.to_string()
                        };
                        return Ok(display_id);
                    }
                }
            }
        }

        Err("No active display found".to_string())
    }

    fn get_display_id_legacy(&mut self) -> Result<String, String> {
        let output = self.execute_shell_command_string(&["dumpsys", "display"])?;

        // Look for DisplayViewport entries with isActive=true and type=INTERNAL
        for line in output.lines() {
            if line.contains("DisplayViewport{type=INTERNAL") && line.contains("isActive=true") {
                if let Some(start) = line.find("uniqueId='") {
                    let start = start + "uniqueId='".len();
                    if let Some(end) = line[start..].find('\'') {
                        let unique_id = &line[start..start + end];
                        let display_id = if let Some(stripped) = unique_id.strip_prefix("local:") {
                            stripped.to_string()
                        } else {
                            unique_id.to_string()
                        };
                        return Ok(display_id);
                    }
                }
            }
        }

        Err("No active internal display found".to_string())
    }

    pub fn tap(&mut self, x: u32, y: u32) -> Result<(), String> {
        self.log_debug(&format!("Tapping at ({}, {})", x, y));
        self.execute_shell_command(&["input", "tap", &x.to_string(), &y.to_string()])?;
        Ok(())
    }

    pub fn long_press(&mut self, x: u32, y: u32) -> Result<(), String> {
        self.log_debug(&format!("Long pressing at ({}, {})", x, y));
        // Long press is implemented as a swipe with no movement and long duration
        self.execute_shell_command(&[
            "input",
            "swipe",
            &x.to_string(),
            &y.to_string(),
            &x.to_string(),
            &y.to_string(),
            "500",
        ])?;
        Ok(())
    }

    pub fn double_tap(&mut self, x: u32, y: u32) -> Result<(), String> {
        self.log_debug(&format!("Double tapping at ({}, {})", x, y));
        self.tap(x, y)?;
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.tap(x, y)?;
        Ok(())
    }

    pub fn send_keys(&mut self, text: &str) -> Result<(), String> {
        if text.is_empty() {
            return Ok(());
        }

        self.log_debug(&format!("Sending keys: {}", text));

        if Self::is_ascii_safe(text) {
            // Use input text command for ASCII text
            let escaped_text = self.escape_shell_text(text);
            self.execute_shell_command(&["input", "text", &escaped_text])?;
        } else if self.is_device_kit_installed()? {
            // Use DeviceKit for non-ASCII text
            self.send_keys_via_clipboard(text)?;
        } else {
            return Err(
                "Non-ASCII text is not supported. Please install mobilenext devicekit".to_string(),
            );
        }

        Ok(())
    }

    fn is_ascii_safe(text: &str) -> bool {
        text.is_ascii()
    }

    fn escape_shell_text(&self, text: &str) -> String {
        // Escape shell special characters
        text.replace('\\', "\\\\")
            .replace('\'', "\\'")
            .replace('"', "\\\"")
            .replace('`', "\\`")
            .replace(' ', "%s") // Android input text uses %s for spaces
            .replace('\t', "\\t")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
            .replace('|', "\\|")
            .replace('&', "\\&")
            .replace(';', "\\;")
            .replace('(', "\\(")
            .replace(')', "\\)")
            .replace('<', "\\<")
            .replace('>', "\\>")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('[', "\\[")
            .replace(']', "\\]")
            .replace('$', "\\$")
            .replace('*', "\\*")
            .replace('?', "\\?")
    }

    fn is_device_kit_installed(&mut self) -> Result<bool, String> {
        let output = self.execute_shell_command_string(&["pm", "list", "packages"])?;
        Ok(output
            .lines()
            .any(|line| line.contains("com.mobilenext.devicekit")))
    }

    fn send_keys_via_clipboard(&mut self, text: &str) -> Result<(), String> {
        let base64_text =
            base64::Engine::encode(&base64::engine::general_purpose::STANDARD, text.as_bytes());

        // Send text to clipboard
        self.execute_shell_command(&[
            "am",
            "broadcast",
            "-a",
            "devicekit.clipboard.set",
            "-e",
            "encoding",
            "base64",
            "-e",
            "text",
            &base64_text,
            "-n",
            "com.mobilenext.devicekit/.ClipboardBroadcastReceiver",
        ])?;

        // Paste from clipboard
        self.execute_shell_command(&["input", "keyevent", "KEYCODE_PASTE"])?;

        // Clear clipboard
        self.execute_shell_command(&[
            "am",
            "broadcast",
            "-a",
            "devicekit.clipboard.clear",
            "-n",
            "com.mobilenext.devicekit/.ClipboardBroadcastReceiver",
        ])?;

        Ok(())
    }

    pub fn press_button(&mut self, button: Button) -> Result<(), String> {
        let keycode = match button {
            Button::Back => "KEYCODE_BACK",
            Button::Home => "KEYCODE_HOME",
            Button::Menu => "KEYCODE_MENU",
            Button::Power => "KEYCODE_POWER",
            Button::Camera => "KEYCODE_CAMERA",
            Button::VolumeUp => "KEYCODE_VOLUME_UP",
            Button::VolumeDown => "KEYCODE_VOLUME_DOWN",
            Button::Enter => "KEYCODE_ENTER",
            Button::DpadCenter => "KEYCODE_DPAD_CENTER",
            Button::DpadUp => "KEYCODE_DPAD_UP",
            Button::DpadDown => "KEYCODE_DPAD_DOWN",
            Button::DpadLeft => "KEYCODE_DPAD_LEFT",
            Button::DpadRight => "KEYCODE_DPAD_RIGHT",
        };

        self.log_debug(&format!("Pressing button: {:?}", button));
        self.execute_shell_command(&["input", "keyevent", keycode])?;
        Ok(())
    }

    pub fn set_orientation(&mut self, orientation: Orientation) -> Result<(), String> {
        let value = match orientation {
            Orientation::Portrait => "0",
            Orientation::Landscape => "1",
        };

        self.log_debug(&format!("Setting orientation to: {:?}", orientation));

        // Disable auto-rotation
        self.execute_shell_command(&["settings", "put", "system", "accelerometer_rotation", "0"])?;

        // Set orientation
        self.execute_shell_command(&[
            "content",
            "insert",
            "--uri",
            "content://settings/system",
            "--bind",
            "name:s:user_rotation",
            "--bind",
            &format!("value:i:{}", value),
        ])?;

        Ok(())
    }

    pub fn get_orientation(&mut self) -> Result<Orientation, String> {
        let output =
            self.execute_shell_command_string(&["settings", "get", "system", "user_rotation"])?;

        let rotation = output.trim();
        Ok(if rotation == "0" {
            Orientation::Portrait
        } else {
            Orientation::Landscape
        })
    }

    pub fn open_url(&mut self, url: &str) -> Result<(), String> {
        self.log_debug(&format!("Opening URL: {}", url));
        self.execute_shell_command(&[
            "am",
            "start",
            "-a",
            "android.intent.action.VIEW",
            "-d",
            url,
        ])?;
        Ok(())
    }

    pub fn install_app(&mut self, apk_path: &str) -> Result<(), String> {
        self.log_debug(&format!("Installing APK: {}", apk_path));
        let mut device = self.get_device()?;
        // Note: This would need to be implemented with file transfer first
        // For now, we'll use a simplified approach assuming the APK is on the device
        let mut output = Vec::new();
        device
            .shell_command(&["pm", "install", "-r", apk_path], &mut output)
            .map_err(|e| format!("APK installation failed: {:?}", e))?;

        let result = String::from_utf8_lossy(&output);
        if result.contains("Success") {
            Ok(())
        } else {
            Err(format!("Installation failed: {}", result))
        }
    }

    pub fn uninstall_app(&mut self, package_name: &str) -> Result<(), String> {
        self.log_debug(&format!("Uninstalling package: {}", package_name));
        let mut device = self.get_device()?;
        let mut output = Vec::new();
        device
            .shell_command(&["pm", "uninstall", package_name], &mut output)
            .map_err(|e| format!("Uninstallation failed: {:?}", e))?;

        let result = String::from_utf8_lossy(&output);
        if result.contains("Success") {
            Ok(())
        } else {
            Err(format!("Uninstallation failed: {}", result))
        }
    }
}

pub struct AndroidDeviceManager {
    debug: bool,
    server: ADBServer,
}

impl AndroidDeviceManager {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            server: ADBServer::default(),
        }
    }

    fn log_debug(&self, message: &str) {
        if self.debug {
            eprintln!("[DEBUG] Android Manager: {}", message);
        }
    }

    pub fn get_connected_devices(&mut self) -> Result<Vec<AndroidDevice>, String> {
        self.log_debug("Getting connected Android devices");

        let devices = self
            .server
            .devices()
            .map_err(|e| format!("Failed to get devices: {:?}", e))?;

        let mut android_devices = Vec::new();

        for device in devices {
            if matches!(device.state, DeviceState::Device) {
                let device_type = self.get_device_type(&device.identifier)?;
                android_devices.push(AndroidDevice {
                    device_id: device.identifier,
                    device_type,
                });
            }
        }

        Ok(android_devices)
    }

    fn get_device_type(&mut self, device_id: &str) -> Result<AndroidDeviceType, String> {
        let mut robot = AndroidRobot::new(device_id.to_string(), self.debug);
        let features = robot.get_system_features()?;

        if features.contains(&"android.software.leanback".to_string())
            || features.contains(&"android.hardware.type.television".to_string())
        {
            Ok(AndroidDeviceType::TV)
        } else {
            Ok(AndroidDeviceType::Mobile)
        }
    }

    pub fn create_robot(&self, device_id: String) -> AndroidRobot {
        AndroidRobot::new(device_id, self.debug)
    }

    pub fn list_devices(&mut self) -> Result<Vec<DeviceInfo>, String> {
        let android_devices = self.get_connected_devices()?;

        Ok(android_devices
            .into_iter()
            .map(|device| {
                let device_type_str = match device.device_type {
                    AndroidDeviceType::Mobile => "mobile",
                    AndroidDeviceType::TV => "tv",
                };

                DeviceInfo {
                    id: device.device_id.clone(),
                    name: format!("Android {} ({})", device_type_str, &device.device_id[..8]),
                    platform: "android".to_string(),
                    device_type: device_type_str.to_string(),
                    state: "connected".to_string(),
                }
            })
            .collect())
    }

    pub fn take_screenshot(&mut self, device_id: &str) -> Result<Vec<u8>, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.get_screenshot()
    }

    pub fn tap_screen(&mut self, device_id: &str, x: f64, y: f64) -> Result<String, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.tap(x as u32, y as u32)?;
        Ok(format!("Tapped at ({}, {}) on device {}", x, y, device_id))
    }

    pub fn type_text(&mut self, device_id: &str, text: &str) -> Result<String, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.send_keys(text)?;
        Ok(format!("Typed '{}' on device {}", text, device_id))
    }

    #[allow(dead_code)]
    pub fn swipe(&mut self, device_id: &str, direction: SwipeDirection) -> Result<String, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.swipe(direction.clone())?;
        Ok(format!("Swiped {:?} on device {}", direction, device_id))
    }

    #[allow(dead_code)]
    pub fn press_button(&mut self, device_id: &str, button: Button) -> Result<String, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.press_button(button.clone())?;
        Ok(format!(
            "Pressed {:?} button on device {}",
            button, device_id
        ))
    }

    #[allow(dead_code)]
    pub fn launch_app(&mut self, device_id: &str, package_name: &str) -> Result<String, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.launch_app(package_name)?;
        Ok(format!(
            "Launched app {} on device {}",
            package_name, device_id
        ))
    }

    #[allow(dead_code)]
    pub fn list_apps(&mut self, device_id: &str) -> Result<Vec<InstalledApp>, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.list_apps()
    }

    #[allow(dead_code)]
    pub fn get_screen_size(&mut self, device_id: &str) -> Result<ScreenSize, String> {
        let mut robot = self.create_robot(device_id.to_string());
        robot.get_screen_size()
    }
}
