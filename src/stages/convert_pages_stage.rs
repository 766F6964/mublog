use std::fs;

use anyhow::Context;

use crate::blog::BlogContext;
use crate::page;
use crate::pipeline::pipeline_stage::PipelineStage;
use crate::utils;

pub struct ConvertPagesStage;

impl PipelineStage for ConvertPagesStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPagesStage: Initialize ...");
        ctx.pages = page::get_pages(&ctx.base_dir);
        println!("Found: {} pages", ctx.pages.len());
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPagesStage: Process ...");
        // Process all pages
        for page in &ctx.pages {
            let page_filename = if page.index {
                utils::derive_filename("index", ".html", &ctx.base_dir)
                    .context("Failed to derive a unique filename for page.")?
            } else {
                utils::derive_filename(&page.title, ".html", &ctx.base_dir)
                    .context("Failed to derive a unique filename for page.")?
            };
            let content_html = markdown::to_html(&page.content);
            let html_filename = page_filename.replace(".md", ".html");
            fs::write(ctx.build_dir.join(html_filename).as_path(), content_html)?;
            println!("Successfully built page '{}'", page.title);
        }
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPagesStage: Finalize ...");
        Ok(())
    }
}
