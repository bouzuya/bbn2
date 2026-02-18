Move Subcommand definitions and dispatch logic to Command::handle

Move `Subcommand` and `HatenaBlogSubcommand` enum definitions from
`main.rs` to `command.rs`, and implement `Subcommand::handle` method
to encapsulate the dispatch logic previously in `main`'s match block.
Simplify `main.rs` to only define the top-level `Command` struct and
call `subcommand.handle().await`.

Also fix `link_completion.rs` to use the correct module path
`crate::date_like::DateLike` instead of the non-existent `crate::DateLike`.
