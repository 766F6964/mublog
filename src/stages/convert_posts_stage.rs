use std::fs;

use anyhow::Context;

use crate::{blog::BlogContext, page, pipeline::pipeline_stage::PipelineStage, post, utils};

pub struct ConvertPostsStage;

impl PipelineStage for ConvertPostsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Initialize ...");
        ctx.posts = post::get_posts(&ctx.posts_dir);
        Ok(())
    }

    fn process(&self, context: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Process ...");
        // Process all posts
        for post in &context.posts {
            let filename =
                utils::derive_filename(&post.header.title, ".html", &context.build_posts_dir)
                    .context("Failed to derive a unique filename for page.")?;

            let content_html = markdown::to_html(&post.content);
            let html_filename = filename.replace(".md", ".html");
            fs::write(
                context.build_posts_dir.join(html_filename).as_path(),
                content_html,
            )?;
        }
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Finalize ...");
        Ok(())
    }
}
