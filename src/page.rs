use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::collections;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct Page {
    pub content: String,
    pub draft: bool,
    pub index: bool,
    pub title: String,
}

pub fn get_pages(pages_dir: &Path) -> Result<Vec<Page>> {
    let entries = WalkDir::new(pages_dir);
    let mut pages = vec![];
    for entry in entries.into_iter().filter_map(std::result::Result::ok) {
        let page_path = entry.path();
        if page_path.extension().and_then(OsStr::to_str) == Some("md") {
            let file_content = fs::read_to_string(page_path).with_context(|| {
                format!(
                    "Failed to open page file '{}' for reading",
                    page_path.display()
                )
            })?;
            let page = parse_from_string(&file_content)
                .with_context(|| format!("Failed to parse page '{}'", page_path.display()))?;
            pages.push(page);
        }
    }
    Ok(pages)
}

pub fn parse_to_string(page: &Page) -> String {
    let page_str = format!(
        "---\ntitle: {}\ndraft: {}\nindex: {}\n---\n{}",
        page.title, page.draft, page.index, page.content
    );
    page_str
}

pub fn parse_from_string(data: &str) -> Result<Page> {
    let lines: Vec<String> = data.lines().map(ToOwned::to_owned).collect();

    // Check minimal length required to contain header
    if lines.len() < 4 {
        bail!("Page header is incomplete");
    }

    // Check starting marker presence
    if lines[0].trim() != "---" {
        bail!("Page start marker missing or formatted incorrectly");
    }

    // Check ending marker presence
    if lines[4].trim() != "---" {
        bail!("Page end marker missing or formatted incorrectly");
    }

    // Check header fields presence
    let mut page = Page::default();
    let mut processed_fields: HashSet<String> = collections::HashSet::new();

    for line in &lines[1..4] {
        let (key, value) = line
            .split_once(':')
            .with_context(|| format!("Invalid page header line: '{line}'"))?;

        // Check each field only exist exactly once
        if !processed_fields.insert(key.to_owned()) {
            bail!("Duplicate page header field found: '{key}'");
        }
        match key {
            "draft" => page.draft = parse_draft(value)?,
            "index" => page.index = parse_index(value)?,
            "title" => page.title = parse_title(value)?,
            _ => bail!("Unsupported page header field: '{key}'"),
        }
    }

    // Read remaining content
    if lines.len() > 4 {
        page.content = lines[5..].join("\n");
    }
    Ok(page)
}

fn parse_index(index: &str) -> Result<bool> {
    index
        .trim()
        .parse::<bool>()
        .map_err(|_| anyhow!("Draft field must be either 'true' or 'false'."))
}

fn parse_draft(draft: &str) -> Result<bool> {
    draft
        .trim()
        .parse::<bool>()
        .map_err(|_| anyhow!("Draft field must be either 'true' or 'false'."))
}

fn parse_title(mut title: &str) -> Result<String> {
    title = title.trim();
    if title.is_empty() {
        bail!("The title cannot be empty or consist of only whitespace characters.");
    }
    Ok(title.to_owned())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_page_header_incomplete() {
        let exp = "---\ntitle: test title\n---";
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Page header is incomplete");
    }

    #[test]
    fn parse_page_header_missing_start_marker() {
        let exp = "title: test\ndraft: false\nindex: false\n---\nHello World";
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Page start marker missing or formatted incorrectly");
    }

    #[test]
    fn parse_page_header_missing_end_marker() {
        let exp = "---\ntitle: test\ndraft: false\nindex: false\nHello World";
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Page end marker missing or formatted incorrectly");
    }

    #[test]
    fn parse_page_header_duplicate_fields() {
        let exp = "---\ntitle: test\ndraft: false\ndraft: true\n---\nHello World";
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Duplicate page header field found: 'draft'");
    }

    #[test]
    fn parse_page_valid_with_content() {
        let exp = "---\ntitle: my page\ndraft: false\nindex: true\n---\nhello";
        let res = parse_from_string(exp).expect("Page should be valid");
        assert_eq!(res.title, "my page");
        assert_eq!(res.draft, false);
        assert_eq!(res.index, true);
        assert_eq!(res.content, "hello");
    }

    #[test]
    fn parse_page_valid_without_content() {
        let exp = "---\ntitle: my page\ndraft: false\nindex: true\n---\n";
        let res = parse_from_string(exp).expect("Page should be valid");
        assert_eq!(res.title, "my page");
        assert_eq!(res.draft, false);
        assert_eq!(res.index, true);
        assert_eq!(res.content, String::new());
    }
}
