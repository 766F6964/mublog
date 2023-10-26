use crate::blog_registry::SiteComponentRegistry;
use crate::config;
use crate::config::BlogConfig;
use crate::embedded_resources;
use crate::features::NavbarFeature;
use crate::features::PostListingFeature;
use crate::input::CommaListValidator;
use crate::input::EmptyOrWhitespaceValidator;
use crate::page;
use crate::page::Page;
use crate::path_config::PathConfig;
use crate::pipeline::Pipeline;
use crate::post;
use crate::post::Post;
use crate::stages::ApplyGlobalVarsStage;
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
use crate::utils::TruncWithDots;
use anyhow::bail;
use anyhow::Context;
use chrono::Local;
use chrono::NaiveDate;
use colored::Colorize;
use inquire::formatter::DEFAULT_DATE_FORMATTER;
use inquire::Confirm;
use inquire::CustomType;
use inquire::Text;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct BlogContext {
    pub config: BlogConfig,
    pub paths: PathConfig,
    pub registry: SiteComponentRegistry,
}

impl BlogContext {
    pub fn init(paths: PathConfig, config: BlogConfig) -> Self {
        // TODO: Maybe we need something like a service provider, because otherwise
        // we create unnecessary deps. we could have a serivce provider that
        // creates/returns singletons
        Self {
            config,
            paths,
            registry: SiteComponentRegistry::init(),
        }
    }
}

pub fn init(working_dir: PathBuf, blog_dir_name: &str) -> anyhow::Result<()> {
    if is_blog_directory(&working_dir) {
        bail!("Can't initialize blog environment in existing blog environment");
    }
    let blog_dir = working_dir.join(blog_dir_name);
    fs::create_dir(&blog_dir).context("Failed to create blog directory")?;

    let cfg = PathConfig::new(blog_dir);
    fs::create_dir(cfg.assets_dir.clone()).context("Failed to create blog/assets directory")?;
    fs::create_dir(cfg.css_dir.clone()).context("Failed to create blog/css/ directory")?;
    fs::create_dir(cfg.meta_dir.clone()).context("Failed to create blog/meta directory")?;
    fs::create_dir(cfg.posts_dir.clone()).context("Failed to create blog/posts directory")?;
    fs::create_dir(cfg.pages_dir.clone()).context("Failed to create blog/pages directory")?;

    let assets_resources = embedded_resources::get_resources("assets")
        .context("Failed to extract resources from embedded directory 'assets'")?;
    embedded_resources::write_resources(assets_resources, cfg.assets_dir.as_path())
        .context("Failed to write assets-resources to disk")?;

    let css_resources = embedded_resources::get_resources("css")
        .context("Failed to extract resources from embedded directory 'css'")?;
    embedded_resources::write_resources(css_resources, cfg.css_dir.as_path())
        .context("Failed to write css-resources to disk")?;

    let meta_resources = embedded_resources::get_resources("meta")
        .context("Failed to extract resources from embedded directory 'meta'")?;
    embedded_resources::write_resources(meta_resources, cfg.meta_dir.as_path())
        .context("Failed to write meta-resources to disk")?;

    let posts_resources = embedded_resources::get_resources("posts")
        .context("Failed to extract resources from embedded directory 'posts'")?;
    embedded_resources::write_resources(posts_resources, cfg.posts_dir.as_path())
        .context("Failed to write posts-resources to disk")?;

    let pages_resources = embedded_resources::get_resources("pages")
        .context("Failed to extract resources from embedded directory 'pages'")?;
    embedded_resources::write_resources(pages_resources, cfg.pages_dir.as_path())
        .context("Failed to write pages-resources to disk")?;

    let config_file_resource = embedded_resources::get_resource_file("mublog.toml")
        .context("Failed to extract mublog config file from embedded resources")?;
    embedded_resources::write_resource_file(config_file_resource, cfg.config_file.as_path())
        .context("Failed to write mublog.toml resource to disk")?;

    Ok(())
}

pub fn build(working_dir: PathBuf) -> anyhow::Result<()> {
    if !is_blog_directory(&working_dir) {
        bail!("The current directory is not a mublog environment");
    }

    let path_cfg = PathConfig::new(working_dir);
    let config = config::parse_config(&path_cfg.config_file)
        .context("Failed to parse mublog.toml config file")?;

    let context = BlogContext::init(path_cfg, config);

    let mut pipeline = Pipeline::new(context);
    pipeline.add_stage(CreateBuildDirectoriesStage);
    pipeline.add_stage(LoadStylesheetsStage);
    pipeline.add_stage(LoadAssetsStage);
    pipeline.add_stage(LoadPostsStage);
    pipeline.add_stage(LoadPagesStage);
    pipeline.add_stage(ApplyGlobalVarsStage);
    pipeline.add_stage(ConvertPostsStage);
    pipeline.add_stage(ConvertPagesStage);
    pipeline.add_stage(WrapPostsStage);
    pipeline.add_stage(WrapPagesStage);
    pipeline.add_stage(WriteStylesheetsStage);
    pipeline.add_stage(WriteAssetsStage);
    pipeline.add_stage(WritePagesStage);
    pipeline.add_stage(WritePostsStage);

    // TODO: Enable features based on mublog.toml config file
    pipeline.add_feature::<PostListingFeature>();
    pipeline.add_feature::<NavbarFeature>();
    pipeline.run().context("Build process failed")?;

    println!("Build process completed");
    Ok(())
}

