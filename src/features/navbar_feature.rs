use anyhow::Context;

use crate::blog::BlogContext;
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::ConvertPostsStage;
use crate::utils;
pub struct NavbarFeature;

impl Feature for NavbarFeature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized,
    {
        registry.register::<ConvertPostsStage, Self>(PipelineStageLifetime::PostProcess, Self);
    }

    fn run(
        &mut self,
        ctx: &mut BlogContext,
        pipeline_type: std::any::TypeId,
        lifetime: PipelineStageLifetime,
    ) {
        println!("Executing NavFeature ...");
        if pipeline_type == std::any::TypeId::of::<ConvertPostsStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_in_post(ctx);
        }
    }
}

fn inject_navbar_in_post(ctx: &mut BlogContext) {
    println!("Navbar Feature execution ...");

    let mut nav_html = "<nav>".to_owned();
    for page in &mut ctx.pages {
        // TODO: We should store the filenames in the Page/Post struct, so we don't have to rebuild it all the time
        // TODO: This is very poorly written, we need to refactor this in the future
        // TODO: Every feature hook should return an anyhow<Result>
        let filename = match page.index {
            true => utils::derive_filename("index", ".html", &ctx.build_pages_dir)
                .context("Failed to derive a unique filename for page.")
                .unwrap(),
            false => utils::derive_filename(&page.title, ".html", &ctx.build_pages_dir)
                .context("Failed to derive a unique filename for page.")
                .unwrap(),
        };
        let title = &page.title;
        nav_html
            .push_str(format!("<a href=\"/{filename}\" title=\"{title}\">{title}</a>\n").as_str());
    }
    nav_html.push_str("</nav>");

    // Inject navbar html at the top of each converted post
    for post in &mut ctx.posts {
        post.content = format!("{nav_html}{}", post.content);
    }
}
