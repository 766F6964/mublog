use std::fs;

use anyhow::Context;

use crate::{blog::BlogContext, page, pipeline::pipeline_stage::PipelineStage, post, utils};

pub struct ConvertPostsStage;

impl PipelineStage for ConvertPostsStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, context: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Process ...");
        // Process all posts
        for post in &mut context.posts {
            post.content = markdown::to_html(&post.content);
            println!("Successfully converted post '{}'", post.header.title);
        }
        Ok(())
    }

    fn finalize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ConvertPostsStage: Finalize ...");
        Ok(())
    }
}
