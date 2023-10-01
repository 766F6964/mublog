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
        ctx.posts = post::get_posts(&ctx.posts_dir).context("Failed to get posts")?;
        println!("Loaded {} posts", ctx.posts.len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPostsStage: Finalize ...");
        Ok(())
    }
}
