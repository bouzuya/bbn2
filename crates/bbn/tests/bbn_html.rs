use std::fs;

use assert_cmd::Command;
use tempfile::tempdir;

#[test]
fn test_bbn_build_html() -> anyhow::Result<()> {
    let temp_dir = tempdir()?;

    let config_dir = temp_dir.path().join("config");
    let data_dir = temp_dir.path().join("data");
    let entry_dir = data_dir.join("2021").join("02");
    fs::create_dir_all(entry_dir.as_path())?;
    let meta1 = entry_dir.join("2021-02-03-TITLE.json");
    fs::write(
        meta1,
        r#"{"minutes":5,"pubdate":"2021-02-03T00:00:00+09:00","tags":[],"title":"TITLE1"}"#,
    )?;
    let content1 = entry_dir.join("2021-02-03-TITLE.md");
    fs::write(content1, "hello")?;
    let meta2 = entry_dir.join("2021-02-04.json");
    fs::write(
        meta2,
        r#"{"minutes":5,"pubdate":"2021-02-04T00:00:00+09:00","tags":["tag1"],"title":"TITLE2"}"#,
    )?;
    let content2 = entry_dir.join("2021-02-04.md");
    fs::write(content2, "good bye")?;

    let out_dir = temp_dir.path().join("out");
    fs::create_dir_all(out_dir.as_path())?;

    let hatena_blog_data_file = temp_dir.path().join("hatena-blog.db");
    Command::cargo_bin("bbn")?
        .arg("config")
        .arg("init")
        .arg("--data-dir")
        .arg(&data_dir)
        .arg("--hatena-blog-data-file")
        .arg(&hatena_blog_data_file)
        .arg("--out-dir")
        .arg(out_dir.as_path())
        .env("BBN_TEST_CONFIG_DIR", config_dir.as_path())
        .assert()
        .success();

    // build --json を実行
    Command::cargo_bin("bbn")?
        .arg("build")
        .arg("--json")
        .env("BBN_TEST_CONFIG_DIR", config_dir.as_path())
        .assert()
        .success();

    // build --html を実行
    Command::cargo_bin("bbn")?
        .arg("build")
        .arg("--html")
        .env("BBN_TEST_CONFIG_DIR", config_dir.as_path())
        .assert()
        .success();

    // ルートページが存在すること
    let root_html = fs::read_to_string(out_dir.join("index.html"))?;
    assert!(root_html.contains("blog.bouzuya.net"));
    assert!(root_html.contains("最近の記事"));
    assert!(root_html.contains("2021-02-03"));
    assert!(root_html.contains("2021-02-04"));
    assert!(root_html.contains("TITLE1"));
    assert!(root_html.contains("TITLE2"));

    // entry-detail ページが存在すること
    let detail_html = fs::read_to_string(out_dir.join("2021/02/03/index.html"))?;
    assert!(detail_html.contains("2021-02-03 TITLE1 - blog.bouzuya.net"));
    assert!(detail_html.contains("<p>hello</p>"));
    assert!(detail_html.contains("og:url"));
    // .html 形式も存在すること
    assert!(fs::read_to_string(out_dir.join("2021/02/03.html")).is_ok());

    let detail_html2 = fs::read_to_string(out_dir.join("2021/02/04/index.html"))?;
    assert!(detail_html2.contains("TITLE2"));

    // entry-detail ページの nav に prev/next リンクがあること
    assert!(detail_html.contains("nav-next"));
    assert!(detail_html.contains("/2021/02/04/"));
    assert!(detail_html2.contains("nav-prev"));
    assert!(detail_html2.contains("/2021/02/03/"));

    // related ページが存在すること
    let related_html = fs::read_to_string(out_dir.join("2021/02/03/related/index.html"))?;
    assert!(related_html.contains("の関連記事"));
    assert!(fs::read_to_string(out_dir.join("2021/02/03/related.html")).is_ok());

    // idTitle ページが存在すること（2021-02-03 は TITLE という idTitle を持つ）
    let id_title_html = fs::read_to_string(out_dir.join("2021/02/03/TITLE/index.html"))?;
    assert!(id_title_html.contains("2021-02-03 TITLE1 - blog.bouzuya.net"));
    assert!(fs::read_to_string(out_dir.join("2021/02/03/TITLE.html")).is_ok());

    // idTitle なしのエントリは diary がデフォルト
    let diary_html = fs::read_to_string(out_dir.join("2021/02/04/diary/index.html"))?;
    assert!(diary_html.contains("TITLE2"));
    assert!(fs::read_to_string(out_dir.join("2021/02/04/diary.html")).is_ok());

    Ok(())
}
