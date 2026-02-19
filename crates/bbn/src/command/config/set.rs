use std::path::PathBuf;

use anyhow::Context;

use crate::config::Config;
use crate::config_repository::ConfigRepository;

use super::Key;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(name = "KEY", help = "the key")]
    pub key: Key,
    #[arg(name = "VALUE", help = "the value")]
    pub value: String,
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        let config_repository = ConfigRepository::new()?;
        let config = config_repository
            .load()
            .context("The configuration file does not found. Use `bbn config init` command.")?;
        let config = match self.key {
            Key::DataDir => Config::new(
                PathBuf::from(&self.value),
                config.hatena_blog_data_file().to_path_buf(),
                config.link_completion_rules_file().map(|p| p.to_path_buf()),
                config.out_dir().map(|p| p.to_path_buf()),
            ),
            Key::HatenaBlogDataFile => Config::new(
                config.data_dir().to_path_buf(),
                PathBuf::from(&self.value),
                config.link_completion_rules_file().map(|p| p.to_path_buf()),
                config.out_dir().map(|p| p.to_path_buf()),
            ),
            Key::LinkCompletionRulesFile => Config::new(
                config.data_dir().to_path_buf(),
                config.hatena_blog_data_file().to_path_buf(),
                Some(PathBuf::from(&self.value)),
                config.out_dir().map(|p| p.to_path_buf()),
            ),
            Key::OutDir => Config::new(
                config.data_dir().to_path_buf(),
                config.hatena_blog_data_file().to_path_buf(),
                config.link_completion_rules_file().map(|p| p.to_path_buf()),
                Some(PathBuf::from(&self.value)),
            ),
        };
        config_repository.save(config)?;
        Ok(())
    }
}
