use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DatabaseError: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("MigrationError: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("Tantivy Error: {0}")]
    TantivyError(#[from] tantivy::TantivyError),
    #[error("SerdeJson Error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("The data manager has already been closed.")]
    DataManagerClosed,
    #[error("prehnite id was missing on metadata table")]
    PrehniteIdMissing,
    #[error("TimeBridgeError: {0}")]
    TimeBridgeError(#[from] prehnite_time_bridge::TimeBridgeError),
    #[error("FailedToParseTantivyQuery: {0}")]
    TantivyQueryParseError(#[from] tantivy::query::QueryParserError),
    #[error("FailedToParseTantivyQuery: {0}")]
    TantivyQueryParseErrorQueryGrammar(String),
    #[error("FailedToOpenIndexDirectory: {0}")]
    TantivyOpenDirError(#[from] tantivy::directory::error::OpenDirectoryError),
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Tag {0} is missing.")]
    MissingTag(String),
    #[error("jiff error: {0}")]
    JiffError(#[from] prehnite_time_bridge::jiff::Error),
}
