Refactor config subcommand into sub-subcommands

Split the config subcommand into five sub-subcommands:

- `bbn config get <key>`: Print the value of a single config key
- `bbn config init`: Initialize all config values at once (replaces the
  previous `bbn config` behavior)
- `bbn config list`: Print all config key-value pairs
- `bbn config set <key> <value>`: Update a single config value
- `bbn config unset <key>`: Clear an optional config value

Keys for get/set: data-dir, hatena-blog-data-file,
link-completion-rules-file, out-dir

Keys for unset (optional fields only): link-completion-rules-file, out-dir

Also add link-completion-rules-file argument to `config init` which was
previously missing (hardcoded to None with a FIXME comment).

Update integration tests to use `bbn config init` instead of `bbn config`.
