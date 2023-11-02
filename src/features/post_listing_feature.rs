use crate::features::FeatureConfig;
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::post::Post;
use crate::stages::ConvertPagesStage;
use crate::{blog::BlogContext, stages::ConvertPostsStage};
use anyhow::{bail, Context, Ok};
use build_html::{Container, ContainerType, Html, HtmlContainer};
use chrono::format;
use serde::Deserialize;
use std::collections::HashMap;
use std::{any::TypeId, path::Path};
pub struct PostListingFeature;

impl Feature for PostListingFeature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized,
    {
        registry.register::<ConvertPostsStage, Self>(PipelineStageLifetime::PostProcess, Self);
        registry.register::<ConvertPagesStage, Self>(PipelineStageLifetime::PostProcess, Self);
    }

    // TODO: Have a generic get_config fn in the Feature trait

    fn run(
        &mut self,
        ctx: &mut BlogContext,
        pipeline_type: TypeId,
        lifetime: PipelineStageLifetime,
    ) -> anyhow::Result<()> {
        println!("Executing NavFeature ...");
        if pipeline_type == TypeId::of::<ConvertPagesStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            let features: Vec<FeatureConfig> = ctx.config.features.clone();
            if let Some(post_cfg) = features.iter().find_map(|config| {
                if let FeatureConfig::Postlisting(post_config) = config {
                    Some(post_config)
                } else {
                    None
                }
            }) {
                // TODO: Add pagination support, configurable via config
                // let post_listing_page = try_get_post_listing_page(ctx, post_cfg)?;
                insert_tags_in_posts(ctx, post_cfg)?;
                insert_tags_listing_in_page(ctx, post_cfg)?;
                insert_post_listing_in_page(ctx, post_cfg)?;
            } else {
                bail!("PostListingConfig not found in the vector");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum PostSortingOrder {
    OldestOnTop,
    NewestOnTop,
}

#[derive(Debug, Deserialize, Clone)]
pub enum TagSortingOrder {
    Count,
    Alphabetic,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostlistingConfig {
    pub tags_enabled: bool,
    pub tag_listing_page: String,
    pub tag_listing_order: TagSortingOrder,
    pub post_listing_page: String,
    pub post_listing_order: PostSortingOrder,
}

fn insert_tags_in_posts(ctx: &mut BlogContext, cfg: &PostlistingConfig) -> anyhow::Result<()> {
    let registry = &mut ctx.registry;
    for post in registry.get_posts_mut() {
        let tag_html = generate_post_tags_html(&post, &cfg);
        post.content.push_str(&tag_html);
    }
    Ok(())
}

fn insert_tags_listing_in_page(
    ctx: &mut BlogContext,
    cfg: &PostlistingConfig,
) -> anyhow::Result<()> {
    // TODO: How do we verify that the content contains the replacement pattern? Maybe with .contains() ?
    let mut inserted_tag_listing = false;
    let html = generate_tag_listing_html(ctx, cfg);
    for page in ctx.registry.get_pages_mut() {
        if page.md_filename == cfg.tag_listing_page {
            // TODO: Insert tag listing html, error if page is missing
            page.content = page.content.replace("{{{TAG_LISTING}}}", html.as_str());
            inserted_tag_listing = true;
        }
    }
    if !inserted_tag_listing {
        bail!(format!(
            "No marker found in {} to insert tag listing",
            cfg.tag_listing_page
        ));
    }
    Ok(())
}

fn insert_post_listing_in_page(
    ctx: &mut BlogContext,
    cfg: &PostlistingConfig,
) -> anyhow::Result<()> {
    let html = generate_post_listing_html(ctx, &cfg.post_listing_order);
    let mut inserted_post_listing = false;
    for page in ctx.registry.get_pages_mut() {
        if page.md_filename == cfg.post_listing_page {
            page.content = page.content.replace("{{{POST_LISTING}}}", html.as_str());
            inserted_post_listing = true;
        }
    }
    if !inserted_post_listing {
        bail!(format!(
            "No marker found in {} to insert post listing",
            cfg.post_listing_page
        ));
    }
    Ok(())
}

fn generate_post_listing_html(ctx: &mut BlogContext, sort: &PostSortingOrder) -> String {
    let posts = ctx.registry.get_posts_mut();

    match sort {
        PostSortingOrder::OldestOnTop => {
            posts.sort_by(|a, b| a.date.cmp(&b.date));
        }
        PostSortingOrder::NewestOnTop => {
            posts.sort_by(|a, b| b.date.cmp(&a.date));
        }
    }

    let mut articles =
        Container::new(ContainerType::UnorderedList).with_attributes(vec![("class", "articles")]);
    for post in posts {
        let path = Path::new("posts").join(&post.html_filename);
        let post_entry = Container::new(ContainerType::Div)
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes(vec![("class", "post_entry_date")])
                    .with_raw(post.date.to_string().as_str().to_string()),
            )
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes(vec![("class", "post_entry_link")])
                    .with_link(path.display(), &post.title),
            )
            .with_attributes(vec![("class", "post_entry")]);
        articles = articles.with_html(post_entry)
    }
    articles.to_html_string()
}

fn generate_tag_listing_html(ctx: &mut BlogContext, cfg: &PostlistingConfig) -> String {
    let posts = ctx.registry.get_posts();
    let mut tag_counts: HashMap<String, usize> = HashMap::new();
    for post in posts {
        for tag in &post.tags {
            let count = tag_counts.entry(tag.clone()).or_insert(0);
            *count += 1;
        }
    }
    // TODO: Seems like there is an issue where tag names are not trimmed
    let tags: Vec<(String, usize)> = tag_counts.into_iter().collect();
    // TODO: Replace the following with proper HTML construction.

    let mut post_tags =
        Container::new(build_html::ContainerType::Div).with_attributes(vec![("class", "tags")]);
    for tag in tags {
        // TODO: we need the html filename here
        let query_url = Path::new("posts").join(&cfg.post_listing_page);
        let post_tag = Container::new(ContainerType::Div)
            .with_attributes(vec![
                ("class", "tag"),
                ("onclick", query_url.display().to_string().as_str()),
            ])
            .with_container(Container::new(ContainerType::Div))
            .with_attributes(vec![("class", "tag-text")])
            .with_link(format!("http://localhost:8000/posts.html"), tag.0.as_str());
        post_tags = post_tags.with_html(post_tag)
    }
    post_tags.to_html_string()
}

fn generate_post_tags_html(post: &Post, cfg: &PostlistingConfig) -> String {
    // TODO: Validate that the post_listing_page from the config actually exists
    let mut post_tags =
        Container::new(build_html::ContainerType::Div).with_attributes(vec![("class", "tags")]);
    for tag in &post.tags {
        let query_url = Path::new("posts").join(&cfg.post_listing_page);
        let post_tag = Container::new(ContainerType::Div)
            .with_attributes(vec![
                ("class", "tag"),
                ("onclick", query_url.display().to_string().as_str()),
            ])
            .with_container(Container::new(ContainerType::Div))
            .with_attributes(vec![("class", "tag-text")])
            .with_link("http://localhost:8000", tag.as_str());
        post_tags = post_tags.with_html(post_tag)
    }
    post_tags.to_html_string()
}

// fn generate_tags_listing_html() {}
