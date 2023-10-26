use std::{any::TypeId, collections::HashMap};

use anyhow::Context;

use super::{
    feature::Feature, pipeline_stage::PipelineStage, pipeline_stage_lifetime::PipelineStageLifetime,
};

#[derive(Default)]
pub struct FeatureRegistry(HashMap<(TypeId, PipelineStageLifetime), Vec<Box<dyn Feature>>>);

impl FeatureRegistry {
    pub fn run_hooks(
        &mut self,
        context: &mut super::BlogContext,
        type_id: TypeId,
        lifetime: PipelineStageLifetime,
    ) -> anyhow::Result<()> {
        // let Some(features) = self.0.get_mut(&(type_id, lifetime)) else {
        //     Ok(())
        // };

        match self.0.get_mut(&(type_id, lifetime)) {
            Some(features) => {
                // Handle the Some case here
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
