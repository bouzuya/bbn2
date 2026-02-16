# 016-result: build-html サブコマンドの構成検討

## 方針

- TypeScript ではなく Rust で実装
- サーバーなし、静的サイトジェネレーター
- 仮想 DOM 不使用、単純な HTML テンプレート（文字列結合）
- JavaScript なし（HTML のみ）
- CSS なし（後で対応）

## 入力

build-json の出力ディレクトリをそのまま入力として使用する。

### 読み込む JSON ファイル

1. **`{out_dir}/posts.json`** — 全エントリの一覧
2. **`{out_dir}/{yyyy}/{mm}/{dd}/index.json`** — 各エントリの詳細データ

### データ型（デシリアライズ用）

```rust
// posts.json
struct PostEntry {
    date: String,       // "YYYY-MM-DD"
    minutes: u32,
    pubdate: String,    // "YYYY-MM-DDTHH:MM:SSZ"
    tags: Vec<String>,
    title: String,
}

// {yyyy}/{mm}/{dd}/index.json
struct EntryDetail {
    data: String,       // markdown テキスト
    date: String,       // "YYYY-MM-DD"
    html: String,       // HTML 変換済みコンテンツ
    minutes: u32,
    pubdate: String,
    tags: Vec<String>,
    title: String,
}
```

## 出力

### ページタイプ

4種のページを生成する:

| ページタイプ | パスパターン | 説明 |
|-------------|------------|------|
| entry-list (ルート) | `/` | 最新エントリ一覧 |
| entry-detail | `/{yyyy}/{mm}/{dd}/` | エントリ詳細ページ |
| entry-list (related) | `/{yyyy}/{mm}/{dd}/related/` | 関連エントリ一覧 |
| entry-detail (idTitle) | `/{yyyy}/{mm}/{dd}/{idTitle}/` | entry-detail と同内容（旧 URL 互換） |

### 出力ファイル

各パスについて2形式:

- `{out_dir}/{path}/index.html`
- `{out_dir}/{path}.html`（ルートパスを除く）

例: エントリ `2024-01-15` (idTitle なし → `"diary"`) の場合:

```
{out_dir}/index.html                          (ルート: entry-list)
{out_dir}/2024/01/15/index.html               (entry-detail)
{out_dir}/2024/01/15.html                     (entry-detail)
{out_dir}/2024/01/15/related/index.html       (entry-list)
{out_dir}/2024/01/15/related.html             (entry-list)
{out_dir}/2024/01/15/diary/index.html         (entry-detail と同内容)
{out_dir}/2024/01/15/diary.html               (entry-detail と同内容)
```

エントリ数 N に対して: 1 + 6N ファイル

## HTML テンプレート構造

### 共通レイアウト

mr-jums の構造を踏襲するが、JS 関連の要素を除外する。

```html
<!DOCTYPE html>
<html lang="ja" prefix="og: http://ogp.me/ns#">
<head>
  <meta charset="UTF-8">
  <title>{title} - blog.bouzuya.net</title>
  <meta name="robots" content="index, follow">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <!-- Twitter Card -->
  <meta name="twitter:card" content="summary">
  <meta name="twitter:site" content="@bouzuya">
  <meta name="twitter:creator" content="@bouzuya">
  <!-- Open Graph -->
  <meta property="og:title" content="{title}">
  <meta property="og:url" content="{canonical_url}">
  <meta property="og:image" content="https://blog.bouzuya.net/images/favicon.png">
  <meta property="og:description" content="{description}">
  <meta property="og:site_name" content="blog.bouzuya.net">
  <meta name="theme-color" content="#4e6a41">
  <link rel="alternate" type="application/atom+xml" href="/atom.xml">
  <link rel="icon" sizes="192x192" href="https://blog.bouzuya.net/images/favicon.png">
  <link rel="apple-touch-icon" sizes="192x192" href="https://blog.bouzuya.net/images/favicon.png">
</head>
<body>
  <div class="app">
    <header class="header">
      <h1 class="title"><a href="/">blog.bouzuya.net</a></h1>
      {nav}
    </header>
    <div class="body">
      {content}
    </div>
    <footer class="footer">
      {nav}
    </footer>
  </div>
</body>
</html>
```

