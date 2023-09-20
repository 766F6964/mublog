use anyhow::{anyhow, bail, Context, Ok};
use chrono::NaiveDate;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Default)]
pub struct Post {
    pub content: String,
    pub header: PostHeader,
}

impl Post {
    fn new(header: PostHeader, content: String) -> Self {
        Self { header, content }
    }
}

#[derive(Debug, Default)]
pub struct PostHeader {
    pub date: NaiveDate,
    pub description: String,
    pub draft: bool,
    pub tags: Vec<String>,
    pub title: String,
}

fn parse_header(lines: Vec<String>) -> anyhow::Result<PostHeader> {
    // Ensure minimal length required for header
    if lines.len() < 7 {
        bail!("File contains an incomplete header.");
    }

    // Check starting marker presence
    if lines[0].trim() != "---" {
        bail!("Starting marker missing or formatted incorrectly");
    }

    // Check ending marker presence
    if lines[6].trim() != "---" {
        bail!("Ending marker missing or formatted incorrectly");
    }

    // Check header fields
    let mut post = PostHeader::default();

    for line in &lines[1..6] {
        let (key, value) = line
            .split_once(':')
            .with_context(|| format!("Failed to parse line '{line}' into key value pair."))?;
        match key {
            "title" => post.title = parse_title(value)?,
            "description" => post.description = parse_description(value)?,
            "date" => post.date = parse_date(value)?,
            "tags" => post.tags = parse_tags(value)?,
            "draft" => post.draft = parse_draft(value)?,
            _ => bail!("Unsupported header field: {key}"),
        }
    }
    Ok(post)
}

pub fn parse_title(mut title: &str) -> anyhow::Result<String> {
    title = title.trim();
    if title.is_empty() {
        bail!("The title cannot be empty or consist of only whitespace characters.");
    }
    Ok(title.to_owned())
}

pub fn parse_description(mut description: &str) -> anyhow::Result<String> {
    description = description.trim();
    if description.is_empty() {
        bail!("The description cannot be empty or consist of only whitespace characters.");
    }
    Ok(description.to_owned())
}

pub fn parse_date(date: &str) -> anyhow::Result<NaiveDate> {
    let parsed_date = NaiveDate::parse_from_str(date.trim(), "%Y-%m-%d")
        .context("The date must be a valid string in YYYY-MM-DD format")?;
    Ok(parsed_date)
}
pub fn parse_tags(tags: &str) -> anyhow::Result<Vec<String>> {
    let tags_vec: Vec<&str> = tags.trim().split(',').collect();
    if tags_vec.is_empty() || tags_vec.iter().any(|&s| s.is_empty()) {
        bail!("The tags field requires at least one non-empty value.");
    }
    Ok(tags_vec.into_iter().map(|s| s.to_string()).collect())
}

pub fn parse_draft(draft: &str) -> anyhow::Result<bool> {
    draft
        .trim()
        .parse::<bool>()
        .map_err(|_| anyhow!("Draft field must be either 'true' or 'false'."))
}

