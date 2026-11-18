mod error;

pub use crate::error::*;
pub use jiff;
pub use time;

use tantivy::schema::Value;
use time::UtcOffset;

pub trait TryIntoJiff {
    fn try_into_jiff(self) -> Result<jiff::Timestamp, TimeBridgeError>;
    fn try_into_jiff_opt(self) -> Option<jiff::Timestamp>
    where
        Self: Sized,
    {
        self.try_into_jiff().ok()
    }
}

pub trait TryIntoTime {
    fn try_into_time(self) -> Result<time::Timestamp, TimeBridgeError>;
    fn try_into_time_opt(self) -> Option<time::Timestamp>
    where
        Self: Sized,
    {
        self.try_into_time().ok()
    }
}

pub trait TryIntoTantivyDate {
    fn try_into_tantivy_date(self) -> Result<tantivy::DateTime, TimeBridgeError>;
    fn try_into_tantivy_date_opt(self) -> Option<tantivy::DateTime>
    where
        Self: Sized,
    {
        self.try_into_tantivy_date().ok()
    }
}

impl TryIntoTime for &tantivy::schema::OwnedValue {
    fn try_into_time(self) -> Result<time::Timestamp, TimeBridgeError> {
        Ok(self
            .as_datetime()
            .ok_or(TimeBridgeError::TryIntoTimeFromOwnedValue)?
            .try_into_time()?)
    }
}

impl TryIntoTime for tantivy::DateTime {
    fn try_into_time(self) -> Result<time::Timestamp, TimeBridgeError> {
        Ok(time::Timestamp::from_nanoseconds(
            self.into_utc().unix_timestamp_nanos(),
        )?)
    }
}

impl TryIntoTime for jiff::Timestamp {
    fn try_into_time(self) -> Result<time::Timestamp, TimeBridgeError> {
        Ok(time::Timestamp::from_nanoseconds(self.as_nanosecond())?)
    }
}

impl TryIntoTime for time::Timestamp {
    fn try_into_time(self) -> Result<time::Timestamp, TimeBridgeError> {
        Ok(self)
    }
}

impl<T: TryIntoTime> TryIntoTantivyDate for T {
    fn try_into_tantivy_date(self) -> Result<tantivy::DateTime, TimeBridgeError> {
        Ok(
            tantivy::DateTime::try_from_utc(self.try_into_time()?.to_offset(UtcOffset::UTC))
                .ok_or(TimeBridgeError::TryIntoTantivy)?,
        )
    }
}

impl<T: TryIntoTime> TryIntoJiff for T {
    fn try_into_jiff(self) -> Result<jiff::Timestamp, TimeBridgeError> {
        Ok(jiff::Timestamp::from_nanosecond(
            self.try_into_time()?.as_nanoseconds(),
        )?)
    }
}
