use std::fmt;
use std::io::IsTerminal;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use axum::extract::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::field::{Field, Visit};
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent, FormatFields};
use tracing_subscriber::registry::LookupSpan;

use crate::config::Config;

/*
 * Console output for Snatchr.
 *
 * Provides three things:
 * - A compact, colorized tracing formatter (`HH:MM:SS LEVEL target message key=value`).
 * - A startup banner that mirrors the web UI's violet-to-cyan palette.
 * - An HTTP middleware that logs every request with status and latency.
 *
 * Colors are automatically disabled when stdout is not a terminal.
 * The log level can be overridden with the RUST_LOG environment variable.
 */

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const GRAY: &str = "\x1b[38;5;244m";
const VIOLET: &str = "\x1b[38;2;167;139;250m";
const CYAN: &str = "\x1b[38;2;103;232;249m";
const GREEN: &str = "\x1b[38;2;52;211;153m";
const AMBER: &str = "\x1b[38;2;251;191;36m";
const ROSE: &str = "\x1b[38;2;251;113;133m";
const BLUE: &str = "\x1b[38;2;147;197;253m";

/// Initializes the global tracing subscriber with the custom formatter.
pub fn init() {
    let ansi = std::io::stdout().is_terminal();
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .event_format(SnatchrFormatter { ansi })
        .init();
}

struct SnatchrFormatter {
    ansi: bool,
}

impl SnatchrFormatter {
    fn paint(&self, code: &str, text: &str) -> String {
        if self.ansi {
            format!("{code}{text}{RESET}")
        } else {
            text.to_string()
        }
    }

    fn level_badge(&self, level: &Level) -> String {
        let (code, label) = match *level {
            Level::ERROR => (ROSE, "ERROR"),
            Level::WARN => (AMBER, " WARN"),
            Level::INFO => (GREEN, " INFO"),
            Level::DEBUG => (BLUE, "DEBUG"),
            Level::TRACE => (GRAY, "TRACE"),
        };
        if self.ansi {
            format!("{BOLD}{code}{label}{RESET}")
        } else {
            label.to_string()
        }
    }
}

impl<S, N> FormatEvent<S, N> for SnatchrFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let mut visitor = EventVisitor::default();
        event.record(&mut visitor);

        let (h, m, s) = clock_hms();
        let time = format!("{h:02}:{m:02}:{s:02}");

        // Shorten "snatchr::utils::cleanup" to "cleanup" and pad for column alignment
        let target = event
            .metadata()
            .target()
            .rsplit("::")
            .next()
            .unwrap_or("app");
        let target = format!("{:<10.10}", target);

        write!(
            writer,
            "{} {} {} ",
            self.paint(GRAY, &time),
            self.level_badge(event.metadata().level()),
            self.paint(VIOLET, &target),
        )?;

        if visitor.fields.is_empty() {
            write!(writer, "{}", visitor.message)?;
        } else {
            // Pad short messages so field columns roughly line up across lines
            write!(writer, "{:<36}", visitor.message)?;
            for (key, value) in &visitor.fields {
                write!(
                    writer,
                    " {}{}{}",
                    self.paint(CYAN, key),
                    self.paint(GRAY, "="),
                    self.paint(DIM, value),
                )?;
            }
        }

        writeln!(writer)
    }
}

#[derive(Default)]
struct EventVisitor {
    message: String,
    fields: Vec<(&'static str, String)>,
}

impl Visit for EventVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = value.to_string();
        } else {
            self.fields.push((field.name(), value.to_string()));
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        if field.name() == "message" {
            self.message = format!("{value:?}");
        } else {
            self.fields.push((field.name(), format!("{value:?}")));
        }
    }
}

// Wall clock as UTC hours/minutes/seconds, no chrono dependency needed
fn clock_hms() -> (u64, u64, u64) {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let day = secs % 86_400;
    (day / 3_600, (day % 3_600) / 60, day % 60)
}

