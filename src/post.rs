use anyhow::{Context, Ok};
use chrono::{format, prelude::*};
use std::{
    fs,
    io::{BufReader, Lines},
    iter::Peekable,
    str::Chars,
};
use std::{fs::File, path::Path};

struct Post {
    title: String,
    description: String,
    date: String,
    tags: Vec<String>,
    draft: bool,
    content: Vec<u8>,
}

pub fn from_file(filepath: &Path) -> anyhow::Result<()> {
    println!("Parsing post from file ...");
    // 1. Check that the path is valid, and that it is a .md file
    if filepath.exists() && filepath.is_file() {
        // Read the file
        let file = fs::read_to_string(filepath)
            .with_context(|| format!("Failed to read file {}", filepath.display()))?;
        // Parse the file contents to check if the md header is present
        let mut lines = file.lines();

        if lines.next().expect("Header has no starting marker") == "---" {
            for line in lines {
                let (key, value) = line.split_once(":").with_context(|| {
                    format!("Failed to parse line '{}' into key value pair.", line)
                })?;
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
                        return Err(anyhow::anyhow!("Unsupported header field"));
                    }
                }
            }
        }
    }
    // Check if file exist and is accessible, when that is the case, do validation
    // Return Option<Post>
    Ok(())
}
