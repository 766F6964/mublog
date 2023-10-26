use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
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
        for post in ctx.registry.get_posts() {
            fs::write(
                ctx.paths
                    .build_posts_dir
                    .join(post.html_filename.as_str())
                    .as_path(),
                post.content.clone(),
            )
            .with_context(|| format!("Failed to write post '{}' to disk", post.title))?;
            println!(
                "Successfully wrote post '{}' to disk ({} bytes)",
                post.title,
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
