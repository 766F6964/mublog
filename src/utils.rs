use anyhow::bail;
use anyhow::Context;
use std::{fs, path::Path};

pub trait TruncWithDots {
    fn trunc_with_dots(&self, max_length: usize) -> String;
    // fn derive_unique_filename(&self, directory: &Path) -> anyhow::Result<String>;
}

impl TruncWithDots for String {
    fn trunc_with_dots(&self, max_length: usize) -> String {
        if self.len() <= max_length {
            return self.clone();
        }
        let truncated = &self[..max_length - 3];
        let result = format!("{truncated}...");
        result
    }
}

pub fn derive_filename(title: &str, ext: &str, posts_dir: &Path) -> anyhow::Result<String> {
    let stripped_title = title.trim().replace([' ', '.'], "_").to_lowercase();
    let filename = format!("{stripped_title}{ext}");
    let file_path = posts_dir.join(filename);

    if !file_path.exists() {
        return Ok(format!("{stripped_title}{ext}"));
    }

    for i in 0..=128 {
        let suffix = if i == 0 { format!("") } else { format!("{i}") };
        let suffixed_filename = format!("{stripped_title}{suffix}{ext}");
        let suffixed_file_path = posts_dir.join(&suffixed_filename);

        if !suffixed_file_path.exists() {
            return Ok(suffixed_filename);
        }
    }

    bail!("Failed to derive a unique filename for the given title.")
}
pub fn is_valid_dir(path: &Path) -> anyhow::Result<()> {
    if !path.is_dir() {
        bail!("Path is not a directory.")
    }
    fs::metadata(path).context("Directory not present or inaccessible")?;
    Ok(())
}
