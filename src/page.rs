use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::collections;
use std::collections::HashSet;

#[derive(Debug, Default)]
struct Page {
    content: String,
    draft: bool,
    index: bool,
    title: String,
}

pub fn parse_from_string(data: String) -> Result<()> {
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
            "index" => page.draft = parse_index(value)?,
            "title" => page.title = parse_title(value)?,
            _ => bail!("Unsupported page header field: '{key}'"),
        }
    }

    // Read remaining content
    if lines.len() > 3 {
        page.content = lines[4..].join("\n");
    }
    Ok(())
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
        let exp = "---\ntitle: test title\n---".to_owned();
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Page header is incomplete");
    }

    #[test]
    fn parse_page_header_missing_start_marker() {
        let exp = "title: test\ndraft: false\nindex: false\n---\nHello World".to_owned();
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Page start marker missing or formatted incorrectly");
    }

    #[test]
    fn parse_page_header_missing_end_marker() {
        let exp = "---\ntitle: test\ndraft: false\nindex: false\nHello World".to_owned();
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Page end marker missing or formatted incorrectly");
    }

    #[test]
    fn parse_page_header_duplicate_fields() {
        let exp = "---\ntitle: test\ndraft: false\ndraft: true\n---\nHello World".to_owned();
        let res = parse_from_string(exp).unwrap_err().to_string();
        assert_eq!(res, "Duplicate page header field found: 'draft'");
    }
}
