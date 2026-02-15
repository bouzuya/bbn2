use chrono::{FixedOffset, Local, NaiveDateTime, TimeZone, Timelike};
use hatena_blog_api::FixedDateTime;

use crate::data::Timestamp;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTime(chrono::DateTime<FixedOffset>);

#[derive(Debug, Eq, thiserror::Error, PartialEq)]
#[error("parse date time error")]
pub struct ParseDateTimeError;

impl DateTime {
    pub fn local_from_timestamp(timestamp: Timestamp) -> Self {
        let utc_naive_datetime = NaiveDateTime::from_timestamp(i64::from(timestamp), 0);
        let local_datetime = Local.from_utc_datetime(&utc_naive_datetime);
        let fixed_offset = FixedOffset::from_offset(local_datetime.offset());
        let fixed_datetime = fixed_offset.from_utc_datetime(&utc_naive_datetime);
        Self(fixed_datetime)
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
        )
    }
}

impl std::str::FromStr for DateTime {
    type Err = ParseDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dt = chrono::DateTime::<FixedOffset>::from_str(s).map_err(|_| ParseDateTimeError)?;
        if dt != dt.with_nanosecond(0).unwrap() {
            return Err(ParseDateTimeError);
        }
        Ok(Self(dt))
    }
}

impl From<FixedDateTime> for DateTime {
    fn from(dt: FixedDateTime) -> Self {
        Self(chrono::DateTime::<FixedOffset>::from(dt))
    }
}

impl From<DateTime> for FixedDateTime {
    fn from(dt: DateTime) -> Self {
        Self::from(dt.0)
    }
}

impl From<DateTime> for Timestamp {
    fn from(dt: DateTime) -> Self {
        Timestamp::try_from(dt.0.timestamp()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_local_from_timestamp_roundtrip() -> anyhow::Result<()> {
        // タイムスタンプからの変換後、Timestamp に戻すと元の値と一致する
        let ts = Timestamp::try_from(1612369038_i64)?;
        let dt = DateTime::local_from_timestamp(ts);
        assert_eq!(Timestamp::from(dt), ts);
        Ok(())
    }

    #[test]
    fn test_local_from_timestamp_seconds_precision() -> anyhow::Result<()> {
        // 秒精度であること（サブ秒部分が0）
        let ts = Timestamp::try_from(1612369038_i64)?;
        let dt = DateTime::local_from_timestamp(ts);
        assert_eq!(dt.0.nanosecond(), 0);
        Ok(())
    }

    #[test]
    fn test_local_from_timestamp_epoch() -> anyhow::Result<()> {
        let ts = Timestamp::try_from(0_i64)?;
        let dt = DateTime::local_from_timestamp(ts);
        assert_eq!(Timestamp::from(dt), ts);
        Ok(())
    }

    #[test]
    fn test_local_from_timestamp_display_format() -> anyhow::Result<()> {
        // 表示形式が秒精度の RFC 3339 であること
        let ts = Timestamp::try_from(1612369038_i64)?;
        let dt = DateTime::local_from_timestamp(ts);
        let s = dt.to_string();
        // "YYYY-MM-DDTHH:MM:SS" の後に "Z" または "+HH:MM" / "-HH:MM" が続く
        assert!(s.len() == 20 || s.len() == 25);
        assert!(s.contains('T'));
        // 小数点が含まれないこと
        assert!(!s.contains('.'));
        Ok(())
    }

    #[test]
    fn string_conversion_test() {
        let f = DateTime::from_str;
        let g = |dt: DateTime| dt.to_string();
        let s1 = "2021-02-03T16:17:18Z";
        let s2 = "2021-02-03T16:17:18+00:00";
        let s3 = "2021-02-03T16:17:18+09:00";
        assert!(f(s1).is_ok());
        assert!(f(s2).is_ok());
        assert_eq!(f(s1), f(s2));
        assert_eq!(f(s1).map(g), Ok(s1.to_string()));
        assert_eq!(f(s2).map(g), Ok(s1.to_string())); // +00:00 -> Z
        assert_eq!(f(s3).map(g), Ok(s3.to_string()));
    }

    #[test]
    fn timestamp_conversion_test() {
        let f = |s| DateTime::from_str(s).unwrap();
        let g = Timestamp::from;
        let s1 = "2021-02-03T16:17:18+00:00";
        let s2 = "2021-02-04T01:17:18+09:00";
        assert_eq!(g(f(s1)), Timestamp::try_from(1612369038).unwrap());
        assert_eq!(g(f(s1)), g(f(s2)));
    }
}
