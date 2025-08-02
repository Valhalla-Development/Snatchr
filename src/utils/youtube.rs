use regex::Regex;

/*
 * YouTube URL utilities for validation and video ID extraction.
 * Centralizes regex patterns to avoid duplication across the codebase.
 */

/// Validates if a URL is a valid YouTube URL and extracts the video ID
pub fn validate_and_extract_video_id(url: &str) -> Option<String> {
    let pattern = r"https?://(?:www\.|m\.)?youtube\.com/(?:watch\?v=|shorts/)([a-zA-Z0-9_-]{11})|https?://youtu\.be/([a-zA-Z0-9_-]{11})";

    if let Ok(regex) = Regex::new(pattern) {
        if let Some(captures) = regex.captures(url) {
            // Check both capture groups (group 1 for youtube.com, group 2 for youtu.be)
            if let Some(video_id) = captures.get(1).or_else(|| captures.get(2)) {
                return Some(video_id.as_str().to_string());
            }
        }
    }
    None
}

/// Validates if URL is a valid YouTube URL
pub fn is_valid_youtube_url(url: &str) -> bool {
    validate_and_extract_video_id(url).is_some()
}
