use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use anyhow::Context;
use std::fs;

pub struct WriteAssetsStage;

impl PipelineStage for WriteAssetsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Process ...");

        for asset in ctx.registry.get_assets() {
            fs::write(
                ctx.paths.build_assets_dir.join(&asset.filename),
                &asset.content,
            )
            .with_context(|| format!("Failed to write asset '{}' to disk", asset.filename))?;

            println!(
                "Successfully wrote asset '{}' to disk ({} bytes)",
                asset.filename,
                asset.content.len()
            );
        }

        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteAssetsStage: Finalize ...");
        Ok(())
    }
}
