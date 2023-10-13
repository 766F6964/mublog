use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use anyhow::Context;

pub struct LoadStylesheetsStage;

impl PipelineStage for LoadStylesheetsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Process ...");
        ctx.registry
            .init_stylesheets(&ctx.paths.css_dir)
            .context("Failed to load stylesheets from disk")?;
        for stylesheet in ctx.registry.get_stylesheets() {
            println!("Stylesheet: {}", stylesheet.filename);
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Finalize ...");
        Ok(())
    }
}
