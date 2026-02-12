/// `DateTime` のパースエラー型。
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseDateTimeError;

impl std::fmt::Display for ParseDateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid date-time format")
    }
}

impl std::error::Error for ParseDateTimeError {}
