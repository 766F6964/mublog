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
        // Check that there is only one index page
        // TODO: I think this is unnecessary and already covered by the ComponentRegistry
        let index_page_cnt = ctx.registry.get_pages().iter().filter(|&p| p.index).count();
        if index_page_cnt == 0 {
            println!("WARN: You don't have a landing page");
        } else if index_page_cnt > 1 {
            bail!("ERR: Can't have more than one index page");
        }
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WritePagesStage: Process ...");
        for page in ctx.registry.get_pages() {
            // Check if the page is the index page
            let filename = match page.index {
                true => utils::derive_filename("index", ".html", &ctx.paths.base_dir)
                    .context("Failed to derive a unique filename for page.")?,
                false => utils::derive_filename(&page.title, ".html", &ctx.paths.base_dir)
                    .context("Failed to derive a unique filename for page.")?,
            };

            fs::write(
                ctx.paths.build_pages_dir.join(filename).as_path(),
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
