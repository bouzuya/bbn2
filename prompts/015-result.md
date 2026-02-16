# 015-result: build-html スクリプトの挙動解析

## 概要

`scripts/build-html` は `@bouzuya/mr-jums` パッケージの `build` 関数を呼び出す Node.js スクリプト。

```javascript
build({
  dstDir: 'dist',
  imageBaseUrl: 'https://blog.bouzuya.net',
  incremental: true,
  jsonBaseUrl: 'file:./dist'
});
```

mr-jums は Cycle.js (snabbdom) ベースのアプリケーションで、仮想 DOM をサーバーサイドレンダリングして静的 HTML を生成する。

## 入力

`jsonBaseUrl: 'file:./dist'` により、ローカルの `dist` ディレクトリから JSON ファイルを読み込む。
これらは `build-json` サブコマンドの出力に相当する。

### 読み込む JSON ファイル

1. **`dist/posts.json`** — 全エントリの一覧
   ```json
   [{"date": "2024-01-15", "minutes": 5, "pubdate": "2024-01-15T00:00:00+09:00", "tags": ["tag1"], "title": "タイトル"}, ...]
   ```

2. **`dist/{yyyy}/{mm}/{dd}/index.json`** — 各エントリの詳細データ
   ```json
   {
     "data": "markdownテキスト",
     "date": "2024-01-15",
     "html": "<p>HTML変換済みコンテンツ</p>",
     "minutes": 5,
     "pubdate": "2024-01-15T00:00:00+09:00",
     "tags": ["tag1"],
     "title": "タイトル"
   }
   ```

3. **`dist/{yyyy}/{mm}/{dd}/related.json`** — 関連エントリデータ（インクリメンタルビルドのキャッシュ検証のみに使用、HTML レンダリングには不使用）

## 出力

### ルーティング（4種のパス）

| ルート名 | パスパターン | 説明 |
|---------|------------|------|
| デフォルト | `/` | エントリ一覧（最新） |
| entry-detail | `/{yyyy}/{mm}/{dd}/` | エントリ詳細ページ |
| entry-list | `/{yyyy}/{mm}/{dd}/related/` | 関連エントリ一覧 |
| permanent-redirect | `/{yyyy}/{mm}/{dd}/{idTitle}/` | entry-detail にリダイレクト（同内容で生成） |

`idTitle` が未定義の場合は `"diary"` がデフォルト値として使われる。

### 出力ファイル

各パスについて2形式のファイルを出力:

- `dist/{path}/index.html` — ディレクトリインデックス形式
- `dist/{path}.html` — 拡張子形式（ルートパス `/` を除く）

例: エントリ `2024-01-15` (idTitle: `"diary"`) の場合:

```
dist/index.html                          (ルート: エントリ一覧)
dist/2024/01/15/index.html               (entry-detail)
dist/2024/01/15.html                     (entry-detail)
dist/2024/01/15/related/index.html       (entry-list)
dist/2024/01/15/related.html             (entry-list)
dist/2024/01/15/diary/index.html         (permanent-redirect → entry-detail と同内容)
dist/2024/01/15/diary.html               (permanent-redirect → entry-detail と同内容)
```

エントリ数 N に対して: 1 + 6N ファイル

### CSS/JS アセット

ビルド済みの CSS/JS ファイルを `dist/styles/` と `dist/scripts/` にコピーする。
`rev.json` にハッシュ付きファイル名が記録されており、HTML 内のリンクに反映される。

## HTML テンプレート構造

### 全体構造 (`html.ts`)

```html
<!DOCTYPE html>
<html lang="ja" prefix="og: http://ogp.me/ns#">
<head>
  <meta charset="UTF-8">
  <title>{entry.date} {entry.title} - blog.bouzuya.net</title>
  <meta name="robots" content="index, follow">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <!-- Twitter Card -->
  <meta name="twitter:card" content="summary">
  <meta name="twitter:site" content="@bouzuya">
  <meta name="twitter:creator" content="@bouzuya">
  <!-- Open Graph -->
  <meta name="og:title" content="{title}">
  <meta name="og:url" content="https://blog.bouzuya.net/{yyyy}/{mm}/{dd}/">
  <meta name="og:image" content="https://blog.bouzuya.net/images/favicon.png">
  <meta name="og:description" content="{entry.data の先頭100文字}">
  <meta name="og:site_name" content="blog.bouzuya.net">
  <meta name="theme-color" content="#4e6a41">
  <meta name="google-site-verification" content="...">
  <link rel="stylesheet" type="text/css" href="{styleUrl}">
  <link rel="alternate" type="application/atom+xml" href="/atom.xml">
  <link rel="icon" sizes="192x192" href="{imageBaseUrl}/images/favicon.png">
  <link rel="apple-touch-icon" sizes="192x192" href="{imageBaseUrl}/images/favicon.png">
  <script id="initial-state" data-initial-state="{serializedState}"></script>
</head>
<body>
  <div id="app"><!-- appView --></div>
  <script src="{scriptUrl}"></script>
</body>
</html>
```

