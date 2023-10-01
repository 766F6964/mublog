use anyhow::Context;

use crate::{blog::BlogContext, page, pipeline::pipeline_stage::PipelineStage, post};

pub struct LoadPostsStage;

impl PipelineStage for LoadPostsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPostsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPostsStage: Process ...");
        ctx.posts = post::get_posts(&ctx.posts_dir).context("Failed to get posts")?;
        println!("Loaded {} posts", ctx.posts.len());
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPostsStage: Finalize ...");
        Ok(())
    }
}
