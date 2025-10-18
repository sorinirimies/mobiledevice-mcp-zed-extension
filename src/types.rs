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
