use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use crate::utils;
use anyhow::Context;
use std::fs;

pub struct WritePostsStage;

impl PipelineStage for WritePostsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePostsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePostsStage: Process ...");
        for post in &mut ctx.posts {
            let filename =
                utils::derive_filename(&post.header.title, ".html", &ctx.build_posts_dir)
                    .context("Failed to derive a unique filename for page.")?;

            fs::write(
                ctx.build_posts_dir.join(filename).as_path(),
                post.content.clone(),
            )
            .with_context(|| format!("Failed to write post '{}' to disk", post.header.title))?;
            println!(
                "Successfully wrote post '{}' to disk ({} bytes)",
                post.header.title,
                post.content.len()
            );
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePostsStage: Finalize ...");
        Ok(())
    }
}
