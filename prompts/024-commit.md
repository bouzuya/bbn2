Merge build-html and build-json subcommands into a single build subcommand

Replace the separate build-html and build-json subcommands with a
unified build subcommand that accepts --html and --json flags. When
neither flag is specified, both JSON and HTML are built. The build
subcommand reads data_dir and out_dir from the config file instead of
accepting them as command-line arguments. Update tests to configure
out_dir via `bbn config --out-dir` and use the new `bbn build` syntax.
