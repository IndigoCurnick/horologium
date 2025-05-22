use horologium::Temporal;
use time::{Date, Month};

#[test]
fn test_date() {
    let date = "2025-05-22";

    let correct = Temporal::Date(Date::from_calendar_date(2025, Month::May, 22).unwrap());

    let temporal = Temporal::try_from(date).unwrap();

    assert_eq!(correct, temporal);
}

#[test]
fn test_time() {}

#[test]
fn test_date_time() {}

#[test]
fn test_offset_date_time() {}
