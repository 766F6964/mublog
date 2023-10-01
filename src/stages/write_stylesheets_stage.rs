use std::fs;

use anyhow::{bail, Context};

use crate::{blog::BlogContext, pipeline::pipeline_stage::PipelineStage};

pub struct WriteStylesheetsStage;

impl PipelineStage for WriteStylesheetsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteStylesheetsStage: Initialize ...");
        if *(&ctx.stylesheets.iter().count()) == 0 {
            bail!("No stylesheets loaded.");
        }
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteStylesheetsStage: Process ...");
        for stylesheet in &mut ctx.stylesheets {
            fs::write(
                ctx.build_css_dir.join(&stylesheet.name),
                &stylesheet.content,
            )
            .with_context(|| format!("Failed to write stylesheet '{}' to disk", stylesheet.name))?;

            println!(
                "Successfully wrote stylesheet '{}' to disk ({} bytes)",
                stylesheet.name,
                stylesheet.content.len()
            );
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WriteStylesheetsStage: Finalize ...");
        Ok(())
    }
}
