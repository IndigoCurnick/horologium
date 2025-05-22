use std::fmt::Display;

use time::{
    Date, OffsetDateTime, PrimitiveDateTime, Time,
    format_description::well_known::{Iso8601, Rfc3339},
};

#[derive(Debug, PartialEq)]
pub enum Temporal {
    Date(Date),
    Time(Time),
    DateTime(PrimitiveDateTime),
    OffsetDateTime(OffsetDateTime),
}

impl TryFrom<&str> for Temporal {
    type Error = HorologiumError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match Date::parse(value, &Iso8601::DATE) {
            Ok(x) => return Ok(Self::Date(x)),
            Err(_) => {}
        }

        match Time::parse(value, &Rfc3339) {
            Ok(x) => return Ok(Self::Time(x)),
            Err(_) => {}
        }

        match PrimitiveDateTime::parse(value, &Rfc3339) {
            Ok(x) => return Ok(Self::DateTime(x)),
            Err(_) => {}
        }

        match OffsetDateTime::parse(value, &Rfc3339) {
            Ok(x) => return Ok(Self::OffsetDateTime(x)),
            Err(_) => {}
        }

        return Err(HorologiumError::UnableToDetermineType);
    }
}

#[derive(Debug)]
pub enum HorologiumError {
    UnableToDetermineType,
}

impl Display for HorologiumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnableToDetermineType => write!(
                f,
                "Unable to determine date-time type: are you sure this is a valid date/time string?"
            ),
        }
    }
}
