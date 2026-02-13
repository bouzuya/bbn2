use crate::ParseDateTimeError;

/// 許容する最小値: 1970-01-01T00:00:00Z (0 秒)
const MIN_UNIX_TIMESTAMP: i64 = 0;
/// 許容する最大値: 9999-12-31T23:59:59Z
const MAX_UNIX_TIMESTAMP: i64 = 253_402_300_799;

/// 秒精度の UTC 日時型。内部で `chrono::DateTime<Utc>` をラップする。
///
/// 許容範囲: `1970-01-01T00:00:00Z` から `9999-12-31T23:59:59Z` まで。
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

/// `Duration` が秒精度であることを検証し、秒数を返す。
fn duration_as_secs_exact(d: std::time::Duration) -> Result<i64, ParseDateTimeError> {
    if d.subsec_nanos() != 0 {
        return Err(ParseDateTimeError);
    }
    i64::try_from(d.as_secs()).map_err(|_| ParseDateTimeError)
}

impl DateTime {
    /// 秒単位の Unix タイムスタンプから `DateTime` を生成する。
    /// タイムスタンプが範囲外の場合は `Err` を返す。
    pub fn from_unix_timestamp(timestamp: i64) -> Result<Self, ParseDateTimeError> {
        if timestamp < MIN_UNIX_TIMESTAMP || timestamp > MAX_UNIX_TIMESTAMP {
            return Err(ParseDateTimeError);
        }
        chrono::DateTime::from_timestamp(timestamp, 0)
            .map(Self)
            .ok_or(ParseDateTimeError)
    }

    /// 秒単位の Unix タイムスタンプを返す。
    pub fn unix_timestamp(&self) -> i64 {
        self.0.timestamp()
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

impl std::str::FromStr for DateTime {
    type Err = ParseDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 位置19がタイムゾーン開始 ('Z' or '+' or '-') であること (YYYY-MM-DDTHH:MM:SS...)
        if s.len() < 20 {
            return Err(ParseDateTimeError);
        }

        // 秒精度のため、位置19以降はタイムゾーン部分のみ
        let tz_start = 19;
        // 小数点がある場合は拒否（秒精度のため）
        if s.as_bytes()[tz_start] == b'.' {
            return Err(ParseDateTimeError);
        }

        // タイムゾーン部分の検証
        let tz_part = &s[tz_start..];
        match tz_part {
            "Z" => {}
            _ if tz_part.len() == 6 && (tz_part.starts_with('+') || tz_part.starts_with('-')) => {
                // +HH:MM or -HH:MM 形式
            }
            _ => return Err(ParseDateTimeError),
        }

        // chrono でパース
        let parsed = chrono::DateTime::parse_from_rfc3339(s)
            .map_err(|_| ParseDateTimeError)?
            .with_timezone(&chrono::Utc);

        // 秒精度の確認: サブ秒部分が0であること
        if parsed.timestamp_subsec_nanos() != 0 {
            return Err(ParseDateTimeError);
        }

        // 範囲チェック
        let secs = parsed.timestamp();
        Self::from_unix_timestamp(secs)
    }
}

impl std::ops::Add<std::time::Duration> for DateTime {
    type Output = Result<Self, ParseDateTimeError>;

    fn add(self, rhs: std::time::Duration) -> Self::Output {
        let duration_secs = duration_as_secs_exact(rhs)?;
        let new_secs = self
            .unix_timestamp()
            .checked_add(duration_secs)
            .ok_or(ParseDateTimeError)?;
        Self::from_unix_timestamp(new_secs)
    }
}

impl std::ops::Sub<std::time::Duration> for DateTime {
    type Output = Result<Self, ParseDateTimeError>;

    fn sub(self, rhs: std::time::Duration) -> Self::Output {
        let duration_secs = duration_as_secs_exact(rhs)?;
        let new_secs = self
            .unix_timestamp()
            .checked_sub(duration_secs)
            .ok_or(ParseDateTimeError)?;
        Self::from_unix_timestamp(new_secs)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_from_unix_timestamp_to_string() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(1612369038)?;
        assert_eq!(dt.to_string(), "2021-02-03T16:17:18Z");
        Ok(())
    }

