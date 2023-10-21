use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::ConvertPagesStage;
use crate::{blog::BlogContext, post::Post};
use anyhow::Context;
use build_html::{Container, ContainerType, Html, HtmlContainer};
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
            inject_post_listing_html(ctx)
                .context("Failed to inject post listing HTML into page")?;
        }
        Ok(())
    }
}

// TODO: Add pagination support, configurable via config
fn inject_post_listing_html(ctx: &mut BlogContext) -> anyhow::Result<()> {
    println!("Injecting post listing html ...");
    let html = generate_post_listing_html(ctx);
    for page in ctx.registry.get_pages_mut() {
        page.content = page.content.replace("{{{POST_LISTING}}}", html.as_str());
    }
    Ok(())
}

fn generate_post_listing_html(ctx: &mut BlogContext) -> String {
    // let mut posts = &ctx.registry.get_posts_mut();
    // &posts.sort_by(|a, b| a.header.date.cmp(&b.header.date));
    let posts = ctx.registry.get_posts_mut();
    posts.sort_by(|a, b| b.header.date.cmp(&a.header.date));

    let mut articles =
        Container::new(ContainerType::UnorderedList).with_attributes(vec![("class", "articles")]);
    for post in posts {
        let path = Path::new("posts").join(&post.html_filename);
        let post_entry = Container::new(ContainerType::Div)
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes(vec![("class", "post_entry_date")])
                    .with_raw(format!("{}", post.header.date.to_string().as_str())),
            )
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes(vec![("class", "post_entry_link")])
                    .with_link(path.display(), &post.header.title),
            )
            .with_attributes(vec![("class", "post_entry")]);
        articles = articles.with_html(post_entry)
    }
    return articles.to_html_string();
}
