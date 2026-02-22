use anyhow::Context;
use bbn_data::EntryKey;
use bbn_repository::BbnRepository;
use bbn_repository::Query;
use pulldown_cmark::Parser;
use pulldown_cmark::html;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::fs::File;
use std::fs::{self};
use std::io::BufWriter;
use std::path::Path;
use std::path::PathBuf;
use std::str::FromStr;

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#all-json>
// all json (`/posts.json`)
#[derive(serde::Serialize)]
pub struct AllJson(pub Vec<AllJsonItem>);

#[derive(serde::Serialize)]
pub struct AllJsonItem {
    pub date: String, // "YYYY-MM-DD"
    pub minutes: u32,
    pub pubdate: String, // "YYYY-MM-DDTHH:MM:SSZ"
    pub tags: Vec<String>,
    pub title: String,
}

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#daily-json>
#[derive(serde::Serialize)]
pub struct DailyJson {
    pub data: String, // "markdown"
    pub date: String, // "YYYY-MM-DD" in "+09:00"
    pub minutes: u32,
    pub html: String, // "<p>markdown</p>"
    #[serde(skip_serializing)]
    pub id_title: Option<String>, // "title" (obsolete)
    pub pubdate: String, // "YYYY-MM-DDTHH:MM:SSZ"
    pub tags: Vec<String>,
    pub title: String,
}

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#tags-json>
// tags json (`/tags.json`)
#[derive(serde::Serialize)]
pub struct TagsJson(pub Vec<TagsJsonItem>);

#[derive(serde::Serialize)]
pub struct TagsJsonItem {
    pub name: String,
    pub count: u32,
}

fn write_all_json(out_dir: &Path, all_json: &AllJson, verbose: bool) -> anyhow::Result<()> {
    let path = out_dir.join("posts.json");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, all_json)?;
    if verbose {
        println!("{}", path.display());
    }
    Ok(())
}

