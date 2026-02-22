use anyhow::Context;
use askama::Template;
use include_dir::{Dir, include_dir};
use std::fs::File;
use std::fs::{self};
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

static PUBLIC_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/public");

#[derive(serde::Deserialize)]
struct PostEntry {
    date: String,
    #[allow(dead_code)]
    minutes: u32,
    #[allow(dead_code)]
    pubdate: String,
    #[allow(dead_code)]
    tags: Vec<String>,
    title: String,
}

#[derive(serde::Deserialize)]
struct EntryDetail {
    data: String,
    date: String,
    html: String,
    #[allow(dead_code)]
    minutes: u32,
    pubdate: String,
    #[allow(dead_code)]
    tags: Vec<String>,
    title: String,
}

struct DateParts {
    yyyy: String,
    mm: String,
    dd: String,
}

fn parse_date(date: &str) -> anyhow::Result<DateParts> {
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        anyhow::bail!("invalid date format: {}", date);
    }
    Ok(DateParts {
        yyyy: parts[0].to_string(),
        mm: parts[1].to_string(),
        dd: parts[2].to_string(),
    })
}

fn entry_path(date: &DateParts) -> String {
    format!("/{}/{}/{}/", date.yyyy, date.mm, date.dd)
}

fn og_description(data: &str) -> String {
    data.chars().take(100).collect()
}

#[derive(Template)]
#[template(path = "page.html")]
struct PageTemplate {
    canonical_url: String,
    content: String,
    description: String,
    nav: String,
    title: String,
}

#[derive(Template)]
#[template(path = "nav_entry_detail.html")]
struct NavEntryDetailTemplate {
    dd: String,
    mm: String,
    next_path: Option<String>,
    prev_path: Option<String>,
    yyyy: String,
}

#[derive(Template)]
#[template(path = "entry_detail_content.html")]
struct EntryDetailContentTemplate {
    date_str: String,
    html: String,
    path: String,
    pubdate: String,
    title: String,
}

struct EntryListItem {
    date_str: String,
    path: String,
    title: String,
}

#[derive(Template)]
#[template(path = "entry_list_content.html")]
struct EntryListContentTemplate {
    items: Vec<EntryListItem>,
    list_title: String,
    list_url: String,
}

fn render_nav_entry_detail(
    prev: Option<&PostEntry>,
    next: Option<&PostEntry>,
    date: &DateParts,
) -> anyhow::Result<String> {
    let prev_path = prev
        .map(|p| parse_date(&p.date))
        .transpose()?
        .map(|d| entry_path(&d));
    let next_path = next
        .map(|n| parse_date(&n.date))
        .transpose()?
        .map(|d| entry_path(&d));
    NavEntryDetailTemplate {
        prev_path,
        next_path,
        yyyy: date.yyyy.clone(),
        mm: date.mm.clone(),
        dd: date.dd.clone(),
    }
    .render()
    .context("nav_entry_detail テンプレートのレンダリングに失敗")
}

fn render_nav_entry_list() -> String {
    String::new()
}

fn render_entry_detail_content(detail: &EntryDetail, date: &DateParts) -> anyhow::Result<String> {
    let path = entry_path(date);
    EntryDetailContentTemplate {
        path,
        date_str: detail.date.clone(),
        title: detail.title.clone(),
        html: detail.html.clone(),
        pubdate: detail.pubdate.clone(),
    }
    .render()
    .context("entry_detail_content テンプレートのレンダリングに失敗")
}

