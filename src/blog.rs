use crate::blog_registry::SiteComponentRegistry;
use crate::config;
use crate::embedded_resources;
use crate::features::NavbarFeature;
use crate::input::CommaListValidator;
use crate::input::EmptyOrWhitespaceValidator;
use crate::page;
use crate::page::Page;
use crate::path_config::PathConfig;
use crate::pipeline::Pipeline;
use crate::post;
use crate::post::Post;
use crate::stages::ConvertPagesStage;
use crate::stages::ConvertPostsStage;
use crate::stages::CreateBuildDirectoriesStage;
use crate::stages::LoadAssetsStage;
use crate::stages::LoadPagesStage;
use crate::stages::LoadPostsStage;
use crate::stages::LoadStylesheetsStage;
use crate::stages::WrapPagesStage;
use crate::stages::WrapPostsStage;
use crate::stages::WriteAssetsStage;
use crate::stages::WritePagesStage;
use crate::stages::WritePostsStage;
use crate::stages::WriteStylesheetsStage;
use crate::stylesheet::Stylesheet;
use crate::utils;
use crate::utils::TruncWithDots;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use chrono::Local;
use chrono::NaiveDate;
use colored::Colorize;
use include_dir::File;
use inquire::formatter::DEFAULT_DATE_FORMATTER;
use inquire::Confirm;
use inquire::CustomType;
use inquire::Text;
use std::fs;
use std::path::PathBuf;

struct BlogConfig {
    // TODO: This is populated after parsing the config file
    blog_author: String,
}

#[derive(Debug, Default)]
pub struct BlogContext {
    pub paths: PathConfig,
    pub registry: SiteComponentRegistry,
}

impl BlogContext {
    pub fn init(cfg: PathConfig) -> Self {
        Self {
            paths: cfg,
            registry: SiteComponentRegistry::new(),
        }
    }
}

pub fn init(working_dir: PathBuf, blog_dir_name: &str) -> anyhow::Result<()> {
    let cfg = PathConfig::new(working_dir);
    if is_blog_directory(&cfg.base_dir) {
        bail!("Can't initialize blog environment in existing blog environment");
    }

    let blog_dir = cfg.base_dir.join(blog_dir_name);
    fs::create_dir(blog_dir).context("Failed to create blog directory")?;
    fs::create_dir(cfg.assets_dir.clone()).context("Failed to create blog/assets directory")?;
    fs::create_dir(cfg.css_dir.clone()).context("Failed to create blog/css/ directory")?;
    fs::create_dir(cfg.meta_dir.clone()).context("Failed to create blog/meta directory")?;
    fs::create_dir(cfg.posts_dir.clone()).context("Failed to create blog/posts directory")?;
    fs::create_dir(cfg.pages_dir.clone()).context("Failed to create blog/pages directory")?;

    let assets_resources = embedded_resources::get_resources("assets")
        .context("Failed to extract resources from embedded directory 'assets'")?;
    embedded_resources::write_resources(assets_resources, cfg.assets_dir.as_path())?;

    let css_resources = embedded_resources::get_resources("css")
        .context("Failed to extract resources from embedded directory 'css'")?;
    embedded_resources::write_resources(css_resources, cfg.css_dir.as_path())?;

    let meta_resources = embedded_resources::get_resources("meta")
        .context("Failed to extract resources from embedded directory 'meta'")?;
    embedded_resources::write_resources(meta_resources, cfg.meta_dir.as_path())
        .context("Failed to write resources to disk")?;

    let posts_resources = embedded_resources::get_resources("posts")
        .context("Failed to extract resources from embedded directory 'posts'")?;
    embedded_resources::write_resources(posts_resources, cfg.posts_dir.as_path())?;

    let pages_resources = embedded_resources::get_resources("pages")
        .context("Failed to extract resources from embedded directory 'pages'")?;
    embedded_resources::write_resources(pages_resources, cfg.pages_dir.as_path())?;

    let config_file_resource = embedded_resources::get_resource_file("mublog.toml")
        .context("Failed to extract config file from embedded resources.")?;
    embedded_resources::write_resource_file(config_file_resource, cfg.config_file.as_path())?;

    Ok(())
}

pub fn build(working_dir: PathBuf) -> anyhow::Result<()> {
    if !is_blog_directory(&working_dir) {
        bail!("The current directory is not a mublog environment.");
    }

    let path_cfg = PathConfig::new(working_dir);
    let context = BlogContext::init(path_cfg);
    let config =
        config::parse_config(&context.paths.config_file).context("Failed to parse mublog.conf")?;
    println!("{config:#?}");

    let mut pipeline = Pipeline::new(context);
    pipeline.add_stage(CreateBuildDirectoriesStage);
    pipeline.add_stage(LoadStylesheetsStage);
    pipeline.add_stage(LoadAssetsStage);
    pipeline.add_stage(LoadPostsStage);
    pipeline.add_stage(LoadPagesStage);
    pipeline.add_stage(ConvertPostsStage);
    pipeline.add_stage(ConvertPagesStage);
    pipeline.add_stage(WrapPostsStage);
    pipeline.add_stage(WrapPagesStage);
    pipeline.add_stage(WriteStylesheetsStage);
    pipeline.add_stage(WriteAssetsStage);
    pipeline.add_stage(WritePagesStage);
    pipeline.add_stage(WritePostsStage);
    pipeline.add_feature::<NavbarFeature>();
    pipeline.run().context("Build process failed")?;

    println!("Build process completed.");
    Ok(())
}

pub fn info(working_dir: PathBuf) -> anyhow::Result<()> {
    let cfg = PathConfig::new(working_dir);

    if is_blog_directory(&cfg.base_dir) {
        bail!("Directory is not a mublog environment");
    }

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

    let posts = post::get_posts(&cfg.posts_dir)?;
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

    let pages = page::get_pages(&cfg.pages_dir)?;
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

pub fn create_post(working_dir: PathBuf) -> anyhow::Result<()> {
    let cfg = PathConfig::new(working_dir);
    let registry = SiteComponentRegistry::new();

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

    let filename = registry
        .get_post_filename(&post)
        .with_context(|| format!("Failed to obtain filename for post '{}'", post.header.title))?;
    fs::write(cfg.posts_dir.join(filename), post.content)
        .with_context(|| format!("Failed to write post '{}' to disk", post.header.title))?;
    Ok(())
}

pub fn create_page(path: PathBuf) -> anyhow::Result<()> {
    let cfg = PathConfig::new(path);
    let context = BlogContext::init(cfg);
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

    let filename = utils::derive_filename(&page.title, ".md", &context.paths.base_dir)?;
    let contents = page::parse_to_string(&page);

    fs::write(context.paths.pages_dir.join(filename), contents)?;

    Ok(())
}
fn is_blog_directory(path: &PathBuf) -> bool {
    // TODO: Maybe we can reuse the paths stored in the context
    // That way we dont have to rebuild the paths here, because
    // that is more error prone (typos)
    if path.is_dir() {
        let blog_meta_file = path.join("mublog.toml");
        let posts_dir = path.join("posts");
        let pages_dir = path.join("pages");
        let meta_dir = path.join("meta");
        let css_dir = path.join("css");
        let assets_dir = path.join("assets");
        if blog_meta_file.exists()
            && blog_meta_file.is_file()
            && posts_dir.exists()
            && posts_dir.is_dir()
            && pages_dir.exists()
            && pages_dir.is_dir()
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
    // TODO: Better error handling. Currently we just say its not a mublog
    // environment, if a single dir is missing. It might be worth mentioning
    // what dir is missing to make it a valid mublog env
    false
}
