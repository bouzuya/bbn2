use anyhow::Context;

use crate::config::Config;
use crate::config_repository::ConfigRepository;

use super::OptionalKey;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(name = "KEY", help = "the key")]
    pub key: OptionalKey,
}

impl Command {
    pub async fn handle(self) -> anyhow::Result<()> {
        let config_repository = ConfigRepository::new()?;
        let config = config_repository
            .load()
            .context("The configuration file does not found. Use `bbn config init` command.")?;
        let config = match self.key {
            OptionalKey::LinkCompletionRulesFile => Config::new(
                config.data_dir().to_path_buf(),
                config.hatena_blog_data_file().to_path_buf(),
                None,
                config.out_dir().map(|p| p.to_path_buf()),
            ),
        };
        config_repository.save(config)?;
        Ok(())
    }
}
