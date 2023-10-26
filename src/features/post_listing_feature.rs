use crate::features::FeatureConfig;
// use crate::config::{FeatureConfig, SortingOrder};
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::ConvertPagesStage;
use crate::{blog::BlogContext};
use anyhow::bail;
// use anyhow::{bail, Context};
use build_html::{Container, ContainerType, Html, HtmlContainer};
use serde::Deserialize;
use std::{any::TypeId, path::Path};
pub struct PostListingFeature;

impl Feature for PostListingFeature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized,
    {
        registry.register::<ConvertPagesStage, Self>(PipelineStageLifetime::PostProcess, Self);
    }

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

            // Extract and work with post_cfg
            if let Some(post_cfg) = features.iter().find_map(|config| {
                if let FeatureConfig::Postlisting(post_config) = config {
                    Some(post_config.clone()) // Clone post_config to avoid borrow conflicts
                } else {
                    None
                }
            }) {
                // Mutable borrow of ctx here is fine because post_cfg is a cloned copy
                let ctx = &mut *ctx;
                println!("SORT ORDER: {:#?}", post_cfg.sort);
                inject_post_listing_html(ctx, &post_cfg)?;
            } else {
                // Handle case when PostListingConfig is not found in features
                bail!("PostListingConfig not found in the vector");
            }
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub enum SortingOrder {
    OldestOnTop,
    NewestOnTop,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PostlistingConfig {
    pub sort: SortingOrder,
}

// TODO: Add pagination support, configurable via config
fn inject_post_listing_html(ctx: &mut BlogContext, cfg: &PostlistingConfig) -> anyhow::Result<()> {
    let html = generate_post_listing_html(ctx, &cfg.sort);
    for page in ctx.registry.get_pages_mut() {
        page.content = page.content.replace("{{{POST_LISTING}}}", html.as_str());
    }
    Ok(())
}

fn generate_post_listing_html(ctx: &mut BlogContext, sort: &SortingOrder) -> String {
    let posts = ctx.registry.get_posts_mut();

    match sort {
        SortingOrder::OldestOnTop => {
            posts.sort_by(|a, b| a.header.date.cmp(&b.header.date));
        }
        SortingOrder::NewestOnTop => {
            posts.sort_by(|a, b| b.header.date.cmp(&a.header.date));
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
                    .with_raw(post.header.date.to_string().as_str().to_string()),
            )
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes(vec![("class", "post_entry_link")])
                    .with_link(path.display(), &post.header.title),
            )
            .with_attributes(vec![("class", "post_entry")]);
        articles = articles.with_html(post_entry)
    }
    articles.to_html_string()
}
