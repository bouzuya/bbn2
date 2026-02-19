mod get;
mod init;
mod list;
mod set;
mod unset;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    /// Gets a configuration value
    Get(get::Command),
    /// Initializes the configuration
    Init(init::Command),
    /// Lists all configuration values
    List(list::Command),
    /// Sets a configuration value
    Set(set::Command),
    /// Deletes a configuration value
    Unset(unset::Command),
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Key {
    DataDir,
    HatenaBlogDataFile,
    LinkCompletionRulesFile,
    OutDir,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum OptionalKey {
    LinkCompletionRulesFile,
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::Get(command) => command.handle().await,
            Subcommand::Init(command) => command.handle().await,
            Subcommand::List(command) => command.handle().await,
            Subcommand::Set(command) => command.handle().await,
            Subcommand::Unset(command) => command.handle().await,
        }
    }
}
