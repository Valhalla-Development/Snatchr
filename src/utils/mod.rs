/*
 * Module declaration for the utils.
 *
 * Contains cleanup, console logging, yt-dlp updates, and URL→cache-id helpers.
 */
pub mod cleanup;
pub mod logger;
pub mod video_id;
pub mod ytdlp_update;

/// File types published by the downloader and managed by cache cleanup.
pub(crate) fn is_media_file(path: &std::path::Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("mp4" | "m4a" | "mp3" | "opus" | "ogg" | "webm" | "wav" | "flac" | "aac")
    )
}

pub(crate) fn is_incomplete_media_file(path: &std::path::Path) -> bool {
    is_media_file(path)
        && path
            .file_stem()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.starts_with('.') && name.ends_with(".tmp"))
}
