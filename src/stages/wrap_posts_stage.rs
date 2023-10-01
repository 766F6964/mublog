use crate::{blog::BlogContext, pipeline::pipeline_stage::PipelineStage};

pub struct WrapPostsStage;

impl PipelineStage for WrapPostsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPostsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPostsStage: Process ...");
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPostsStage: Finalize ...");
        Ok(())
    }
}
