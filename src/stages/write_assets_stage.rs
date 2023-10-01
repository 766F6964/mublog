use anyhow::Context;

use crate::{
    blog::BlogContext, embedded_resources, page, pipeline::pipeline_stage::PipelineStage, post,
};

pub struct WriteAssetsStage;

impl PipelineStage for WriteAssetsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Process ...");
        embedded_resources::write_resources(ctx.assets.clone(), &ctx.build_assets_dir)
            .context("Failed to write assets to disk")?;
        println!("Wrote {} assets to disk", ctx.assets.len());
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Finalize ...");
        Ok(())
    }
}
