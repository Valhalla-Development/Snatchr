use crate::config::Config;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use tokio::time::interval;
use tracing::{error, info, warn};

/*
 * Cleanup utility for removing old downloaded files.
 *
 * Scans the download directory and removes files older than the configured
 * cleanup_after_minutes setting. Uses job ID directories to track downloads.
 */

// Custom error type for cleanup operations
#[derive(Debug)]
pub enum CleanupError {
    IoError(std::io::Error),
    TimeError(std::time::SystemTimeError),
    DirectoryNotFound,
    InvalidConfiguration,
}

impl std::fmt::Display for CleanupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CleanupError::IoError(e) => write!(f, "IO error: {}", e),
            CleanupError::TimeError(e) => write!(f, "Time error: {}", e),
            CleanupError::DirectoryNotFound => {
                write!(f, "Download directory not found or could not be created")
            }
            CleanupError::InvalidConfiguration => write!(f, "Invalid cleanup configuration"),
        }
    }
}

impl std::error::Error for CleanupError {}

impl From<std::io::Error> for CleanupError {
    fn from(err: std::io::Error) -> Self {
        CleanupError::IoError(err)
    }
}

impl From<std::time::SystemTimeError> for CleanupError {
    fn from(err: std::time::SystemTimeError) -> Self {
        CleanupError::TimeError(err)
    }
}

// Removes all files older than the configured cleanup time
pub fn cleanup_old_files() -> Result<usize, CleanupError> {
    let config = Config::from_env();

    // Validate configuration
    if config.cleanup_after_minutes == 0 {
        return Err(CleanupError::InvalidConfiguration);
    }

    let download_dir = PathBuf::from(&config.download_dir);

    if !download_dir.exists() {
        // Try to create the directory if it doesn't exist
        std::fs::create_dir_all(&download_dir).map_err(|_| CleanupError::DirectoryNotFound)?;
    }

    // Calculate cutoff time (files older than this will be removed)
    let now = SystemTime::now();
    let cutoff_duration = Duration::from_secs(config.cleanup_after_minutes as u64 * 60);
    let cutoff_time = now
        .checked_sub(cutoff_duration)
        .ok_or(CleanupError::InvalidConfiguration)?;

    let mut removed_count = 0;

    // Read all entries in the download directory
    let entries = fs::read_dir(&download_dir)?;

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                warn!("Failed to read directory entry: {}", e);
                continue;
            }
        };

        let path = entry.path();

        if path.is_dir() {
            // Check if this is a video directory (video_id directory)
            if is_video_directory(&path) {
                if let Ok(()) = remove_if_old(&path, cutoff_time) {
                    info!("Removed old download directory: {}", path.display());
                    removed_count += 1;
                }
            }
        } else if path.is_file() {
            // Remove temporary files immediately (don't check age)
            if is_temporary_file(&path) {
                match fs::remove_file(&path) {
                    Ok(_) => {
                        info!("Removed temporary file: {}", path.display());
                        removed_count += 1;
                    }
                    Err(e) => {
                        error!("Failed to remove temporary file {}: {}", path.display(), e);
                    }
                }
            }
        }
    }

    log_cleanup_result(removed_count);
    Ok(removed_count)
}

// Removes a file or directory if it's older than the cutoff time
fn remove_if_old(path: &PathBuf, cutoff_time: SystemTime) -> Result<(), CleanupError> {
    // For video directories, check the .last_accessed marker file
    let access_marker = path.join(".last_accessed");
    let access_time = if access_marker.exists() {
        // Use the modification time of the access marker
        fs::metadata(&access_marker)?.modified()?
    } else {
        // Fall back to directory creation time for directories without access markers
        fs::metadata(path)?.modified()?
    };

    // Remove if file is older than cutoff time
    if access_time < cutoff_time {
        if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
        Ok(())
    } else {
        Err(CleanupError::IoError(std::io::Error::new(
            std::io::ErrorKind::Other,
            "File is not old enough for removal",
        )))
    }
}

// Checks if a file is a temporary file that should be removed immediately
fn is_temporary_file(path: &PathBuf) -> bool {
    if let Some(file_name) = path.file_name() {
        if let Some(name_str) = file_name.to_str() {
            return name_str.starts_with("temp_audio")
                || name_str.starts_with("temp_video")
                || name_str.ends_with(".tmp")
                || name_str.ends_with(".temp");
        }
    }
    false
}

// Checks if a directory name is not "cache"
fn is_video_directory(path: &PathBuf) -> bool {
    if let Some(name) = path.file_name() {
        if let Some(name_str) = name.to_str() {
            return name_str != "cache";
        }
    }
    false
}

// Logs the cleanup result with appropriate message
fn log_cleanup_result(removed_count: usize) {
    if removed_count > 0 {
        info!(
            "Cleanup completed: removed {} old files/directories",
            removed_count
        );
    } else {
        info!("Cleanup completed: no old files found");
    }
}

// Runs cleanup in a background task with periodic execution
pub async fn start_cleanup_scheduler() {
    let config = Config::from_env();

    // Validate configuration
    if config.cleanup_after_minutes == 0 {
        error!("Invalid cleanup configuration: cleanup_after_minutes cannot be 0");
        return;
    }

    // Run cleanup every quarter of the expiry time (more frequent checks)
    let cleanup_interval = Duration::from_secs(config.cleanup_after_minutes as u64 * 60);
    let mut interval_timer = interval(cleanup_interval);

    info!(
        "Starting cleanup scheduler with interval: {:?}",
        cleanup_interval
    );

    // Run initial cleanup
    info!("Running initial cleanup check...");
    if let Err(e) = cleanup_old_files() {
        error!("Initial cleanup failed: {}", e);
    }

    // Wait for first interval, then start the loop
    interval_timer.tick().await;

    // Main cleanup loop
    loop {
        interval_timer.tick().await;
        info!("Running scheduled cleanup...");

        if let Err(e) = cleanup_old_files() {
            error!("Cleanup task failed: {}", e);
        }
    }
}
