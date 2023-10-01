use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
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

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPagesStage: Finalize ...");
        Ok(())
    }
}
