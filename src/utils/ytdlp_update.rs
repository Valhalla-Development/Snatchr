/*
 * Keeps the yt-dlp binary current.
 *
 * Reuses libs/ when present, then runs `yt-dlp --update` on boot and once a day.
 * Failures are logged and ignored so the server stays up.
 */

use std::path::PathBuf;
use std::time::Duration;

use tracing::{info, warn};
use yt_dlp::Downloader;

use crate::config::Config;

const UPDATE_EVERY: Duration = Duration::from_secs(24 * 60 * 60);

fn yt_dlp_binary_path() -> PathBuf {
    let name = if cfg!(windows) { "yt-dlp.exe" } else { "yt-dlp" };
    PathBuf::from("libs").join(name)
}

/// Ensure binaries exist, then ask yt-dlp to self-update and log the result.
async fn update_ytdlp() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::from_env();
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from(&config.download_dir);

    // Install only if missing; we capture `--update` output ourselves for clearer logs.
    Downloader::with_new_binaries(libraries_dir, output_dir)
        .await?
        .build()
        .await?;

    let binary = yt_dlp_binary_path();
    let output = tokio::process::Command::new(&binary)
        .arg("--update")
        .output()
        .await?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}{stderr}");
    let detail = combined.trim();

    if !output.status.success() {
        return Err(format!("yt-dlp --update failed: {detail}").into());
    }

    let lower = detail.to_ascii_lowercase();
    if lower.contains("up to date") {
        info!(detail, "yt-dlp is already up to date");
    } else if lower.contains("updated") {
        info!(detail, "yt-dlp update installed");
    } else {
        info!(detail, "yt-dlp update check finished");
    }

    Ok(())
}

/// Runs an update immediately, then every 24 hours.
pub async fn start_ytdlp_update_scheduler() {
    info!("yt-dlp update scheduler started");

    if let Err(e) = update_ytdlp().await {
        warn!(error = %e, "yt-dlp update check failed (boot)");
    }

    let mut interval_timer = tokio::time::interval(UPDATE_EVERY);
    // First tick completes immediately; skip it so the next wait is a full day.
    interval_timer.tick().await;

    loop {
        interval_timer.tick().await;
        info!("Running scheduled yt-dlp update check");

        if let Err(e) = update_ytdlp().await {
            warn!(error = %e, "yt-dlp update check failed");
        }
    }
}
