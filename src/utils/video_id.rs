use regex::Regex;
use std::sync::LazyLock;

/*
 * Cheap URL → platform video-id extraction for cache lookups.
 *
 * Lets us serve a cached file without calling yt-dlp for metadata.
 * Patterns mirror the IDs yt-dlp typically uses for these hosts.
 * If extraction fails or disagrees, the normal metadata path still runs.
 */

static YOUTUBE_WATCH: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:youtube\.com|youtube-nocookie\.com)/watch\?(?:[^#]*&)?v=([A-Za-z0-9_-]{11})").unwrap()
});
static YOUTUBE_SHORTS: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:youtube\.com|youtube-nocookie\.com)/shorts/([A-Za-z0-9_-]{11})").unwrap()
});
static YOUTUBE_EMBED: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:youtube\.com|youtube-nocookie\.com)/embed/([A-Za-z0-9_-]{11})").unwrap()
});
static YOUTUBE_LIVE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:youtube\.com|youtube-nocookie\.com)/live/([A-Za-z0-9_-]{11})").unwrap()
});
static YOUTU_BE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)youtu\.be/([A-Za-z0-9_-]{11})").unwrap());
static TIKTOK_VIDEO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)tiktok\.com/(?:@[^/]+/)?video/(\d+)").unwrap());
static VIMEO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)vimeo\.com/(?:video/)?(\d+)").unwrap());
static INSTAGRAM: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)instagram\.com/(?:reel|p|tv)/([A-Za-z0-9_-]+)").unwrap());
static TWITTER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:twitter|x)\.com/[^/]+/status/(\d+)").unwrap());
static TWITCH_CLIP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i)(?:clips\.twitch\.tv/|twitch\.tv/[^/]+/clip/)([A-Za-z0-9_-]+)").unwrap()
});
static FACEBOOK_VIDEO: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)facebook\.com/.+/videos/(\d+)").unwrap());
static FACEBOOK_WATCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)(?:facebook\.com/watch/?\?v=|fb\.watch/)(\d+)").unwrap());

/// Extracts a cache key (platform video id) from a URL, when the format is known.
pub fn extract_cache_id(url: &str) -> Option<String> {
    for re in [
        &*YOUTUBE_WATCH,
        &*YOUTUBE_SHORTS,
        &*YOUTUBE_EMBED,
        &*YOUTUBE_LIVE,
        &*YOUTU_BE,
        &*TIKTOK_VIDEO,
        &*VIMEO,
        &*INSTAGRAM,
        &*TWITTER,
        &*TWITCH_CLIP,
        &*FACEBOOK_VIDEO,
        &*FACEBOOK_WATCH,
    ] {
        if let Some(caps) = re.captures(url) {
            if let Some(id) = caps.get(1) {
                return Some(id.as_str().to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_youtube_variants() {
        assert_eq!(
            extract_cache_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ").as_deref(),
            Some("dQw4w9WgXcQ")
        );
        assert_eq!(
            extract_cache_id("https://youtube.com/watch?v=dQw4w9WgXcQ&t=30").as_deref(),
            Some("dQw4w9WgXcQ")
        );
        assert_eq!(
            extract_cache_id("https://youtu.be/dQw4w9WgXcQ").as_deref(),
            Some("dQw4w9WgXcQ")
        );
        assert_eq!(
            extract_cache_id("https://www.youtube.com/shorts/dQw4w9WgXcQ").as_deref(),
            Some("dQw4w9WgXcQ")
        );
        assert_eq!(
            extract_cache_id("https://www.youtube.com/embed/dQw4w9WgXcQ").as_deref(),
            Some("dQw4w9WgXcQ")
        );
        assert_eq!(
            extract_cache_id("https://m.youtube.com/watch?v=dQw4w9WgXcQ").as_deref(),
            Some("dQw4w9WgXcQ")
        );
    }

    #[test]
    fn extracts_other_supported_platforms() {
        assert_eq!(
            extract_cache_id("https://www.tiktok.com/@user/video/7123456789012345678").as_deref(),
            Some("7123456789012345678")
        );
        assert_eq!(
            extract_cache_id("https://vimeo.com/123456789").as_deref(),
            Some("123456789")
        );
        assert_eq!(
            extract_cache_id("https://www.instagram.com/reel/AbCdEfGhIjK/").as_deref(),
            Some("AbCdEfGhIjK")
        );
        assert_eq!(
            extract_cache_id("https://x.com/someone/status/1234567890123456789").as_deref(),
            Some("1234567890123456789")
        );
        assert_eq!(
            extract_cache_id("https://clips.twitch.tv/SomeClipSlug-abc").as_deref(),
            Some("SomeClipSlug-abc")
        );
        assert_eq!(
            extract_cache_id("https://www.facebook.com/watch/?v=1234567890").as_deref(),
            Some("1234567890")
        );
    }

    #[test]
    fn returns_none_for_unknown_or_short_links() {
        assert!(extract_cache_id("https://vm.tiktok.com/ZMabcdef/").is_none());
        assert!(extract_cache_id("https://example.com/video/123").is_none());
        assert!(extract_cache_id("not a url").is_none());
    }
}
