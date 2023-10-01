use std::fs;

use anyhow::{bail, Context};

use crate::{blog::BlogContext, pipeline::pipeline_stage::PipelineStage, utils};

pub struct WritePagesStage;

impl PipelineStage for WritePagesStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePagesStage: Initialize ...");
        // Check that there is only one index page

        let index_page_cnt = &ctx.pages.iter().filter(|&p| p.index).count();
        if *index_page_cnt == 0 {
            println!("WARN: You don't have a landing page");
        } else if *index_page_cnt > 1 {
            bail!("ERR: Can't have more than one index page");
        }
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePagesStage: Process ...");
        for page in &mut ctx.pages {
            // Check if the page is the index page
            let filename = match page.index {
                true => utils::derive_filename("index", ".html", &ctx.base_dir)
                    .context("Failed to derive a unique filename for page.")?,
                false => utils::derive_filename(&page.title, ".html", &ctx.base_dir)
                    .context("Failed to derive a unique filename for page.")?,
            };

            fs::write(
                ctx.build_pages_dir.join(filename).as_path(),
                page.content.clone(),
            )?;
            println!(
                "Successfully wrote page '{}' to disk ({} bytes)",
                page.title,
                page.content.len()
            );
        }
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePagesStage: Finalize ...");
        Ok(())
    }
}
