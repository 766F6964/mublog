use std::ffi::OsStr;
use std::fs::{self, read_to_string};
use std::path::PathBuf;

use crate::page;
use crate::page::Page;
use crate::path_config::PathConfig;
use crate::post;
use crate::post::Post;
use anyhow::{bail, Context, Ok};
use walkdir::WalkDir;

#[derive(Debug, Default)]
pub struct SiteComponentRegistry {
    pages: Vec<Page>,
    posts: Vec<Post>,
}

impl SiteComponentRegistry {
    pub fn new() -> Self {
        Self {
            pages: vec![],
            posts: vec![],
        }
    }

    pub fn initialize(&mut self, cfg: &PathConfig) -> anyhow::Result<()> {
        println!("Initializing SiteComponentRegistry ...");
        self.load_posts_from_disk(&cfg.posts_dir)
            .context("Failed to load posts from disk")?;
        self.load_pages_from_disk(&cfg.pages_dir)
            .context("Failed to load pages from disk")?;
        Ok(())
    }

    pub fn register_post(&mut self, mut post: Post) -> anyhow::Result<()> {
        // TODO: Validate the post
        let (html_name, md_name) = self.get_post_filename(&post)?;
        post.html_filename = html_name;
        post.md_filename = md_name;
        self.posts.push(post);
        Ok(())
    }

    pub fn register_page(&mut self, mut page: Page) -> anyhow::Result<()> {
        // TODO: Maybe we should insert in a hashmap instead of a vec
        let (html_name, md_name) = self.get_page_filename(&page)?;
        println!("HTML Filename: {html_name}");
        println!("MD Filename: {md_name}");
        page.html_filename = html_name;
        page.md_filename = md_name;
        self.pages.push(page);
        Ok(())
    }

    pub fn get_pages_mut(&mut self) -> &mut Vec<Page> {
        &mut self.pages
    }

    pub fn get_posts_mut(&mut self) -> &mut Vec<Post> {
        &mut self.posts
    }

    pub fn get_pages(&self) -> &Vec<Page> {
        &self.pages
    }

    pub fn get_posts(&self) -> &Vec<Post> {
        &self.posts
    }

    pub fn get_page_filename(&self, page: &Page) -> anyhow::Result<(String, String)> {
        // TODO
        let title = page.title.trim().replace(" ", "_").to_lowercase();
        if page.index && self.contains_index_page() {
            bail!("Duplicate index pages are not allowed.");
        }
        if !page.index && title == "index" {
            bail!("Non-index page can't be named 'index'");
        }

        let mut html_fname = format!("{}.html", title).to_owned();
        let mut md_fname = format!("{}.md", title).to_owned();
        let mut count = 1;
        while self.contains_page_filename(&html_fname, &md_fname) {
            html_fname = format!("{}_{}.html", title, count);
            md_fname = format!("{}_{}.md", title, count);
            count += 1;
        }
        Ok((html_fname, md_fname))
    }

    pub fn get_post_filename(&self, post: &Post) -> anyhow::Result<(String, String)> {
        let title = post.header.title.trim().replace(" ", "_").to_lowercase();
        let mut html_fname = format!("{}.html", title).to_owned();
        let mut md_fname = format!("{}.md", title).to_owned();
        let mut count = 1;
        while self.contains_post_filename(&html_fname, &md_fname) {
            html_fname = format!("{}_{}.html", title, count);
            md_fname = format!("{}_{}.md", title, count);
            count += 1;
        }
        Ok((html_fname, md_fname))
    }

    fn load_posts_from_disk(&mut self, posts_dir: &PathBuf) -> anyhow::Result<()> {
        let entries = WalkDir::new(posts_dir);
        for entry in entries.into_iter().filter_map(Result::ok) {
            let post_path = entry.path();
            if post_path.extension().and_then(OsStr::to_str) == Some("md") {
                let file_content = read_to_string(post_path).with_context(|| {
                    format!(
                        "Failed to open post file '{}' for reading",
                        post_path.display()
                    )
                })?;
                let post = post::parse_from_string(&file_content)
                    .with_context(|| format!("Failed to parse post '{}'", post_path.display()))?;
                self.register_post(post)
                    .context("Failed to register post")?;
            }
        }
        Ok(())
    }

