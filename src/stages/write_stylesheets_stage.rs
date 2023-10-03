use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use anyhow::bail;
use anyhow::Context;
use std::fs;

pub struct WriteStylesheetsStage;

impl PipelineStage for WriteStylesheetsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteStylesheetsStage: Initialize ...");
        // if ctx.stylesheets.is_empty() {
        //     bail!("No stylesheets loaded.");
        // }
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteStylesheetsStage: Process ...");
        // TODO: Temporarily disabled for now

        // for stylesheet in &mut ctx.stylesheets {
        //     fs::write(
        //         ctx.paths.build_css_dir.join(&stylesheet.name),
        //         &stylesheet.content,
        //     )
        //     .with_context(|| format!("Failed to write stylesheet '{}' to disk", stylesheet.name))?;

        //     println!(
        //         "Successfully wrote stylesheet '{}' to disk ({} bytes)",
        //         stylesheet.name,
        //         stylesheet.content.len()
        //     );
        // }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteStylesheetsStage: Finalize ...");
        Ok(())
    }
}
