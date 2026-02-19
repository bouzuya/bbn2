# CLAUDE.md

このファイルは Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイダンスを提供します。

## 言語設定

- 応答・コメントは日本語で記述すること
- コード内のコメントも日本語を使用

## プロジェクト概要

bbn は blog.bouzuya.net を管理するための CLI ツールです。ブログエントリのローカル markdown/JSON ストレージと Hatena Blog プラットフォームとの同期機能を提供します。

## ビルドコマンド

```bash
cargo build                    # 全クレートをビルド
cargo test                     # 全テストを実行
cargo test -p bbn              # 特定クレートのテストを実行
cargo +nightly fmt             # コードフォーマット
cargo clippy                   # リント
```

## アーキテクチャ

4つのクレートで構成される Cargo ワークスペース:

- **bbn** (`crates/bbn/`): clap を使用したコマンドルーティングを行うメイン CLI バイナリ
- **bbn-data** (`crates/bbn-data/`): コアドメイン型 (Entry, EntryId, EntryMeta, DateTime)
- **bbn-repository** (`crates/bbn-repository/`): エントリ CRUD 用のファイルベースリポジトリパターン
- **bbn-hatena-blog** (`crates/bbn-hatena-blog/`): SQLite ストレージを使用した Hatena Blog API 連携

データフロー: CLI コマンド → Repository/HatenaBlog ハンドラー → bbn-data 型

エントリ保存形式: `data/YYYY/MM/YYYY-MM-DD(-ID_TITLE).[json|md]` (JSON はメタデータ、MD はコンテンツ)

## コード規約

- エラーハンドリング: 伝播には `anyhow::Result<T>`、カスタムエラー型には `thiserror`
- 非同期: I/O 操作 (API 呼び出し、ファイル操作) には tokio ランタイム
- パース: 日付/クエリ DSL には nom パーサーコンビネータ
- 設定: `xdg` クレートによる XDG 準拠の設定パス
- `{mod_name}/mod.rs` を使用せず `{mod_name}.rs` を使用する

## 外部依存関係

一部のワークスペース依存関係は兄弟パス (`../date-range`, `../markdown-link-helper`) や外部 git リポジトリ (`hatena-blog-api`) を参照しています。ビルドにはこれらが存在する必要があります。
