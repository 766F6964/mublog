use anyhow::Context;

use crate::{
    blog::BlogContext, embedded_resources, page, pipeline::pipeline_stage::PipelineStage, post,
};

pub struct LoadAssetsStage;

impl PipelineStage for LoadAssetsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadAssetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadAssetsStage: Process ...");

        let assets_resources = embedded_resources::get_resources("assets")
            .context("Failed to extract resources from embedded directory 'assets'")?;
        ctx.assets = assets_resources;
        println!("Loaded {} assets", ctx.assets.len());
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadAssetsStage: Finalize ...");
        Ok(())
    }
}
