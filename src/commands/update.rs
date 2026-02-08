use std::fs;
use std::io::Read;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use anyhow::{bail, Context, Result};
use flate2::read::GzDecoder;
use serde::Deserialize;
use tar::Archive;

const GITHUB_RELEASES_URL: &str =
    "https://api.github.com/repos/wyMinLwin/mahoraga/releases/latest";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
    assets: Vec<Asset>,
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

fn platform_target() -> Result<&'static str> {
    if cfg!(target_os = "macos") && cfg!(target_arch = "x86_64") {
        Ok("x86_64-apple-darwin")
    } else if cfg!(target_os = "macos") && cfg!(target_arch = "aarch64") {
        Ok("aarch64-apple-darwin")
    } else if cfg!(target_os = "linux") && cfg!(target_arch = "x86_64") {
        Ok("x86_64-unknown-linux-gnu")
    } else if cfg!(target_os = "linux") && cfg!(target_arch = "aarch64") {
        Ok("aarch64-unknown-linux-gnu")
    } else {
        bail!("Unsupported platform. Pre-built binaries are available for macOS (x86_64, aarch64) and Linux (x86_64, aarch64).")
    }
}

fn parse_version(v: &str) -> Result<(u64, u64, u64)> {
    let v = v.strip_prefix('v').unwrap_or(v);
    let parts: Vec<&str> = v.split('.').collect();
    if parts.len() != 3 {
        bail!("Invalid version format: {v}");
    }
    let major = parts[0].parse::<u64>().context("Invalid major version")?;
    let minor = parts[1].parse::<u64>().context("Invalid minor version")?;
    let patch = parts[2].parse::<u64>().context("Invalid patch version")?;
    Ok((major, minor, patch))
}

pub async fn run_update() -> Result<()> {
    let current_version = env!("CARGO_PKG_VERSION");
    println!("Current version: v{current_version}");
    println!("Checking for updates...");

    let client = reqwest::Client::new();
    let response = client
        .get(GITHUB_RELEASES_URL)
        .header("User-Agent", "mahoraga-updater")
        .send()
        .await
        .context("Failed to fetch latest release from GitHub")?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        println!("No releases found on GitHub.");
        return Ok(());
    }

    if !response.status().is_success() {
        bail!(
            "GitHub API returned status {}: {}",
            response.status(),
            response.text().await.unwrap_or_default()
        );
    }

    let release: Release = response
        .json()
        .await
        .context("Failed to parse release data from GitHub")?;

    let current = parse_version(current_version)?;
    let latest = parse_version(&release.tag_name)?;

    if latest <= current {
        println!("Already up to date (v{current_version}).");
        return Ok(());
    }

    println!("New version available: {}", release.tag_name);

    let platform = platform_target()?;
    let expected_name = format!("mahoraga-{}-{platform}.tar.gz", release.tag_name);

    let asset = release
        .assets
        .iter()
        .find(|a| a.name == expected_name)
        .with_context(|| {
            format!("No matching asset found for this platform ({platform}). Available assets: {}",
                release.assets.iter().map(|a| a.name.as_str()).collect::<Vec<_>>().join(", "))
        })?;

    println!("Downloading {}...", asset.name);
    let archive_bytes = client
        .get(&asset.browser_download_url)
        .header("User-Agent", "mahoraga-updater")
        .send()
        .await
        .context("Failed to download release asset")?
        .bytes()
        .await
        .context("Failed to read release asset bytes")?;

    println!("Extracting binary...");
    let decoder = GzDecoder::new(&archive_bytes[..]);
    let mut archive = Archive::new(decoder);

    let mut new_binary: Option<Vec<u8>> = None;
    for entry in archive.entries().context("Failed to read tar entries")? {
        let mut entry = entry.context("Failed to read tar entry")?;
        let path = entry.path().context("Failed to read entry path")?;
        let path_str = path.to_string_lossy();
        if path_str.ends_with("/mahoraga") || path_str == "mahoraga" {
            let mut buf = Vec::new();
            entry
                .read_to_end(&mut buf)
                .context("Failed to read binary from archive")?;
            new_binary = Some(buf);
            break;
        }
    }

    let new_binary = new_binary.context("Binary 'mahoraga' not found in the archive")?;

    let current_exe = std::env::current_exe().context("Failed to determine current executable path")?;
    let exe_dir = current_exe
        .parent()
        .context("Failed to determine executable directory")?;
    let tmp_path = exe_dir.join(".mahoraga_update_tmp");

    fs::write(&tmp_path, &new_binary).context("Failed to write temporary update file")?;

    #[cfg(unix)]
    fs::set_permissions(&tmp_path, fs::Permissions::from_mode(0o755))
        .context("Failed to set executable permissions")?;

    fs::rename(&tmp_path, &current_exe).context(
        "Failed to replace binary. You may need to run with elevated permissions (e.g., sudo).",
    )?;

    println!(
        "Successfully updated to {} !",
        release.tag_name
    );

    Ok(())
}
