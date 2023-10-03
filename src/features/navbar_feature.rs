use std::any::{self, TypeId};

use anyhow::Context;

use crate::blog::BlogContext;
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::{ConvertPagesStage, ConvertPostsStage};
use crate::utils;
pub struct NavbarFeature;

impl Feature for NavbarFeature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized,
    {
        registry.register::<ConvertPostsStage, Self>(PipelineStageLifetime::PostProcess, Self);
        registry.register::<ConvertPagesStage, Self>(PipelineStageLifetime::PostProcess, Self);
    }

    fn run(
        &mut self,
        ctx: &mut BlogContext,
        pipeline_type: TypeId,
        lifetime: PipelineStageLifetime,
    ) {
        println!("Executing NavFeature ...");
        if pipeline_type == TypeId::of::<ConvertPostsStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_in_post(ctx);
        } else if pipeline_type == TypeId::of::<ConvertPagesStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_in_page(ctx);
        }
    }
}

fn inject_navbar_in_post(ctx: &mut BlogContext) {
    println!("Navbar Feature execution ...");
    let mut nav_html = "<nav>".to_owned();
    for page in ctx.registry.get_pages() {
        // TODO: We should store the filenames in the Page/Post struct, so we don't have to rebuild it all the time
        // TODO: This is very poorly written, we need to refactor this in the future
        // TODO: Every feature hook should return an anyhow<Result>
        let filename = match page.index {
            true => utils::derive_filename("index", ".html", &ctx.paths.build_pages_dir)
                .context("Failed to derive a unique filename for page.")
                .unwrap(),
            false => utils::derive_filename(&page.title, ".html", &ctx.paths.build_pages_dir)
                .context("Failed to derive a unique filename for page.")
                .unwrap(),
        };
        let title = &page.title;
        nav_html
            .push_str(format!("<a href=\"/{filename}\" title=\"{title}\">{title}</a>\n").as_str());
    }
    nav_html.push_str("</nav>");

    // Inject navbar html at the top of each converted post
    for post in ctx.registry.get_posts_mut() {
        post.content = format!("{nav_html}{}", post.content);
    }
}
fn inject_navbar_in_page(ctx: &mut BlogContext) {
    println!("Navbar Feature execution ...");

    let mut nav_html = "<nav>".to_owned();
    for page in ctx.registry.get_pages() {
        // TODO: We should store the filenames in the Page/Post struct, so we don't have to rebuild it all the time
        // TODO: This is very poorly written, we need to refactor this in the future
        // TODO: Every feature hook should return an anyhow<Result>
        let filename = match page.index {
            true => utils::derive_filename("index", ".html", &ctx.paths.build_pages_dir)
                .context("Failed to derive a unique filename for page.")
                .unwrap(),
            false => utils::derive_filename(&page.title, ".html", &ctx.paths.build_pages_dir)
                .context("Failed to derive a unique filename for page.")
                .unwrap(),
        };
        let title = &page.title;
        nav_html
            .push_str(format!("<a href=\"/{filename}\" title=\"{title}\">{title}</a>\n").as_str());
    }
    nav_html.push_str(
        "</nav>
        <hr>",
    );

    // Inject navbar html at the top of each converted post
    for page in ctx.registry.get_pages_mut() {
        page.content = format!("{nav_html}{}", page.content);
    }
}
