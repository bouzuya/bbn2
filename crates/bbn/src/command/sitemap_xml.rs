use std::{fs::File, io::BufWriter, path::PathBuf};

#[derive(Debug, clap::Args)]
pub struct Command {
    pub out_dir: PathBuf,
}

use anyhow::Context;
use bbn_repository::{BbnRepository, Query};
use sitemap_xml_writer::{SitemapWriter, Url};

use crate::config_repository::ConfigRepository;

impl Command {
    pub fn handle(self) -> anyhow::Result<()> {
        run(self.out_dir)
    }
}

fn run(out_dir: PathBuf) -> anyhow::Result<()> {
    let config_repository = ConfigRepository::new()?;
    let config = config_repository
        .load()
        .context("The configuration file does not found. Use `bbn config` command.")?;
    let data_dir = config.data_dir().to_path_buf();

    let bbn_repository = BbnRepository::new(data_dir);
    let query = Query::try_from("date:1970-01-01/9999-12-31")?;
    let entry_ids = bbn_repository.find_ids_by_query(query)?;

    let path = out_dir.join("sitemap.xml");
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    let mut writer = SitemapWriter::start(writer)?;

    for entry_id in entry_ids {
        let meta = bbn_repository
            .find_meta_by_id(&entry_id)?
            .context("meta not found")?;

        let date = entry_id.date();
        let yyyy = date.year().to_string();
        let mm = date.month().to_string();
        let dd = date.day_of_month().to_string();
        writer.write(
            Url::loc(format!("https://blog.bouzuya.net/{yyyy}/{mm}/{dd}/").as_str())?
                .lastmod(meta.pubdate.to_string().as_str())?,
        )?;
    }

    writer.end()?;

    Ok(())
}
