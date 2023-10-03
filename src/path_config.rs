use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct PathConfig {
    pub config_file: PathBuf,
    pub assets_dir: PathBuf,
    pub base_dir: PathBuf,
    pub build_assets_dir: PathBuf,
    pub build_css_dir: PathBuf,
    pub build_dir: PathBuf,
    pub build_meta_dir: PathBuf,
    pub build_pages_dir: PathBuf,
    pub build_posts_dir: PathBuf,
    pub css_dir: PathBuf,
    pub meta_dir: PathBuf,
    pub pages_dir: PathBuf,
    pub posts_dir: PathBuf,
}

impl PathConfig {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            config_file: working_dir.join("mublog.toml"),
            base_dir: working_dir.to_path_buf(),
            build_dir: working_dir.join("build"),
            build_pages_dir: working_dir.join("build"),
            build_assets_dir: working_dir.join("build").join("assets"),
            build_css_dir: working_dir.join("build").join("css"),
            build_posts_dir: working_dir.join("build").join("posts"),
            build_meta_dir: working_dir.join("build").join("meta"),
            assets_dir: working_dir.join("assets"),
            css_dir: working_dir.join("css"),
            posts_dir: working_dir.join("posts"),
            meta_dir: working_dir.join("meta"),
            pages_dir: working_dir.join("pages"),
        }
    }
}
