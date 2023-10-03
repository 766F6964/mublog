use crate::blog::BlogContext;
use crate::embedded_resources;
use crate::pipeline::pipeline_stage::PipelineStage;
use anyhow::Context;

pub struct LoadAssetsStage;

impl PipelineStage for LoadAssetsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadAssetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadAssetsStage: Process ...");
        // TODO: Temporarily disabled this stage

        // let assets_resources = embedded_resources::get_resources("assets")
        // .context("Failed to extract resources from embedded directory 'assets'")?;
        // ctx.assets = assets_resources;
        // println!("Loaded {} assets", ctx.assets.len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadAssetsStage: Finalize ...");
        Ok(())
    }
}
