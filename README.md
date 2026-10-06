<div align="center">
  <img id="top" src="https://share.valhalladev.org/u/Snatchr.png" width="100%" alt="Snatchr Banner">

# 🎬 Snatchr: Lightning-Fast Video Downloader! ⚡

  <p>
    <a href="https://discord.gg/Q3ZhdRJ"><img src="https://img.shields.io/discord/495602800802398212.svg?colorB=5865F2&logo=discord&logoColor=white&style=for-the-badge" alt="Discord"></a>
    <a href="https://github.com/Valhalla-Development/Snatchr/stargazers"><img src="https://img.shields.io/github/stars/Valhalla-Development/Snatchr.svg?style=for-the-badge&color=yellow" alt="Stars"></a>
    <a href="https://github.com/Valhalla-Development/Snatchr/network/members"><img src="https://img.shields.io/github/forks/Valhalla-Development/Snatchr.svg?style=for-the-badge&color=orange" alt="Forks"></a>
    <a href="https://github.com/Valhalla-Development/Snatchr/issues"><img src="https://img.shields.io/github/issues/Valhalla-Development/Snatchr.svg?style=for-the-badge&color=red" alt="Issues"></a>
    <a href="https://github.com/Valhalla-Development/Snatchr/blob/main/LICENSE"><img src="https://img.shields.io/github/license/Valhalla-Development/Snatchr.svg?style=for-the-badge&color=blue" alt="License"></a>
    <br>
    <a href="https://app.codacy.com/gh/Valhalla-Development/Snatchr/dashboard?utm_source=gh&utm_medium=referral&utm_content=&utm_campaign=Badge_grade"><img src="https://img.shields.io/codacy/grade/c9e654da36684620b3f6ef6d6afa8216?style=for-the-badge&color=brightgreen" alt="Codacy"></a>
    <a href="#"><img src="https://img.shields.io/badge/Powered%20by-Axum-5865F2?style=for-the-badge&logo=rust&logoColor=white" alt="Powered by Axum"></a>
    <a href="#"><img src="https://img.shields.io/badge/Made%20with-Rust-3178C6?style=for-the-badge&logo=rust&logoColor=white" alt="Made with Rust"></a>
  </p>

  <p><em>A blazing-fast HTTP API for downloading video and audio from multiple sites with automatic cleanup, file serving, and a modern web interface!</em></p>
</div>

---
## 🌟 Welcome to Snatchr, the Ultimate Video Download API!

