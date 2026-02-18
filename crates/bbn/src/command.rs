mod build_html;
mod build_json;
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
    #[command(name = "build-html", about = "Builds HTML files")]
    BuildHtml(build_html::Command),
    #[command(name = "build-json", about = "Builds JSON files")]
    BuildJson(build_json::Command),
    #[command(name = "completion", about = "Prints the shell's completion script")]
    Completion(completion::Command),
    #[command(name = "config", about = "Updates the configuration file")]
    Config(config::Command),
    #[command(name = "date-range", about = "Prints the date range")]
    DateRange(date_range::Command),
    #[command(name = "hatena-blog", about = "hatena-blog")]
    HatenaBlog(hatena_blog::Command),
    #[command(name = "link-completion", about = "Completes links")]
    LinkCompletion(link_completion::Command),
    #[command(name = "list", about = "Lists the blog posts")]
    List(list::Command),
    #[command(name = "sitemap-xml", about = "...")]
    SitemapXml(sitemap_xml::Command),
    #[command(name = "view", about = "Views the blog post")]
    View(view::Command),
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::BuildHtml(command) => command.handle(),
            Subcommand::BuildJson(command) => command.handle(),
            Subcommand::Completion(command) => command.handle::<Command>(),
            Subcommand::Config(command) => command.handle(),
            Subcommand::DateRange(command) => command.handle(),
            Subcommand::HatenaBlog(command) => command.handle().await,
            Subcommand::LinkCompletion(command) => command.handle(),
            Subcommand::List(command) => command.handle(),
            Subcommand::SitemapXml(command) => command.handle(),
            Subcommand::View(command) => command.handle(),
        }
    }
}
