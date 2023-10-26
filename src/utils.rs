use anyhow::bail;
use anyhow::Context;
use std::{fs, path::Path};

pub trait TruncWithDots {
    fn trunc_with_dots(&self, max_length: usize) -> String;
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

pub fn is_valid_dir(path: &Path) -> anyhow::Result<()> {
    if !path.is_dir() {
        bail!("Path is not a directory.")
    }
    fs::metadata(path).context("Directory not present or inaccessible")?;
    Ok(())
}
