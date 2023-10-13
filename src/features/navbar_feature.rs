use std::any::{self, TypeId};

use anyhow::Context;
use build_html::{Container, Html, HtmlContainer};
use chrono::format;

use crate::blog::BlogContext;
use crate::pipeline::feature::Feature;
use crate::pipeline::feature_registry::FeatureRegistry;
use crate::pipeline::pipeline_stage_lifetime::PipelineStageLifetime;
use crate::stages::{ConvertPagesStage, ConvertPostsStage};
use crate::utils;
pub struct NavbarFeature;

impl Feature for NavbarFeature {
    fn register(registry: &mut FeatureRegistry)
    where
        Self: Sized,
    {
        registry.register::<ConvertPostsStage, Self>(PipelineStageLifetime::PostProcess, Self);
        registry.register::<ConvertPagesStage, Self>(PipelineStageLifetime::PostProcess, Self);
    }

    fn run(
        &mut self,
        ctx: &mut BlogContext,
        pipeline_type: TypeId,
        lifetime: PipelineStageLifetime,
    ) {
        println!("Executing NavFeature ...");
        if pipeline_type == TypeId::of::<ConvertPostsStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_in_post(ctx);
        } else if pipeline_type == TypeId::of::<ConvertPagesStage>()
            && lifetime == PipelineStageLifetime::PostProcess
        {
            inject_navbar_in_page(ctx);
        }
    }
}

fn inject_navbar_in_post(ctx: &mut BlogContext) {
    println!("Navbar Feature execution ...");
    let mut nav_html = "<nav>".to_owned();
    for page in ctx.registry.get_pages() {
        // TODO: This is very poorly written, we need to refactor this in the future
        // TODO: Every feature hook should return an anyhow<Result>
        let title = &page.title;
        let filename = &page.html_filename;
        nav_html
            .push_str(format!("<a href=\"/{filename}\" title=\"{title}\">{title}</a>\n").as_str());
    }
    nav_html.push_str("</nav>");

    // Inject navbar html at the top of each converted post
    for post in ctx.registry.get_posts_mut() {
        post.content = format!("{nav_html}{}", post.content);
    }
}
fn inject_navbar_in_page(ctx: &mut BlogContext) {
    println!("Navbar Feature execution ...");
    // TODO: This is very poorly written, we need to refactor this in the future
    // TODO: Every feature hook should return an anyhow<Result>

    // let mut nav_html = "<nav>".to_owned();
    // for page in ctx.registry.get_pages() {
    //     let title = &page.title;
    //     let filename = &page.html_filename;
    //     nav_html
    //         .push_str(format!("<a href=\"/{filename}\" title=\"{title}\">{title}</a>\n").as_str());
    // }
    // nav_html.push_str(
    //     "</nav>
    // <hr>",
    // );

    let mut nav = Container::new(build_html::ContainerType::Nav);
    for page in ctx.registry.get_pages() {
        nav = nav.with_link_attr(
            format!("/{}", page.html_filename),
            format!("{}", page.title),
            [("title", page.title.as_str())],
        );
    }
    nav = nav.with_raw("<hr>");
    println!("NAV: {}", nav.to_html_string());

    // Inject navbar html at the top of each converted post
    for page in ctx.registry.get_pages_mut() {
        // page.content = format!("{}{}", nav_html, page.content);
        page.content = format!("{}{}", nav.to_html_string(), page.content);
    }
}
