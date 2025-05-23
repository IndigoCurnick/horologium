use horologium::Temporal;
use time::{Date, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};

#[test]
fn test_date() {
    let date = "2025-05-22";

    let correct = Temporal::Date(Date::from_calendar_date(2025, Month::May, 22).unwrap());

    let temporal = Temporal::try_from(date).unwrap();

    assert_eq!(correct, temporal);
}

#[test]
fn test_time() {
    let time = "12:15:02";

    let correct = Temporal::Time(Time::from_hms(12, 15, 2).unwrap());

    let temporal = Temporal::try_from(time).unwrap();

    assert_eq!(correct, temporal);
}

#[test]
fn test_date_time() {
    let datetime = "2025-05-22 12:15:02";

    let correct = Temporal::DateTime(PrimitiveDateTime::new(
        Date::from_calendar_date(2025, Month::May, 22).unwrap(),
        Time::from_hms(12, 15, 2).unwrap(),
    ));

    let temporal = Temporal::try_from(datetime).unwrap();

    assert_eq!(correct, temporal);
}

#[test]
fn test_offset_date_time() {
    let datetimeoff = "2025-05-23T14:37:00+01";

    let correct = Temporal::OffsetDateTime(OffsetDateTime::new_in_offset(
        Date::from_calendar_date(2025, Month::May, 23).unwrap(),
        Time::from_hms(14, 37, 00).unwrap(),
        UtcOffset::from_hms(1, 0, 0).unwrap(),
    ));

    let temporal = Temporal::try_from(datetimeoff).unwrap();

    assert_eq!(correct, temporal);
}
