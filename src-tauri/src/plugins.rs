//! Plugin execution module to run custom external parsers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tokio::process::Command;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PluginConfig {
    pub mappings: HashMap<String, PluginMapping>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PluginMapping {
    pub command: String,
    pub args: Vec<String>,
}

/// Helper to load the plugins config (`parsers.json`)
pub fn get_plugin_config(data_dir: &Path) -> Option<PluginConfig> {
    let config_path = data_dir.join("parsers.json");
    if config_path.exists() {
        if let Ok(content) = fs::read_to_string(&config_path) {
            match serde_json::from_str::<PluginConfig>(&content) {
                Ok(conf) => return Some(conf),
                Err(e) => {
                    log::warn!("Failed to parse parsers.json: {}", e);
                }
            }
        }
    }
    None
}

/// Helper to execute the external parser plugin
/// Returns `Ok(())` if the script executed and returned a 0 exit status.
pub async fn run_plugin(
    mapping: &PluginMapping,
    input_path: &Path,
    output_path: &Path,
) -> Result<(), String> {
    let mut cmd = Command::new(&mapping.command);

    for arg in &mapping.args {
        let arg_str = arg
            .replace("$INPUT", input_path.to_str().unwrap_or(""))
            .replace("$OUTPUT", output_path.to_str().unwrap_or(""));
        cmd.arg(arg_str);
    }

    log::info!(
        "Executing custom parser plugin subprocess: {:?}",
        cmd
    );

    let status = cmd
        .status()
        .await
        .map_err(|e| format!("Failed to spawn plugin subprocess: {}", e))?;

    if !status.success() {
        return Err(format!("Plugin exited with status: {}", status));
    }

    Ok(())
}
