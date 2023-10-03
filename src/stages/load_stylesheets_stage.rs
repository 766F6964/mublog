use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use crate::stylesheet;
use anyhow::Context;

pub struct LoadStylesheetsStage;

impl PipelineStage for LoadStylesheetsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Process ...");
        // ctx.stylesheets =
        // stylesheet::get_stylesheets(&ctx.paths.css_dir).context("Failed to get stylesheets")?;
        // println!("Loaded {} stylesheets", ctx.stylesheets.len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Finalize ...");
        Ok(())
    }
}
