use std::{ffi::OsStr, fs, path::Path};

use anyhow::Context;
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct Stylesheet {
    pub content: String,
    pub filename: String,
}
