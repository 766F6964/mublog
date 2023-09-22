use crate::embedded_resources;
use crate::post;
use crate::post::Post;

use crate::utils::TruncWithDots;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use chrono::Local;
use chrono::NaiveDate;
use colored::Colorize;
use inquire::formatter::DEFAULT_DATE_FORMATTER;
use inquire::validator::StringValidator;
use inquire::validator::Validation;
use inquire::Confirm;
use inquire::CustomType;
use inquire::CustomUserError;
use inquire::Text;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Default)]
struct BlogInfo {
    active_posts: u32,
    draft_posts: u32,
}

impl BlogInfo {
    fn new(active_posts: u32, draft_posts: u32) -> Self {
        Self {
            active_posts,
            draft_posts,
        }
    }
}

pub fn init(target_path: &Path, blog_dir_name: &str) -> anyhow::Result<()> {
    let blog_dir = target_path.join(blog_dir_name);
    let meta_dir = blog_dir.join("meta");
    let posts_dir = blog_dir.join("posts");
    let assets_dir = blog_dir.join("assets");
    let patches_dir = blog_dir.join("patches");
    let css_dir = blog_dir.join("css");
    let config_path = blog_dir.join("mublog.toml");

    fs::create_dir(blog_dir.as_path())
        .with_context(|| format!("Failed to create blog directory {blog_dir:?}"))?;
    fs::create_dir(meta_dir.as_path())
        .with_context(|| format!("Failed to create directory meta/ in {blog_dir:?}"))?;
    fs::create_dir(posts_dir.as_path())
        .with_context(|| format!("Failed to create directory posts/ in {blog_dir:?}"))?;
    fs::create_dir(assets_dir.as_path())
        .with_context(|| format!("Failed to create directory assets/ in {blog_dir:?}"))?;
    fs::create_dir(css_dir.as_path())
        .with_context(|| format!("Failed to create directory css/ in {blog_dir:?}"))?;
    fs::create_dir(patches_dir.as_path())
        .with_context(|| format!("Failed to create directory patches/ in {blog_dir:?}"))?;

    let meta_resources = embedded_resources::get_resources("meta")
        .context("Failed to extract resources from embedded directory 'meta'")?;
    embedded_resources::write_resources(meta_resources, meta_dir.as_path())
        .context("Failed to write resources to disk")?;

    let assets_resources = embedded_resources::get_resources("assets")
        .context("Failed to extract resources from embedded directory 'assets'")?;
    embedded_resources::write_resources(assets_resources, assets_dir.as_path())?;

    let posts_resources = embedded_resources::get_resources("posts")
        .context("Failed to extract resources from embedded directory 'posts'")?;
    embedded_resources::write_resources(posts_resources, posts_dir.as_path())?;

    let css_resources = embedded_resources::get_resources("css")
        .context("Failed to extract resources from embedded directory 'css'")?;
    embedded_resources::write_resources(css_resources, css_dir.as_path())?;

    let patches_resources = embedded_resources::get_resources("patches")
        .context("Failed to extract resources from embedded directory 'patches'")?;
    embedded_resources::write_resources(patches_resources, patches_dir.as_path())?;

    let config_file_resource = embedded_resources::get_resource_file("mublog.toml")
        .context("Failed to extract config file from embedded resources.")?;
    embedded_resources::write_resource_file(config_file_resource, config_path.as_path())?;

    Ok(())
}

