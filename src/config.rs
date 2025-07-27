use dotenvy::dotenv;
use std::env;
use strum_macros::{EnumIter, EnumString};
use yt_dlp::model::{AudioCodecPreference, AudioQuality, VideoCodecPreference, VideoQuality};

// Environment-parseable enums (with FromStr via strum)
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
#[strum(serialize_all = "PascalCase")]
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
#[strum(serialize_all = "PascalCase")]
pub enum AudioCodecPreferenceEnv {
    Opus,
    AAC,
    MP3,
    Any,
}

// Convert env enums to yt_dlp enums
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

#[derive(Debug)]
pub struct Config {
    // Server
    pub port: u16,
    pub host: String,

    // Download paths & cleanup
    pub download_dir: String,
    pub cleanup_after_minutes: u64,

    // Quality settings
    pub video_quality: VideoQuality,
    pub video_codec: VideoCodecPreference,
    pub audio_quality: AudioQuality,
    pub audio_codec: AudioCodecPreference,

    // Performance
    pub max_concurrent_downloads: usize,
    pub timeout_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "127.0.0.1".to_string(),
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
    pub fn from_env() -> Self {
        dotenv().ok();
        let default = Self::default();

        Self {
            port: parse_env("PORT", default.port),
            host: parse_env("HOST", default.host),
            download_dir: parse_env("DOWNLOAD_DIR", default.download_dir),
            cleanup_after_minutes: parse_env(
                "CLEANUP_AFTER_MINUTES",
                default.cleanup_after_minutes,
            ),
            video_quality: parse_env_enum("VIDEO_QUALITY", VideoQualityEnv::Best).into(),
            video_codec: parse_env_enum("VIDEO_CODEC", VideoCodecPreferenceEnv::VP9).into(),
            audio_quality: parse_env_enum("AUDIO_QUALITY", AudioQualityEnv::Best).into(),
            audio_codec: parse_env_enum("AUDIO_CODEC", AudioCodecPreferenceEnv::Opus).into(),
            max_concurrent_downloads: parse_env(
                "MAX_CONCURRENT_DOWNLOADS",
                default.max_concurrent_downloads,
            ),
            timeout_seconds: parse_env("TIMEOUT_SECONDS", default.timeout_seconds),
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn parse_env<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr + Clone,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}

fn parse_env_enum<T>(key: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
