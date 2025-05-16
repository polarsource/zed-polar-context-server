use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{self as zed, serde_json, Command, ContextServerId, Project, Result};

const PACKAGE_NAME: &str = "@polar-sh/sdk";
const PACKAGE_VERSION: &str = "latest";
const SERVER_PATH: &str = "node_modules/@polar-sh/sdk/bin/mcp-server.js";

struct PolarModelContextExtension;

#[derive(Debug, Deserialize)]
struct PolarContextServerSettings {
    access_token: String,
}

impl zed::Extension for PolarModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("polar-context-server", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `access_token` setting".into());
        };
        let settings: PolarContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(SERVER_PATH)
                    .to_string_lossy()
                    .to_string(),
                "start".to_string(),
                "--access-token".to_string(),
                settings.access_token,
            ],
            env: vec![],
        })
    }
}

zed::register_extension!(PolarModelContextExtension);
