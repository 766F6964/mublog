use anyhow::{Context, Ok};
use std::fs;
use std::path::Path;

struct Post {
    content: Vec<u8>,
    date: String,
    description: String,
    draft: bool,
    tags: Vec<String>,
    title: String,
}

pub fn from_file(filepath: &Path) -> anyhow::Result<()> {
    println!("Parsing post from file ...");

    // TODO:
    // - Check that file extension is .md/.MD
    // - Check that end marker is present
    // - After parsing, verify all fields are set.
    // - Add dedicated methods to parse each field
    // - Ensure proper error propagation on failure

    if filepath.exists() && filepath.is_file() {
        let file = fs::read_to_string(filepath)
            .with_context(|| format!("Failed to read file {}", filepath.display()))?;
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
    Ok(())
}
