use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    data_dir: PathBuf,
    hatena_blog_data_file: PathBuf,
    link_completion_rules_file: Option<PathBuf>,
    out_dir: Option<PathBuf>,
}

impl Config {
    pub fn new(
        data_dir: PathBuf,
        hatena_blog_data_file: PathBuf,
        link_completion_rules_file: Option<PathBuf>,
        out_dir: Option<PathBuf>,
    ) -> Self {
        Self {
            data_dir,
            hatena_blog_data_file,
            link_completion_rules_file,
            out_dir,
        }
    }

    pub fn data_dir(&self) -> &Path {
        self.data_dir.as_path()
    }

    pub fn hatena_blog_data_file(&self) -> &Path {
        self.hatena_blog_data_file.as_path()
    }

    pub fn link_completion_rules_file(&self) -> Option<&Path> {
        self.link_completion_rules_file.as_deref()
    }

    pub fn out_dir(&self) -> Option<&Path> {
        self.out_dir.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn config_test() -> anyhow::Result<()> {
        let temp_dir = tempdir()?;
        let data_dir = temp_dir.path().join("data");
        fs::create_dir_all(data_dir.as_path())?;
        let hatena_blog_data_file = temp_dir.path().join("hatena_blog.db");
        let link_completion_rules_file = temp_dir.path().join("link_completion_rules.json");
        let out_dir = temp_dir.path().join("out");

        let config = Config::new(data_dir.clone(), hatena_blog_data_file.clone(), None, None);
        assert_eq!(config.data_dir(), data_dir.as_path());
        assert_eq!(
            config.hatena_blog_data_file(),
            hatena_blog_data_file.as_path()
        );
        assert_eq!(config.link_completion_rules_file(), None);
        assert_eq!(config.out_dir(), None);
        assert_eq!(config.clone(), config);

        let config = Config::new(
            data_dir.clone(),
            hatena_blog_data_file.clone(),
            Some(link_completion_rules_file.clone()),
            Some(out_dir.clone()),
        );
        assert_eq!(config.data_dir(), data_dir.as_path());
        assert_eq!(
            config.hatena_blog_data_file(),
            hatena_blog_data_file.as_path()
        );
        assert_eq!(
            config.link_completion_rules_file(),
            Some(link_completion_rules_file.as_path())
        );
        assert_eq!(config.out_dir(), Some(out_dir.as_path()));
        assert_eq!(config.clone(), config);
        Ok(())
    }
}
