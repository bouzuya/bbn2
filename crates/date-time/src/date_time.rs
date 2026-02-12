use crate::ParseDateTimeError;

/// 許容する最小値: 1970-01-01T00:00:00.000Z (0 ms)
const MIN_UNIX_TIMESTAMP_MS: i64 = 0;
/// 許容する最大値: 9999-12-31T23:59:59.999Z
const MAX_UNIX_TIMESTAMP_MS: i64 = 253_402_300_799_999;

/// ミリ秒精度の UTC 日時型。内部で `chrono::DateTime<Utc>` をラップする。
///
/// 許容範囲: `1970-01-01T00:00:00.000Z` から `9999-12-31T23:59:59.999Z` まで。
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct DateTime(chrono::DateTime<chrono::Utc>);

/// `Duration` がミリ秒精度であることを検証し、ミリ秒値を返す。
fn duration_as_millis_exact(d: std::time::Duration) -> Result<i64, ParseDateTimeError> {
    if d.subsec_nanos() % 1_000_000 != 0 {
        return Err(ParseDateTimeError);
    }
    i64::try_from(d.as_millis()).map_err(|_| ParseDateTimeError)
}

impl DateTime {
    /// ミリ秒単位の Unix タイムスタンプから `DateTime` を生成する。
    /// タイムスタンプが範囲外の場合は `Err` を返す。
    pub fn from_unix_timestamp_as_millis(timestamp_ms: i64) -> Result<Self, ParseDateTimeError> {
        if timestamp_ms < MIN_UNIX_TIMESTAMP_MS || timestamp_ms > MAX_UNIX_TIMESTAMP_MS {
            return Err(ParseDateTimeError);
        }
        let secs = timestamp_ms.div_euclid(1000);
        let nanos = (timestamp_ms.rem_euclid(1000) * 1_000_000) as u32;
        chrono::DateTime::from_timestamp(secs, nanos)
            .map(Self)
            .ok_or(ParseDateTimeError)
    }

    /// ミリ秒単位の Unix タイムスタンプを返す。
    pub fn unix_timestamp_as_millis(&self) -> i64 {
        self.0.timestamp_millis()
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S%.3fZ"))
    }
}

impl std::str::FromStr for DateTime {
    type Err = ParseDateTimeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 位置19が '.' であること (YYYY-MM-DDTHH:MM:SS....)
        if s.len() < 20 || s.as_bytes()[19] != b'.' {
            return Err(ParseDateTimeError);
        }

        // ミリ秒部分が3桁であること (位置20..23)
        // 位置23はタイムゾーン開始 ('Z' or '+' or '-')
        if s.len() < 24 {
            return Err(ParseDateTimeError);
        }
        if !s.as_bytes()[20].is_ascii_digit()
            || !s.as_bytes()[21].is_ascii_digit()
            || !s.as_bytes()[22].is_ascii_digit()
        {
            return Err(ParseDateTimeError);
        }

        // タイムゾーン部分の検証: 位置23以降が "Z" または "+HH:MM" / "-HH:MM"
        let tz_part = &s[23..];
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

        // ミリ秒精度の確認: ナノ秒のうちミリ秒未満が0であること
        let nanos = parsed.timestamp_subsec_nanos();
        if nanos % 1_000_000 != 0 {
            return Err(ParseDateTimeError);
        }

        // 範囲チェック
        let ms = parsed.timestamp_millis();
        Self::from_unix_timestamp_as_millis(ms)
    }
}

impl std::ops::Add<std::time::Duration> for DateTime {
    type Output = Result<Self, ParseDateTimeError>;

    fn add(self, rhs: std::time::Duration) -> Self::Output {
        let duration_ms = duration_as_millis_exact(rhs)?;
        let new_ms = self
            .unix_timestamp_as_millis()
            .checked_add(duration_ms)
            .ok_or(ParseDateTimeError)?;
        Self::from_unix_timestamp_as_millis(new_ms)
    }
}

impl std::ops::Sub<std::time::Duration> for DateTime {
    type Output = Result<Self, ParseDateTimeError>;

    fn sub(self, rhs: std::time::Duration) -> Self::Output {
        let duration_ms = duration_as_millis_exact(rhs)?;
        let new_ms = self
            .unix_timestamp_as_millis()
            .checked_sub(duration_ms)
            .ok_or(ParseDateTimeError)?;
        Self::from_unix_timestamp_as_millis(new_ms)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_from_unix_timestamp_as_millis_to_string() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(1612369038500)?;
        assert_eq!(dt.to_string(), "2021-02-03T16:17:18.500Z");
        Ok(())
    }

