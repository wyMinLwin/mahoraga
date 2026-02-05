use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

use crate::types::Config;

/// Get the configuration directory path
pub fn config_dir() -> Result<PathBuf> {
    let config_dir = dirs::config_dir()
        .context("Could not determine config directory")?
        .join("mahoraga");
    Ok(config_dir)
}

/// Get the configuration file path
pub fn config_path() -> Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}

/// Load configuration from disk, or return default if not found
pub fn load_config() -> Result<Config> {
    let path = config_path()?;

    if !path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let config: Config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

    Ok(config)
}

/// Save configuration to disk
pub fn save_config(config: &Config) -> Result<()> {
    let dir = config_dir()?;
    let path = config_path()?;

    // Create directory if it doesn't exist
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create config directory: {}", dir.display()))?;
    }

    let content = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;

    fs::write(&path, content)
        .with_context(|| format!("Failed to write config file: {}", path.display()))?;

    Ok(())
}

/// Reset configuration to defaults
pub fn reset_config() -> Result<Config> {
    let config = Config::default();
    save_config(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.provider.active.as_str(), "azure");
    }
}
