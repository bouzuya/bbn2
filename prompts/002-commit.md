# コミットメッセージ案

```
Use git dependencies for date-range and markdown-link-helper

date-range と markdown-link-helper の依存を外部パス (../date-range,
../markdown-link-helper) から git 依存
(https://github.com/bouzuya/rust-sandbox) に変更した。

これにより外部パス依存がなくなり cargo check が通るようになった。
```
