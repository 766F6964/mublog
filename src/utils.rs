use std::path::Path;

use anyhow::bail;

pub fn trunc_with_dots(input: String, max_length: usize) -> String {
    if input.len() <= max_length {
        return input; // No need to truncate
    }

    let truncated = &input[..max_length - 3]; // Leave room for "..."
    let result = format!("{truncated}...");

    result
}

pub fn derive_unique_filename(title: String, directory: &Path) -> anyhow::Result<String> {
    let mut filename = title.replace(" ", "_").replace(".", "_").to_lowercase();
    filename.push_str(".md");
    let file_path = directory.join(&filename);

    if !file_path.exists() {
        return Ok(filename);
    }

    for i in 0..=128 {
        let suffix = if i == 0 {
            "".to_string()
        } else {
            format!("_{}", i)
        };
        let suffixed_filename = format!("{}{}", filename, suffix);
        let suffixed_file_path = directory.join(&suffixed_filename);

        if !suffixed_file_path.exists() {
            return Ok(suffixed_filename);
        }
    }

    bail!("Unable to find a unique filename.")
}

// fn main() {
//     let title = "My File Title".to_string();
//     let directory = Path::new("/path/to/your/directory");

//     match derive_unique_filename(title, directory) {
//         Ok(unique_filename) => println!("Unique filename: {}", unique_filename),
//         Err(err) => println!("Error: {}", err),
//     }
// }
