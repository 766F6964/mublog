use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use crate::post;
use anyhow::Context;

pub struct LoadPostsStage;

impl PipelineStage for LoadPostsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPostsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPostsStage: Process ...");
        ctx.registry
            .init_posts(&ctx.paths.posts_dir)
            .context("Failed to load posts from disk")?;
        println!("Loaded {} posts", ctx.registry.get_posts().len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPostsStage: Finalize ...");
        Ok(())
    }
}