// TODO:
// - Check that file extension is .md/.MD
// - Check that end marker is present
// - After parsing, verify all fields are set.
// - Ensure proper error propagation on failure
pub fn from_file(filepath: &Path) -> anyhow::Result<Post> {
    let path = filepath.display();
    let file = fs::File::open(filepath).context("Failed to open file.")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let header =
        parse_header(lines).with_context(|| format!("Failed to parse header in {path}"))?;
    let post = Post::new(header, String::new());
    anyhow::Ok(post)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_header_incomplete() {
        let expected = "---\ntitle: test title\ndescription: test description\n---"
            .lines()
            .map(String::from)
            .collect();

        let res = parse_header(expected).unwrap_err();
        assert_eq!(res.to_string(), "File contains an incomplete header.");
    }

    #[test]
    fn parse_header_missing_start_marker() {
        let expected = "title: test title\ndescription: test description\ntags: test,test2,test3\ndate: 2023-01-23\ndraft: false\n---\nmore data\nsome more text"
            .lines()
            .map(String::from)
            .collect();

        let res = parse_header(expected).unwrap_err();
        assert_eq!(
            res.to_string(),
            "Starting marker missing or formatted incorrectly"
        );
    }

    #[test]
    fn parse_header_missing_end_marker() {
        let expected = "---\ntitle: test title\ndescription: test description\ntags: test,test2,test3\ndate: 2023-01-23\ndraft: false\nsome more data\ngoes here\n"
        .lines()
        .map(String::from)
        .collect();

        let res = parse_header(expected).unwrap_err();
        assert_eq!(
            res.to_string(),
            "Ending marker missing or formatted incorrectly"
        );
    }

    #[test]
    fn parse_header_unsupported_field() {
        let expected = "---\ntitle: test title\ndescription: test description\nunsupported: test,test2,test3\ndate: 2023-01-23\ndraft: false\n---\nsome more data"
        .lines()
        .map(String::from)
        .collect();

        let res = parse_header(expected).unwrap_err();
        assert_eq!(res.to_string(), "Unsupported header field: unsupported");
    }

    #[test]
    fn parse_header_valid_with_content() {
        let expected: Vec<String> = "---\ntitle: test title\ndescription: test description\ntags: test,test2,test3\ndate: 2023-01-23\ndraft: false\n---\nsome more data"
        .lines()
        .map(String::from)
        .collect();

        let res = parse_header(expected).expect("Header should be valid"); // TODO: Validate parsed fields properly
        assert_eq!(res.title, "test title");
        assert_eq!(res.description, "test description");
        assert_eq!(res.tags, vec!["test", "test2", "test3"]);
        assert_eq!(
            res.date,
            NaiveDate::parse_from_str("2023-01-23", "%Y-%m-%d")
                .expect("Date conversion should pass")
        );
        assert_eq!(res.draft, false);
        // assert_eq!(res.content, "some more data");
    }

    #[test]
    fn parse_header_valid_no_content() {
        let expected: Vec<String> = "---\ntitle: test title\ndescription: test description\ntags: test,test2,test3\ndate: 2023-01-23\ndraft: false\n---"
        .lines()
        .map(String::from)
        .collect();

        let res = parse_header(expected).expect("Header should be valid"); // TODO: Validate parsed fields properly
        assert_eq!(res.title, "test title");
        assert_eq!(res.description, "test description");
        assert_eq!(res.tags, vec!["test", "test2", "test3"]);
        assert_eq!(
            res.date,
            NaiveDate::parse_from_str("2023-01-23", "%Y-%m-%d")
                .expect("Date conversion should pass")
        );
        assert_eq!(res.draft, false);
        // assert_eq!(res.content, "");
    }

    #[test]
    fn parse_title_valid() {
        let title = "My title";
        assert!(parse_title(title).is_ok());
    }

    #[test]
    fn parse_title_empty() {
        let title = parse_title("").unwrap_err().to_string();
        assert_eq!(
            title,
            "The title cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_title_whitespace() {
        let title = parse_title(" \t").unwrap_err().to_string();
        assert_eq!(
            title,
            "The title cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_description_valid() {
        assert!(parse_description("My description").is_ok());
    }

    #[test]
    fn parse_description_empty() {
        assert_eq!(
            parse_description("").unwrap_err().to_string(),
            "The description cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_description_whitespace() {
        assert_eq!(
            parse_description(" \r\t").unwrap_err().to_string(),
            "The description cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_date_valid() {
        assert!(parse_date("2023-03-17").is_ok());
    }

    #[test]
    fn parse_date_invalid() {
        assert_eq!(
            parse_date(" \t\r").unwrap_err().to_string(),
            "The date must be a valid string in YYYY-MM-DD format"
        );
    }

    #[test]
    fn parse_tags_valid() {
        let tags: Vec<String> = parse_tags("tag1,tag2,tag3").unwrap();
        assert_eq!(vec!["tag1", "tag2", "tag3"], tags);
    }

    #[test]
    fn parse_tags_no_tags() {
        let tags = parse_tags("").unwrap_err().to_string();
        assert_eq!(
            "The tags field requires at least one non-empty value.",
            tags
        );
    }

    #[test]
    fn parse_tags_no_empty_tags() {
        let tags = parse_tags("test1,test2,,test3,test4")
            .unwrap_err()
            .to_string();
        assert_eq!(
            "The tags field requires at least one non-empty value.",
            tags
        );
    }

    #[test]
    fn parse_draft_valid() {
        let r1 = parse_draft("true").ok();
        let r2 = parse_draft("false").ok();
        assert_eq!(r1.unwrap(), true);
        assert_eq!(r2.unwrap(), false);
    }

    #[test]
    fn parse_draft_invalid() {
        let draft_res = parse_draft("test").unwrap_err().to_string();
        assert_eq!("Draft field must be either 'true' or 'false'.", draft_res);
    }
}
