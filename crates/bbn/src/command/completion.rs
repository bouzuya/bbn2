use clap_complete::{Shell, generate};
use std::io;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(name = "SHELL", help = "the shell", value_enum)]
    pub shell: Shell,
}

impl Command {
    pub fn handle<C: clap::CommandFactory>(self) -> anyhow::Result<()> {
        let mut c = C::command();
        generate(self.shell, &mut c, "bbn", &mut io::stdout());
        Ok(())
    }
}
