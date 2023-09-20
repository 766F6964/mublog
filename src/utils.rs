pub fn trunc_with_dots(input: String, max_length: usize) -> String {
    if input.len() <= max_length {
        return input.to_string(); // No need to truncate
    }

    let truncated = &input[..max_length - 3]; // Leave room for "..."
    let result = format!("{}...", truncated);

    result
}
