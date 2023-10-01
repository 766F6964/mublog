use crate::embedded_resources;
use crate::features::NavbarFeature;
use crate::page;
use crate::page::Page;
use crate::pipeline::Pipeline;
use crate::post;
use crate::post::Post;
use crate::stages::WrapPostsStage;
use crate::utils;
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
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct BlogContext {
    posts: Vec<Post>,
    pages: Vec<Page>,
    config_file: PathBuf,
    base_dir: PathBuf,

    build_dir: PathBuf,
    build_posts_dir: PathBuf,
    build_assets_dir: PathBuf,
    build_css_dir: PathBuf,
    build_meta_dir: PathBuf,

    posts_dir: PathBuf,
    assets_dir: PathBuf,
    css_dir: PathBuf,
    meta_dir: PathBuf,
}

impl BlogContext {
    pub fn from_path(base_path: &Path) -> Result<Self> {
        if !is_blog_directory(base_path) {
            bail!("The current directory is not a mublog environment.");
        }
        Ok(Self {
            posts: vec![],
            pages: vec![],
            config_file: base_path.join("mublog.toml"),
            base_dir: base_path.to_path_buf(),
            build_dir: base_path.join("build"),
            build_assets_dir: base_path.join("build").join("assets"),
            build_css_dir: base_path.join("build").join("css"),
            build_posts_dir: base_path.join("build").join("posts"),
            build_meta_dir: base_path.join("build").join("meta"),
            assets_dir: base_path.join("assets"),
            css_dir: base_path.join("css"),
            posts_dir: base_path.join("posts"),
            meta_dir: base_path.join("meta"),
        })
    }
}

#[derive(Clone)]
pub struct EmptyOrWhitespaceValidator {
    message: String,
}

#[derive(Clone)]
pub struct CommaListValidator {
    message: String,
}

impl Default for EmptyOrWhitespaceValidator {
    fn default() -> Self {
        Self {
            message: "Value must consist of printable characters".to_owned(),
        }
    }
}

