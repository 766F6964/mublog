use crate::{blog::BlogContext, pipeline::pipeline_stage::PipelineStage};

pub struct WrapPostsStage;

impl PipelineStage for WrapPostsStage {
    fn initialize(&self, ctx: &mut BlogContext) {
        println!("WrapPostsStage: Initialize ...");
    }

    fn process(&self, _ctx: &mut BlogContext) {
        println!("WrapPostsStage: Process ...");
    }

    fn finalize(&self, ctx: &mut BlogContext) {
        println!("WrapPostsStage: Finalize ...");
    }
}
