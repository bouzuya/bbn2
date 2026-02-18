use std::path::PathBuf;

use anyhow::Context;

use crate::{config::Config, config_repository::ConfigRepository};

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(long = "data-dir", name = "DATA_DIR", help = "the data dir")]
    pub data_dir: PathBuf,
    #[arg(
        long = "hatena-blog-data-file",
        name = "HATENA_BLOG_DATA_FILE",
        help = "the hatena-blog data file"
    )]
    pub hatena_blog_data_file: PathBuf,
}

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        config(self.data_dir, self.hatena_blog_data_file)
    }
}

fn config(data_dir: PathBuf, hatena_blog_data_file: PathBuf) -> anyhow::Result<()> {
    // FIXME: Add argument to add link_completion_rules_file
    let config_repository = ConfigRepository::new()?;
    let config = Config::new(data_dir, hatena_blog_data_file, None);
    config_repository.save(config)?;
    println!(
        "The configuration has been written to {}",
        config_repository
            .path()?
            .to_str()
            .context("The configuration file path is not UTF-8")?
    );
    Ok(())
}