This project provides a robust HTTP API for downloading video and audio from multiple sites using [yt-dlp](https://github.com/yt-dlp/yt-dlp), built with [Rust](https://www.rust-lang.org/) and [Axum](https://github.com/tokio-rs/axum) for maximum performance and reliability.

## Confirmed working sources

Public examples from these sources have been downloaded and decoded by FFmpeg in the live test suite:

| Source | Verified media |
| --- | --- |
| YouTube | Video with audio |
| TikTok | Video |
| Instagram | Public reel |
| SoundCloud | Audio track |
| Facebook | Public video |
| X / Twitter | Public video |
| Reddit | Video with separate audio and silent video |

These checks confirm the tested examples. Individual posts can require login, expire, or become unavailable. Other yt-dlp sources may work, but are not part of this confirmed list.

If a source is broken, [open an issue](https://github.com/Valhalla-Development/Snatchr/issues/new) with a public URL, the Snatchr revision, your yt-dlp binary version, and the error. We will investigate and fix it. Remove cookies, tokens, and signed CDN URLs from any logs you share.

## Why Snatchr uses a Rust yt-dlp fork

Snatchr uses [our Rust wrapper fork](https://github.com/Valhalla-Development/yt-dlp), based on [boul2gom/yt-dlp](https://github.com/boul2gom/yt-dlp), together with the [Python yt-dlp binary](https://github.com/yt-dlp/yt-dlp). Python yt-dlp supplies the site extractors. The Rust fork fixes how Snatchr reads metadata, selects formats, and downloads media: optional metadata, decimal durations and timestamps, unlabeled MP4 formats, protected CDN requests, DASH/HLS merging, silent video, audio-only sources, and output-directory handling.

The fork retains upstream improvements while keeping these compatibility fixes. The crates.io package does not include our fork changes. Snatchr's Git dependency and `Cargo.lock` select the wrapper revision; the downloaded Python binary has its own version.

## 🎮 Features That Power Your Downloads

<table>
  <tr>
    <td width="50%">
      <h3>🚀 Lightning-Fast Downloads</h3>
      <p>Powered by yt-dlp with concurrent downloads and optimized performance settings.</p>
    </td>
    <td width="50%">
      <h3>📁 Automatic File Serving</h3>
      <p>Direct HTTP access to downloaded files with streaming support.</p>
    </td>
  </tr>
  <tr>
    <td width="50%">
      <h3>🌐 Modern Web Interface</h3>
      <p>Beautiful, responsive web UI with video preview, download history, and drag-and-drop support.</p>
    </td>
    <td width="50%">
      <h3>🔧 Configurable Quality</h3>
      <p>Choose video/audio quality, codecs, and format preferences through environment configuration.</p>
    </td>
  </tr>
  <tr>
    <td width="50%">
      <h3>🧹 Smart Cleanup System</h3>
      <p>Automated cleanup of old downloads with configurable retention periods and scheduling.</p>
    </td>
    <td width="50%">
      <h3>⚡ Intelligent Caching</h3>
      <p>Reuse completed media by source ID while it remains cached.</p>
    </td>
  </tr>
  <tr>
    <td width="50%">
      <h3>📊 Comprehensive Logging</h3>
      <p>Detailed request tracking, error handling, and performance monitoring with structured logging.</p>
    </td>
    <td width="50%">
      <h3>🎯 Request Tracking</h3>
      <p>Unique job IDs for each request with full traceability from HTTP handler to download completion.</p>
    </td>
  </tr>
</table>

## 🚀 Requirements

- [Rust](https://rustup.rs/) with the current stable toolchain (edition 2024 and locked dependencies)

## 🛠️ Setup Guide

## 🌐 Web Interface

Snatchr includes a beautiful, modern web interface for easy video downloading!

https://github.com/user-attachments/assets/77745296-b7db-4814-93c2-5002b0eabd56

### ✨ Features
- **🎬 Instant Video Preview** - Watch downloaded videos directly in the browser with our sleek video player
- **📋 Smart Download History** - Keep track of all your previous downloads with timestamps and file info
- **🎨 Modern & Responsive** - Beautiful design that works perfectly on desktop, tablet, and mobile
- **⚡ Lightning-Fast** - Just paste a supported media URL and watch your video download in seconds
- **🎯 One-Click Downloads** - No complex settings, just pure simplicity and speed

### 🚀 How to Use
1. Start your Snatchr server
2. Open your browser and navigate to `http://localhost:3000`
3. Paste a supported video or audio URL in the elegant input field
4. Click "Download Video" and watch the magic happen!

### 🎯 Pro Tips
- **📋 Clipboard Integration** - Use the "Paste" button for instant URL pasting from your clipboard
- **🎬 Instant Preview** - Downloaded videos appear immediately in the video player for instant gratification
- **📱 Mobile-First** - The interface is optimized for mobile devices with touch-friendly controls
- **🔄 Auto-Refresh** - The interface automatically updates to show your latest downloads

### ⚠️ Important Note
If you're running locally and your server is bound to `0.0.0.0`, some browsers and extensions may block video playback. Use `localhost` in your browser URL instead of `0.0.0.0` for the best experience.

### 🔧 Configuration
You can disable the web interface by setting the environment variable:
```bash
ENABLE_WEB_UI=false
```
When disabled, only the API endpoints will be available.

## 📱 iOS Shortcut (Beta)

Download videos directly from your Apple device with our iOS Shortcut integration!

**🔗 Download:** [Snatchr iOS Shortcut](https://www.icloud.com/shortcuts/816a21dbfeba4e078397a9d1c48cef4e)

### ⚙️ Setup Instructions

1. **Configure Server URL:** After downloading, open the shortcut and update the `EXTERNAL_URL` in the dictionary at the top
   - Include the `/download` endpoint (e.g., `https://your-server.com/download`)

2. **Codec Compatibility:** For automatic saving to your photo gallery, ensure your Snatchr instance uses compatible codecs:
   - **Video:** `AVC1` (recommended)
   - **Audio:** `AAC` (recommended)

3. **Alternative Save Method:** If you experience issues with gallery saving, edit the shortcut and change `Save URL to Recents` to `Save File` instead

> **Note:** This shortcut is currently in beta. Please report any issues on our [GitHub Issues](https://github.com/Valhalla-Development/Snatchr/issues) page. 


<details>
<summary>🐳 Deploy with Docker</summary>
<br>
1. Download the <a href="https://github.com/Valhalla-Development/Snatchr/blob/main/docker-compose.yml">docker-compose.yml</a> file or clone the repository:

   ```bash
   git clone https://github.com/Valhalla-Development/Snatchr.git
   cd Snatchr
   ```

2. Review and modify the environment variables in docker-compose.yml if needed (PORT, DOWNLOAD_DIR, etc.)

3. Run with Docker Compose:
   ```bash
   docker-compose up -d
   ```

4. The server will be available at <a href="http://localhost:3000">http://localhost:3000</a>
</details>

<details>
<summary>🚀 Build Locally (Rust)</summary>
<br>
1. <a href="https://github.com/Valhalla-Development/Snatchr/releases">Download</a> the latest release or clone the repository:

   ```bash
   git clone https://github.com/Valhalla-Development/Snatchr.git
   ```

2. Navigate to the project directory:
   ```bash
   cd Snatchr
   ```

3. Install <a href="https://www.rust-lang.org/">Rust</a> (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

4. Rename or copy the `.env.example` > `.env` and fill in the required variables.

5. Build the project:
   ```bash
   cargo build --release
   ```

6. Run the server:
   ```bash
   cargo run --release
   ```
</details>

## 📡 API Usage

### Download video or audio
```bash
curl -X POST "http://localhost:3000/download" \
  -H "Content-Type: application/json" \
  -d '{"url": "https://www.youtube.com/watch?v=dQw4w9WgXcQ"}'
```

**Response:**
```json
{
  "success": true,
  "file_url": "http://localhost:3000/files/dQw4w9WgXcQ/file_name.mp4"
}
```

Send one public media URL in the `url` field. Snatchr downloads one item rather than a playlist. Video responses normally point to an MP4; audio-only sources such as SoundCloud retain the selected audio container, for example M4A. Use the returned `file_url` instead of assuming a file extension.

Check the JSON `success` field. Download errors can return HTTP 200 with `success: false` and an `error` message. A successful response means the file is available at the returned URL until cleanup removes it.

## 🧹 Automatic Cleanup

The server automatically cleans up old downloads based on your `CLEANUP_AFTER_MINUTES` setting. Cleanup runs periodically in the background and logs all operations.

## ⚡ Smart Caching System

Snatchr features intelligent video caching that dramatically improves performance:

### 🎯 How It Works
- **Video ID Caching**: Media is stored using a platform ID extracted from the URL when possible, otherwise the canonical ID returned by yt-dlp
- **Instant Returns**: If completed media is already cached, it's returned immediately without re-downloading
- **Bandwidth Savings**: Reuse downloads until cleanup removes them
- **Basic Validation**: Incomplete temporary files and files below the minimum size are excluded from cache hits

### 📁 Cache Structure
```
downloads/
├── dQw4w9WgXcQ/          # Video ID directory
│   └── video_title.mp4   # Cached video file
├── zwMEhBq4kYM/          # Another video ID
│   └── another_video.mp4
└── cache/                # System cache (preserved)
```

### 🚀 Performance Benefits
- **First Download**: Normal speed (downloads and caches)
- **Subsequent Downloads**: Instant! (returns cached file)
- **Multiple Users**: Share cached content across all users
- **Server Efficiency**: Reduced bandwidth and processing load

## Tests

Run the regular regression tests:

```bash
cargo test --locked
```

The opt-in tests download actual public examples from all seven confirmed sources, decode the media with `libs/ffmpeg`, and verify cache reuse. Reddit includes both an audio/video example and a silent example. Ensure `libs/yt-dlp` and `libs/ffmpeg` are installed, then run serially because the tests temporarily change process-wide configuration:

```bash
cargo test --locked downloads_real_ -- --ignored --test-threads=1
```

To replace an expired public example, set `SNATCHR_TEST_YOUTUBE_URL`, `SNATCHR_TEST_TIKTOK_URL`, `SNATCHR_TEST_INSTAGRAM_URL`, `SNATCHR_TEST_SOUNDCLOUD_URL`, `SNATCHR_TEST_FACEBOOK_URL`, `SNATCHR_TEST_X_URL`, `SNATCHR_TEST_REDDIT_URL`, or `SNATCHR_TEST_REDDIT_SILENT_URL`. Live checks are ignored by default because site availability can change independently of Snatchr.

## 🤝 Contributing

We welcome contributions to improve Snatchr! If you'd like to contribute:

1. Fork the repository
2. Create a new branch for your feature or bug fix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes and commit them with a clear, descriptive message:
   ```bash
   git commit -m '✨ (downloads): Add support for a source'
   ```
4. Push your changes to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
5. Open a Pull Request against the main repository's `main` branch

Please ensure your code adheres to the project's coding standards and include tests for new features or bug fixes where applicable. We appreciate detailed descriptions in your Pull Request to help with the review process.

## 📜 License

This project is licensed under the GPL-3.0 License - see the LICENSE file for details. (It's mostly "Share the love, and keep it open!")

## 🙏 Acknowledgements

- [yt-dlp](https://github.com/yt-dlp/yt-dlp) for the powerful video download engine
- [Axum](https://github.com/tokio-rs/axum) for the high-performance web framework
- [Tokio](https://tokio.rs/) for the async runtime
- [Rust](https://www.rust-lang.org/) for the blazing-fast and memory-safe foundation
- All contributors who help improve this project

## 🚀 What's Next?

We're constantly working to make Snatchr even better! Here's what we're cooking up:

### 🎯 Upcoming Features
- **🌐 More Confirmed Sources** - Expand live coverage and fix reported source failures

### 🤝 Community Ideas
Have a feature request or idea? We'd love to hear it! Open an issue or join our [Discord](https://discord.gg/Q3ZhdRJ) to discuss.

---

## 📬 Support & Community

Got questions or need help? Join our [Discord server](https://discord.gg/Q3ZhdRJ) for support and to connect with other bot developers!

---

<div align="center">

💻 Crafted with ❤️ by [Valhalla-Development](https://github.com/Valhalla-Development)

[🐛 Spotted an issue?](https://github.com/Valhalla-Development/Snatchr/issues/new?assignees=&labels=bug&projects=&template=bug_report.yml&title=%5BBUG%5D+Short+Description) | [💡 Got an idea?](https://github.com/Valhalla-Development/Snatchr/issues/new?assignees=&labels=enhancement&projects=&template=feature_request.yml&title=%5BFeature%5D+Short+Description) | [🤔 Need help?](https://discord.gg/Q3ZhdRJ)

<a href="#top">🔝 Back to Top</a>
</div>
