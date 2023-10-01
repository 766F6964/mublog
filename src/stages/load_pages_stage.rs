use crate::{blog::BlogContext, page, pipeline::pipeline_stage::PipelineStage};

pub struct LoadPagesStage;

impl PipelineStage for LoadPagesStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPagesStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPagesStage: Process ...");
        ctx.pages = page::get_pages(&ctx.base_dir);
        println!("Loaded {} pages", ctx.pages.len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadPagesStage: Finalize ...");
        Ok(())
    }
}
