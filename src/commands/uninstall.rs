use std::fs;

use anyhow::{Context, Result};

pub fn run_uninstall() -> Result<()> {
    let exe_path = std::env::current_exe().context("Failed to determine current executable path")?;

    // Remove config directory
    if let Some(config_dir) = dirs::config_dir() {
        let mahoraga_config = config_dir.join("mahoraga");
        if mahoraga_config.exists() {
            fs::remove_dir_all(&mahoraga_config).context("Failed to remove config directory")?;
            println!("Removed config directory: {}", mahoraga_config.display());
        } else {
            println!("No config directory found at {}", mahoraga_config.display());
        }
    }

    // Remove the binary itself
    // On Unix, this is safe because the running binary stays in memory
    fs::remove_file(&exe_path).context(
        "Failed to remove binary. You may need to run with elevated permissions (e.g., sudo).",
    )?;
    println!("Removed binary: {}", exe_path.display());

    println!("Mahoraga has been uninstalled successfully.");

    Ok(())
}
