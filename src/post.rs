use anyhow::{anyhow, Context, Ok, Result};
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::path::Path;

struct Post {
    content: String,
    date: String,
    description: String,
    draft: bool,
    tags: Vec<String>,
    title: String,
}

fn parse_header(lines: Vec<String>) -> Result<()> {
    // Ensure minimal length required for header
    if lines.len() < 7 {
        return Err(anyhow!("File contains an incomplete header."));
    }

    // Check the starting marker
    if lines[0].trim() != "---" {
        return Err(anyhow!("Starting marker missing or formatted incorrectly"))?;
    }

    // Check the ending marker
    if lines[6].trim() != "---" {
        return Err(anyhow!("Ending marker missing or formatted incorrectly"))?;
    }
    // Initialize fields
    let mut title = "";
    let mut description = "";
    let mut date = "";
    let mut tags = "";
    let mut draft = "";
    for line in &lines[1..6] {
        let (key, value) = line
            .split_once(":")
            .with_context(|| format!("Failed to parse line '{}' into key value pair.", line))?;
        match key {
            "title" => {
                println!("Title value: {}", value)
            }
            "description" => {
                println!("Description value: {}", value)
            }
            "date" => {
                println!("Date value: {}", value)
            }
            "tags" => {
                println!("Tags value: {}", value)
            }
            "draft" => {
                println!("Draft value: {}", value)
            }
            _ => {
                return Err(anyhow!("Unsupported header field: {}", key))?;
            }
        }
    }
    Ok(())
}
pub fn from_file(filepath: &Path) -> anyhow::Result<()> {
    println!("Parsing {}", filepath.display());
    let file = fs::File::open(filepath).context("Failed to open file.")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    _ = parse_header(lines);
    // TODO:
    // - Check that file extension is .md/.MD
    // - Check that end marker is present
    // - After parsing, verify all fields are set.
    // - Add dedicated methods to parse each field
    // - Ensure proper error propagation on failure
    Ok(())
}

#[cfg(test)]
mod post_test {
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
        let expected = "---
title: test title
description: test description
unsupported: test,test2,test3
date: 2023-01-23
draft: false
---
some more data
"
        .lines()
        .map(String::from)
        .collect();

        let res = parse_header(expected).unwrap_err();
        assert_eq!(res.to_string(), "Unsupported header field: unsupported");
    }
}