impl Default for CommaListValidator {
    fn default() -> Self {
        Self {
            message: "Requires comma-separated, non-empty values.".to_owned(),
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

pub fn init(target_path: &Path, blog_dir_name: &str) -> anyhow::Result<()> {
    if is_blog_directory(target_path) {
        bail!("Can't initialize blog environment in existing blog environment");
    }

    let blog_dir = target_path.join(blog_dir_name);
    let config_file = blog_dir.join("mublog.toml");
    let assets_dir = blog_dir.join("assets");
    let css_dir = blog_dir.join("css");
    let meta_dir = blog_dir.join("meta");
    let posts_dir = blog_dir.join("posts");

    fs::create_dir(blog_dir).context("Failed to create blog directory")?;
    fs::create_dir(assets_dir.clone()).context("Failed to create blog/assets directory")?;
    fs::create_dir(css_dir.clone()).context("Failed to create blog/css/ directory")?;
    fs::create_dir(meta_dir.clone()).context("Failed to create blog/meta directory")?;
    fs::create_dir(posts_dir.clone()).context("Failed to create blog/posts directory")?;

    let assets_resources = embedded_resources::get_resources("assets")
        .context("Failed to extract resources from embedded directory 'assets'")?;
    embedded_resources::write_resources(assets_resources, &assets_dir)?;

    let css_resources = embedded_resources::get_resources("css")
        .context("Failed to extract resources from embedded directory 'css'")?;
    embedded_resources::write_resources(css_resources, &css_dir)?;

    let meta_resources = embedded_resources::get_resources("meta")
        .context("Failed to extract resources from embedded directory 'meta'")?;
    embedded_resources::write_resources(meta_resources, &meta_dir)
        .context("Failed to write resources to disk")?;

    let posts_resources = embedded_resources::get_resources("posts")
        .context("Failed to extract resources from embedded directory 'posts'")?;
    embedded_resources::write_resources(posts_resources, &posts_dir)?;

    let config_file_resource = embedded_resources::get_resource_file("mublog.toml")
        .context("Failed to extract config file from embedded resources.")?;
    embedded_resources::write_resource_file(config_file_resource, &config_file)?;

    Ok(())
}

pub fn build(path: &Path) -> anyhow::Result<()> {
    let mut context = BlogContext::from_path(path).context("Failed to initialize build context")?;

    let mut pipeline = Pipeline::new(context);
    pipeline.add_stage(WrapPostsStage);
    pipeline.add_feature::<NavbarFeature>();

    pipeline.run();
    // setup_build_config(&context).context("Failed to configure build environment")?;
    // prepare_build_env(&mut context).context("Failed to prepare build environment")?;
    // start_build(context).context("Failed to build blog")?;

    println!("Build process completed.");
    Ok(())
}

fn start_build(context: BlogContext) -> Result<()> {
    // Process all posts
    for post in context.posts {
        let filename =
            utils::derive_filename(&post.header.title, ".html", &context.build_posts_dir)
                .context("Failed to derive a unique filename for page.")?;

        let content_html = markdown::to_html(&post.content);
        let html_filename = filename.replace(".md", ".html");
        fs::write(
            context.build_posts_dir.join(html_filename).as_path(),
            content_html,
        )?;
        println!("Successfully built post '{}'", post.header.title);
    }
    // Process all pages
    for page in context.pages {
        let page_filename = if page.index == true {
            utils::derive_filename("index", ".html", &context.base_dir)
                .context("Failed to derive a unique filename for page.")?
        } else {
            utils::derive_filename(&page.title, ".html", &context.base_dir)
                .context("Failed to derive a unique filename for page.")?
        };
        let content_html = markdown::to_html(&page.content);
        let html_filename = page_filename.replace(".md", ".html");
        fs::write(
            context.build_dir.join(html_filename).as_path(),
            content_html,
        )?;
        println!("Successfully built page '{}'", page.title);
    }
    Ok(())
}

fn setup_build_config(_context: &BlogContext) -> Result<()> {
    // TODO: Read config from mublog.toml, and specify what plugins to enable etc
    println!("Configuring build ...");
    Ok(())
}

fn prepare_build_env(context: &mut BlogContext) -> Result<()> {
    // Delete previous build environment, if present
    if fs::metadata(context.build_dir.as_path()).is_ok() {
        fs::remove_dir_all(context.build_dir.as_path())
            .context("Failed to remove existing build environment.")?;
    }
    // Check source directories exist
    utils::is_valid_dir(&context.assets_dir)
        .context("Assets directory could not be found or is inaccessible.")?;
    utils::is_valid_dir(&context.css_dir)
        .context("CSS directory could not be found or is inaccessible.")?;
    utils::is_valid_dir(&context.posts_dir)
        .context("Posts directory could not be found or is inaccessible.")?;

    // Setup build directory and subdirectories
    fs::create_dir(context.build_dir.as_path())
        .with_context(|| format!("Failed to create directory {:?}", context.build_dir))?;
    fs::create_dir(context.build_posts_dir.as_path())
        .with_context(|| format!("Failed to create directory {:?}", context.build_posts_dir))?;
    fs::create_dir(context.build_css_dir.as_path())
        .with_context(|| format!("Failed to create directory {:?}", context.build_css_dir))?;
    fs::create_dir(context.build_assets_dir.as_path())
        .with_context(|| format!("Failed to create directory {:?}", context.build_assets_dir))?;

    context.posts = post::get_posts(&context.posts_dir);
    context.pages = page::get_pages(&context.base_dir);
    Ok(())
}

pub fn info(path: &Path) -> anyhow::Result<()> {
    let context = BlogContext::from_path(path).context("Failed to initialize info context")?;

    let title_alignment = 30;
    let date_alignment = 12;
    let draft_alignment = 12;
    let page_type_alignment = 12;

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

    let posts = post::get_posts(&context.posts_dir);
    let draft_post_count = posts.iter().filter(|post| post.header.draft).count();
    let finalized_post_count = posts.iter().filter(|post| !post.header.draft).count();

    for post in posts {
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
    println!();
    // Print header
    println!(
        "{0: <title_alignment$}  {1: >date_alignment$}  {2: >draft_alignment$}",
        "Page Title".bold(),
        "Draft".bold(),
        "Index".bold(),
        title_alignment = title_alignment,
        date_alignment = date_alignment,
        draft_alignment = draft_alignment,
    );
    // Print separator line
    println!(
        "{}",
        "—".repeat(title_alignment + page_type_alignment + draft_alignment + 4)
    );

    let pages = page::get_pages(&context.base_dir);
    let draft_page_count = pages.iter().filter(|page| page.draft).count();
    let finalized_page_count = pages.iter().filter(|page| !page.draft).count();

    for page in pages {
        println!(
            "{0: <title_alignment$}  {1: >date_alignment$}  {2: >draft_alignment$}",
            page.title.trunc_with_dots(title_alignment),
            page.draft.to_string(),
            page.index.to_string(),
            title_alignment = title_alignment,
            date_alignment = date_alignment,
            draft_alignment = draft_alignment,
        );
    }
    // Print general statistics
    println!();
    println!("{}", "Statistics:".bold());
    println!(
        "{}",
        "—".repeat(title_alignment + date_alignment + draft_alignment + 4)
    );
    println!(
        "  {} Posts ({} Finalized, {} Drafts)",
        finalized_post_count + draft_post_count,
        finalized_post_count,
        draft_post_count
    );
    println!(
        "  {} Pages ({} Finalized, {} Drafts)",
        finalized_page_count + draft_page_count,
        finalized_page_count,
        draft_page_count
    );
    Ok(())
}

pub fn create_post(path: &Path) -> anyhow::Result<()> {
    let context = BlogContext::from_path(path).context("Failed to initialize context")?;
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

    let filename = utils::derive_filename(&post.header.title, ".md", &context.posts_dir)?;
    let contents = post::parse_to_string(&post);

    fs::write(context.posts_dir.join(filename), contents)?;
    Ok(())
}

pub fn create_page(path: &Path) -> anyhow::Result<()> {
    let context = BlogContext::from_path(path).context("Failed to initialize context")?;
    let mut page = Page::default();

    page.title = Text::new("Page Title")
        .with_placeholder("Default Title")
        .with_default("Default Title")
        .with_validator(EmptyOrWhitespaceValidator::default())
        .prompt()?;
    page.draft = Confirm::new("Draft")
        .with_default(false)
        .with_placeholder("Specify if the post is a draft (y/n)")
        .with_parser(&|ans| match ans {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            _ => Err(()),
        })
        .prompt()?;
    page.index = Confirm::new("Is Index Page")
        .with_default(false)
        .with_placeholder("Specify if page is landing page (y/n)")
        .with_parser(&|ans| match ans {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            _ => Err(()),
        })
        .prompt()?;

    let filename = utils::derive_filename(&page.title, ".md", &context.base_dir)?;
    let contents = page::parse_to_string(&page);

    fs::write(context.base_dir.join(filename), contents)?;

    Ok(())
}
fn is_blog_directory(path: &Path) -> bool {
    if path.is_dir() {
        let blog_meta_file = path.join("mublog.toml");
        let posts_dir = path.join("posts");
        let meta_dir = path.join("meta");
        let css_dir = path.join("css");
        let assets_dir = path.join("assets");
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
        {
            return true;
        }
    }
    false
}
