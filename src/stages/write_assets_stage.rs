use crate::blog::BlogContext;
use crate::embedded_resources;
use crate::pipeline::pipeline_stage::PipelineStage;
use anyhow::Context;

pub struct WriteAssetsStage;

impl PipelineStage for WriteAssetsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Process ...");
        // TODO: Temporarily disabled too
        // embedded_resources::write_resources(ctx.assets.clone(), &ctx.paths.build_assets_dir)
        // .context("Failed to write assets to disk")?;
        // println!("Wrote {} assets to disk", ctx.assets.len());
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Finalize ...");
        Ok(())
    }
}
