Output static files from public directory in build --html command

Embed `crates/bbn/public/**/*` at compile time using the `include_dir` crate
and copy all files to `out_dir` when building HTML with `bbn build --html`.
