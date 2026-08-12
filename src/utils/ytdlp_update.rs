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

/// Ensure binaries exist, then ask yt-dlp to self-update.
async fn update_ytdlp() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = Config::from_env();
    let libraries_dir = PathBuf::from("libs");
    let output_dir = PathBuf::from(&config.download_dir);

    let downloader = Downloader::with_new_binaries(libraries_dir, output_dir)
        .await?
        .build()
        .await?;

    downloader.update_downloader().await?;
    Ok(())
}

/// Runs an update immediately, then every 24 hours.
pub async fn start_ytdlp_update_scheduler() {
    info!("yt-dlp update scheduler started");

    match update_ytdlp().await {
        Ok(()) => info!("yt-dlp update check finished"),
        Err(e) => warn!(error = %e, "yt-dlp update check failed (boot)"),
    }

    let mut interval_timer = tokio::time::interval(UPDATE_EVERY);
    // First tick completes immediately; skip it so the next wait is a full day.
    interval_timer.tick().await;

    loop {
        interval_timer.tick().await;
        info!("Running scheduled yt-dlp update check");

        match update_ytdlp().await {
            Ok(()) => info!("yt-dlp update check finished"),
            Err(e) => warn!(error = %e, "yt-dlp update check failed"),
        }
    }
}