### 除外する mr-jums の要素

- `<script id="initial-state" ...>` — クライアント JS のハイドレーション用なので不要
- `<script src="{scriptUrl}">` — JavaScript なしなので不要
- `<link rel="stylesheet" href="{styleUrl}">` — CSS は後で対応
- `google-site-verification` — ビルドツールにハードコードすべきでないため除外（必要なら設定で対応）

### entry-detail ページの content

```html
<div class="entry-detail">
  <article class="entry">
    <header class="header">
      <h1 class="id-title">
        <a href="/{yyyy}/{mm}/{dd}/">
          <span class="id">{date}</span>
          <span class="separator"> </span>
          <span class="title">{title}</span>
        </a>
      </h1>
    </header>
    <div class="body">
      <section class="content">{entry.html}</section>
    </div>
    <footer class="footer">
      <a class="permalink" href="/{yyyy}/{mm}/{dd}/">
        <time class="pubdate" datetime="{pubdate}">{pubdate}</time>
      </a>
    </footer>
  </article>
</div>
```

### entry-list ページの content

```html
<div class="entry-list">
  <nav>
    <header class="header">
      <h1><a href="{list_url}">{list_title}</a></h1>
    </header>
    <div class="body">
      <ul class="entry-list">
        <li class="entry-list-item">
          <div class="entry">
            <a href="/{yyyy}/{mm}/{dd}/">
              <span class="id">{date}</span>
              <span class="separator"> </span>
              <span class="title">{title}</span>
            </a>
          </div>
        </li>
        ...
      </ul>
    </div>
    <footer class="footer"></footer>
  </nav>
</div>
```

- ルートページ: `list_title` = "最近の記事", `list_url` = "/"
- related ページ: `list_title` = "{date} {title} の関連記事", `list_url` = "/{yyyy}/{mm}/{dd}/related/"

### nav 構造

mr-jums では JS 駆動の三角形ナビゲーション（◀ ▲ ▼ ▶）を使用しているが、JS なしのため HTML リンクに置き換える。

entry-detail ページ:
- 前のエントリへのリンク
- 次のエントリへのリンク
- related (一覧) へのリンク

entry-list ページ:
- ナビゲーションは一覧内のリンクで代替するため、最小限にする

## エントリリストのページネーション

mr-jums ではフォーカスされたエントリの前後4件（最大9件）を表示する。

- ルートページ: 最新エントリから直近9件を表示
- related ページ: フォーカスされたエントリの前後4件（最大9件）を表示

## コマンドインターフェース

```
bbn build-html <OUT_DIR>
```

- `OUT_DIR`: build-json の出力ディレクトリと同じディレクトリを指定する（JSON を読み込み、同じディレクトリに HTML を出力する）

## 実装の構成

`crates/bbn/src/command/build_html.rs` に実装する。

### 処理フロー

1. `{out_dir}/posts.json` を読み込み、全エントリ一覧を取得
2. 各エントリについて `{out_dir}/{yyyy}/{mm}/{dd}/index.json` を読み込み
3. 各ページタイプの HTML を生成して出力:
   a. ルートページ（`/index.html`）— 最新エントリ一覧
   b. 各エントリの entry-detail ページ
   c. 各エントリの entry-list (related) ページ
   d. 各エントリの idTitle ページ（entry-detail と同内容）

### 新規依存クレート

- なし（`serde_json` は既に依存に含まれている。HTML は文字列結合で生成する）

## インクリメンタルビルド

初回実装ではインクリメンタルビルドは含めない（全件再生成する）。
将来的に必要になった場合に MD5/SHA256 ベースのキャッシュ機構を追加する。

## 今回含めないもの

- CSS（後で対応）
- JavaScript
- インクリメンタルビルド
- google-site-verification（必要なら設定で対応）
- atom.xml の生成（別サブコマンドで対応済みまたは将来対応）
