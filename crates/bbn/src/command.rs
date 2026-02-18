mod build_html;
mod build_json;
mod config;
mod date_range;
pub mod hatena_blog;
pub mod link_completion;
mod list;
mod sitemap_xml;
mod view;

use std::{io, path::PathBuf};

use clap_complete::{Shell, generate};
use ::date_range::date::Date;

use crate::date_like::DateLike;

#[derive(Debug, clap::Parser)]
pub struct Command {
    #[command(subcommand)]
    subcommand: Subcommand,
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        match self.subcommand {
            Subcommand::BuildHtml { out_dir } => build_html::run(out_dir),
            Subcommand::BuildJson { out_dir } => build_json::run(out_dir),
            Subcommand::Completion { shell } => {
                let mut cmd = <Command as clap::CommandFactory>::command();
                generate(shell, &mut cmd, "bbn", &mut io::stdout());
                Ok(())
            }
            Subcommand::Config {
                data_dir,
                hatena_blog_data_file,
            } => config::config(data_dir, hatena_blog_data_file),
            Subcommand::DateRange { month, week_date } => {
                date_range::date_range(month, week_date)
            }
            Subcommand::HatenaBlog { subcommand } => match subcommand {
                HatenaBlogSubcommand::Diff { date } => hatena_blog::diff(date).await,
                HatenaBlogSubcommand::Download {
                    date,
                    data_file_only,
                } => hatena_blog::download(data_file_only, date).await,
                HatenaBlogSubcommand::List => hatena_blog::list().await,
                HatenaBlogSubcommand::Upload {
                    date,
                    draft,
                    interactive,
                } => hatena_blog::upload(date, draft, interactive).await,
                HatenaBlogSubcommand::View {
                    content,
                    date,
                    hatena_blog_id,
                    meta,
                    web,
                } => hatena_blog::view(content, date, hatena_blog_id, meta, web).await,
            },
            Subcommand::LinkCompletion { date_like } => {
                link_completion::run(link_completion::Params { date_like })
            }
            Subcommand::List { json, query } => list::list(json, query),
            Subcommand::SitemapXml { out_dir } => sitemap_xml::run(out_dir),
            Subcommand::View {
                content,
                date_like,
                json,
                meta,
                web,
            } => view::view(date_like, content, json, meta, web),
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
    #[command(name = "build-html", about = "Builds HTML files")]
    BuildHtml { out_dir: PathBuf },
    #[command(name = "build-json", about = "Builds JSON files")]
    BuildJson { out_dir: PathBuf },
    #[command(name = "completion", about = "Prints the shell's completion script")]
    Completion {
        #[arg(name = "SHELL", help = "the shell", value_enum)]
        shell: Shell,
    },
    #[command(name = "config", about = "Updates the configuration file")]
    Config {
        #[arg(long = "data-dir", name = "DATA_DIR", help = "the data dir")]
        data_dir: PathBuf,
        #[arg(
            long = "hatena-blog-data-file",
            name = "HATENA_BLOG_DATA_FILE",
            help = "the hatena-blog data file"
        )]
        hatena_blog_data_file: PathBuf,
    },
    #[command(name = "date-range", about = "Prints the date range")]
    DateRange {
        #[arg(name = "month", help = "YYYY-MM")]
        month: String,
        #[arg(long = "week-date", help = "Prints the date range as week date")]
        week_date: bool,
    },
    #[command(name = "hatena-blog", about = "hatena-blog")]
    HatenaBlog {
        #[command(subcommand)]
        subcommand: HatenaBlogSubcommand,
    },
    #[command(name = "link-completion", about = "Completes links")]
    LinkCompletion {
        #[arg(name = "DATE_LIKE", help = "the date. e.g. 2021-02-03 or 2021-W05-3")]
        date_like: DateLike,
    },
    #[command(name = "list", about = "Lists the blog posts")]
    List {
        #[arg(long = "json", help = "json")]
        json: bool,
        #[arg(
            name = "query",
            help = "query. e.g. date:2021 or date:2021-02 or date:2021-02-03 or date:--02-03 or date:---03"
        )]
        query: Option<String>,
    },
    #[command(name = "sitemap-xml", about = "...")]
    SitemapXml { out_dir: PathBuf },
    #[command(name = "view", about = "Views the blog post")]
    View {
        #[arg(long = "content", help = "Prints the contents of the entry")]
        content: bool,
        #[arg(name = "DATE_LIKE", help = "the date. e.g. 2021-02-03 or 2021-W05-3")]
        date_like: DateLike,
        #[arg(long = "json", help = "Prints in the JSON format")]
        json: bool,
        #[arg(long = "meta", help = "Prints the meta data of the entry")]
        meta: bool,
        #[arg(long = "web", help = "Open the entry in the browser")]
        web: bool,
    },
}

#[derive(Debug, clap::Subcommand)]
pub enum HatenaBlogSubcommand {
    #[command(name = "diff", about = "diff")]
    Diff {
        #[arg(name = "DATE", help = "the entry id")]
        date: Option<String>,
    },
    #[command(name = "download", about = "Download to the hatena blog")]
    Download {
        #[arg(long = "data-file-only")]
        data_file_only: bool,
        #[arg(name = "DATE")]
        date: Option<Date>,
    },
    #[command(name = "list")]
    List,
    #[command(name = "upload", about = "Upload to the hatena blog")]
    Upload {
        #[arg(name = "DATE", help = "date")]
        date: Option<Date>,
        #[arg(long = "draft")]
        draft: bool,
        #[arg(long = "interactive")]
        interactive: bool,
    },
    #[command(name = "view", about = "view")]
    View {
        #[arg(long = "content")]
        content: bool,
        #[arg(name = "DATE", help = "the entry id")]
        date: Date,
        #[arg(long = "hatena-blog-id", env = "HATENA_BLOG_ID")]
        hatena_blog_id: String,
        #[arg(long = "meta")]
        meta: bool,
        #[arg(long = "web", help = "Open the entry in the browser")]
        web: bool,
    },
}
