mod diff;
mod download;
mod list;
mod upload;
mod view;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    #[command(name = "diff", about = "diff")]
    Diff(diff::Command),
    #[command(name = "download", about = "Download to the hatena blog")]
    Download(download::Command),
    #[command(name = "list")]
    List(list::Command),
    #[command(name = "upload", about = "Upload to the hatena blog")]
    Upload(upload::Command),
    #[command(name = "view", about = "view")]
    View(view::Command),
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::Diff(command) => command.handle().await,
            Subcommand::Download(command) => command.handle().await,
            Subcommand::List(command) => command.handle().await,
            Subcommand::Upload(command) => command.handle().await,
            Subcommand::View(command) => command.handle().await,
        }
    }
}
