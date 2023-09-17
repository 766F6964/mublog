use crate::embedded_resources;
use anyhow::{Context, Ok};
use std::fs;
use std::path::Path;

// TODO: Handle blog directory creation better, error non-empty/exist already
pub fn init(target_path: &Path, blog_dir_name: &str) -> anyhow::Result<()> {
    let blog_dir = target_path.join(blog_dir_name);
    let meta_dir = blog_dir.join("meta");
    let posts_dir = blog_dir.join("posts");
    let assets_dir = blog_dir.join("assets");
    let patches_dir = blog_dir.join("patches");
    let css_dir = blog_dir.join("css");

    fs::create_dir(blog_dir.as_path())
        .with_context(|| format!("Failed to create blog directory: {blog_dir:?}"))?;
    fs::create_dir(meta_dir.as_path())
        .with_context(|| format!("Failed to create directory meta/ in directory {blog_dir:?}"))?;
    fs::create_dir(posts_dir.as_path())
        .with_context(|| format!("Failed to create directory posts/ in directory {blog_dir:?}"))?;
    fs::create_dir(assets_dir.as_path())
        .with_context(|| format!("Failed to create directory assets/ in directory {blog_dir:?}"))?;
    fs::create_dir(css_dir.as_path())
        .with_context(|| format!("Failed to create directory css/ in directory {blog_dir:?}"))?;
    fs::create_dir(patches_dir.as_path()).with_context(|| {
        format!("Failed to create directory patches/ in directory {blog_dir:?}")
    })?;

    let meta_resources = embedded_resources::get_resources("meta")
        .context("Failed to extract resources from embedded directory 'meta'")?;
    embedded_resources::write_resources(meta_resources, &meta_dir.as_path())?;

    let assets_resources = embedded_resources::get_resources("assets")
        .context("Failed to extract resources from embedded directory 'assets'")?;
    embedded_resources::write_resources(assets_resources, &assets_dir.as_path())?;

    let posts_resources = embedded_resources::get_resources("posts")
        .context("Failed to extract resources from embedded directory 'posts'")?;
    embedded_resources::write_resources(posts_resources, &posts_dir.as_path())?;

    let css_resources = embedded_resources::get_resources("css")
        .context("Failed to extract resources from embedded directory 'css'")?;
    embedded_resources::write_resources(css_resources, &css_dir.as_path())?;

    let patches_resources = embedded_resources::get_resources("patches")
        .context("Failed to extract resources from embedded directory 'patches'")?;
    embedded_resources::write_resources(patches_resources, &patches_dir.as_path())?;

    Ok(())
}
pub fn info(path: &Path) -> anyhow::Result<()> {
    println!("Obtaining blog site info ...");
    println!("Current path: {}", path.display());
    if !is_blog_directory(path) {
        return Err(anyhow::anyhow!(
            "The current directory is not a mublog environment."
        ))?;
    }
    Ok(())
}

// .mublog.toml
// Contents:
// - Configuration options, e.g.
//   - Enabled plugins
//   - Blog settings, e.g. author name, copyright year etc

pub fn is_blog_directory(path: &Path) -> bool {
    if path.is_dir() {
        let blog_meta_file = path.join("mublog.toml");
        let posts_dir = path.join("posts");
        let meta_dir = path.join("meta");
        let css_dir = path.join("css");
        let assets_dir = path.join("assets");
        let patches_dir = path.join("patches");
        if blog_meta_file.exists()
            && blog_meta_file.is_file()
            && posts_dir.exists()
            && posts_dir.is_dir()
            && meta_dir.exists()
            && meta_dir.is_dir()
            && css_dir.exists()
            && css_dir.is_dir()
            && assets_dir.exists()
            && assets_dir.is_dir()
            && patches_dir.exists()
            && patches_dir.is_dir()
        {
            return true;
        }
    }
    false
}
