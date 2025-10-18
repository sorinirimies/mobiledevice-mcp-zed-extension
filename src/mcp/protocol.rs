// mobile-mcp-zed-extension/src/mcp/protocol.rs
// MCP Protocol Message Handling
//
// This module implements the Model Context Protocol (MCP) data structures
// following the JSON-RPC 2.0 specification. MCP is a protocol for communication
// between AI assistants and context servers (like this mobile device server).
//
// Key concepts:
// - All messages use JSON-RPC 2.0 format
// - Requests have: jsonrpc, id, method, params
// - Responses have: jsonrpc, id, result OR error
// - The initialize handshake establishes capabilities
// - Tools are discovered via tools/list and invoked via tools/call

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// MCP JSON-RPC Request
///
/// Represents an incoming request from an MCP client (like Zed's AI assistant).
/// All requests follow JSON-RPC 2.0 format with a method name and optional params.
///
/// Common methods:
/// - "initialize" - Handshake to establish protocol version and capabilities
/// - "tools/list" - Request list of available tools
/// - "tools/call" - Invoke a specific tool with arguments
///
/// # Example
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "method": "tools/call",
///   "params": {
///     "name": "mobile_device_mcp_take_screenshot",
///     "arguments": {"device_id": "emulator-5554", "platform": "android"}
///   }
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct McpRequest {
    /// Always "2.0" for JSON-RPC 2.0 protocol
    #[allow(dead_code)]
    pub jsonrpc: String,
    /// Request identifier - used to match responses to requests
    pub id: Option<Value>,
    /// Method name to invoke (e.g., "initialize", "tools/list", "tools/call")
    pub method: String,
    /// Method-specific parameters (optional)
    pub params: Option<Value>,
}

/// MCP JSON-RPC Response
///
/// Represents a successful response to an MCP request. Contains the result
/// of the operation. The id field matches the request id for correlation.
///
/// # Example
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "result": {
///     "content": [{"type": "text", "text": "Success!"}]
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct McpResponse {
    /// Always "2.0" for JSON-RPC 2.0 protocol
    pub jsonrpc: String,
    /// Request identifier from the original request
    pub id: Value,
    /// The successful result data
    pub result: Value,
}

/// MCP JSON-RPC Error Response
///
/// Represents an error response when a request fails. Contains structured
/// error information with code and message.
///
/// # Example
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "error": {
///     "code": -1,
///     "message": "Device not found"
///   }
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct McpErrorResponse {
    /// Always "2.0" for JSON-RPC 2.0 protocol
    pub jsonrpc: String,
    /// Request identifier from the original request
    pub id: Value,
    /// The error details
    pub error: McpError,
}

/// MCP Error
///
/// Error information structure used in error responses.
/// Code -1 is used for general application errors.
#[derive(Debug, Serialize)]
pub struct McpError {
    /// Error code (-1 for general errors)
    pub code: i32,
    /// Human-readable error message
    pub message: String,
}

/// MCP Initialize Result
///
/// Response data for the "initialize" method. This is sent during the
/// initial handshake to tell the client what protocol version and
/// capabilities the server supports.
///
/// # Example
/// ```json
/// {
///   "protocolVersion": "2024-11-05",
///   "capabilities": {"tools": {}},
///   "serverInfo": {"name": "mobile-device-mcp-server", "version": "1.0.0"}
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct InitializeResult {
    /// MCP protocol version (currently "2024-11-05")
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Server capabilities (what features are supported)
    pub capabilities: Capabilities,
    /// Information about this server
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
}

/// MCP Capabilities
///
/// Describes what features the server supports. Currently only tools
/// are supported (no resources, prompts, or sampling).
#[derive(Debug, Serialize)]
pub struct Capabilities {
    /// Tools capability - indicates server can provide tools
    pub tools: ToolsCapability,
}

/// Tools Capability
///
/// Empty struct indicating that tools are supported. The actual tools
/// are discovered via the "tools/list" method.
#[derive(Debug, Serialize)]
pub struct ToolsCapability {}

/// Server Info
///
/// Metadata about the MCP server including name and version.
/// This helps clients identify what server they're connected to.
#[derive(Debug, Serialize)]
pub struct ServerInfo {
    /// Server name (e.g., "mobile-device-mcp-server")
    pub name: String,
    /// Server version (e.g., "1.0.0")
    pub version: String,
}

/// Tool Call Parameters
///
/// Parameters for the "tools/call" method. Specifies which tool to invoke
/// and what arguments to pass to it.
///
/// # Example
/// ```json
/// {
///   "name": "mobile_device_mcp_take_screenshot",
///   "arguments": {
///     "device_id": "emulator-5554",
///     "platform": "android"
///   }
/// }
/// ```
#[derive(Debug, Deserialize)]
pub struct ToolCallParams {
    /// Name of the tool to invoke (must match a tool from tools/list)
    pub name: String,
    /// Tool-specific arguments as a JSON object
    pub arguments: Value,
}

impl McpRequest {
    /// Parse a request from JSON string
    ///
    /// Deserializes a JSON-RPC request from a string. This is the entry point
    /// for processing incoming MCP requests.
    ///
    /// # Arguments
    /// * `json_str` - JSON string containing the request
    ///
    /// # Returns
    /// Parsed request or error message if JSON is invalid
    ///
    /// # Example
    /// ```rust
    /// let request = McpRequest::from_json(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#)?;
    /// ```
    pub fn from_json(json_str: &str) -> Result<Self, String> {
        serde_json::from_str(json_str).map_err(|e| format!("Failed to parse request: {}", e))
    }
}

impl McpResponse {
    /// Create a new success response
    ///
    /// Constructs an MCP response for a successful operation. The result
    /// should be formatted according to MCP content specification (usually
    /// a JSON object with a "content" array).
    ///
    /// # Arguments
    /// * `id` - Request ID to match with the original request
    /// * `result` - Result data (must be JSON-serializable)
    ///
    /// # Example
    /// ```rust
    /// let response = McpResponse::success(
    ///     json!(1),
    ///     json!({"content": [{"type": "text", "text": "Done!"}]})
    /// );
    /// ```
    pub fn success(id: Value, result: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            result,
        }
    }

    /// Convert to JSON string
    ///
    /// Serializes the response to a JSON string suitable for sending to
    /// the MCP client over stdout.
    ///
    /// # Returns
    /// JSON string or error if serialization fails
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Failed to serialize response: {}", e))
    }
}

