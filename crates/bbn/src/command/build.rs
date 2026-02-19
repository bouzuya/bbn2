mod html;
mod json;

use anyhow::Context;

use crate::config_repository::ConfigRepository;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(long = "html", help = "Builds HTML files")]
    html: bool,
    #[arg(long = "json", help = "Builds JSON files")]
    json: bool,
}

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        let config_repository = ConfigRepository::new()?;
        let config = config_repository
            .load()
            .context("The configuration file does not found. Use `bbn config` command.")?;
        let out_dir = config
            .out_dir()
            .context("out_dir is not configured. Use `bbn config --out-dir` command.")?
            .to_path_buf();

        // --json または両方未指定のとき JSON を生成する
        let run_json = self.json || !self.html;
        // --html または両方未指定のとき HTML を生成する
        let run_html = self.html || !self.json;

        if run_json {
            let data_dir = config.data_dir().to_path_buf();
            self::json::run(data_dir, out_dir.clone())?;
        }
        if run_html {
            self::html::run(out_dir)?;
        }

        Ok(())
    }
}
