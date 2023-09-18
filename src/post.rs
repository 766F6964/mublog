use anyhow::{anyhow, Context, Ok};
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

pub fn from_file(filepath: &Path) -> anyhow::Result<()> {
    println!("Parsing {}", filepath.display());
    let file = fs::File::open(filepath).context("Failed to open file.")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

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
                return Err(anyhow!("Unsupported header field"))?;
            }
        }
    }
    // TODO:
    // - Check that file extension is .md/.MD
    // - Check that end marker is present
    // - After parsing, verify all fields are set.
    // - Add dedicated methods to parse each field
    // - Ensure proper error propagation on failure

    // if filepath.exists() && filepath.is_file() {
    //     let file = fs::read_to_string(filepath)
    //         .with_context(|| format!("Failed to read file {}", filepath.display()))?;
    //     let mut lines = file.lines();

    //     // Ensure first line is the starting marker.
    //     let start_marker = lines.next().unwrap_or_default();
    //     if start_marker != "---" {
    //         return Err(anyhow::anyhow!("No starting marker found in post header."))?;
    //     }
    //     // Ensure the presence of all header fields.
    //     for line in lines {
    //         let (key, value) = line
    //             .split_once(":")
    //             .with_context(|| format!("Failed to parse line '{}' into key value pair.", line))?;
    //         match key {
    //             "title" => {
    //                 println!("Title value: {}", value)
    //             }
    //             "description" => {
    //                 println!("Description value: {}", value)
    //             }
    //             "date" => {
    //                 println!("Date value: {}", value)
    //             }
    //             "tags" => {
    //                 println!("Tags value: {}", value)
    //             }
    //             "draft" => {
    //                 println!("Draft value: {}", value)
    //             }
    //             _ => {
    //                 return Err(anyhow::anyhow!("Unsupported header field"));
    //             }
    //         }
    //     }

    //     let end_marker = lines.next().unwrap_or_default();
    //     if end_marker != "---" {
    //         return Err(anyhow::anyhow!("No ending marker found in post header."))?;
    //     }
    // }
    Ok(())
}
