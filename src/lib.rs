use std::fmt::Display;

use time::{
    Date, OffsetDateTime, PrimitiveDateTime, Time, format_description::well_known::Iso8601,
    macros::format_description,
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
        match OffsetDateTime::parse(value, &Iso8601::DATE_TIME_OFFSET) {
            Ok(x) => return Ok(Self::OffsetDateTime(x)),
            Err(_) => {}
        }

        let fmt = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");
        match PrimitiveDateTime::parse(value, &fmt) {
            Ok(x) => return Ok(Self::DateTime(x)),
            Err(_) => {}
        }

        match Date::parse(value, &Iso8601::DATE) {
            Ok(x) => return Ok(Self::Date(x)),
            Err(_) => {}
        }

        match Time::parse(value, &Iso8601::TIME) {
            Ok(x) => return Ok(Self::Time(x)),
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
