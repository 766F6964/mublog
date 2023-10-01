use anyhow::Context;

use crate::{blog::BlogContext, pipeline::pipeline_stage::PipelineStage, stylesheet};

pub struct LoadStylesheetsStage;

impl PipelineStage for LoadStylesheetsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Process ...");
        ctx.stylesheets =
            stylesheet::get_stylesheets(&ctx.css_dir).context("Failed to get stylesheets")?;
        println!("Loaded {} stylesheets", ctx.stylesheets.len());
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadStylesheetsStage: Finalize ...");
        Ok(())
    }
}
