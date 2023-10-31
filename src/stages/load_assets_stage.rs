use crate::blog::BlogContext;
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

        ctx.registry
            .init_assets(&ctx.paths.assets_dir)
            .context("Failed to load assets from disk")?;
        for asset in ctx.registry.get_assets() {
            println!("Asset: {}", asset.filename);
        }

        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("LoadAssetsStage: Finalize ...");
        Ok(())
    }
}
