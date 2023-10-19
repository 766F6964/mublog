use crate::blog::BlogContext;
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::{ConvertPagesStage, ConvertPostsStage, LoadStylesheetsStage};
use anyhow::{bail, Context, Ok};
use build_html::{Container, Html, HtmlContainer};
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
            inject_navbar_in_post(ctx).context("Failed to inject navbar into post")
        } else if pipeline_type == TypeId::of::<ConvertPagesStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_in_page(ctx).context("Failed to inject navbar into page")
        } else if pipeline_type == TypeId::of::<LoadStylesheetsStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_css(ctx).context("Failed to inject navbar css")
        } else {
            Ok(())
        }
    }
}

// TODO: Maybe every feature hook should return an anyhow<Result>
// TODO: Inject CSS into default CSS template

fn inject_navbar_css(ctx: &mut BlogContext) -> anyhow::Result<()> {
    println!("Injecting Navbar CSS into layout.css");
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
        println!("Completed!");
        Ok(())
    } else {
        // TODO: Don't panic, instead propagate an error
        bail!("Layout.css should be in SiteComponentRegistry.")
    }
}

fn inject_navbar_in_post(ctx: &mut BlogContext) -> anyhow::Result<()> {
    let nav = create_navbar_html(ctx);
    for post in ctx.registry.get_posts_mut() {
        post.content = format!("{}{}", nav.to_html_string(), post.content);
    }
    Ok(())
}
fn inject_navbar_in_page(ctx: &mut BlogContext) -> anyhow::Result<()> {
    let nav = create_navbar_html(ctx);
    for page in ctx.registry.get_pages_mut() {
        page.content = format!("{}{}", nav.to_html_string(), page.content);
    }
    Ok(())
}

fn create_navbar_html(ctx: &mut BlogContext) -> Container {
    let mut nav = Container::new(build_html::ContainerType::Nav);
    for page in ctx.registry.get_pages() {
        nav = nav.with_link_attr(
            format!("/{}", page.html_filename),
            format!("{}", page.title),
            [("title", page.title.as_str())],
        );
    }
    nav = nav.with_raw("<hr>");
    nav
}
