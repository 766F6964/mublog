use crate::blog::BlogContext;
use crate::page;
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
        ctx.pages = page::get_pages(&ctx.pages_dir).context("Failed to get pages")?;
        println!("Loaded {} pages", ctx.pages.len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPagesStage: Finalize ...");
        Ok(())
    }
}