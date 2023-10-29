pub mod feature;
pub mod feature_registry;
pub mod pipeline_stage;
pub mod pipeline_stage_lifetime;
use self::feature::Feature;
use self::feature_registry::FeatureRegistry;
use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use anyhow::Context;
use std::any::Any;
use std::any::TypeId;

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

#[cfg(test)]
mod test {
    use crate::blog::BlogContext;
    use crate::pipeline::pipeline_stage::PipelineStage;
    use crate::pipeline::Pipeline;

    #[test]
    fn pipeline_execution_order() {
        struct DummyStage1;
        struct DummyStage2;
        struct DummyStage3;

        impl PipelineStage for DummyStage1 {
            fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("1i");
                Ok(())
            }

            fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("1p");
                Ok(())
            }

            fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("1f");
                Ok(())
            }
        }

        impl PipelineStage for DummyStage2 {
            fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("2i");
                Ok(())
            }

            fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("2p");
                Ok(())
            }

            fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("2f");
                Ok(())
            }
        }

        impl PipelineStage for DummyStage3 {
            fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("3i");
                Ok(())
            }

            fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("3p");
                Ok(())
            }

            fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
                ctx.config.blog_author.push_str("3f");
                Ok(())
            }
        }

        let ctx = BlogContext::default();
        let mut pipeline = Pipeline::new(ctx);
        pipeline.add_stage(DummyStage1);
        pipeline.add_stage(DummyStage2);
        pipeline.add_stage(DummyStage3);
        let res = pipeline.run();
        assert!(res.is_ok());
        assert_eq!(pipeline.context.config.blog_author, "1i1p1f2i2p2f3i3p3f");
    }
}