### App 構造 (`app.ts`)

```html
<div class="app {is-menu}">
  <header class="header">
    <h1 class="title"><a href="/">blog.bouzuya.net</a></h1>
    <!-- nav -->
  </header>
  <div class="body">
    <!-- entry-detail view -->
    <!-- entry-list view -->
  </div>
  <footer class="footer">
    <!-- nav (header と同じ) -->
  </footer>
</div>
```

### Nav 構造 (`nav.ts`)

4つのナビゲーションリンク（三角形アイコン）: メニュー（左 ◀）、次（下 ▼）、前（上 ▲）、開く（右 ▶）

### Entry Detail (`entry-detail.ts`)

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
      <section class="content">{innerHTML: entry.html}</section>
    </div>
    <footer class="footer">
      <a class="permalink" href="/{yyyy}/{mm}/{dd}/">
        <time class="pubdate" datetime="{pubdate}">{pubdate}</time>
      </a>
    </footer>
  </article>
</div>
```

### Entry List (`entry-list.ts`)

```html
<div class="entry-list">
  <nav>
    <header class="header">
      <h1><a href="/{yyyy}/{mm}/{dd}/related/">{date} {title} の関連記事</a></h1>
      <!-- ルートの場合: -->
      <h1><a href="/">最近の記事</a></h1>
    </header>
    <div class="body">
      <ul class="entry-list index-{N} count-{M}">
        <li class="entry-list-item entry-id-{id} {is-focused} {is-selected}">
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

エントリリストはフォーカスされたエントリの前後4件、最大9件を表示する。

## ページタイプ別の State 構築

### entry-detail (`/{yyyy}/{mm}/{dd}/`)

1. `/{yyyy}/{mm}/{dd}/index.json` と `/posts.json` を読み込み
2. `selectedEntryDetail` にエントリ詳細を設定
3. `partialEntries` にフォーカス前後15件のエントリを設定
4. `focus` を `'entry-detail'` に設定

### entry-list (`/{yyyy}/{mm}/{dd}/related/` または `/`)

1. `/posts.json` のみ読み込み
2. `selectedEntryDetail` は `null`
3. `partialEntries` にフォーカス前後15件を設定
4. `focus` を `'entry-list'` に設定
5. ルートパスの場合 `focusedEntryId` は `null`

### permanent-redirect (`/{yyyy}/{mm}/{dd}/{idTitle}/`)

パスの末尾コンポーネントを除去し、entry-detail と同内容の HTML を生成する。

## インクリメンタルビルド

1. `dist/.mr-jums/cache.json` からキャッシュ（各 JSON の MD5 ハッシュ）を読み込み
2. `index.json` と `related.json` の MD5 ハッシュが変わったエントリのみを再生成対象にする
3. ルートパス `/` は常に再生成される
4. ビルド後、全エントリのハッシュを `cache.json` に保存

## imageBaseUrl の影響

- favicon URL (`<link rel="icon">`, `<link rel="apple-touch-icon">`) の `href` に使用
- OG image (`og:image`) は `https://blog.bouzuya.net/images/favicon.png` にハードコード

## initial-state

各ページに `<script id="initial-state" data-initial-state="{json}">` が埋め込まれる。
クライアントサイドの Cycle.js アプリケーションがハイドレーションするための初期状態。

```json
{
  "focusedEntryId": "2024-01-15",
  "selectedEntryDetail": { ... },
  "partialEntries": [{"id": "...", "title": "..."}, ...]
}
```

## Rust 移植における検討事項

1. **JSON 読み込み**: `posts.json` と `{yyyy}/{mm}/{dd}/index.json` — build-json の出力をそのまま使用
2. **HTML テンプレート**: Cycle.js/snabbdom の仮想 DOM は不要。文字列結合またはテンプレートエンジンで生成可能
3. **initial-state**: クライアントサイド JS を使わない場合は不要
4. **CSS/JS アセット**: mr-jums の Webpack ビルド済みアセットは別途用意が必要
5. **インクリメンタルビルド**: MD5 ベースのキャッシュ機構は必要に応じて実装
6. **出力ファイル形式**: 各パスに `index.html` と `.html` の2形式
