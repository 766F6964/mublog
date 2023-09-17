use std::fs;
use std::path::Path;
use anyhow::Context;
use include_dir::{Dir, File, include_dir};

static EMBEDDED_RES: Dir = include_dir!("$CARGO_MANIFEST_DIR/res/");

pub fn get_resources(dir_path: &str) -> anyhow::Result<Vec<&File>, anyhow::Error> {
    let mut files = vec![];
    let resources = EMBEDDED_RES.get_dir(dir_path)
        .with_context(|| format!("Failed to locate embedded resource {}", dir_path))
        .unwrap();

    for file in resources.files() {
        files.push(file);
    }
    if files.is_empty() {
        return Err(anyhow::anyhow!("No files found in embedded directory: {}", dir_path));
    }

    Ok(files)
}

pub fn write_resources(resources: Vec<&File>, dst_dir: &Path) -> anyhow::Result<(), anyhow::Error> {
    if resources.is_empty() {
        return Err(anyhow::anyhow!("Empty resource vector is not writable"));
    }
    if !dst_dir.exists() {
        let path_str = dst_dir.to_str();
        return Err(anyhow::anyhow!(format!("Can't write embedded resources. Path '{path_str:?}' does not exist.")));
    }
    for resource in resources {
        let res_path = resource.path();
        let filename = res_path.file_name()
            .with_context(|| format!("Failed to resolve filename from path {res_path:?}"))
            .unwrap();
        fs::write(dst_dir.join(filename), resource.contents())
            .with_context(|| format!("Failed to write resource {res_path:?} to disk"))?;
    }
    Ok(())
}
