use crate::blog::BlogContext;

use crate::pipeline::pipeline_stage::PipelineStage;
use anyhow::Context;

pub struct LoadPagesStage;

impl PipelineStage for LoadPagesStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPagesStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPagesStage: Process ...");
        ctx.registry
            .init_pages(&ctx.paths.pages_dir)
            .context("Failed to load pages from disk")?;
        println!("Loaded {} pages", ctx.registry.get_pages().len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPagesStage: Finalize ...");
        Ok(())
    }
}
