use super::feature::Feature;
use super::pipeline_stage::PipelineStage;
use super::pipeline_stage_lifetime::PipelineStageLifetime;
use anyhow::Context;
use std::any::TypeId;
use std::collections::HashMap;

#[derive(Default)]
pub struct FeatureRegistry(HashMap<(TypeId, PipelineStageLifetime), Vec<Box<dyn Feature>>>);

impl FeatureRegistry {
    pub fn run_hooks(
        &mut self,
        context: &mut super::BlogContext,
        type_id: TypeId,
        lifetime: PipelineStageLifetime,
    ) -> anyhow::Result<()> {
        match self.0.get_mut(&(type_id, lifetime)) {
            Some(features) => {
                for feature in features {
                    feature
                        .run(context, type_id, lifetime)
                        .context("Feature execution failed")?;
                }
                Ok(())
            }
            None => Ok(()),
        }
    }

    pub fn register<TStage: PipelineStage + 'static, TFeature: Feature + 'static>(
        &mut self,
        stage: PipelineStageLifetime,
        feature: TFeature,
    ) {
        self.0
            .entry((TypeId::of::<TStage>(), stage))
            .or_default()
            .push(Box::new(feature));
    }
}
