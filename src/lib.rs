use schemars::JsonSchema;
use serde::Deserialize;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, Command as ZedCommand,
    ContextServerConfiguration, ContextServerId, Project, Result,
};

/// Settings for the Mobile Device MCP Server
#[derive(Debug, Deserialize, JsonSchema, Clone)]
struct MobileMcpSettings {
    /// Enable debug logging for troubleshooting
    #[serde(default)]
    debug: bool,

    /// Preferred platform (ios, android, or auto)
    #[serde(default = "default_platform")]
    platform: String,
}

fn default_platform() -> String {
    "auto".to_string()
}

impl Default for MobileMcpSettings {
    fn default() -> Self {
        Self {
            debug: false,
            platform: default_platform(),
        }
    }
}

struct MobileMcpExtension;

impl zed::Extension for MobileMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<ZedCommand> {
        // Load user settings
        let settings = ContextServerSettings::for_project("mcp-server-mobile-device", project)?;
        let settings: MobileMcpSettings = if let Some(settings) = settings.settings {
            serde_json::from_value(settings)
                .map_err(|e| format!("Invalid settings configuration: {}", e))?
        } else {
            MobileMcpSettings::default()
        };

        // Build environment variables from settings
        let mut env = vec![];

        if settings.debug {
            env.push(("MOBILE_DEVICE_MCP_DEBUG".to_string(), "1".to_string()));
        }

        if settings.platform != "auto" {
            env.push(("MOBILE_PLATFORM".to_string(), settings.platform.clone()));
        }

        // Use absolute path to the binary in ~/.cargo/bin
        // Zed runs MCP servers from the work directory, so we need the full path
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| {
                // Try to get home directory from user info
                if let Ok(user) = std::env::var("USER") {
                    format!("/Users/{}", user)
                } else {
                    String::from("/Users/EBEMXC4")
                }
            });
        let binary_path = format!("{}/.cargo/bin/mobile-device-mcp-server", home);

        // Simply launch the native binary
        // The binary must be installed via `cargo install mobile-device-mcp-server`
        Ok(ZedCommand {
            command: binary_path,
            args: vec![],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();

        let settings_schema = serde_json::to_string(&schemars::schema_for!(MobileMcpSettings))
            .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(MobileMcpExtension);
