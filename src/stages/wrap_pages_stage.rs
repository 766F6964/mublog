use build_html::{Container, ContainerType, Html, HtmlContainer};

use crate::blog::BlogContext;
use crate::page;
use crate::pipeline::pipeline_stage::PipelineStage;

pub struct WrapPagesStage;
// TODO: The entire HTML site wrapping can be abstracted into dedicated classes to improve readablity a lot.
impl PipelineStage for WrapPagesStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPagesStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPagesStage: Process ...");
        for page in ctx.registry.get_pages_mut() {
            let doc = build_html::HtmlPage::new()
                .with_meta(vec![("charset", "utf-8")])
                .with_title("PAGE_TITLE")
                .with_meta(vec![
                    ("name", "viewport"),
                    ("content", "width=device-width, initial-scale=1"),
                ])
                .with_meta(vec![("name", "robots"), ("content", "index, archive")])
                .with_meta(vec![("name", "canonical"), ("content", "BLOG_URL")])
                .with_meta(vec![
                    ("name", "description"),
                    ("content", "BLOG_DESCRIPTION"),
                ])
                .with_meta(vec![("property", "og:type"), ("content", "article")])
                .with_meta(vec![("property", "og:locale"), ("content", "en_US")])
                .with_meta(vec![
                    ("property", "og:site_name"),
                    ("content", "BLOG_TITLE"),
                ])
                .with_meta(vec![("property", "og:title"), ("content", "POST_TITLE")])
                .with_meta(vec![
                    ("property", "og:description"),
                    ("content", "PAGE_DESCRIPTION"),
                ])
                .with_meta(vec![("property", "og:url"), ("content", "POST_URL")])
                .with_meta(vec![
                    ("property", "og:article:published_time"),
                    ("content", "PAGE_DATE"),
                ])
                .with_meta(vec![
                    ("property", "og:article:author"),
                    ("content", "BLOG_AUTHOR"),
                ])
                .with_stylesheet("/css/layout.css")
                // .with_stylesheet("/css/normalize.css") // Currently breaks body margin
                .with_head_link("/meta/webmanifest.xml", "manifest")
                .with_head_link_attr(
                    "/meta/apple-touch-icon.png",
                    "apple-touch-icon",
                    [("sizes", "180x180")],
                )
                .with_head_link_attr(
                    "/meta/favicon-32x32.png",
                    "icon",
                    [("type", "image/png"), ("sizes", "32x32")],
                )
                .with_head_link_attr(
                    "/meta/favicon-16x16.png",
                    "icon",
                    [("type", "image/png"), ("sizes", "16x16")],
                )
                .with_head_link("/meta/favicon.ico", "favicon")
                .with_title("PAGE_TITLE")
                // .with_raw("<hr>")
                .with_container(Container::new(ContainerType::Main).with_raw(&page.content))
                .with_container(
                    Container::new(ContainerType::Footer)
                        .with_raw("<hr>")
                        .with_container(
                            Container::new(ContainerType::Div)
                                .with_attributes(vec![("class", "footer-elements")])
                                .with_container(
                                    Container::new(ContainerType::Div)
                                        .with_attributes(vec![("class", "footer-copyright")])
                                        .with_raw("BLOG_COPYRIGHT_YEAR"),
                                ),
                        ),
                )
                .to_html_string();
            println!("{}", doc);
            // TODO: Instead of directly converting to string, store the HTML document structure to allow
            // for easier modifications by features (e.g. if they need to insert new dom elements)
            page.content = doc;
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("WrapPagesStage: Finalize ...");
        Ok(())
    }
}
