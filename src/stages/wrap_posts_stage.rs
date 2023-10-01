use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;

pub struct WrapPostsStage;

impl PipelineStage for WrapPostsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPostsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPostsStage: Process ...");
        for post in &mut ctx.posts {
            let mut post_html = String::new();
            post_html.push_str(html_tag_start().as_str());
            post_html.push_str(head_tag_start().as_str());
            post_html.push_str(&meta_tags(
                &post.header.title,
                "TODO_AUTHOR",
                "TODO_URL",
                "TODO_DESCRIPTION",
                "TODO_DESCRIPTION",
                &post.header.title,
                "TODO_URL",
                post.header.date.to_string().as_str(),
            ));
            post_html.push_str(stylesheet_refs().as_str());
            post_html.push_str(head_tag_end().as_str());
            post_html.push_str(body_tag_start().as_str());
            post_html.push_str(&post.content);
            post_html.push_str(body_tag_end().as_str());
            post_html.push_str(html_tag_end().as_str());

            post.content = post_html;
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPostsStage: Finalize ...");
        Ok(())
    }
}

fn html_tag_start() -> String {
    r#"
<!DOCTYPE html>
<html lang="en-US">"#
        .into()
}

fn html_tag_end() -> String {
    r#"
    </html>"#
        .into()
}

fn head_tag_start() -> String {
    r#"
    <head>"#
        .into()
}
fn head_tag_end() -> String {
    r#"
    </head>"#
        .into()
}

fn meta_tags(
    blog_title: &str,
    blog_author: &str,
    blog_url: &str,
    blog_description: &str,
    post_description: &str,
    post_title: &str,
    post_url: &str,
    post_date: &str,
) -> String {
    format!(
        r#"
        <meta charset="utf-8">
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta name="robots" content="index, archive">
        <meta name="canonical" content="{blog_url}">
        <meta name="description" content="{blog_description}">
        <meta property="og:type" content="article" />
        <meta property="og:locale" content="en_US" />
        <meta property="og:site_name" content="${blog_title}" />
        <meta property="og:title" content="{post_title}" />
        <meta property="og:description" content="${post_description}" />
        <meta property="og:url" content="{post_url}/${post_title}.html" />
        <meta property="og:article:published_time" content="{post_date}" />
        <meta property="og:article:author" content="{blog_author}" />"#
    )
}

fn stylesheet_refs() -> String {
    r#"
    <link rel="stylesheet" type="text/css" media="all" href="/css/layout.css">"#
        .into()
}
fn body_tag_start() -> String {
    r#"
        <body class="${blog_theme}">
        <main>
        <hr>"#
        .into()
}
fn body_tag_end() -> String {
    r#"
        </main>
        </body>"#
        .into()
}
