mod html;
mod json;

use anyhow::Context;
use std::path::PathBuf;

use crate::config_repository::ConfigRepository;

#[derive(Debug, clap::Args)]
pub struct Command {
    #[arg(long = "data-dir", help = "Data directory path")]
    data_dir: Option<PathBuf>,
    #[arg(long = "html", help = "Builds HTML files")]
    html: bool,
    #[arg(long = "json", help = "Builds JSON files")]
    json: bool,
    #[arg(long = "out-dir", help = "Output directory path")]
    out_dir: Option<PathBuf>,
    #[arg(long = "verbose", help = "Prints written file paths to stdout")]
    verbose: bool,
}

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        // --json または両方未指定のとき JSON を生成する
        let run_json = self.json || !self.html;
        // --html または両方未指定のとき HTML を生成する
        let run_html = self.html || !self.json;

        // CLI オプションが不足している場合のみ設定ファイルを読み込む
        let need_config = self.out_dir.is_none() || (run_json && self.data_dir.is_none());
        let config = if need_config {
            let config_repository = ConfigRepository::new()?;
            Some(
                config_repository
                    .load()
                    .context("The configuration file does not found. Use `bbn config` command.")?,
            )
        } else {
            None
        };

        let out_dir = match self.out_dir {
            Some(d) => d,
            None => config
                .as_ref()
                .unwrap()
                .out_dir()
                .context("out_dir is not configured. Use `bbn config --out-dir` command.")?
                .to_path_buf(),
        };

        if run_json {
            let data_dir = match self.data_dir {
                Some(d) => d,
                None => config.as_ref().unwrap().data_dir().to_path_buf(),
            };
            self::json::run(data_dir, out_dir.clone(), self.verbose)?;
        }
        if run_html {
            self::html::run(out_dir, self.verbose)?;
        }

        Ok(())
    }
}
