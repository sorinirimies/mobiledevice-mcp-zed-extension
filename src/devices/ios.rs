// mobile-mcp-zed-extension/src/devices/ios.rs
// iOS Device Management Module with native idevice Rust crate integration

use crate::types::DeviceInfo;
use std::process::{Command, Stdio};

#[cfg(all(target_os = "macos", feature = "ios-support"))]
use idevice::usbmuxd::UsbmuxdConnection;

pub struct IOSDeviceManager {
    debug: bool,
    idevice_available: bool,
    xcrun_available: bool,
}

impl IOSDeviceManager {
    pub fn new(debug: bool) -> Self {
        let idevice_available = Self::is_idevice_available();
        let xcrun_available = Self::is_xcrun_available();

        Self {
            debug,
            idevice_available,
            xcrun_available,
        }
    }

    fn log_debug(&self, message: &str) {
        if self.debug {
            eprintln!("[DEBUG] iOS: {}", message);
        }
    }

    /// List all available iOS devices (real devices + simulators)
    #[cfg(target_os = "macos")]
    pub fn list_devices(&self) -> Result<Vec<DeviceInfo>, String> {
        let mut devices = Vec::new();

        // Get real iOS devices using idevice crate
        devices.extend(self.list_real_devices()?);

        // Get iOS simulators using xcrun simctl
        devices.extend(self.list_simulators()?);

        Ok(devices)
    }

    #[cfg(not(target_os = "macos"))]
    pub fn list_devices(&self) -> Result<Vec<DeviceInfo>, String> {
        Ok(Vec::new())
    }

    /// List real iOS devices using native idevice crate
    #[cfg(target_os = "macos")]
    fn list_real_devices(&self) -> Result<Vec<DeviceInfo>, String> {
        #[cfg(all(target_os = "macos", feature = "ios-support"))]
        let mut devices = Vec::new();
        #[cfg(not(all(target_os = "macos", feature = "ios-support")))]
        let devices = Vec::new();

        if !self.idevice_available {
            self.log_debug("idevice crate not available - skipping real iOS devices");
            return Ok(devices);
        }

        self.log_debug("Querying real iOS devices with idevice crate");

        // Try to connect to usbmuxd to get device list
        #[cfg(all(target_os = "macos", feature = "ios-support"))]
        {
            let rt = match tokio::runtime::Runtime::new() {
                Ok(rt) => rt,
                Err(e) => {
                    self.log_debug(&format!("Failed to create tokio runtime: {:?}", e));
                    return Ok(devices);
                }
            };

            match rt.block_on(async {
                let mut usbmuxd = UsbmuxdConnection::default().await?;
                usbmuxd.get_devices().await
            }) {
                Ok(device_list) => {
                    for device in device_list {
                        let device_info = self.get_real_device_info_from_usbmuxd(device);
                        devices.push(device_info);
                    }
                }
                Err(e) => {
                    self.log_debug(&format!("Failed to get device list: {:?}", e));
                }
            }
        }

        Ok(devices)
    }

    #[cfg(not(target_os = "macos"))]
    fn list_real_devices(&self) -> Result<Vec<DeviceInfo>, String> {
        Ok(Vec::new())
    }

    /// Get detailed information about a real iOS device using idevice crate
    #[cfg(all(target_os = "macos", feature = "ios-support"))]
    fn get_real_device_info_from_usbmuxd(
        &self,
        device: idevice::usbmuxd::UsbmuxdDevice,
    ) -> DeviceInfo {
        let device_id = device.udid.clone();

        // For now, use basic info from usbmuxd device
        // In a full implementation, we would connect to lockdown to get more details
        // To get the actual device name, we would need to connect to lockdown service
        let device_name = format!(
            "iOS Device ({})",
            &device_id[..std::cmp::min(8, device_id.len())]
        );

        self.log_debug(&format!(
            "Real device found: {} ({})",
            device_name, device_id
        ));

        DeviceInfo {
            id: device_id,
            name: device_name,
            platform: "ios".to_string(),
            device_type: "physical".to_string(),
            state: "booted".to_string(),
        }
    }

    #[cfg(not(all(target_os = "macos", feature = "ios-support")))]
    #[allow(dead_code)]
    fn get_real_device_info_from_usbmuxd(&self, device_id: String) -> DeviceInfo {
        // Stub implementation when idevice is not available
        DeviceInfo {
            id: device_id,
            name: "iOS Device (unavailable)".to_string(),
            platform: "ios".to_string(),
            device_type: "physical".to_string(),
            state: "unavailable".to_string(),
        }
    }