    #[test]
    fn test_parse_utc_to_timestamp() -> anyhow::Result<()> {
        let dt: DateTime = "2021-02-03T16:17:18Z".parse()?;
        assert_eq!(dt.unix_timestamp(), 1612369038);
        Ok(())
    }

    #[test]
    fn test_parse_positive_offset() -> anyhow::Result<()> {
        let dt: DateTime = "2021-02-04T01:17:18+09:00".parse()?;
        assert_eq!(dt.unix_timestamp(), 1612369038);
        assert_eq!(dt.to_string(), "2021-02-03T16:17:18Z");
        Ok(())
    }

    #[test]
    fn test_parse_negative_offset() -> anyhow::Result<()> {
        let dt: DateTime = "2021-02-03T11:17:18-05:00".parse()?;
        assert_eq!(dt.unix_timestamp(), 1612369038);
        Ok(())
    }

    #[test]
    fn test_roundtrip() -> anyhow::Result<()> {
        let original_secs: i64 = 1612369038;
        let dt = DateTime::from_unix_timestamp(original_secs)?;
        let s = dt.to_string();
        let parsed: DateTime = s.parse()?;
        assert_eq!(parsed.unix_timestamp(), original_secs);
        Ok(())
    }

    #[test]
    fn test_reject_fractional_seconds() {
        assert!("2021-02-03T16:17:18.500Z".parse::<DateTime>().is_err());
    }

    #[test]
    fn test_reject_no_dot_at_position_19() {
        assert!("2021-02-03T16:17:18_500Z".parse::<DateTime>().is_err());
    }

    #[test]
    fn test_reject_invalid_tz_suffix() {
        assert!("2021-02-03T16:17:18X".parse::<DateTime>().is_err());
    }

    #[test]
    fn test_epoch() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(0)?;
        assert_eq!(dt.to_string(), "1970-01-01T00:00:00Z");
        assert_eq!(dt.unix_timestamp(), 0);
        Ok(())
    }

    #[test]
    fn test_negative_timestamp_rejected() {
        assert!(DateTime::from_unix_timestamp(-1).is_err());
        assert!(DateTime::from_unix_timestamp(-1000).is_err());
    }

    #[test]
    fn test_max_boundary() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(MAX_UNIX_TIMESTAMP)?;
        assert_eq!(dt.to_string(), "9999-12-31T23:59:59Z");
        Ok(())
    }

    #[test]
    fn test_over_max_rejected() {
        assert!(DateTime::from_unix_timestamp(MAX_UNIX_TIMESTAMP + 1).is_err());
    }

    #[test]
    fn test_add_duration() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(1000)?;
        let result = (dt + Duration::from_secs(60))?;
        assert_eq!(result.unix_timestamp(), 1060);
        Ok(())
    }

    #[test]
    fn test_sub_duration() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(1060)?;
        let result = (dt - Duration::from_secs(60))?;
        assert_eq!(result.unix_timestamp(), 1000);
        Ok(())
    }

    #[test]
    fn test_add_duration_overflow_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(MAX_UNIX_TIMESTAMP)?;
        assert!((dt + Duration::from_secs(1)).is_err());
        Ok(())
    }

    #[test]
    fn test_sub_duration_underflow_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(0)?;
        assert!((dt - Duration::from_secs(1)).is_err());
        Ok(())
    }

    #[test]
    fn test_add_duration_with_millis_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(1000)?;
        assert!((dt + Duration::from_millis(500)).is_err());
        Ok(())
    }

    #[test]
    fn test_sub_duration_with_nanos_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(1000)?;
        assert!((dt - Duration::from_nanos(1)).is_err());
        Ok(())
    }

    #[test]
    fn test_add_duration_secs_boundary_ok() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(0)?;
        let result = (dt + Duration::from_secs(1))?;
        assert_eq!(result.unix_timestamp(), 1);
        Ok(())
    }

    #[test]
    fn test_duration_with_mixed_nanos_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp(1000)?;
        // 1秒 + 1ナノ秒 → 秒精度ではないのでエラー
        assert!((dt + Duration::new(1, 1)).is_err());
        Ok(())
    }
}