    #[test]
    fn test_parse_utc_to_timestamp() -> anyhow::Result<()> {
        let dt: DateTime = "2021-02-03T16:17:18.500Z".parse()?;
        assert_eq!(dt.unix_timestamp_as_millis(), 1612369038500);
        Ok(())
    }

    #[test]
    fn test_parse_positive_offset() -> anyhow::Result<()> {
        let dt: DateTime = "2021-02-04T01:17:18.500+09:00".parse()?;
        assert_eq!(dt.unix_timestamp_as_millis(), 1612369038500);
        assert_eq!(dt.to_string(), "2021-02-03T16:17:18.500Z");
        Ok(())
    }

    #[test]
    fn test_parse_negative_offset() -> anyhow::Result<()> {
        let dt: DateTime = "2021-02-03T11:17:18.500-05:00".parse()?;
        assert_eq!(dt.unix_timestamp_as_millis(), 1612369038500);
        Ok(())
    }

    #[test]
    fn test_roundtrip_with_millis() -> anyhow::Result<()> {
        let original_ms: i64 = 1612369038123;
        let dt = DateTime::from_unix_timestamp_as_millis(original_ms)?;
        let s = dt.to_string();
        let parsed: DateTime = s.parse()?;
        assert_eq!(parsed.unix_timestamp_as_millis(), original_ms);
        Ok(())
    }

    #[test]
    fn test_roundtrip_zero_millis() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(1612369038000)?;
        assert_eq!(dt.to_string(), "2021-02-03T16:17:18.000Z");
        let parsed: DateTime = "2021-02-03T16:17:18.000Z".parse()?;
        assert_eq!(parsed.unix_timestamp_as_millis(), 1612369038000);
        Ok(())
    }

    #[test]
    fn test_reject_no_dot_at_position_19() {
        assert!("2021-02-03T16:17:18_500Z".parse::<DateTime>().is_err());
    }

    #[test]
    fn test_reject_short_millis() {
        assert!("2021-02-03T16:17:18.50Z".parse::<DateTime>().is_err());
    }

    #[test]
    fn test_reject_long_millis() {
        assert!("2021-02-03T16:17:18.5000Z".parse::<DateTime>().is_err());
    }

    #[test]
    fn test_reject_invalid_tz_suffix() {
        assert!("2021-02-03T16:17:18.500X".parse::<DateTime>().is_err());
    }

    #[test]
    fn test_epoch() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(0)?;
        assert_eq!(dt.to_string(), "1970-01-01T00:00:00.000Z");
        assert_eq!(dt.unix_timestamp_as_millis(), 0);
        Ok(())
    }

    #[test]
    fn test_negative_timestamp_rejected() {
        assert!(DateTime::from_unix_timestamp_as_millis(-1).is_err());
        assert!(DateTime::from_unix_timestamp_as_millis(-1000).is_err());
    }

    #[test]
    fn test_max_boundary() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(MAX_UNIX_TIMESTAMP_MS)?;
        assert_eq!(dt.to_string(), "9999-12-31T23:59:59.999Z");
        Ok(())
    }

    #[test]
    fn test_over_max_rejected() {
        assert!(DateTime::from_unix_timestamp_as_millis(MAX_UNIX_TIMESTAMP_MS + 1).is_err());
    }

    #[test]
    fn test_add_duration() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(1000)?;
        let result = (dt + Duration::from_millis(500))?;
        assert_eq!(result.unix_timestamp_as_millis(), 1500);
        Ok(())
    }

    #[test]
    fn test_sub_duration() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(1500)?;
        let result = (dt - Duration::from_millis(500))?;
        assert_eq!(result.unix_timestamp_as_millis(), 1000);
        Ok(())
    }

    #[test]
    fn test_add_duration_overflow_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(MAX_UNIX_TIMESTAMP_MS)?;
        assert!((dt + Duration::from_millis(1)).is_err());
        Ok(())
    }

    #[test]
    fn test_sub_duration_underflow_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(0)?;
        assert!((dt - Duration::from_millis(1)).is_err());
        Ok(())
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

    #[test]
    fn test_add_duration_millis_boundary_ok() -> anyhow::Result<()> {
        // 1ms ちょうどは OK
        let dt = DateTime::from_unix_timestamp_as_millis(0)?;
        let result = (dt + Duration::from_millis(1))?;
        assert_eq!(result.unix_timestamp_as_millis(), 1);
        Ok(())
    }

    #[test]
    fn test_duration_with_mixed_micros_returns_err() -> anyhow::Result<()> {
        let dt = DateTime::from_unix_timestamp_as_millis(1000)?;
        // 1001 マイクロ秒 = 1ms + 1μs → ミリ秒精度ではないのでエラー
        assert!((dt + Duration::from_micros(1001)).is_err());
        Ok(())
    }
}
