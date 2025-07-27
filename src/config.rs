use yt_dlp::model::{AudioCodecPreference, AudioQuality, VideoCodecPreference, VideoQuality};

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
            video_quality: VideoQuality::High,
            video_codec: VideoCodecPreference::VP9,
            audio_quality: AudioQuality::High,
            audio_codec: AudioCodecPreference::Opus,
        }
    }

    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
