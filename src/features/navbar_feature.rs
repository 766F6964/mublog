use crate::blog::BlogContext;
use crate::features::FeatureConfig;
use crate::page::Page;
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::{ConvertPagesStage, ConvertPostsStage, LoadStylesheetsStage};
use anyhow::Context;
use anyhow::{bail, Ok};
use build_html::{Container, Html, HtmlContainer};
use serde::Deserialize;
use std::any::TypeId;
pub struct NavbarFeature;

impl Feature for NavbarFeature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized,
    {
        registry.register::<ConvertPostsStage, Self>(PipelineStageLifetime::PostProcess, Self);
        registry.register::<ConvertPagesStage, Self>(PipelineStageLifetime::PostProcess, Self);
        registry.register::<LoadStylesheetsStage, Self>(PipelineStageLifetime::PostProcess, Self);
    }

    fn run(
        &mut self,
        ctx: &mut BlogContext,
        pipeline_type: TypeId,
        lifetime: PipelineStageLifetime,
    ) -> anyhow::Result<()> {
        println!("Executing NavFeature ...");
        if pipeline_type == TypeId::of::<ConvertPostsStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            let nav_cfg =
                get_navbar_config(ctx).context("Failed to retrieve NavbarFeature configuration")?;
            let nav = create_navbar_html(ctx, &nav_cfg)
                .context("Failed to generate navbar html structure")?;
            inject_navbar_in_post(ctx, nav)
                .context("Failed to inject navbar HTML structure into post")?;
            Ok(())
        } else if pipeline_type == TypeId::of::<ConvertPagesStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            let nav_cfg =
                get_navbar_config(ctx).context("Failed to retrieve NavbarFeature configuration")?;
            let nav = create_navbar_html(ctx, &nav_cfg)
                .context("Failed to generate navbar html structure")?;
            inject_navbar_in_page(ctx, nav).context("Failed to inject navbar into page")?;
            Ok(())
        } else if pipeline_type == TypeId::of::<LoadStylesheetsStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_css(ctx).context("Failed to inject navbar css")
        } else {
            Ok(())
        }
    }
}

fn get_navbar_config(ctx: &mut BlogContext) -> anyhow::Result<NavbarConfig> {
    let res = ctx
        .config
        .features
        .iter()
        .find_map(|config| {
            if let FeatureConfig::Navbar(nav_cfg) = config {
                Some(nav_cfg.clone())
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow::anyhow!("Failed to find navbar cfg in features vector"))?;
    Ok(res)
}

#[derive(Debug, Deserialize, Clone)]
pub struct NavbarConfig {
    pub links: Vec<String>,
}

fn inject_navbar_css(ctx: &mut BlogContext) -> anyhow::Result<()> {
    let nav_css = r#"
        body nav {
            font-size: 1.2rem;
            text-align: center;
            margin-bottom: 30px;
            margin-top: 30px;
        }
        nav a {
            margin-left: 0.4rem;
            margin-right: 0.4rem;
        }
    "#;
    if let Some(layout) = ctx
        .registry
        .get_stylesheets_mut()
        .iter_mut()
        .find(|s| s.filename == "layout.css")
    {
        layout.content.push_str(nav_css);
        Ok(())
    } else {
        bail!("No such file layout.css in SiteComponentRegistry.")
    }
}

fn inject_navbar_in_post(ctx: &mut BlogContext, nav: Container) -> anyhow::Result<()> {
    for post in ctx.registry.get_posts_mut() {
        post.content = format!("{}{}", nav.to_html_string(), post.content);
    }
    Ok(())
}
fn inject_navbar_in_page(ctx: &mut BlogContext, nav: Container) -> anyhow::Result<()> {
    for page in ctx.registry.get_pages_mut() {
        page.content = format!("{}{}", nav.to_html_string(), page.content);
    }
    Ok(())
}

fn create_navbar_html(ctx: &mut BlogContext, nav_cfg: &NavbarConfig) -> anyhow::Result<Container> {
    let mut nav = Container::new(build_html::ContainerType::Nav);

    let mut navbar_elements: Vec<Page> = Vec::new();
    for cfg_entry in nav_cfg.links.clone() {
        let mut found = false;
        for page in ctx.registry.get_pages() {
            if cfg_entry == page.md_filename {
                navbar_elements.push(page.clone());
                found = true;
                break;
            }
        }
        if !found {
            bail!(format!(
                "Navbar configuration expected a page '{}'. No such page was found.",
                cfg_entry
            ))
        }
    }
    for page in navbar_elements {
        nav = nav.with_link_attr(
            format!("/{}", page.html_filename),
            page.title.to_string(),
            [("title", page.title.as_str())],
        );
    }
    nav = nav.with_raw("<hr>");
    Ok(nav)
}
