// mobile-mcp-zed-extension/src/types.rs
// Shared types for the Mobile Device MCP Server

#[cfg(feature = "zed-extension")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ============================================================================
// Settings and Configuration
// ============================================================================

#[derive(Debug, Deserialize, Clone, Serialize)]
#[cfg_attr(feature = "zed-extension", derive(JsonSchema))]
pub struct MobileDeviceMcpSettings {
    /// Enable debug logging
    #[serde(default)]
    pub debug: bool,

    /// Platform to target: "android", "ios", or "auto"
    #[serde(default = "default_platform")]
    pub platform: String,
}

fn default_platform() -> String {
    "auto".to_string()
}

impl Default for MobileDeviceMcpSettings {
    fn default() -> Self {
        Self {
            debug: false,
            platform: default_platform(),
        }
    }
}

// ============================================================================
// Device Information
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub device_type: String,
    pub state: String,
}

// ============================================================================
// MCP Protocol Structures
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    #[serde(default)]
    pub params: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<McpError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct McpError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ToolCallParams {
    pub name: String,
    pub arguments: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_info_creation() {
        let device = DeviceInfo {
            id: "emulator-5554".to_string(),
            name: "Pixel 6".to_string(),
            platform: "android".to_string(),
            device_type: "emulator".to_string(),
            state: "connected".to_string(),
        };

        assert_eq!(device.id, "emulator-5554");
        assert_eq!(device.name, "Pixel 6");
        assert_eq!(device.platform, "android");
        assert_eq!(device.device_type, "emulator");
        assert_eq!(device.state, "connected");
    }

    #[test]
    fn test_device_info_clone() {
        let device1 = DeviceInfo {
            id: "test-device".to_string(),
            name: "Test Device".to_string(),
            platform: "ios".to_string(),
            device_type: "simulator".to_string(),
            state: "booted".to_string(),
        };

        let device2 = device1.clone();
        assert_eq!(device1.id, device2.id);
        assert_eq!(device1.name, device2.name);
    }

    #[test]
    fn test_settings_default() {
        let settings = MobileDeviceMcpSettings::default();
        assert!(!settings.debug);
        assert_eq!(settings.platform, "auto");
    }

    #[test]
    fn test_settings_custom() {
        let settings = MobileDeviceMcpSettings {
            debug: true,
            platform: "android".to_string(),
        };

        assert!(settings.debug);
        assert_eq!(settings.platform, "android");
    }

    #[test]
    fn test_settings_serialization() {
        let settings = MobileDeviceMcpSettings {
            debug: true,
            platform: "ios".to_string(),
        };

        let json = serde_json::to_string(&settings).unwrap();
        assert!(json.contains("true"));
        assert!(json.contains("ios"));
    }

    #[test]
    fn test_settings_deserialization() {
        let json = r#"{"debug": true, "platform": "android"}"#;
        let settings: MobileDeviceMcpSettings = serde_json::from_str(json).unwrap();

        assert!(settings.debug);
        assert_eq!(settings.platform, "android");
    }

    #[test]
    fn test_settings_partial_deserialization() {
        // Test with missing fields (should use defaults)
        let json = r#"{}"#;
        let settings: MobileDeviceMcpSettings = serde_json::from_str(json).unwrap();

        assert!(!settings.debug);
        assert_eq!(settings.platform, "auto");
    }

    #[test]
    fn test_mcp_request_structure() {
        let request = McpRequest {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::json!(1)),
            method: "initialize".to_string(),
            params: serde_json::json!({}),
        };

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.method, "initialize");
    }

    #[test]
    fn test_mcp_response_structure() {
        let response = McpResponse {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(1),
            result: Some(serde_json::json!({"success": true})),
            error: None,
        };

        assert_eq!(response.jsonrpc, "2.0");
        assert!(response.result.is_some());
        assert!(response.error.is_none());
    }

    #[test]
    fn test_mcp_error_structure() {
        let error = McpError {
            code: -1,
            message: "Test error".to_string(),
        };

        assert_eq!(error.code, -1);
        assert_eq!(error.message, "Test error");
    }

    #[test]
    fn test_tool_call_params() {
        let params = ToolCallParams {
            name: "mobile_device_mcp_take_screenshot".to_string(),
            arguments: serde_json::json!({
                "device_id": "emulator-5554",
                "platform": "android"
            }),
        };

        assert_eq!(params.name, "mobile_device_mcp_take_screenshot");
        assert!(params.arguments.is_object());
    }

    #[test]
    fn test_device_info_serialization() {
        let device = DeviceInfo {
            id: "test-id".to_string(),
            name: "Test Device".to_string(),
            platform: "android".to_string(),
            device_type: "physical".to_string(),
            state: "connected".to_string(),
        };

        let json = serde_json::to_string(&device).unwrap();
        assert!(json.contains("test-id"));
        assert!(json.contains("Test Device"));
        assert!(json.contains("android"));
    }

    #[test]
    fn test_device_info_deserialization() {
        let json = r#"{
            "id": "emulator-5554",
            "name": "Pixel 6",
            "platform": "android",
            "device_type": "emulator",
            "state": "connected"
        }"#;

        let device: DeviceInfo = serde_json::from_str(json).unwrap();
        assert_eq!(device.id, "emulator-5554");
        assert_eq!(device.name, "Pixel 6");
    }

    #[test]
    fn test_platform_values() {
        let platforms = vec!["android", "ios", "auto"];

        for platform in platforms {
            let settings = MobileDeviceMcpSettings {
                debug: false,
                platform: platform.to_string(),
            };
            assert_eq!(settings.platform, platform);
        }
    }

    #[test]
    fn test_mcp_response_with_error() {
        let response = McpResponse {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(1),
            result: None,
            error: Some(McpError {
                code: -1,
                message: "Device not found".to_string(),
            }),
        };

        assert!(response.result.is_none());
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().message, "Device not found");
    }
}
