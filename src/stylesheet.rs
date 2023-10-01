use std::{ffi::OsStr, fs, path::Path};

use anyhow::Context;
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct Stylesheet {
    pub content: String,
    pub name: String,
}

pub fn get_stylesheets(css_dir: &Path) -> anyhow::Result<Vec<Stylesheet>> {
    let entries = WalkDir::new(css_dir);
    let mut stylesheets = vec![];
    for entry in entries.into_iter().filter_map(std::result::Result::ok) {
        let css_path = entry.path();
        let css_filename = css_path
            .file_name()
            .context("Failed to obtain filename for css file")?
            .to_string_lossy()
            .to_string();
        if css_path.extension().and_then(OsStr::to_str) == Some("css") {
            let file_content = fs::read_to_string(css_path).with_context(|| {
                format!(
                    "Failed to open stylesheet file '{css_filename}' for reading"
                )
            })?;
            stylesheets.push(Stylesheet {
                content: file_content,
                name: css_filename,
            });
        }
    }
    Ok(stylesheets)
}
