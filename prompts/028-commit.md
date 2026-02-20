# 028 commit message

```
Use askama crate for HTML generation in build command

Replace manual format!-based HTML rendering in crates/bbn/src/command/build/html.rs
with askama template engine. Add template files under crates/bbn/templates/ for page
layout, entry detail content, entry list content, and entry detail navigation. HTML
escaping is now handled automatically by askama instead of a hand-rolled html_escape
function.
```
