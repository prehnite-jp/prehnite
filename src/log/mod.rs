use crate::util::app_global::global_dir;
use std::path::{Path, PathBuf};
use tracing::Level;
use tracing_appender::rolling;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::fmt::writer::MakeWriterExt;
use tracing_subscriber::EnvFilter;

pub const ERROR_LOG_FILENAME: &str = "error.log";
pub const INFO_LOG_FILENAME: &str = "info.log";
#[cfg(debug_assertions)]
pub const DEBUG_LOG_FILENAME: &str = "debug.log";

pub fn log_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        ".".into()
    } else {
        let mut dir = global_dir();
        dir.push("log");
        dir
    }
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
) -> rolling::RollingFileAppender {
    rolling::Builder::new()
        .rotation(rotation)
        .filename_prefix(filename_prefix)
        .max_log_files(MAX_LOG_FILES)
        .build(directory)
        .expect("Failed to initialize logger.")
}

pub fn initialize_logger() {
    const ERROR_LOG_FILE_LEVEL: Level = Level::ERROR;
    const INFO_LOG_FILE_LEVEL: Level = Level::INFO;

    // 標準出力にはすべての情報を出力します。
    let stdout_logger = std::io::stdout.with_max_level(Level::TRACE);

    // デバッグログにはすべての情報を出力します。
    #[cfg(debug_assertions)]
    let stdout_logger = stdout_logger
        .and(rolling::never(log_dir(), DEBUG_LOG_FILENAME).with_max_level(Level::TRACE));

    let error_appender = appender(ROTATION_CYCLE, log_dir(), ERROR_LOG_FILENAME)
        .with_max_level(ERROR_LOG_FILE_LEVEL);

    let info_appender =
        appender(ROTATION_CYCLE, log_dir(), INFO_LOG_FILENAME).with_max_level(INFO_LOG_FILE_LEVEL);

    let writer = stdout_logger.and(error_appender.and(info_appender));

    tracing_subscriber::fmt::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::TRACE.into())
                .from_env_lossy(),
        )
        .with_ansi(false)
        .with_file(cfg!(debug_assertions))
        .with_writer(writer)
        .init();
}
