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

    fn process(&self, context: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Process ...");
        // Process all posts
        for post in &mut context.posts {
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
            println!("Successfully converted post '{}'", post.header.title);
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Finalize ...");
        Ok(())
    }
}