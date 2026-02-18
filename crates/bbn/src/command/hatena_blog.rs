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
    /// Diffs the local and remote blog posts
    Diff(diff::Command),
    /// Downloads the blog posts from the Hatena Blog
    Download(download::Command),
    /// Lists the blog posts
    List(list::Command),
    /// Uploads the blog posts to the Hatena Blog
    Upload(upload::Command),
    /// Views the blog posts
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
