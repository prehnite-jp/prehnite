#![doc = "ロギングの実装"]
use crate::env::ENV_KEY_LOG;
use crate::constants::global_dir;
use std::path::{Path, PathBuf};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::filter::Directive;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::EnvFilter;

pub use rolling::InitError;

const DEFAULT_LOG_LEVEL: &str = "info";

#[cfg(debug_assertions)]
const DEFAULT_LOG_FILTER: &[&str] = &[
    "sqlx::query=trace",
    "iced_wgpu::window::compositor=warn",
    "iced_winit=warn",
];
#[cfg(not(debug_assertions))]
const DEFAULT_LOG_FILTER: &[&str] = &["iced_wgpu::window::compositor=warn", "iced_winit=warn"];

const ERROR_LOG_FILE_LEVEL: Level = Level::ERROR;
const INFO_LOG_FILE_LEVEL: Level = Level::INFO;
const STDOUT_LOG_LEVEL: Level = Level::TRACE;
const STDERR_LOG_LEVEL: Level = Level::WARN;
const DEBUG_LOG_FILE_LEVEL: Level = Level::TRACE;

pub const ERROR_LOG_FILENAME: &str = "error.log";
pub const INFO_LOG_FILENAME: &str = "info.log";
#[cfg(debug_assertions)]
pub const DEBUG_LOG_FILENAME: &str = "debug.log";

#[cfg(debug_assertions)]
const LOG_DIR_NAME: &str = "";
#[cfg(not(debug_assertions))]
const LOG_DIR_NAME: &str = "log";

/// ログディレクトリのパスを取得します。
pub fn log_dir() -> PathBuf {
    let mut dir = global_dir();
    dir.push(LOG_DIR_NAME);
    dir
}

const MAX_LOG_FILES: usize = 5;

#[cfg(debug_assertions)]
const ROTATION_CYCLE: rolling::Rotation = rolling::Rotation::NEVER;
#[cfg(not(debug_assertions))]
const ROTATION_CYCLE: rolling::Rotation = rolling::Rotation::WEEKLY;

fn appender(
    rotation: rolling::Rotation,
    directory: impl AsRef<Path>,
    filename_prefix: impl Into<String>,
) -> Result<rolling::RollingFileAppender, InitError> {
    Ok(rolling::Builder::new()
        .rotation(rotation)
        .filename_prefix(filename_prefix)
        .max_log_files(MAX_LOG_FILES)
        .build(directory)?)
}

fn default_env_filter() -> EnvFilter {
    EnvFilter::try_from_env(ENV_KEY_LOG).unwrap_or(EnvFilter::new(DEFAULT_LOG_LEVEL))
}

fn add_directive(env_filter: EnvFilter, directive: Directive) -> EnvFilter {
    env_filter.add_directive(directive)
}

/// ロガーを初期化します。
pub fn initialize_logger() -> Result<(), InitError> {
    let default_log_filter: EnvFilter = DEFAULT_LOG_FILTER
        .iter()
        .map(|v| v.parse().unwrap())
        .fold(default_env_filter(), add_directive);

    let stdout_logger = std::io::stdout.with_max_level(STDOUT_LOG_LEVEL);
    let stderr_logger = std::io::stderr.with_max_level(STDERR_LOG_LEVEL);
    let error_appender = appender(ROTATION_CYCLE, log_dir(), ERROR_LOG_FILENAME)?
        .with_max_level(ERROR_LOG_FILE_LEVEL);
    let info_appender =
        appender(ROTATION_CYCLE, log_dir(), INFO_LOG_FILENAME)?.with_max_level(INFO_LOG_FILE_LEVEL);

    let info_debug_appender = info_appender;
    #[cfg(debug_assertions)]
    let info_debug_appender = info_debug_appender.or_else(
        rolling::never(log_dir(), DEBUG_LOG_FILENAME).with_max_level(DEBUG_LOG_FILE_LEVEL),
    );

    let writer = stderr_logger
        .or_else(stdout_logger)
        .and(error_appender.or_else(info_debug_appender));

    Ok(tracing_subscriber::fmt::fmt()
        .with_env_filter(default_log_filter)
        .with_ansi(false)
        .with_file(cfg!(debug_assertions))
        .with_line_number(cfg!(debug_assertions))
        .with_writer(writer)
        .init())
}