fn write_daily_json(out_dir: &Path, daily_json: &DailyJson, verbose: bool) -> anyhow::Result<()> {
    let date = daily_json.date.split('-').collect::<Vec<&str>>();
    let yyyy = date[0];
    let mm = date[1];
    let dd = date[2];
    let id_title = daily_json.id_title.as_deref().unwrap_or("diary");
    let file_names = vec![
        format!("{yyyy}/{mm}/{dd}.json"),
        format!("{yyyy}/{mm}/{dd}/index.json"),
        format!("{yyyy}/{mm}/{dd}/{id_title}.json"),
        format!("{yyyy}/{mm}/{dd}/{id_title}/index.json"),
    ];
    for file_name in file_names {
        let path = out_dir.join(file_name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = File::create(&path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, daily_json)?;
        if verbose {
            println!("{}", path.display());
        }
    }
    Ok(())
}

// <https://github.com/bouzuya/kraken/tree/v4.0.2/doc#related-json>
// related json (`/YYYY/MM/DD/related.json`)
#[derive(serde::Serialize)]
pub struct RelatedJson {
    pub inbound: Vec<String>,
    pub next: Vec<String>,
    pub outbound: Vec<String>,
    pub prev: Vec<String>,
    pub same: Vec<String>,
}

fn write_related_json(
    out_dir: &Path,
    date: &str,
    related_json: &RelatedJson,
    verbose: bool,
) -> anyhow::Result<()> {
    let parts = date.split('-').collect::<Vec<&str>>();
    let yyyy = parts[0];
    let mm = parts[1];
    let dd = parts[2];
    let file_names = vec![
        format!("{yyyy}/{mm}/{dd}/related.json"),
        format!("{yyyy}/{mm}/{dd}/related/index.json"),
    ];
    for file_name in file_names {
        let path = out_dir.join(file_name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = File::create(&path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, related_json)?;
        if verbose {
            println!("{}", path.display());
        }
    }
    Ok(())
}

fn write_linked_json(
    out_dir: &Path,
    inbounds: &BTreeMap<EntryKey, BTreeSet<EntryKey>>,
    verbose: bool,
) -> anyhow::Result<()> {
    let mut linked = BTreeMap::new();
    for (k, v) in inbounds.iter() {
        linked.insert(
            k.to_string(),
            v.iter().map(|v_i| v_i.to_string()).collect::<Vec<String>>(),
        );
    }
    let path = out_dir.join("linked.json");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, &linked)?;
    if verbose {
        println!("{}", path.display());
    }
    Ok(())
}

fn write_tags_json(out_dir: &Path, tags_json: &TagsJson, verbose: bool) -> anyhow::Result<()> {
    let path = out_dir.join("tags.json");
    let file = File::create(&path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, tags_json)?;
    if verbose {
        println!("{}", path.display());
    }
    Ok(())
}

fn markdown_to_html(markdown: &str) -> String {
    let mut html_output = String::new();
    html::push_html(&mut html_output, Parser::new(markdown));
    html_output
}

fn parse_links(markdown: &str) -> anyhow::Result<BTreeSet<EntryKey>> {
    let mut links = BTreeSet::new();
    let regex = Regex::new(r#"\[([0-9]{4}-[0-1][0-9]-[0-3][0-9])\]"#)?;
    for captures in regex.captures_iter(markdown) {
        let m = captures.get(1).context("no captures")?;
        links.insert(date_range::date::Date::from_str(m.as_str())?);
    }
    Ok(links)
}

pub fn run(data_dir: PathBuf, out_dir: PathBuf, verbose: bool) -> anyhow::Result<()> {
    let bbn_repository = BbnRepository::new(data_dir);
    let query = Query::try_from("date:1970-01-01/9999-12-31")?;
    let mut entry_ids = bbn_repository.find_ids_by_query(query)?;
    entry_ids.sort();

    let mut all_json_items = vec![];
    let mut tag_count_map = BTreeMap::new();
    let mut inbounds = BTreeMap::new();
    let mut outbounds = BTreeMap::new();
    let mut same_days = BTreeMap::new();
    for entry_id in entry_ids {
        let meta = bbn_repository
            .find_meta_by_id(&entry_id)?
            .context("meta not found")?;
        let content = bbn_repository
            .find_content_by_id(&entry_id)?
            .context("content not found")?;
        let links = parse_links(&content)?;

        for name in meta.tags.clone() {
            *tag_count_map.entry(name).or_insert(0) += 1;
        }

        let all_json_item = AllJsonItem {
            date: entry_id.date().to_string(),
            minutes: u32::try_from(meta.minutes)?,
            pubdate: meta.pubdate.to_string(),
            tags: meta.tags.clone(),
            title: meta.title.clone(),
        };
        all_json_items.push(all_json_item);

        let html = markdown_to_html(&content);
        let daily_json = DailyJson {
            data: content,
            date: entry_id.date().to_string(),
            html,
            id_title: entry_id.id_title().map(|s| s.to_owned()),
            minutes: u32::try_from(meta.minutes)?,
            pubdate: meta.pubdate.to_string(),
            tags: meta.tags,
            title: meta.title,
        };
        write_daily_json(out_dir.as_path(), &daily_json, verbose)?;

        for link in links.iter().cloned() {
            inbounds
                .entry(link)
                .or_insert_with(BTreeSet::new)
                .insert(entry_id.date().to_owned());
        }

        outbounds.insert(entry_id.date().to_owned(), links);

        let mmdd = format!(
            "--{}-{}",
            entry_id.date().month(),
            entry_id.date().day_of_month()
        );
        same_days
            .entry(mmdd)
            .or_insert_with(BTreeSet::new)
            .insert(entry_id.date().to_owned());
    }

    let tags_json = TagsJson(
        tag_count_map
            .into_iter()
            .map(|(name, count)| TagsJsonItem { name, count })
            .collect::<Vec<_>>(),
    );

    let all_json = AllJson(all_json_items);

    // related.json の出力
    let dates = all_json
        .0
        .iter()
        .map(|item| item.date.clone())
        .collect::<Vec<_>>();
    for (i, item) in all_json.0.iter().enumerate() {
        let date = &item.date;
        let date_key =
            date_range::date::Date::from_str(date).context("related.json: 日付のパースに失敗")?;

        // inbound: この日付へのリンクを持つエントリ
        let inbound = inbounds
            .get(&date_key)
            .map(|set| set.iter().map(|d| d.to_string()).collect::<Vec<_>>())
            .unwrap_or_default();

        // outbound: このエントリがリンクしているエントリ
        let outbound = outbounds
            .get(&date_key)
            .map(|set| set.iter().map(|d| d.to_string()).collect::<Vec<_>>())
            .unwrap_or_default();

        // next: この後の最大4件（降順）
        let next = dates[i + 1..]
            .iter()
            .take(4)
            .rev()
            .cloned()
            .collect::<Vec<_>>();

        // prev: この前の最大4件
        let prev = dates[..i].iter().rev().take(4).cloned().collect::<Vec<_>>();

        // same: 同じ月日のエントリ（自分自身を除く）
        let mmdd = format!("--{}-{}", date_key.month(), date_key.day_of_month());
        let same = same_days
            .get(&mmdd)
            .map(|set| {
                set.iter()
                    .filter(|d| *d != &date_key)
                    .map(|d| d.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let related_json = RelatedJson {
            inbound,
            next,
            outbound,
            prev,
            same,
        };
        write_related_json(out_dir.as_path(), date, &related_json, verbose)?;
    }

    fs::create_dir_all(out_dir.as_path())?;
    write_all_json(out_dir.as_path(), &all_json, verbose)?;
    write_tags_json(out_dir.as_path(), &tags_json, verbose)?;
    write_linked_json(out_dir.as_path(), &inbounds, verbose)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> anyhow::Result<()> {
        assert_eq!(
            parse_links(
                "[2021-02-03] [2021-02-04]\n\n[2021-02-03]: https://blog.bouzuya.net/2021/02/03/"
            )?,
            {
                let mut set = BTreeSet::new();
                set.insert(date_range::date::Date::from_str("2021-02-03")?);
                set.insert(date_range::date::Date::from_str("2021-02-04")?);
                set
            }
        );
        Ok(())
    }
}
