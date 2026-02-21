# 029 commit message

```
Update dependencies to latest versions

Update all directly specified dependencies in crates/bbn/Cargo.toml and workspace
dependencies in Cargo.toml to their latest versions. Fix compilation breakages caused
by breaking API changes:

- nom 6 -> 8: use Parser::parse() instead of calling combinators as functions;
  replace tuple((a, b, c)) with (a, b, c) tuple syntax
- xdg 2 -> 3: with_prefix() no longer returns Result; get_config_home() now returns
  Option<PathBuf>
```
