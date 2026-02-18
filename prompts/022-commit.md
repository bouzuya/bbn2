Refactor subcommand dispatch to use Command::handle methods

Convert free functions (run, config, date_range, list, view, diff,
download, upload) into Command::handle(self) methods on each
subcommand's Command struct. Add Command structs to the hatena-blog
submodules (diff, download, list, upload, view) and update the
hatena_blog::Subcommand enum variants to use tuple struct types
instead of named fields.
