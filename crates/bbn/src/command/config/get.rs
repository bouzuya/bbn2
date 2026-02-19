use anyhow::Context;

use crate::config_repository::ConfigRepository;

use super::Key;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(name = "KEY", help = "the key")]
    pub key: Key,
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        let config_repository = ConfigRepository::new()?;
        let config = config_repository
            .load()
            .context("The configuration file does not found. Use `bbn config init` command.")?;
        let value = match self.key {
            Key::DataDir => config
                .data_dir()
                .to_str()
                .context("data-dir is not UTF-8")?
                .to_string(),
            Key::HatenaBlogDataFile => config
                .hatena_blog_data_file()
                .to_str()
                .context("hatena-blog-data-file is not UTF-8")?
                .to_string(),
            Key::LinkCompletionRulesFile => config
                .link_completion_rules_file()
                .map(|p| {
                    p.to_str()
                        .context("link-completion-rules-file is not UTF-8")
                })
                .transpose()?
                .unwrap_or("")
                .to_string(),
            Key::OutDir => config
                .out_dir()
                .map(|p| p.to_str().context("out-dir is not UTF-8"))
                .transpose()?
                .unwrap_or("")
                .to_string(),
        };
        println!("{value}");
        Ok(())
    }
}
