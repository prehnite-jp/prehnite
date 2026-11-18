use tantivy::schema::document::DeserializeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimeBridgeError {
    #[error("Error: Failed to tantivy::DateTime::try_from_utc")]
    TryIntoTantivy,
    #[error("Error: OwnedValue is not datetime.")]
    TryIntoTimeFromOwnedValue,
    #[error("TimeErrorComponentRange: {0}")]
    TimeErrorComponentRange(#[from] time::error::ComponentRange),
    #[error("JiffError: {0}")]
    JiffError(#[from] jiff::Error),
}

impl From<TimeBridgeError> for DeserializeError {
    fn from(value: TimeBridgeError) -> Self {
        DeserializeError::Custom(value.to_string())
    }
}
