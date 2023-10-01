use crate::blog::BlogContext;

pub trait PipelineStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()>;
    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()>;
    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()>;
}
