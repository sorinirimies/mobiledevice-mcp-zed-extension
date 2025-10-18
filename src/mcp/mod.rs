// mobile-mcp-zed-extension/src/mcp/mod.rs
// MCP protocol handling module entry point

pub mod protocol;

pub use protocol::{
    Capabilities, InitializeResult, McpError, McpErrorResponse, McpRequest, McpResponse,
    ServerInfo, ToolCallParams, ToolsCapability,
};
