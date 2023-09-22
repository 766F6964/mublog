use std::path::Path;

use anyhow::bail;

pub trait TruncWithDots {
    fn trunc_with_dots(&self, max_length: usize) -> String;
    fn derive_unique_filename(&self, directory: &Path) -> anyhow::Result<String>;
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
    fn derive_unique_filename(&self, directory: &Path) -> anyhow::Result<String> {
        let ext = ".md";
        let stripped_title = &self
            .trim()
            .replace([' ', '.'], "_")
            .to_lowercase();
        let filename = format!("{stripped_title}{ext}");
        let file_path = directory.join(filename);

        if !file_path.exists() {
            return Ok(format!("{stripped_title}{ext}"));
        }

        for i in 0..=128 {
            let suffix = if i == 0 {
                Self::new()
            } else {
                format!("_{i}")
            };
            let suffixed_filename = format!("{stripped_title}{suffix}{ext}");
            let suffixed_file_path = directory.join(&suffixed_filename);

            if !suffixed_file_path.exists() {
                return Ok(suffixed_filename);
            }
        }

        bail!("Failed to derive a unique filename for the given title.")
    }
}
