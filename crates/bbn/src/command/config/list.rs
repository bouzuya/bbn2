use anyhow::Context;

use crate::config_repository::ConfigRepository;

#[derive(Debug, clap::Args)]
pub struct Command;

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        let config_repository = ConfigRepository::new()?;
        let config = config_repository
            .load()
            .context("The configuration file does not found. Use `bbn config init` command.")?;
        println!(
            "data-dir={}",
            config
                .data_dir()
                .to_str()
                .context("data-dir is not UTF-8")?
        );
        println!(
            "hatena-blog-data-file={}",
            config
                .hatena_blog_data_file()
                .to_str()
                .context("hatena-blog-data-file is not UTF-8")?
        );
        if let Some(p) = config.link_completion_rules_file() {
            println!(
                "link-completion-rules-file={}",
                p.to_str()
                    .context("link-completion-rules-file is not UTF-8")?
            );
        }
        if let Some(p) = config.out_dir() {
            println!("out-dir={}", p.to_str().context("out-dir is not UTF-8")?);
        }
        Ok(())
    }
}
