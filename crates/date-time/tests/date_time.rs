use std::time::Duration;

use date_time::{DateTime, ParseDateTimeError};

#[test]
fn test_from_unix_timestamp_as_millis_and_display() -> anyhow::Result<()> {
    let dt = DateTime::from_unix_timestamp_as_millis(1612369038500)?;
    assert_eq!(dt.to_string(), "2021-02-03T16:17:18.500Z");
    Ok(())
}

#[test]
fn test_from_unix_timestamp_as_millis_returns_err_for_out_of_range() {
    assert!(DateTime::from_unix_timestamp_as_millis(i64::MAX).is_err());
    assert!(DateTime::from_unix_timestamp_as_millis(-1).is_err());
}

#[test]
fn test_parse_utc_string() -> anyhow::Result<()> {
    let dt: DateTime = "2021-02-03T16:17:18.500Z".parse()?;
    assert_eq!(dt.unix_timestamp_as_millis(), 1612369038500);
    Ok(())
}

#[test]
fn test_parse_positive_timezone_offset() -> anyhow::Result<()> {
    let dt: DateTime = "2021-02-04T01:17:18.500+09:00".parse()?;
    assert_eq!(dt.to_string(), "2021-02-03T16:17:18.500Z");
    Ok(())
}

#[test]
fn test_parse_negative_timezone_offset() -> anyhow::Result<()> {
    let dt: DateTime = "2021-02-03T11:17:18.500-05:00".parse()?;
    assert_eq!(dt.to_string(), "2021-02-03T16:17:18.500Z");
    Ok(())
}

#[test]
fn test_display_always_utc_with_z() -> anyhow::Result<()> {
    let dt = DateTime::from_unix_timestamp_as_millis(0)?;
    let s = dt.to_string();
    assert!(s.ends_with('Z'));
    assert_eq!(s, "1970-01-01T00:00:00.000Z");
    Ok(())
}

#[test]
fn test_roundtrip_via_string() -> anyhow::Result<()> {
    let original = DateTime::from_unix_timestamp_as_millis(1700000000999)?;
    let s = original.to_string();
    let parsed: DateTime = s.parse()?;
    assert_eq!(original, parsed);
    Ok(())
}

#[test]
fn test_parse_error_type() {
    let err = "invalid".parse::<DateTime>().unwrap_err();
    assert_eq!(err, ParseDateTimeError);
    assert!(!err.to_string().is_empty());
    let _: &dyn std::error::Error = &err;
}

#[test]
fn test_reject_missing_millis_digits() {
    assert!("2021-02-03T16:17:18.50Z".parse::<DateTime>().is_err());
}

#[test]
fn test_reject_extra_millis_digits() {
    assert!("2021-02-03T16:17:18.5000Z".parse::<DateTime>().is_err());
}

#[test]
fn test_reject_no_fractional_seconds() {
    assert!("2021-02-03T16:17:18Z".parse::<DateTime>().is_err());
}

#[test]
fn test_ordering() -> anyhow::Result<()> {
    let a = DateTime::from_unix_timestamp_as_millis(1000)?;
    let b = DateTime::from_unix_timestamp_as_millis(2000)?;
    assert!(a < b);
    assert_eq!(a, a);
    Ok(())
}

#[test]
fn test_clone_and_copy() -> anyhow::Result<()> {
    let dt = DateTime::from_unix_timestamp_as_millis(0)?;
    let cloned = dt.clone();
    let copied = dt;
    assert_eq!(dt, cloned);
    assert_eq!(dt, copied);
    Ok(())
}

#[test]
fn test_add_duration() -> anyhow::Result<()> {
    let dt: DateTime = "2021-02-03T16:17:18.500Z".parse()?;
    let result = (dt + Duration::from_secs(60))?;
    assert_eq!(result.to_string(), "2021-02-03T16:18:18.500Z");
    Ok(())
}

#[test]
fn test_sub_duration() -> anyhow::Result<()> {
    let dt: DateTime = "2021-02-03T16:17:18.500Z".parse()?;
    let result = (dt - Duration::from_secs(60))?;
    assert_eq!(result.to_string(), "2021-02-03T16:16:18.500Z");
    Ok(())
}

#[test]
fn test_sub_duration_below_epoch_returns_err() -> anyhow::Result<()> {
    let dt = DateTime::from_unix_timestamp_as_millis(0)?;
    assert!((dt - Duration::from_millis(1)).is_err());
    Ok(())
}

#[test]
fn test_parse_before_epoch_rejected() {
    assert!("1969-12-31T23:59:59.000Z".parse::<DateTime>().is_err());
}

#[test]
fn test_add_duration_with_micros_returns_err() -> anyhow::Result<()> {
    let dt = DateTime::from_unix_timestamp_as_millis(1000)?;
    assert!((dt + Duration::from_micros(500)).is_err());
    Ok(())
}

#[test]
fn test_sub_duration_with_nanos_returns_err() -> anyhow::Result<()> {
    let dt = DateTime::from_unix_timestamp_as_millis(1000)?;
    assert!((dt - Duration::from_nanos(1)).is_err());
    Ok(())
}
