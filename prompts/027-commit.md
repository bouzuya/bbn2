Extract config sub-subcommands into separate submodules

Move Get, Init, List, Set, and Unset from the config module into
separate submodules (config/get.rs, config/init.rs, config/list.rs,
config/set.rs, config/unset.rs), each with a pub struct Command and
a pub fn handle(self) method.

Keep Key and OptionalKey enums in config.rs as pub so submodules can
reference them via super::Key and super::OptionalKey.
