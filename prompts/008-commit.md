# 008 commit message

## コミットメッセージ案

```
bbn, bbn-data クレートの chrono 依存を date-time クレートに置き換え

date-time crate を除いて chrono への直接依存を削除し、
date-time::DateTime (UTC・ミリ秒精度) で置き換えた。

- bbn-data: DateTime の内部表現を chrono::DateTime<FixedOffset> から
  date_time::DateTime に変更
- bbn/download.rs: chrono の直接使用を DateTime::local_from_timestamp に統一
- bbn/view.rs: chrono を time クレートに置き換え (ローカル時刻変換)
- workspace の chrono 依存を削除
```

## 発生した問題・変更点

### 1. FromStr がミリ秒精度を要求する
- date-time の `DateTime::from_str` は `.fff` (ミリ秒3桁) が必須
- 既存の JSON データ (`pubdate`) は `"2021-02-03T00:00:00+09:00"` のような秒精度 RFC3339
- `bbn_repository.rs:40` の `DateTime::from_str(json.pubdate.as_str())` で既存データの読み込みが失敗する
- テストコード (`bbn_repository.rs`, `entry.rs`) も秒精度の文字列を使用

### 2. Display フォーマットが変わる
- 旧: `2021-02-03T16:17:18+09:00` (タイムゾーンオフセット保持、秒精度)
- 新: `2021-02-03T07:17:18.000Z` (常に UTC、ミリ秒精度)
- JSON ファイルに保存される `pubdate` の形式が変わる
- `bbn_json.rs` 等のテストで期待される出力形式と一致しなくなる

### 3. タイムゾーンオフセット情報の喪失
- `local_from_timestamp` は従来ローカルタイムゾーンオフセット付きの DateTime を返していたが、UTC のみになる
- `download.rs` の `datetime.to_string().get(0..10)` で取得する日付が UTC 日付になる (従来はローカル日付)
- ブログ記事の日付がずれる可能性がある (例: JST 2021-02-04 01:00:00 → UTC 2021-02-03)

### 4. FixedDateTime との相互変換
- `From<FixedDateTime> for DateTime`: `timestamp_millis()` で変換 (chrono のメソッドだが型経由で呼び出し可能)
- `From<DateTime> for FixedDateTime`: `to_string().parse()` で変換 (FixedDateTime が型エイリアスの場合は動作する想定だが、newtype の場合は失敗する可能性あり)

### 5. date-time crate に不足しているメソッド候補
- **秒精度の FromStr 対応**: `.fff` なしの RFC3339 文字列 (`2021-02-03T16:17:18+09:00`) のパース
- **秒単位の Unix タイムスタンプ対応**: `from_unix_timestamp(secs)` / `unix_timestamp()` (ミリ秒版のみ存在)
- **タイムゾーンオフセット付き表示**: RFC3339 形式でオフセット付き文字列を生成する機能
