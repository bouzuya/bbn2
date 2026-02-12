Restrict DateTime range, add Duration arithmetic, and use anyhow in tests

- 許容範囲を 1970-01-01T00:00:00.000Z 〜 9999-12-31T23:59:59.999Z に制限
- 負の unix_timestamp_as_millis を拒否するように変更
- std::time::Duration との Add/Sub を実装 (範囲外は None を返す)
- テストの戻り値を anyhow::Result<()> に統一し unwrap を除去
