Return Result instead of Option and reject sub-millisecond Duration

- from_unix_timestamp_as_millis の戻り値を Option から Result に変更
- Add/Sub の Output も Result<Self, ParseDateTimeError> に変更
- Duration にマイクロ秒・ナノ秒が含まれる場合はエラーを返すように変更