    fn load_pages_from_disk(&mut self, pages_dir: &PathBuf) -> anyhow::Result<()> {
        let entries = WalkDir::new(pages_dir);
        for entry in entries.into_iter().filter_map(Result::ok) {
            let page_path = entry.path();
            if page_path.extension().and_then(OsStr::to_str) == Some("md") {
                let file_content = read_to_string(page_path).with_context(|| {
                    format!(
                        "Failed to open page file '{}' for reading",
                        page_path.display()
                    )
                })?;
                let page = page::parse_from_string(&file_content)
                    .with_context(|| format!("Failed to parse page '{}'", page_path.display()))?;
                self.register_page(page)
                    .context("Failed to register new page")?;
            }
        }
        Ok(())
    }

    fn contains_post_filename(&self, html_name: &str, md_name: &str) -> bool {
        self.posts
            .iter()
            .any(|p| p.html_filename == html_name || p.md_filename == md_name)
    }

    fn contains_page_filename(&self, html_name: &str, md_name: &str) -> bool {
        self.pages
            .iter()
            .any(|p| p.html_filename == html_name || p.md_filename == md_name)
    }

    fn contains_index_page(&self) -> bool {
        self.pages.iter().any(|p| p.index == true)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // TODO: Add unit test that loads posts/pages from disk
    #[test]
    fn register_post_incremental_filename() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Post::default();
        let mut p2 = Post::default();
        let mut p3 = Post::default();
        p1.header.title = "post".into();
        p2.header.title = "post".into();
        p3.header.title = "post".into();

        _ = reg.register_post(p1);
        _ = reg.register_post(p2);
        _ = reg.register_post(p3);

        let posts = reg.get_posts();
        assert_eq!(
            posts.get(0).unwrap().html_filename.clone(),
            "post.html".to_owned()
        );
        assert_eq!(
            posts.get(0).unwrap().md_filename.clone(),
            "post.md".to_owned()
        );
        assert_eq!(
            posts.get(1).unwrap().html_filename.clone(),
            "post_1.html".to_owned()
        );
        assert_eq!(
            posts.get(1).unwrap().md_filename.clone(),
            "post_1.md".to_owned()
        );
        assert_eq!(
            posts.get(2).unwrap().html_filename.clone(),
            "post_2.html".to_owned()
        );
        assert_eq!(
            posts.get(2).unwrap().md_filename.clone(),
            "post_2.md".to_owned()
        );
    }

    #[test]
    fn register_page_incremental_filename() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        let mut p2 = Page::default();
        let mut p3 = Page::default();
        p1.title = "page".into();
        p2.title = "page".into();
        p3.title = "page".into();

        _ = reg.register_page(p1);
        _ = reg.register_page(p2);
        _ = reg.register_page(p3);

        let pages = reg.get_pages();
        assert_eq!(
            pages.get(0).unwrap().html_filename.clone(),
            "page.html".to_owned()
        );
        assert_eq!(
            pages.get(0).unwrap().md_filename.clone(),
            "page.md".to_owned()
        );
        assert_eq!(
            pages.get(1).unwrap().html_filename.clone(),
            "page_1.html".to_owned()
        );
        assert_eq!(
            pages.get(1).unwrap().md_filename.clone(),
            "page_1.md".to_owned()
        );
        assert_eq!(
            pages.get(2).unwrap().html_filename.clone(),
            "page_2.html".to_owned()
        );
        assert_eq!(
            pages.get(2).unwrap().md_filename.clone(),
            "page_2.md".to_owned()
        );
    }

    #[test]
    fn register_page_index_page() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        p1.index = true;
        p1.title = "test".into();

        _ = reg.register_page(p1);

        let pages = reg.get_pages();
        assert_eq!(
            pages.get(0).unwrap().html_filename.clone(),
            "index.html".to_owned()
        );
        assert_eq!(
            pages.get(0).unwrap().md_filename.clone(),
            "test.md".to_owned()
        );
    }

    #[test]
    fn register_page_fake_index_page() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        p1.index = false;
        p1.title = "index".into();

        let err = reg.register_page(p1).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Non-index page can't be named 'index'".to_owned()
        );
    }

    #[test]
    fn register_page_duplicate_index_page() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        let mut p2 = Page::default();
        p1.index = true;
        p1.title = "test".into();
        p2.index = true;
        p2.title = "hello".into();

        let res1 = reg.register_page(p1);
        let res2 = reg.register_page(p2);
        let pages = reg.get_pages();

        assert!(res1.is_ok());
        assert_eq!(
            pages.get(0).unwrap().html_filename.clone(),
            "index.html".to_owned()
        );
        assert_eq!(
            pages.get(0).unwrap().html_filename.clone(),
            "index.html".to_owned()
        );
        assert!(res2.is_err());
        assert_eq!(
            res2.unwrap_err().to_string(),
            "Duplicate index pages are not allowed.".to_owned()
        );
    }
}
