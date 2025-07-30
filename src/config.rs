/*
 * Configuration module for the Snatchr application.
 *
 * Parses environment variables (via dotenvy) to configure
 * server settings, download paths, quality preferences, and performance options.
 *
 * Uses `strum` macros to derive enums that map environment strings
 * to internal quality and codec preference enums used by yt-dlp.
 */

use dotenvy::dotenv;
use std::env;
use strum_macros::{EnumIter, EnumString};
use yt_dlp::model::{AudioCodecPreference, AudioQuality, VideoCodecPreference, VideoQuality};

/*
 * Environment-parseable enums with FromStr implementations.
 * These enums represent user-friendly strings in environment variables,
 * which then get converted to yt_dlp enums.
 */
#[derive(Debug, EnumString, EnumIter)]
#[strum(serialize_all = "PascalCase")]
pub enum VideoQualityEnv {
    Best,
    High,
    Medium,
    Low,
    Worst,
}

#[derive(Debug, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum VideoCodecPreferenceEnv {
    VP9,
    AVC1,
    AV1,
    Any,
}

#[derive(Debug, EnumString, EnumIter)]
#[strum(serialize_all = "PascalCase")]
pub enum AudioQualityEnv {
    Best,
    High,
    Medium,
    Low,
    Worst,
}

#[derive(Debug, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum AudioCodecPreferenceEnv {
    Opus,
    AAC,
    MP3,
    Any,
}

/*
 * Implement conversion from environment enums to yt_dlp enums.
 * This allows seamless mapping after parsing environment variables.
 */
impl From<VideoQualityEnv> for VideoQuality {
    fn from(env: VideoQualityEnv) -> Self {
        match env {
            VideoQualityEnv::Best => VideoQuality::Best,
            VideoQualityEnv::High => VideoQuality::High,
            VideoQualityEnv::Medium => VideoQuality::Medium,
            VideoQualityEnv::Low => VideoQuality::Low,
            VideoQualityEnv::Worst => VideoQuality::Worst,
        }
    }
}

impl From<VideoCodecPreferenceEnv> for VideoCodecPreference {
    fn from(env: VideoCodecPreferenceEnv) -> Self {
        match env {
            VideoCodecPreferenceEnv::VP9 => VideoCodecPreference::VP9,
            VideoCodecPreferenceEnv::AVC1 => VideoCodecPreference::AVC1,
            VideoCodecPreferenceEnv::AV1 => VideoCodecPreference::AV1,
            VideoCodecPreferenceEnv::Any => VideoCodecPreference::Any,
        }
    }
}

impl From<AudioQualityEnv> for AudioQuality {
    fn from(env: AudioQualityEnv) -> Self {
        match env {
            AudioQualityEnv::Best => AudioQuality::Best,
            AudioQualityEnv::High => AudioQuality::High,
            AudioQualityEnv::Medium => AudioQuality::Medium,
            AudioQualityEnv::Low => AudioQuality::Low,
            AudioQualityEnv::Worst => AudioQuality::Worst,
        }
    }
}

impl From<AudioCodecPreferenceEnv> for AudioCodecPreference {
    fn from(env: AudioCodecPreferenceEnv) -> Self {
        match env {
            AudioCodecPreferenceEnv::Opus => AudioCodecPreference::Opus,
            AudioCodecPreferenceEnv::AAC => AudioCodecPreference::AAC,
            AudioCodecPreferenceEnv::MP3 => AudioCodecPreference::MP3,
            AudioCodecPreferenceEnv::Any => AudioCodecPreference::Any,
        }
    }
}

/*
 * Main configuration struct holding all configurable parameters.
 * This struct is used throughout the application to get settings.
 */
#[derive(Debug)]
pub struct Config {
    // Server binding info
    pub port: u16,
    pub host: String,
    pub external_url: String,
    pub use_https: bool,

    // Download directory and cleanup timing (in minutes)
    pub download_dir: String,
    pub cleanup_after_minutes: u64,

    // Video and audio quality and codec preferences
    pub video_quality: VideoQuality,
    pub video_codec: VideoCodecPreference,
    pub audio_quality: AudioQuality,
    pub audio_codec: AudioCodecPreference,

    // Performance tuning parameters
    pub max_concurrent_downloads: usize,
    pub timeout_seconds: u64,
}

/*
 * Default values for the configuration.
 * These are used if no environment variable is set.
 */
impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "0.0.0.0".to_string(),
            external_url: String::new(),
            use_https: false,
            download_dir: "./downloads".to_string(),
            cleanup_after_minutes: 10,
            video_quality: VideoQuality::Best,
            video_codec: VideoCodecPreference::VP9,
            audio_quality: AudioQuality::Best,
            audio_codec: AudioCodecPreference::Opus,
            max_concurrent_downloads: 5,
            timeout_seconds: 300, // 5 minutes
        }
    }
}

impl Config {
    /*
     * Load configuration from environment variables.
     * Falls back to default values when environment variables are missing or invalid.
     */
    pub fn from_env() -> Self {
        dotenv().ok(); // Load .env file if present
        let default = Self::default();

        Self {
            port: parse_env("PORT", default.port),
            host: parse_env("HOST", default.host),
            external_url: parse_env("EXTERNAL_URL", default.external_url),
            use_https: parse_env("USE_HTTPS", default.use_https),
            download_dir: parse_env("DOWNLOAD_DIR", default.download_dir),
            cleanup_after_minutes: parse_env(
                "CLEANUP_AFTER_MINUTES",
                default.cleanup_after_minutes,
            ),
            video_quality: parse_env_enum("VIDEO_QUALITY", VideoQualityEnv::Best).into(),
            video_codec: video_codec.into(),
            audio_quality: parse_env_enum("AUDIO_QUALITY", AudioQualityEnv::Best).into(),
            audio_codec: audio_codec.into(),
            max_concurrent_downloads: parse_env(
                "MAX_CONCURRENT_DOWNLOADS",
                default.max_concurrent_downloads,
            ),
            timeout_seconds: parse_env("TIMEOUT_SECONDS", default.timeout_seconds),
        }
    }

    /*
     * Helper method to get the full address string (host:port)
     * for server binding or connection.
     */
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/*
 * Generic helper to parse an environment variable of type T.
 * Returns default value if the variable is missing or parsing fails.
 */
fn parse_env<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr + Clone,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

/*
 * Helper to parse environment variables into enums (which implement FromStr).
 * Returns default enum value if parsing fails.
 */
fn parse_env_enum<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    env::var(key)
        .ok()
        .and_then(|v| v.to_lowercase().parse().ok())
        .unwrap_or(default)
}
