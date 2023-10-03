use crate::page::{self, Page};
use crate::path_config::PathConfig;
use crate::post::{self, Post};
use anyhow::{bail, Context, Ok};

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
        self.posts = post::get_posts(&cfg.posts_dir).context("Failed to load posts from disk")?;
        self.pages = page::get_pages(&cfg.pages_dir).context("Failed to load pages from disk")?;
        Ok(())
    }

    pub fn add_page(&mut self, mut page: Page) -> anyhow::Result<()> {
        page.fname = self
            .get_page_filename(&page)
            .context("Failed to obtain unique filename")?;
        self.pages.push(page);
        Ok(())
    }

    pub fn add_post(&mut self, mut post: Post) -> anyhow::Result<()> {
        post.fname = self
            .get_post_filename(&post)
            .context("Failed to obtain unique filename")?;
        self.posts.push(post);
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

    pub fn get_page_filename(&self, page: &Page) -> anyhow::Result<String> {
        let title = page.title.replace(" ", "_").to_lowercase();
        let is_index = page.index;
        if is_index && self.contains_index_page() {
            bail!("Only a single index page is allowed.");
        } else if !is_index && title == "index" {
            bail!("A non-index page can't be named index");
        }
        if is_index {
            Ok("index.html".into())
        } else {
            let mut name = format!("{}.html", title).to_owned();
            let mut count = 1;
            while self.contains_page_filename(&name) {
                name = format!("{}_{}.html", title, count);
                count += 1;
            }
            Ok(name)
        }
    }

    pub fn get_post_filename(&self, post: &Post) -> anyhow::Result<String> {
        let title = post.header.title.replace(" ", "_").to_lowercase();
        let mut name = format!("{}.html", title).to_owned();
        let mut count = 1;
        while self.contains_post_filename(&name) {
            name = format!("{}_{}.html", title, count);
            count += 1;
        }
        Ok(name)
    }

    fn contains_page_filename(&self, name: &str) -> bool {
        self.pages.iter().any(|p| p.fname == name)
    }
    fn contains_post_filename(&self, name: &str) -> bool {
        self.posts.iter().any(|p| p.fname == name)
    }
    fn contains_index_page(&self) -> bool {
        self.pages.iter().any(|p| p.index == true)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_post_filename_increments() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Post::default();
        let mut p2 = Post::default();
        let mut p3 = Post::default();
        p1.header.title = "hi".into();
        p2.header.title = "hi".into();
        p3.header.title = "hi".into();

        _ = reg.add_post(p1);
        _ = reg.add_post(p2);
        _ = reg.add_post(p3);

        let pages = reg.get_posts();
        assert_eq!(pages.get(0).unwrap().fname.clone(), "hi.html".to_owned());
        assert_eq!(pages.get(1).unwrap().fname.clone(), "hi_1.html".to_owned());
        assert_eq!(pages.get(2).unwrap().fname.clone(), "hi_2.html".to_owned());
    }
    #[test]
    fn get_page_filename_increments() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        let mut p2 = Page::default();
        let mut p3 = Page::default();
        p1.title = "hi".into();
        p2.title = "hi".into();
        p3.title = "hi".into();

        _ = reg.add_page(p1);
        _ = reg.add_page(p2);
        _ = reg.add_page(p3);

        let pages = reg.get_pages();
        assert_eq!(pages.get(0).unwrap().fname.clone(), "hi.html".to_owned());
        assert_eq!(pages.get(1).unwrap().fname.clone(), "hi_1.html".to_owned());
        assert_eq!(pages.get(2).unwrap().fname.clone(), "hi_2.html".to_owned());
    }
    #[test]
    fn get_page_filename_index_page() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        p1.index = true;
        p1.title = "test".into();

        _ = reg.add_page(p1);

        let pages = reg.get_pages();
        assert_eq!(pages.get(0).unwrap().fname.clone(), "index.html".to_owned());
    }
    #[test]
    fn get_page_filename_fake_index_page() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        p1.index = false;
        p1.title = "index".into();

        let err = reg.add_page(p1).unwrap_err();

        assert_eq!(
            err.to_string(),
            "Failed to obtain unique filename".to_owned()
        );
    }
    #[test]
    fn get_page_filename_duplicate_index_page() {
        let mut reg = SiteComponentRegistry::new();
        let mut p1 = Page::default();
        let mut p2 = Page::default();
        p1.index = true;
        p1.title = "test".into();
        p2.index = true;
        p2.title = "hello".into();

        let res1 = reg.add_page(p1);
        let res2 = reg.add_page(p2);
        let pages = reg.get_pages();

        assert!(res1.is_ok());
        assert_eq!(pages.get(0).unwrap().fname.clone(), "index.html".to_owned());
        assert!(res2.is_err());
        assert_eq!(
            res2.unwrap_err().to_string(),
            "Failed to obtain unique filename".to_owned()
        );
    }
}