fn render_entry_list_content(
    entries: &[&PostEntry],
    list_title: &str,
    list_url: &str,
) -> anyhow::Result<String> {
    let items = entries
        .iter()
        .map(|entry| {
            let date = parse_date(&entry.date)?;
            let path = entry_path(&date);
            Ok(EntryListItem {
                path,
                date_str: entry.date.clone(),
                title: entry.title.clone(),
            })
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    EntryListContentTemplate {
        list_url: list_url.to_string(),
        list_title: list_title.to_string(),
        items,
    }
    .render()
    .context("entry_list_content テンプレートのレンダリングに失敗")
}

fn render_page(
    title: &str,
    canonical_url: &str,
    description: &str,
    nav: &str,
    content: &str,
) -> anyhow::Result<String> {
    PageTemplate {
        title: title.to_string(),
        canonical_url: canonical_url.to_string(),
        description: description.to_string(),
        nav: nav.to_string(),
        content: content.to_string(),
    }
    .render()
    .context("page テンプレートのレンダリングに失敗")
}

fn write_html(out_dir: &Path, path: &str, html: &str, verbose: bool) -> anyhow::Result<()> {
    // path は "/" で始まり "/" で終わる想定
    // index.html を出力
    let index_path = out_dir
        .join(path.trim_start_matches('/'))
        .join("index.html");
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(&index_path)?;
    file.write_all(html.as_bytes())?;
    if verbose {
        println!("{}", index_path.display());
    }

    // ルートパス以外は .html も出力
    if path != "/" {
        let trimmed = path.trim_start_matches('/').trim_end_matches('/');
        let html_path = out_dir.join(format!("{}.html", trimmed));
        if let Some(parent) = html_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&html_path)?;
        file.write_all(html.as_bytes())?;
        if verbose {
            println!("{}", html_path.display());
        }
    }

    Ok(())
}

fn get_page_entries<'a>(
    entries: &'a [PostEntry],
    focus_index: Option<usize>,
) -> Vec<&'a PostEntry> {
    let len = entries.len();
    if len == 0 {
        return vec![];
    }
    match focus_index {
        Some(idx) => {
            let start = idx.saturating_sub(4);
            let end = (idx + 5).min(len);
            entries[start..end].iter().collect()
        }
        // ルートページ: 最新（末尾）から9件
        None => {
            let start = len.saturating_sub(9);
            entries[start..].iter().rev().collect()
        }
    }
}

fn find_id_title(out_dir: &Path, date: &DateParts) -> Option<String> {
    // build-json は {yyyy}/{mm}/{dd}/{idTitle}/index.json を出力する
    // {yyyy}/{mm}/{dd}/ ディレクトリ内のサブディレクトリを探す
    let dir = out_dir.join(&date.yyyy).join(&date.mm).join(&date.dd);
    if let Ok(read_dir) = fs::read_dir(&dir) {
        for entry in read_dir.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    // "related" ディレクトリは build-html が作るものなので除外
                    if name != "related" {
                        return Some(name.to_string());
                    }
                }
            }
        }
    }
    None
}

