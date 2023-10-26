use crate::blog::BlogContext;
use crate::pipeline::pipeline_stage::PipelineStage;

pub struct ApplyGlobalVarsStage;

impl PipelineStage for ApplyGlobalVarsStage {
    fn initialize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ApplyGlobalVarsStage: Initialize ...");
        Ok(())
    }

    fn process(&self, ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ApplyGlobalVarsStage: Process ...");

        let author_name = &ctx.config.blog_author;
        let author_email = &ctx.config.blog_email;
        let copyright_year = ctx.config.blog_copyright_year.to_string();

        for page in ctx.registry.get_pages_mut() {
            page.content = page
                .content
                .replace("{{{AUTHOR_NAME}}}", &author_name)
                .replace("{{{COPYRIGHT_YEAR}}}", &copyright_year)
                .replace("{{{AUTHOR_EMAIL}}}", &author_email);
        }
        for post in ctx.registry.get_posts_mut() {
            post.content = post
                .content
                .replace("{{{AUTHOR_NAME}}}", &author_name)
                .replace("{{{COPYRIGHT_YEAR}}}", &copyright_year)
                .replace("{{{AUTHOR_EMAIL}}}", &author_email);
        }
        Ok(())
    }

    fn finalize(&self, _ctx: &mut BlogContext) -> anyhow::Result<()> {
        println!("ApplyGlobalVarsStage: Finalize ...");
        Ok(())
    }
}
