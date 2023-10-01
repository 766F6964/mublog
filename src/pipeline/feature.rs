use std::any::TypeId;

use super::{feature_registry::FeatureRegistry, pipeline_stage_lifetime::PipelineStageLifetime};

pub trait Feature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized;

    fn run(
        &mut self,
        ctx: &mut super::BlogContext,
        pipeline_type: TypeId,
        lifetime: PipelineStageLifetime,
    );
}