pub fn run(out_dir: PathBuf, verbose: bool) -> anyhow::Result<()> {
    // posts.json を読み込み
    let posts_path = out_dir.join("posts.json");
    let file = File::open(&posts_path)
        .with_context(|| format!("failed to open {}", posts_path.display()))?;
    let reader = BufReader::new(file);
    let posts: Vec<PostEntry> = serde_json::from_reader(reader)
        .with_context(|| format!("failed to parse {}", posts_path.display()))?;

    // 各エントリの詳細を読み込み
    let mut details: Vec<EntryDetail> = Vec::with_capacity(posts.len());
    for post in &posts {
        let date = parse_date(&post.date)?;
        let json_path = out_dir
            .join(&date.yyyy)
            .join(&date.mm)
            .join(&date.dd)
            .join("index.json");
        let file = File::open(&json_path)
            .with_context(|| format!("failed to open {}", json_path.display()))?;
        let reader = BufReader::new(file);
        let detail: EntryDetail = serde_json::from_reader(reader)
            .with_context(|| format!("failed to parse {}", json_path.display()))?;
        details.push(detail);
    }

    // ルートページ（最新エントリ一覧）
    {
        let page_entries = get_page_entries(&posts, None);
        let content = render_entry_list_content(&page_entries, "最近の記事", "/")?;
        let nav = render_nav_entry_list();
        let html = render_page(
            "blog.bouzuya.net",
            "https://blog.bouzuya.net/",
            "",
            &nav,
            &content,
        )?;
        write_html(&out_dir, "/", &html, verbose)?;
    }

    // 各エントリのページ
    for (i, (post, detail)) in posts.iter().zip(details.iter()).enumerate() {
        let date = parse_date(&post.date)?;
        let path = entry_path(&date);

        // entry-detail ページ
        {
            let prev = if i > 0 { Some(&posts[i - 1]) } else { None };
            let next = if i + 1 < posts.len() {
                Some(&posts[i + 1])
            } else {
                None
            };
            let nav = render_nav_entry_detail(prev, next, &date)?;
            let content = render_entry_detail_content(detail, &date)?;
            let title = format!("{} {}", detail.date, detail.title);
            let canonical_url = format!("https://blog.bouzuya.net{}", path);
            let description = og_description(&detail.data);
            let html = render_page(&title, &canonical_url, &description, &nav, &content)?;
            write_html(&out_dir, &path, &html, verbose)?;

            // idTitle ページ（entry-detail と同内容）
            let id_title = find_id_title(&out_dir, &date).unwrap_or_else(|| "diary".to_string());
            let id_title_path = format!(
                "{}{}/",
                path.trim_end_matches('/'),
                format!("/{}", id_title)
            );
            write_html(&out_dir, &id_title_path, &html, verbose)?;
        }

        // entry-list (related) ページ
        {
            let page_entries = get_page_entries(&posts, Some(i));
            let list_title = format!("{} {} の関連記事", detail.date, detail.title);
            let list_url = format!("{}related/", path);
            let content = render_entry_list_content(&page_entries, &list_title, &list_url)?;
            let nav = render_nav_entry_list();
            let title = format!("{} {} の関連記事", detail.date, detail.title);
            let canonical_url = format!("https://blog.bouzuya.net{}related/", path);
            let html = render_page(&title, &canonical_url, "", &nav, &content)?;
            write_html(&out_dir, &format!("{}related/", path), &html, verbose)?;
        }
    }

    // public ディレクトリ内の静的ファイルを out_dir に出力
    for file in PUBLIC_DIR.files() {
        let dest = out_dir.join(file.path());
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut f = File::create(&dest)?;
        f.write_all(file.contents())?;
        if verbose {
            println!("{}", dest.display());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        let date = parse_date("2024-01-15").unwrap();
        assert_eq!(date.yyyy, "2024");
        assert_eq!(date.mm, "01");
        assert_eq!(date.dd, "15");
    }

    #[test]
    fn test_og_description() {
        let short = "hello world";
        assert_eq!(og_description(short), "hello world");

        let long: String = "a".repeat(200);
        assert_eq!(og_description(&long).len(), 100);
    }

    #[test]
    fn test_get_page_entries() {
        let make_post = |date: &str, title: &str| PostEntry {
            date: date.to_string(),
            minutes: 5,
            pubdate: format!("{}T00:00:00+09:00", date),
            tags: vec![],
            title: title.to_string(),
        };

        let posts: Vec<PostEntry> = (1..=20)
            .map(|i| make_post(&format!("2024-01-{:02}", i), &format!("Title {}", i)))
            .collect();

        // ルートページ: 最新9件（逆順）
        let root = get_page_entries(&posts, None);
        assert_eq!(root.len(), 9);
        assert_eq!(root[0].date, "2024-01-20");
        assert_eq!(root[8].date, "2024-01-12");

        // フォーカス位置10（index 9）: 前後4件
        let focused = get_page_entries(&posts, Some(9));
        assert_eq!(focused.len(), 9);
        assert_eq!(focused[0].date, "2024-01-06");
        assert_eq!(focused[4].date, "2024-01-10");
        assert_eq!(focused[8].date, "2024-01-14");

        // 先頭付近
        let near_start = get_page_entries(&posts, Some(1));
        assert_eq!(near_start.len(), 6);
        assert_eq!(near_start[0].date, "2024-01-01");

        // 末尾付近
        let near_end = get_page_entries(&posts, Some(18));
        assert_eq!(near_end.len(), 6);
        assert_eq!(near_end[near_end.len() - 1].date, "2024-01-20");
    }
}
