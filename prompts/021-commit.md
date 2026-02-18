Move Command struct to command module and implement Command::handle

Move the `Command` struct (clap Parser) from `main.rs` into `command.rs`,
and implement `Command::handle` to own the subcommand dispatch logic
previously held by `Subcommand::handle`. Remove `Subcommand::handle`.
Simplify `main.rs` to parse `command::Command` and call `command.handle().await`.
