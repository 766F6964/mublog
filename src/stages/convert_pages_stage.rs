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
        println!("Found: {} pages", ctx.pages.len());
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPagesStage: Process ...");
        // Process all pages
        for page in &mut ctx.pages {
            page.content = markdown::to_html(&page.content);
            println!("Successfully converted page '{}'", page.title);
        }
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPagesStage: Finalize ...");
        Ok(())
    }
}