pub fn info(working_dir: PathBuf) -> anyhow::Result<()> {
    if !is_blog_directory(&working_dir) {
        bail!("The current directory is not a mublog environment");
    }
    let cfg = PathConfig::new(working_dir);
    let mut registry = SiteComponentRegistry::init();
    registry
        .init_posts(&cfg.posts_dir)
        .context("Failed to load posts from disk")?;
    registry
        .init_pages(&cfg.pages_dir)
        .context("Failed to load pages from disk")?;

    let title_col = 30;
    let date_col = 12;
    let draft_col = 12;
    let page_type_col = 12;

    let posts = registry.get_posts();
    let draft_post_count = posts.iter().filter(|post| post.header.draft).count();
    let finalized_post_count = posts.iter().filter(|post| !post.header.draft).count();
    let pages = registry.get_pages();
    let draft_page_count = pages.iter().filter(|page| page.draft).count();
    let finalized_page_count = pages.iter().filter(|page| !page.draft).count();

    info_posts(title_col, date_col, draft_col, posts);
    info_pages(title_col, date_col, page_type_col, draft_col, pages);
    info_general(
        title_col,
        date_col,
        draft_col,
        finalized_post_count,
        draft_post_count,
        finalized_page_count,
        draft_page_count,
    );
    Ok(())
}

fn info_pages(
    title_col: usize,
    date_col: usize,
    page_type_col: usize,
    draft_col: usize,
    pages: &Vec<Page>,
) {
    println!(
        "{0: <title_col$}  {1: >date_col$}  {2: >draft_col$}",
        "Page Title".bold(),
        "Draft".bold(),
        "Index".bold(),
        title_col = title_col,
        date_col = date_col,
        draft_col = draft_col,
    );
    info_separator_line(title_col + page_type_col + draft_col + 4);

    for page in pages {
        println!(
            "{0: <title_col$}  {1: >date_col$}  {2: >draft_col$}",
            page.title.trunc_with_dots(title_col),
            page.draft.to_string(),
            page.index.to_string(),
            title_col = title_col,
            date_col = date_col,
            draft_col = draft_col,
        );
    }
}

fn info_posts(title_col: usize, date_col: usize, draft_col: usize, posts: &Vec<Post>) {
    println!(
        "{0: <title_col$}  {1: >date_col$}  {2: >draft_col$}",
        "Post Title".bold(),
        "Date".bold(),
        "Draft".bold(),
        title_col = title_col,
        date_col = date_col,
        draft_col = draft_col,
    );

    info_separator_line(title_col + date_col + draft_col + 4);

    for post in posts {
        println!(
            "{0: <title_col$}  {1: >date_col$}  {2: >draft_col$}",
            post.header.title.trunc_with_dots(title_col),
            post.header.date.to_string(),
            post.header.draft.to_string(),
            title_col = title_col,
            date_col = date_col,
            draft_col = draft_col,
        );
    }
    println!();
}

fn info_separator_line(line_len: usize) {
    println!("{}", "—".repeat(line_len));
}

fn info_general(
    title_col: usize,
    date_col: usize,
    draft_col: usize,
    finalized_post_count: usize,
    draft_post_count: usize,
    finalized_page_count: usize,
    draft_page_count: usize,
) {
    println!();
    println!("{}", "Statistics:".bold());
    println!("{}", "—".repeat(title_col + date_col + draft_col + 4));
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
}

pub fn create_post(working_dir: PathBuf) -> anyhow::Result<()> {
    if !is_blog_directory(&working_dir) {
        bail!("The current directory is not a mublog environment");
    }
    let cfg = PathConfig::new(working_dir);
    let mut registry = SiteComponentRegistry::init();
    registry
        .init_posts(&cfg.posts_dir)
        .context("Failed to load posts from disk")?;

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
    post.content = "Your content goes here".into();

    registry
        .register_post(post)
        .context("Failed to register new post")?;
    let new_post = registry
        .get_posts()
        .last()
        .context("Failed to load newly added post")?;
    println!("post Filename: {}", &new_post.html_filename);

    let contents = post::parse_to_string(new_post);
    fs::write(cfg.posts_dir.join(&new_post.md_filename), contents)
        .context("Failed to write newly created post to disk.")?;
    Ok(())
}

// TODO: Ensure we can't create more than one index page
// TODO: Ensure that markdown filename is also index.md, if it is
// an index page
pub fn create_page(working_dir: PathBuf) -> anyhow::Result<()> {
    if !is_blog_directory(&working_dir) {
        bail!("The current directory is not a mublog environment.");
    }
    let cfg = PathConfig::new(working_dir);
    let mut registry = SiteComponentRegistry::init();
    registry
        .init_pages(&cfg.pages_dir)
        .context("Failed to load pages from disk")?;

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
    page.content = "Your content goes here".into();

    registry
        .register_page(page)
        .context("Failed to register new page")?;
    let new_page = registry
        .get_pages()
        .last()
        .context("Failed to load newly added page")?;

    let contents = page::parse_to_string(new_page);
    fs::write(cfg.pages_dir.join(&new_page.md_filename), contents)
        .context("Failed to write newly created page to disk.")?;

    Ok(())
}

fn is_blog_directory(working_dir: &PathBuf) -> bool {
    // TODO: Maybe we can reuse the paths stored in the context
    // That way we dont have to rebuild the paths here, because
    // that is more error prone (typos)
    if working_dir.is_dir() {
        let blog_meta_file = working_dir.join("mublog.toml");
        let posts_dir = working_dir.join("posts");
        let pages_dir = working_dir.join("pages");
        let meta_dir = working_dir.join("meta");
        let css_dir = working_dir.join("css");
        let assets_dir = working_dir.join("assets");
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
