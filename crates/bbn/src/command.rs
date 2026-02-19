mod build;
mod completion;
mod config;
mod date_range;
mod hatena_blog;
mod link_completion;
mod list;
mod sitemap_xml;
mod view;

#[derive(Debug, clap::Parser)]
pub struct Command {
    #[command(subcommand)]
    subcommand: Subcommand,
}

#[derive(Debug, clap::Subcommand)]
enum Subcommand {
    /// Builds blog files (JSON and/or HTML)
    Build(build::Command),
    /// Prints the shell's completion script
    Completion(completion::Command),
    /// Updates the configuration file
    Config(config::Command),
    /// Prints the date range
    DateRange(date_range::Command),
    /// Manages Hatena Blog posts
    HatenaBlog(hatena_blog::Command),
    /// Completes links
    LinkCompletion(link_completion::Command),
    /// Lists the blog posts
    List(list::Command),
    /// Builds sitemap.xml
    SitemapXml(sitemap_xml::Command),
    /// Views the blog post
    View(view::Command),
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::Build(command) => command.handle(),
            Subcommand::Completion(command) => command.handle::<Command>(),
            Subcommand::Config(command) => command.handle().await,
            Subcommand::DateRange(command) => command.handle(),
            Subcommand::HatenaBlog(command) => command.handle().await,
            Subcommand::LinkCompletion(command) => command.handle(),
            Subcommand::List(command) => command.handle(),
            Subcommand::SitemapXml(command) => command.handle(),
            Subcommand::View(command) => command.handle(),
        }
    }
}
