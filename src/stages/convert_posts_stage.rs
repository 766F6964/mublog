use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use markdown::CompileOptions;
use markdown::Options;

pub struct ConvertPostsStage;

impl PipelineStage for ConvertPostsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Process ...");
        // Process all posts
        for post in ctx.registry.get_posts_mut() {
            post.content = markdown::to_html_with_options(
                &post.content,
                &Options {
                    compile: CompileOptions {
                        allow_dangerous_html: true,
                        ..CompileOptions::default()
                    },
                    ..Options::default()
                },
            )
            .unwrap(); // We can safely unwrap here!
            println!("Successfully converted post '{}'", post.title);
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Finalize ...");
        Ok(())
    }
}
