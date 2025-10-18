// mobile-mcp-zed-extension/src/devices/mod.rs
// Device Management Modules

pub mod android;
pub mod ios;

pub use android::AndroidDeviceManager;
pub use ios::IOSDeviceManager;