    /// List iOS simulators using xcrun simctl
    #[cfg(target_os = "macos")]
    fn list_simulators(&self) -> Result<Vec<DeviceInfo>, String> {
        let mut devices = Vec::new();

        if !self.xcrun_available {
            self.log_debug("xcrun not available - skipping iOS simulators");
            return Ok(devices);
        }

        match Command::new("xcrun")
            .args(["simctl", "list", "devices", "available", "--json"])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    match serde_json::from_str::<serde_json::Value>(&stdout) {
                        Ok(json) => {
                            if let Some(devices_obj) =
                                json.get("devices").and_then(|d| d.as_object())
                            {
                                for (runtime, device_list) in devices_obj {
                                    if let Some(device_array) = device_list.as_array() {
                                        for device in device_array {
                                            if let (Some(udid), Some(name), Some(state)) = (
                                                device.get("udid").and_then(|u| u.as_str()),
                                                device.get("name").and_then(|n| n.as_str()),
                                                device.get("state").and_then(|s| s.as_str()),
                                            ) {
                                                let status = match state {
                                                    "Booted" => "booted",
                                                    "Shutdown" => "shutdown",
                                                    _ => state,
                                                };

                                                // Extract iOS version from runtime if possible
                                                let ios_version = runtime
                                                    .split('.')
                                                    .next_back()
                                                    .unwrap_or("")
                                                    .replace('-', ".");

                                                let display_name = if ios_version.is_empty() {
                                                    name.to_string()
                                                } else {
                                                    format!("{} (iOS {})", name, ios_version)
                                                };

                                                self.log_debug(&format!(
                                                    "Simulator found: {} ({}) - {}",
                                                    display_name, udid, status
                                                ));

                                                devices.push(DeviceInfo {
                                                    id: udid.to_string(),
                                                    name: display_name,
                                                    platform: "ios".to_string(),
                                                    device_type: "simulator".to_string(),
                                                    state: status.to_string(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            self.log_debug(&format!("Failed to parse simctl JSON: {}", e));
                        }
                    }
                } else {
                    self.log_debug(&format!(
                        "xcrun simctl failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    ));
                }
            }
            Err(e) => {
                self.log_debug(&format!("Failed to execute xcrun simctl: {}", e));
            }
        }

        Ok(devices)
    }

    #[cfg(not(target_os = "macos"))]
    fn list_simulators(&self) -> Result<Vec<DeviceInfo>, String> {
        Ok(Vec::new())
    }

    /// Take a screenshot from an iOS device (real device or simulator)
    #[cfg(target_os = "macos")]
    pub fn take_screenshot(&self, device_id: &str) -> Result<Vec<u8>, String> {
        self.log_debug(&format!("Taking iOS screenshot from device: {}", device_id));

        // First try as a real device using idevice crate
        if self.idevice_available {
            self.log_debug("Attempting real device screenshot with idevice crate");
            if let Ok(screenshot) = self.take_real_device_screenshot(device_id) {
                return Ok(screenshot);
            }
            self.log_debug("Real device screenshot failed, trying simulator");
        }

        // Fall back to simulator using xcrun simctl
        if self.xcrun_available {
            return self.take_simulator_screenshot(device_id);
        }

        Err("Neither idevice crate nor xcrun available for iOS screenshots".to_string())
    }

    #[cfg(not(target_os = "macos"))]
    pub fn take_screenshot(&self, _device_id: &str) -> Result<Vec<u8>, String> {
        Err("iOS screenshots only supported on macOS".to_string())
    }

    /// Take a screenshot from a real iOS device using idevice crate
    #[cfg(target_os = "macos")]
    fn take_real_device_screenshot(&self, device_id: &str) -> Result<Vec<u8>, String> {
        self.log_debug(&format!(
            "Taking real device screenshot using idevice crate: {}",
            device_id
        ));

        // For now, real device screenshots are not supported without additional setup
        // This would require implementing the screenshot service via lockdown
        Err("Real device screenshots not yet implemented. Use simulator screenshots or install additional tools.".to_string())
    }

    /// Take a screenshot from an iOS simulator using xcrun simctl
    #[cfg(target_os = "macos")]
    fn take_simulator_screenshot(&self, device_id: &str) -> Result<Vec<u8>, String> {
        self.log_debug(&format!(
            "Attempting simulator screenshot for: {}",
            device_id
        ));

        match Command::new("xcrun")
            .args(["simctl", "io", device_id, "screenshot", "--type=png", "-"])
            .output()
        {
            Ok(output) => {
                if output.status.success() && !output.stdout.is_empty() {
                    self.log_debug(&format!(
                        "Simulator screenshot captured: {} bytes",
                        output.stdout.len()
                    ));
                    Ok(output.stdout)
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    self.log_debug(&format!("Simulator screenshot failed: {}", error_msg));
                    Err(format!(
                        "Failed to capture simulator screenshot: {}",
                        error_msg
                    ))
                }
            }
            Err(e) => {
                self.log_debug(&format!("xcrun simctl screenshot failed: {}", e));
                Err(format!("Failed to execute xcrun simctl: {}", e))
            }
        }
    }

    /// Tap the screen at specific coordinates (simulator only for now)
    #[cfg(target_os = "macos")]
    pub fn tap_screen(&self, device_id: &str, x: f64, y: f64) -> Result<String, String> {
        self.log_debug(&format!(
            "Tapping iOS screen at ({}, {}) on device: {}",
            x, y, device_id
        ));

        // Real device tapping would require additional automation frameworks
        // idevice crate doesn't provide direct screen interaction capabilities
        // Real device tapping requires WebDriverAgent or similar tools
        // idevice crate doesn't provide direct screen interaction capabilities
        if self.idevice_available {
            self.log_debug("Real device tapping requires WebDriverAgent or similar tools");
            // For now, we'll try simulator mode regardless
        }

        // Fall back to simulator tapping using xcrun simctl
        if !self.xcrun_available {
            return Err("xcrun not available for iOS simulator interaction".to_string());
        }

        match Command::new("xcrun")
            .args([
                "simctl",
                "io",
                device_id,
                "tap",
                &x.to_string(),
                &y.to_string(),
            ])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    self.log_debug("Simulator tap executed successfully");
                    Ok(format!(
                        "Tapped screen at ({}, {}) on device {}",
                        x, y, device_id
                    ))
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    self.log_debug(&format!("Simulator tap failed: {}", error_msg));
                    Err(format!("Failed to tap simulator screen: {}", error_msg))
                }
            }
            Err(e) => {
                self.log_debug(&format!("xcrun simctl tap failed: {}", e));
                Err(format!("Failed to execute xcrun simctl tap: {}", e))
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn tap_screen(&self, _device_id: &str, _x: f64, _y: f64) -> Result<String, String> {
        Err("iOS device interaction only supported on macOS".to_string())
    }

    /// Type text into the current iOS app (simulator only for now)
    #[cfg(target_os = "macos")]
    pub fn type_text(&self, device_id: &str, text: &str) -> Result<String, String> {
        self.log_debug(&format!(
            "Typing text '{}' on iOS device: {}",
            text, device_id
        ));

        // Real device text input would require additional automation frameworks
        if self.idevice_available {
            self.log_debug("Real device text input requires WebDriverAgent or similar tools");
            // For now, we'll try simulator mode regardless
        }

        // Fall back to simulator text input using xcrun simctl
        if !self.xcrun_available {
            return Err("xcrun not available for iOS simulator text input".to_string());
        }

        match Command::new("xcrun")
            .args(["simctl", "io", device_id, "type", text])
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    self.log_debug("Simulator text input executed successfully");
                    Ok(format!("Typed text '{}' on device {}", text, device_id))
                } else {
                    let error_msg = String::from_utf8_lossy(&output.stderr);
                    self.log_debug(&format!("Simulator text input failed: {}", error_msg));
                    Err(format!("Failed to type text on simulator: {}", error_msg))
                }
            }
            Err(e) => {
                self.log_debug(&format!("xcrun simctl type failed: {}", e));
                Err(format!("Failed to execute xcrun simctl type: {}", e))
            }
        }
    }

    #[cfg(not(target_os = "macos"))]
    pub fn type_text(&self, _device_id: &str, _text: &str) -> Result<String, String> {
        Err("iOS device interaction only supported on macOS".to_string())
    }

    /// Check if idevice crate functionality is available
    #[cfg(all(target_os = "macos", feature = "ios-support"))]
    fn is_idevice_available() -> bool {
        // Try to create a usbmuxd connection to test if idevice functionality works
        let rt = match tokio::runtime::Runtime::new() {
            Ok(rt) => rt,
            Err(_) => return false,
        };

        rt.block_on(async { UsbmuxdConnection::default().await.is_ok() })
    }

    #[cfg(not(all(target_os = "macos", feature = "ios-support")))]
    fn is_idevice_available() -> bool {
        false
    }

    /// Check if xcrun (iOS Simulator tools) is available
    #[cfg(target_os = "macos")]
    fn is_xcrun_available() -> bool {
        Command::new("xcrun")
            .args(["simctl", "help"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .map(|status| status.success())
            .unwrap_or(false)
    }

    #[cfg(not(target_os = "macos"))]
    fn is_xcrun_available() -> bool {
        false
    }
}
