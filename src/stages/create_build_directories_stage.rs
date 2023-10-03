use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;
use crate::utils;
use anyhow::Context;
use std::fs;

pub struct CreateBuildDirectoriesStage;

impl PipelineStage for CreateBuildDirectoriesStage {
    fn initialize(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("CreateBuildDirectoriesStage: Initialize ...");

        // Delete previous build environment, if present
        if fs::metadata(ctx.paths.build_dir.as_path()).is_ok() {
            fs::remove_dir_all(ctx.paths.build_dir.as_path())
                .context("Failed to remove existing build environment.")?;
        }

        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("CreateBuildDirectoriesStage: Process ...");

        // Check source directories exist
        utils::is_valid_dir(&ctx.paths.assets_dir)
            .context("Assets directory could not be found or is inaccessible.")?;
        utils::is_valid_dir(&ctx.paths.css_dir)
            .context("CSS directory could not be found or is inaccessible.")?;
        utils::is_valid_dir(&ctx.paths.posts_dir)
            .context("Posts directory could not be found or is inaccessible.")?;
        utils::is_valid_dir(&ctx.paths.pages_dir)
            .context("Pages directory could not be found or is inaccessible.")?;

        // Setup build directory and subdirectories
        fs::create_dir(ctx.paths.build_dir.as_path())
            .with_context(|| format!("Failed to create directory {:?}", ctx.paths.build_dir))?;
        fs::create_dir(ctx.paths.build_posts_dir.as_path()).with_context(|| {
            format!("Failed to create directory {:?}", ctx.paths.build_posts_dir)
        })?;
        fs::create_dir(ctx.paths.build_css_dir.as_path())
            .with_context(|| format!("Failed to create directory {:?}", ctx.paths.build_css_dir))?;
        fs::create_dir(ctx.paths.build_assets_dir.as_path()).with_context(|| {
            format!(
                "Failed to create directory {:?}",
                ctx.paths.build_assets_dir
            )
        })?;

        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("CreateBuildDirectoriesStage: Finalize ...");
        Ok(())
    }
}
