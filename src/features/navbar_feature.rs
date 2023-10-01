use crate::blog::BlogContext;
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::WrapPostsStage;
pub struct NavbarFeature;

impl Feature for NavbarFeature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized,
    {
        registry
            .register::<WrapPostsStage, Self>(PipelineStageLifetime::PostProcess, NavbarFeature);
    }

    fn run(
        &mut self,
        ctx: &mut BlogContext,
        pipeline_type: std::any::TypeId,
        lifetime: PipelineStageLifetime,
    ) {
        println!("Executing NavFeature ...");
        if pipeline_type == std::any::TypeId::of::<WrapPostsStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            modification1(ctx);
        }
    }
}

fn modification1(ctx: &mut BlogContext) {
    println!("Navbar Feature execution ...");
}