/// Axum middleware: logs every request as `METHOD /path status=… ms=…`.
/// Level escalates with the response class: 2xx/3xx info, 4xx warn, 5xx error.
pub async fn log_requests(req: Request, next: Next) -> Response {
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let start = Instant::now();

    let response = next.run(req).await;

    let status = response.status().as_u16();
    let ms = start.elapsed().as_millis();

    if response.status().is_server_error() {
        tracing::error!(target: "http", status, ms, "{method} {path}");
    } else if response.status().is_client_error() {
        tracing::warn!(target: "http", status, ms, "{method} {path}");
    } else {
        tracing::info!(target: "http", status, ms, "{method} {path}");
    }

    response
}

/// Prints the startup banner with the resolved configuration.
pub fn print_banner(config: &Config) {
    let ansi = std::io::stdout().is_terminal();
    let paint = |code: &str, text: &str| {
        if ansi {
            format!("{code}{text}{RESET}")
        } else {
            text.to_string()
        }
    };
    let rule = gradient_rule(52, ansi);
    let row = |label: &str, value: String| {
        println!("  {} {}", paint(GRAY, &format!("{label:<12}")), value);
    };

    let scheme = if config.use_https { "https" } else { "http" };
    let config_source = if std::env::var("DOCKER_ENV").is_ok() {
        "Docker environment".to_string()
    } else if std::fs::read_to_string(".env").is_ok() {
        ".env loaded".to_string()
    } else {
        "defaults (no .env found — copy .env.example)".to_string()
    };

    println!();
    println!(
        "  🎬 {} {}",
        paint(&format!("{BOLD}{VIOLET}"), "SNATCHR"),
        paint(GRAY, &format!("v{}", env!("CARGO_PKG_VERSION"))),
    );
    println!("  {rule}");
    row(
        "Server",
        paint(CYAN, &format!("{scheme}://{}", config.address())),
    );
    if !config.external_url.is_empty() {
        row("Public URL", paint(CYAN, &config.external_url));
    }
    row(
        "Web UI",
        if config.enable_web_ui {
            paint(GREEN, "enabled")
        } else {
            paint(AMBER, "disabled (API only)")
        },
    );
    row(
        "Downloads",
        format!(
            "{} {}",
            config.download_dir,
            paint(
                GRAY,
                &format!("· cleaned every {} min", config.cleanup_after_minutes)
            ),
        ),
    );
    row(
        "Limits",
        format!(
            "{} concurrent {} {}s timeout",
            config.max_concurrent_downloads,
            paint(GRAY, "·"),
            config.timeout_seconds,
        ),
    );
    row(
        "Video",
        format!(
            "{:?} quality {} {:?} codec",
            config.video_quality,
            paint(GRAY, "·"),
            config.video_codec,
        ),
    );
    row(
        "Audio",
        format!(
            "{:?} quality {} {:?} codec",
            config.audio_quality,
            paint(GRAY, "·"),
            config.audio_codec,
        ),
    );
    row("Config", config_source);
    println!("  {rule}");
    println!(
        "  {} {}",
        paint(&format!("{BOLD}{GREEN}"), "Ready to snatch."),
        paint(GRAY, "Ctrl+C to stop · RUST_LOG=debug for more detail"),
    );
    println!();
}

// A horizontal rule that fades from violet to cyan, echoing the web UI gradient
fn gradient_rule(width: usize, ansi: bool) -> String {
    if !ansi {
        return "─".repeat(width);
    }
    let from = (139.0_f32, 92.0, 246.0);
    let to = (34.0_f32, 211.0, 238.0);
    let mut out = String::new();
    for i in 0..width {
        let t = i as f32 / (width.max(2) - 1) as f32;
        let r = (from.0 + (to.0 - from.0) * t) as u8;
        let g = (from.1 + (to.1 - from.1) * t) as u8;
        let b = (from.2 + (to.2 - from.2) * t) as u8;
        out.push_str(&format!("\x1b[38;2;{r};{g};{b}m━"));
    }
    out.push_str(RESET);
    out
}
