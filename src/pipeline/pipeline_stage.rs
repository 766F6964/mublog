use crate::blog::BlogContext;

pub trait PipelineStage {
    fn initialize(&self, ctx: &mut BlogContext);
    fn process(&self, ctx: &mut BlogContext);
    fn finalize(&self, ctx: &mut BlogContext);
}
