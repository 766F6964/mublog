use anyhow::{bail, Context};
use include_dir::{include_dir, Dir, File};
use std::fs;
use std::path::Path;

static EMBEDDED_RES: Dir = include_dir!("$CARGO_MANIFEST_DIR/res/");

pub fn get_resources(dir_path: &str) -> anyhow::Result<Vec<&File>, anyhow::Error> {
    let mut files = vec![];
    let resources = EMBEDDED_RES
        .get_dir(dir_path)
        .with_context(|| format!("Failed to locate embedded resource {dir_path}"))
        .unwrap();

    for file in resources.files() {
        files.push(file);
    }
    if files.is_empty() {
        bail!(format!(
            "No files found in embedded directory: {}",
            dir_path
        ));
    }

    Ok(files)
}

pub fn get_resource_file(file_name: &str) -> anyhow::Result<&File, anyhow::Error> {
    let resource = EMBEDDED_RES
        .get_file(file_name)
        .with_context(|| format!("Failed to locate embedded resource {file_name}"))
        .unwrap();

    Ok(resource)
}

pub fn write_resource_file(resource: &File, dst_path: &Path) -> anyhow::Result<(), anyhow::Error> {
    let parent_dir = dst_path
        .parent()
        .expect("Unable to determine parent directory for specified resource file.");
    if !parent_dir.exists() {
        println!("Resource File Path: {}", dst_path.display());
        bail!(format!(
            "Failed to extract resource file to {}",
            dst_path.display()
        ));
    }

    fs::write(dst_path, resource.contents())
        .with_context(|| "Failed to write resource to disk".to_string())?;
    Ok(())
}

pub fn write_resources(resources: Vec<&File>, dst_dir: &Path) -> anyhow::Result<(), anyhow::Error> {
    if resources.is_empty() {
        bail!("Empty resource vector is not writable");
    }
    if !dst_dir.exists() {
        let path_str = dst_dir.to_str();
        bail!(format!(
            "Can't write embedded resources. Path '{path_str:?}' does not exist."
        ));
    }
    for resource in resources {
        let res_path = resource.path();
        let filename = res_path
            .file_name()
            .with_context(|| format!("Failed to resolve filename from path {res_path:?}"))
            .unwrap();
        fs::write(dst_dir.join(filename), resource.contents())
            .with_context(|| format!("Failed to write resource {res_path:?} to disk"))?;
    }
    Ok(())
}
