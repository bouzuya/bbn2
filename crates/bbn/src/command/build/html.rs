use anyhow::Context;
use std::fs::File;
use std::fs::{self};
use std::io::BufReader;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
    let desc: String = data.chars().take(100).collect();
    html_escape(&desc)
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn render_nav_entry_detail(
    prev: Option<&PostEntry>,
    next: Option<&PostEntry>,
    date: &DateParts,
) -> String {
    let mut nav = String::from("<nav class=\"nav\">");
    if let Some(prev) = prev {
        let prev_date = parse_date(&prev.date).unwrap();
        nav.push_str(&format!(
            "<a class=\"nav-prev\" href=\"{}\">prev</a>",
            entry_path(&prev_date)
        ));
    }
    nav.push_str(&format!(
        "<a class=\"nav-list\" href=\"/{}/{}/{}/related/\">list</a>",
        date.yyyy, date.mm, date.dd
    ));
    if let Some(next) = next {
        let next_date = parse_date(&next.date).unwrap();
        nav.push_str(&format!(
            "<a class=\"nav-next\" href=\"{}\">next</a>",
            entry_path(&next_date)
        ));
    }
    nav.push_str("</nav>");
    nav
}

fn render_nav_entry_list() -> String {
    String::new()
}

fn render_entry_detail_content(detail: &EntryDetail, date: &DateParts) -> String {
    let path = entry_path(date);
    format!(
        r#"<div class="entry-detail"><article class="entry"><header class="header"><h1 class="id-title"><a href="{path}"><span class="id">{date_str}</span><span class="separator"> </span><span class="title">{title}</span></a></h1></header><div class="body"><section class="content">{html}</section></div><footer class="footer"><a class="permalink" href="{path}"><time class="pubdate" datetime="{pubdate}">{pubdate}</time></a></footer></article></div>"#,
        path = path,
        date_str = html_escape(&detail.date),
        title = html_escape(&detail.title),
        html = detail.html,
        pubdate = html_escape(&detail.pubdate),
    )
}

fn render_entry_list_content(entries: &[&PostEntry], list_title: &str, list_url: &str) -> String {
    let mut items = String::new();
    for entry in entries {
        let date = parse_date(&entry.date).unwrap();
        let path = entry_path(&date);
        items.push_str(&format!(
            r#"<li class="entry-list-item"><div class="entry"><a href="{path}"><span class="id">{date_str}</span><span class="separator"> </span><span class="title">{title}</span></a></div></li>"#,
            path = path,
            date_str = html_escape(&entry.date),
            title = html_escape(&entry.title),
        ));
    }
    format!(
        r#"<div class="entry-list"><nav><header class="header"><h1><a href="{list_url}">{list_title}</a></h1></header><div class="body"><ul class="entry-list">{items}</ul></div><footer class="footer"></footer></nav></div>"#,
        list_url = html_escape(list_url),
        list_title = html_escape(list_title),
        items = items,
    )
}

fn render_page(
    title: &str,
    canonical_url: &str,
    description: &str,
    nav: &str,
    content: &str,
) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="ja" prefix="og: http://ogp.me/ns#">
<head>
<meta charset="UTF-8">
<title>{title} - blog.bouzuya.net</title>
<meta name="robots" content="index, follow">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="twitter:card" content="summary">
<meta name="twitter:site" content="@bouzuya">
<meta name="twitter:creator" content="@bouzuya">
<meta property="og:title" content="{og_title}">
<meta property="og:url" content="{canonical_url}">
<meta property="og:image" content="https://blog.bouzuya.net/images/favicon.png">
<meta property="og:description" content="{description}">
<meta property="og:site_name" content="blog.bouzuya.net">
<meta name="theme-color" content="#4e6a41">
<link rel="alternate" type="application/atom+xml" href="/atom.xml">
<link rel="icon" sizes="192x192" href="https://blog.bouzuya.net/images/favicon.png">
<link rel="apple-touch-icon" sizes="192x192" href="https://blog.bouzuya.net/images/favicon.png">
</head>
<body>
<div class="app">
<header class="header">
<h1 class="title"><a href="/">blog.bouzuya.net</a></h1>
{nav}
</header>
<div class="body">
{content}
</div>
<footer class="footer">
{nav}
</footer>
</div>
</body>
</html>"##,
        title = html_escape(title),
        og_title = html_escape(title),
        canonical_url = html_escape(canonical_url),
        description = description,
        nav = nav,
        content = content,
    )
}

fn write_html(out_dir: &Path, path: &str, html: &str) -> anyhow::Result<()> {
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

    // ルートパス以外は .html も出力
    if path != "/" {
        let trimmed = path.trim_start_matches('/').trim_end_matches('/');
        let html_path = out_dir.join(format!("{}.html", trimmed));
        if let Some(parent) = html_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = File::create(&html_path)?;
        file.write_all(html.as_bytes())?;
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

pub fn run(out_dir: PathBuf) -> anyhow::Result<()> {
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
        let content = render_entry_list_content(&page_entries, "最近の記事", "/");
        let nav = render_nav_entry_list();
        let html = render_page(
            "blog.bouzuya.net",
            "https://blog.bouzuya.net/",
            "",
            &nav,
            &content,
        );
        write_html(&out_dir, "/", &html)?;
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
            let nav = render_nav_entry_detail(prev, next, &date);
            let content = render_entry_detail_content(detail, &date);
            let title = format!("{} {}", detail.date, detail.title);
            let canonical_url = format!("https://blog.bouzuya.net{}", path);
            let description = og_description(&detail.data);
            let html = render_page(&title, &canonical_url, &description, &nav, &content);
            write_html(&out_dir, &path, &html)?;

            // idTitle ページ（entry-detail と同内容）
            let id_title = find_id_title(&out_dir, &date).unwrap_or_else(|| "diary".to_string());
            let id_title_path = format!(
                "{}{}/",
                path.trim_end_matches('/'),
                format!("/{}", id_title)
            );
            write_html(&out_dir, &id_title_path, &html)?;
        }

        // entry-list (related) ページ
        {
            let page_entries = get_page_entries(&posts, Some(i));
            let list_title = format!("{} {} の関連記事", detail.date, detail.title);
            let list_url = format!("{}related/", path);
            let content = render_entry_list_content(&page_entries, &list_title, &list_url);
            let nav = render_nav_entry_list();
            let title = format!("{} {} の関連記事", detail.date, detail.title);
            let canonical_url = format!("https://blog.bouzuya.net{}related/", path);
            let html = render_page(&title, &canonical_url, "", &nav, &content);
            write_html(&out_dir, &format!("{}related/", path), &html)?;
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
    fn test_html_escape() {
        assert_eq!(
            html_escape("<a>&\"b\"</a>"),
            "&lt;a&gt;&amp;&quot;b&quot;&lt;/a&gt;"
        );
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