impl McpErrorResponse {
    /// Create a new error response
    ///
    /// Constructs an MCP error response for a failed operation. Use code -1
    /// for general application errors.
    ///
    /// # Arguments
    /// * `id` - Request ID to match with the original request
    /// * `code` - Error code (use -1 for general errors)
    /// * `message` - Human-readable error description
    ///
    /// # Example
    /// ```rust
    /// let error = McpErrorResponse::error(
    ///     json!(1),
    ///     -1,
    ///     "Device not found".to_string()
    /// );
    /// ```
    pub fn error(id: Value, code: i32, message: String) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            error: McpError { code, message },
        }
    }

    /// Convert to JSON string
    ///
    /// Serializes the error response to a JSON string suitable for sending
    /// to the MCP client over stdout.
    ///
    /// # Returns
    /// JSON string or error if serialization fails
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string(self).map_err(|e| format!("Failed to serialize error: {}", e))
    }
}

impl InitializeResult {
    /// Create a new initialize result with default values
    ///
    /// Returns an initialize result with:
    /// - Protocol version: "2024-11-05" (current MCP version)
    /// - Capabilities: Tools only (no resources, prompts, sampling)
    /// - Server info: "mobile-device-mcp-server" v1.0.0
    ///
    /// This is sent in response to the "initialize" method during the
    /// initial handshake between client and server.
    pub fn new() -> Self {
        Self {
            protocol_version: "2024-11-05".to_string(),
            capabilities: Capabilities {
                tools: ToolsCapability {},
            },
            server_info: ServerInfo {
                name: "mobile-device-mcp-server".to_string(),
                version: "1.0.0".to_string(),
            },
        }
    }
}

impl Default for InitializeResult {
    fn default() -> Self {
        Self::new()
    }
}