pub fn info(path: &Path) -> anyhow::Result<()> {
    if !is_blog_directory(path) {
        bail!("The current directory is not a mublog environment.");
    }
    let posts_dir = path.join("posts");
    let post_dir_entries = WalkDir::new(posts_dir);

    let title_alignment = 30;
    let date_alignment = 12;
    let draft_alignment = 12;

    // Print header
    println!(
        "{0: <title_alignment$}  {1: >date_alignment$}  {2: >draft_alignment$}",
        "Post Title".bold(),
        "Date".bold(),
        "Draft".bold(),
        title_alignment = title_alignment,
        date_alignment = date_alignment,
        draft_alignment = draft_alignment,
    );
    // Print separator line
    println!(
        "{}",
        "—".repeat(title_alignment + date_alignment + draft_alignment + 4)
    );
    let mut info = BlogInfo::new(0, 0);
    for entry in post_dir_entries
        .into_iter()
        .filter_map(|f| f.ok().filter(|f| f.file_type().is_file()))
    {
        let path = entry.path();
        let filename = entry.file_name();
        let data = fs::read_to_string(path)
            .with_context(|| format!("Failed to read post {filename:?} from disk."))?;

        if let Ok(post) = post::parse_from_string(data) {
            if post.header.draft {
                info.draft_posts += 1;
            } else {
                info.active_posts += 1;
            }
            // Print row of data
            println!(
                "{0: <title_alignment$}  {1: >date_alignment$}  {2: >draft_alignment$}",
                post.header.title.trunc_with_dots(title_alignment),
                post.header.date.to_string(),
                post.header.draft.to_string(),
                title_alignment = title_alignment,
                date_alignment = date_alignment,
                draft_alignment = draft_alignment,
            );
        }
    }
    // Print general statistics
    println!();
    println!("Statistics:");
    println!(
        "  {} Posts ({} Finalized, {} Drafts)",
        info.active_posts + info.draft_posts,
        info.active_posts,
        info.draft_posts
    );
    Ok(())
}

#[derive(Clone)]
pub struct EmptyOrWhitespaceValidator {
    message: String,
}

impl Default for EmptyOrWhitespaceValidator {
    fn default() -> Self {
        Self {
            message: "Value must consist of printable characters".to_owned(),
        }
    }
}

impl StringValidator for EmptyOrWhitespaceValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        Ok(if input.trim().is_empty() {
            Validation::Invalid(self.message.as_str().into())
        } else {
            Validation::Valid
        })
    }
}

#[derive(Clone)]
pub struct CommaListValidator {
    message: String,
}

impl Default for CommaListValidator {
    fn default() -> Self {
        Self {
            message: "Requires comma-separated, non-empty values.".to_owned(),
        }
    }
}

impl StringValidator for CommaListValidator {
    fn validate(&self, input: &str) -> Result<Validation, CustomUserError> {
        let values: Vec<&str> = input.split(',').collect();
        Ok(
            if values.is_empty() || values.into_iter().any(|s| s.trim().is_empty()) {
                Validation::Invalid(self.message.as_str().into())
            } else {
                Validation::Valid
            },
        )
    }
}

pub fn create(post_dir: &Path) -> anyhow::Result<()> {
    let posts_dir = post_dir.join("posts");
    let mut post = Post::default();
    post.header.title = Text::new("Title")
        .with_placeholder("Default Title")
        .with_default("Default Title")
        .with_validator(EmptyOrWhitespaceValidator::default())
        .prompt()?;
    post.header.description = Text::new("Description")
        .with_placeholder("Default Description")
        .with_default("Default Description")
        .with_validator(EmptyOrWhitespaceValidator::default())
        .prompt()?;
    post.header.date = CustomType::<NaiveDate>::new("Publication Date")
        .with_placeholder("yyyy-mm-dd")
        .with_parser(&|i| NaiveDate::parse_from_str(i, "%Y-%m-%d").map_err(|_e| ()))
        .with_formatter(DEFAULT_DATE_FORMATTER)
        .with_error_message("Please type a valid date.")
        .with_default(Local::now().date_naive())
        .prompt()?;
    post.header.tags = Text::new("Tags")
        .with_placeholder("A comma-separated list of tags that match the posts topic")
        .with_default("creativity,writing,technology")
        .with_validator(CommaListValidator::default())
        .prompt()?
        .split(',')
        .map(std::string::ToString::to_string)
        .collect();
    post.header.draft = Confirm::new("Draft")
        .with_default(false)
        .with_placeholder("Specify if the post is a draft (y/n)")
        .with_parser(&|ans| match ans {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            _ => Err(()),
        })
        .prompt()?;

    println!("{post:#?}");

    let filename = post
        .header
        .title
        .derive_unique_filename(posts_dir.as_path())?;
    fs::write(posts_dir.join(filename), post::parse_to_string(post))?;

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
