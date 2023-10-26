pub mod feature;
pub mod feature_registry;
pub mod pipeline_stage;
pub mod pipeline_stage_lifetime;
use anyhow::{Context, Ok};

use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use std::any::{Any, TypeId};

use self::feature::Feature;
use self::feature_registry::FeatureRegistry;

pub struct Pipeline {
    pipeline_stages: Vec<(Box<dyn pipeline_stage::PipelineStage>, TypeId)>,
    context: BlogContext,
    features: FeatureRegistry,
}

impl Pipeline {
    pub fn new(ctx: BlogContext) -> Self {
        Self {
            context: ctx,
            pipeline_stages: Default::default(),
            features: Default::default(),
        }
    }

    pub fn add_stage<T: pipeline_stage::PipelineStage + 'static>(&mut self, stage: T) {
        println!("Adding stage {}", std::any::type_name::<T>());
        let type_id = stage.type_id();
        self.pipeline_stages.push((Box::new(stage), type_id));
    }

    pub fn add_feature<T: Feature>(&mut self) {
        println!("Adding feature {}", std::any::type_name::<T>());
        T::register(&mut self.features);
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        for (stage, stage_type_id) in &self.pipeline_stages {
            stage.initialize(&mut self.context)?;
            self.features
                .run_hooks(
                    &mut self.context,
                    *stage_type_id,
                    PipelineStageLifetime::PreProcess,
                )
                .context("Stage execution failed during PreProcess step")?;

            stage.process(&mut self.context)?;
            self.features
                .run_hooks(
                    &mut self.context,
                    *stage_type_id,
                    PipelineStageLifetime::PostProcess,
                )
                .context("Stage execution failed during PostProcess step")?;

            stage.finalize(&mut self.context)?;
        }
        Ok(())
    }
}
