use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use crate::utils;
use anyhow::bail;
use anyhow::Context;
use std::fs;

pub struct WritePagesStage;

impl PipelineStage for WritePagesStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePagesStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePagesStage: Process ...");
        for page in ctx.registry.get_pages() {
            fs::write(
                ctx.paths
                    .build_pages_dir
                    .join(page.html_filename.as_str())
                    .as_path(),
                page.content.clone(),
            )
            .with_context(|| format!("Failed to write page '{}' to disk", page.title))?;

            println!(
                "Successfully wrote page '{}' to disk ({} bytes)",
                page.title,
                page.content.len()
            );
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePagesStage: Finalize ...");
        Ok(())
    }
}
