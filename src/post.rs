use anyhow::{bail, Context, Ok, Result};
use chrono::NaiveDate;
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

struct Header {
    date: String,
    description: String,
    draft: bool,
    tags: Vec<String>,
    title: String,
}

impl Header {
    fn new(
        date: String,
        description: String,
        draft: bool,
        tags: Vec<String>,
        title: String,
    ) -> Self {
        Self {
            date,
            description,
            draft,
            tags,
            title,
        }
    }
}

#[derive(Debug)]
struct Post {
    content: String,
    date: NaiveDate,
    description: String,
    draft: bool,
    tags: Vec<String>,
    title: String,
}
impl Post {
    fn new() -> Self {
        Self {
            content: String::new(),
            date: NaiveDate::default(),
            description: String::new(),
            draft: false,
            tags: vec![],
            title: String::new(),
        }
    }
}

impl Post {}

fn parse_header(lines: Vec<String>) -> Result<Post> {
    // Ensure minimal length required for header
    if lines.len() < 7 {
        bail!("File contains an incomplete header.");
    }

    // Check the starting marker
    if lines[0].trim() != "---" {
        bail!("Starting marker missing or formatted incorrectly");
    }

    // Check the ending marker
    if lines[6].trim() != "---" {
        bail!("Ending marker missing or formatted incorrectly");
    }
    let mut post = Post::new();

    for line in &lines[1..6] {
        let (key, value) = line
            .split_once(":")
            .with_context(|| format!("Failed to parse line '{}' into key value pair.", line))?;
        match key {
            "title" => {
                post.title = parse_title(value)?;
            }
            "description" => {
                post.description = parse_description(value)?;
            }
            "date" => {
                post.date = parse_date(value)?;
            }
            "tags" => {
                post.tags = parse_tags(value)?;
            }
            "draft" => {
                post.draft = parse_draft(value)?;
            }
            _ => {
                bail!("Unsupported header field: {}", key);
            }
        }
    }
    println!("{:?}", post); // Only for testing
    Ok(post)
}

pub fn parse_title(mut title: &str) -> anyhow::Result<String> {
    title = title.trim();
    if title.is_empty() {
        bail!("The title cannot be empty or consist of only whitespace characters.");
    }
    return Ok(title.trim().to_owned());
}

pub fn parse_description(mut description: &str) -> anyhow::Result<String> {
    description = description.trim();
    if description.is_empty() {
        bail!("The description cannot be empty or consist of only whitespace characters.");
    }
    return Ok(description.trim().to_owned());
}

pub fn parse_date(date: &str) -> anyhow::Result<NaiveDate> {
    let parsed_date = NaiveDate::parse_from_str(date.trim(), "%Y-%m-%d")
        .context("The date must be a valid string in YYYY-MM-DD format")?;
    return Ok(parsed_date);
}
pub fn parse_tags(tags: &str) -> anyhow::Result<Vec<String>> {
    let tags_vec: Vec<&str> = tags.split(',').collect();

    if tags_vec.is_empty() || tags_vec.iter().any(|&s| s.is_empty()) {
        bail!("The tags field requires at least one non-empty value.");
    }

    Ok(tags_vec.into_iter().map(|s| s.to_string()).collect())
}

pub fn parse_draft(draft: &str) -> anyhow::Result<bool> {
    return match draft.trim() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => bail!("Draft field must be either 'true' or 'false'."),
    };
}

// TODO:
// - Check that file extension is .md/.MD
// - Check that end marker is present
// - After parsing, verify all fields are set.
// - Add dedicated methods to parse each field
// - Ensure proper error propagation on failure
pub fn from_file(filepath: &Path) -> anyhow::Result<()> {
    println!("Parsing {}", filepath.display());
    let file = fs::File::open(filepath).context("Failed to open file.")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let _ = parse_header(lines)?;
    Ok(())
}

#[cfg(test)]
mod post_test {
    use crate::post::parse_date;
    use crate::post::parse_description;
    use crate::post::parse_draft;
    use crate::post::parse_tags;
    use crate::post::parse_title;

    use super::{from_file, parse_header};

    #[test]
    fn parse_header_incomplete() {
        let expected = "---
title: test title
description: test description
---"
        .lines()
        .map(String::from)
        .collect();

        let res = parse_header(expected).unwrap_err();
        assert_eq!(res.to_string(), "File contains an incomplete header.");
    }

    #[test]
    fn parse_header_missing_start_marker() {
        let expected = "title: test title
description: test description
tags: test,test2,test3
date: 2023-01-23
draft: false
---
more data
some more text"
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
        let expected = "---
title: test title
description: test description
tags: test,test2,test3
date: 2023-01-23
draft: false

some more data
goes here
"
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
    fn parse_header_valid() {
        let expected: Vec<String> = "---
title: test title
description: test description
tags: test,test2,test3
date: 2023-01-23
draft: false
---
some more data
"
        .lines()
        .map(String::from)
        .collect();
        // TODO: Validate parsed fields properly
        assert!(parse_header(expected).is_ok());
    }

    #[test]
    fn parse_title_valid() {
        let title = "My title";
        assert!(parse_title(title).is_ok());
    }

    #[test]
    fn parse_title_empty() {
        let title = "";
        assert_eq!(
            parse_title(title).unwrap_err().to_string(),
            "The title cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_title_whitespace() {
        let title = " \t";
        assert_eq!(
            parse_title(title).unwrap_err().to_string(),
            "The title cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_description_valid() {
        let description = "My description";
        assert!(parse_description(description).is_ok());
    }

    #[test]
    fn parse_description_empty() {
        let description = "";
        assert_eq!(
            parse_description(description).unwrap_err().to_string(),
            "The description cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_description_whitespace() {
        let description = " \t";
        assert_eq!(
            parse_description(description).unwrap_err().to_string(),
            "The description cannot be empty or consist of only whitespace characters."
        );
    }

    #[test]
    fn parse_date_valid() {
        let date = "2023-03-17";
        assert!(parse_date(date).is_ok());
    }

    #[test]
    fn parse_date_invalid() {
        let date = " \t";
        assert_eq!(
            parse_date(date).unwrap_err().to_string(),
            "The date must be a valid string in YYYY-MM-DD format"
        );
    }

    #[test]
    fn parse_tags_valid() {
        let tag_str = "tag1,tag2,tag3";
        let tags: Vec<String> = parse_tags(tag_str).unwrap();
        assert_eq!(vec!["tag1", "tag2", "tag3"], tags);
    }

    #[test]
    fn parse_tags_no_tags() {
        let tag_str = "";
        let tags = parse_tags(tag_str).unwrap_err().to_string();
        assert_eq!(
            "The tags field requires at least one non-empty value.",
            tags
        );
    }

    #[test]
    fn parse_tags_no_empty_tags() {
        let tag_str = "test1,test2,,test3,test4";
        let tags = parse_tags(tag_str).unwrap_err().to_string();
        assert_eq!(
            "The tags field requires at least one non-empty value.",
            tags
        );
    }

    #[test]
    fn parse_draft_valid() {
        let draft = "true";
        let draft2 = "false";
        let r1 = parse_draft(draft).ok();
        let r2 = parse_draft(draft2).ok();
        assert_eq!(r1.unwrap(), true);
        assert_eq!(r2.unwrap(), false);
    }

    #[test]
    fn parse_draft_invalid() {
        let draft = "test";
        let draft_res = parse_draft(draft).unwrap_err().to_string();
        assert_eq!("Draft field must be either 'true' or 'false'.", draft_res);
    }
}
