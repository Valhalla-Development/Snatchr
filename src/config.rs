use dotenvy::dotenv;
use std::env;
use strum_macros::{EnumIter, EnumString};
use yt_dlp::model::{AudioCodecPreference, AudioQuality, VideoCodecPreference, VideoQuality};

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
    pub port: u16,
    pub host: String,
    pub video_quality: VideoQuality,
    pub video_codec: VideoCodecPreference,
    pub audio_quality: AudioQuality,
    pub audio_codec: AudioCodecPreference,
}

impl Config {
    pub fn new() -> Self {
        Self {
            port: 3000,
            host: "127.0.0.1".to_string(),
            video_quality: VideoQuality::Best,
            video_codec: VideoCodecPreference::VP9,
            audio_quality: AudioQuality::Best,
            audio_codec: AudioCodecPreference::Opus,
        }
    }

    fn parse_env_enum<T, U>(key: &str, default: T) -> U
    where
        T: std::str::FromStr,
        U: From<T>,
    {
        let env_val = env::var(key)
            .ok()
            .and_then(|v| v.parse::<T>().ok())
            .unwrap_or_else(|| default);

        env_val.into()
    }

    pub fn from_env() -> Self {
        dotenv().ok();

        let port = env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(3000);

        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let video_quality = Self::parse_env_enum("VIDEO_QUALITY", VideoQualityEnv::Worst);
        let video_codec = Self::parse_env_enum("VIDEO_CODEC", VideoCodecPreferenceEnv::Any);
        let audio_quality = Self::parse_env_enum("AUDIO_QUALITY", AudioQualityEnv::Worst);
        let audio_codec = Self::parse_env_enum("AUDIO_CODEC", AudioCodecPreferenceEnv::Any);

        Config {
            port,
            host,
            video_quality,
            video_codec,
            audio_quality,
            audio_codec,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
