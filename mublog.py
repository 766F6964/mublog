import configparser
import datetime
import glob
import logging
import os
import time
import shutil
import subprocess
import re
import html
import urllib.parse
from string import Template
from urllib.parse import urljoin


class PathConfig:
    def __init__(self):
        # Define each individual directory name
        self.dst_dir_name = "dst"
        self.src_dir_name = "src"
        self.post_dir_name = "posts"
        self.assets_dir_name = "assets"
        self.meta_dir_name = "meta"
        self.js_dir_name = "js"
        self.css_dir_name = "css"
        self.templates_dir_name = "templates"

        # Construct local src directory paths
        self.src_dir_path = self.src_dir_name
        self.src_posts_dir_path = os.path.join(self.src_dir_path, self.post_dir_name)
        self.src_assets_dir_path = os.path.join(self.src_dir_path, self.assets_dir_name)
        self.src_meta_dir_path = os.path.join(self.src_dir_path, self.meta_dir_name)
        self.src_css_dir_path = os.path.join(self.src_dir_path, self.css_dir_name)
        self.src_templates_dir_path = os.path.join(self.src_dir_path, self.templates_dir_name)

        # Construct local dst directory paths
        self.dst_dir_path = self.dst_dir_name
        self.dst_posts_dir_path = os.path.join(self.dst_dir_path, self.post_dir_name)
        self.dst_assets_dir_path = os.path.join(self.dst_dir_path, self.assets_dir_name)
        self.dst_meta_dir_path = os.path.join(self.dst_dir_path, self.meta_dir_name)
        self.dst_css_dir_path = os.path.join(self.dst_dir_path, self.css_dir_name)
        self.dst_js_dir_path = os.path.join(self.dst_dir_path, self.js_dir_name)


class BlogConfig:
    def __init__(self):
        self.blog_url = ""
        self.blog_title = ""
        self.blog_description = ""
        self.blog_author_name = ""
        self.blog_author_mail = ""
        self.post_ignore_prefix = ""
        self.blog_author_copyright = ""
        self.blog_theme = ""
        self.blog_theme_can_toggle = ""


class LogFormatter(logging.Formatter):
    FORMATS = {
        logging.DEBUG: "\033[34m[*]\033[0m %(message)s",
        logging.INFO: "\033[32m[+]\033[0m %(message)s",
        logging.WARNING: "\033[33m[!]\033[0m %(message)s",
        logging.ERROR: "\033[31m[x]\033[0m %(message)s",
        logging.CRITICAL: "\033[31m[x]\033[0m %(message)s",
    }

    def format(self, record):
        log_fmt = self.FORMATS.get(record.levelno)
        formatter = logging.Formatter(log_fmt)
        return formatter.format(record)


class Helper:

    @staticmethod
    def pandoc_md_to_html(src_path: str) -> str:
        """
        Convert a markdown file to html using pandoc
        :param src_path: The path to the markdown file
        :return: The html content of the markdown file
        """
        command = ["pandoc", src_path, "-f", "markdown", "-t", "html"]
        try:
            result = subprocess.run(command, check=True, capture_output=True, text=True, encoding="utf-8")
            return result.stdout
        except subprocess.CalledProcessError:
            logger.error(f"Pandoc failed while processing {src_path}")
            exit(1)

    @staticmethod
    def strip_top_directory_in_path(path: str) -> str:
        """
        Strip the top directory in a path
        :param path: The path to strip
        :return: The stripped path
        """
        parts = path.split(os.sep)
        return "/".join(parts[1:]) if len(parts) > 1 else path

    @staticmethod
    def copy_files(src_path: str, dst_path: str) -> None:
        """
        Copy all files from a source directory to a destination directory
        :param src_path: The source directory
        :param dst_path: The destination directory
        """
        try:
            for f in glob.glob(f"{src_path}/*"):
                shutil.copy(f, dst_path)
        except Exception as e:
            logger.error(f"Failed to copy files: {str(e)}")
            exit(1)

    @staticmethod
    def post_src_to_dst_path(src_file_path: str, dst_dir: str, dst_ext: str) -> str:
        """
        Convert a source file path to a destination file path by joining the destination directory with the
        filename and the destination extension.
        :param src_file_path: The source file path
        :param dst_dir: The destination directory
        :param dst_ext: The destination extension
        :return: The converted destination file path as a string
        """
        file_name = os.path.basename(src_file_path)
        base_name, _ = os.path.splitext(file_name)
        return os.path.join(dst_dir, base_name + dst_ext)

    @staticmethod
    def replace_relative_url_with_abs_url(match: re.Match, base_url: str, folder_name: str) -> str:
        """
        Converts a relative url to an absolute url, prefixed with the base url
        :param match: The match object
        :param base_url: The base url which the other urls come from
        :param folder_name: The name of the folder which sits between the base url and the partial
        :return: The absolute url, prefixed with the base url
        """
        if match.group(1).startswith('/'):
            return urljoin(base_url, match.group(1).lstrip('/'))
        elif match.group(1).startswith('../'):
            return urljoin(base_url, match.group(1))
        else:
            return urljoin(base_url, urljoin(folder_name, match.group(1)))

    @staticmethod
    def make_urls_absolute(content: str, base_url: str, folder_name: str) -> str:
        """
        Converts all relative urls in the content to absolute urls, prefixed with the blog url
        :param content: The content in which to convert the urls
        :param base_url: The base url which the other urls come from
        :param folder_name: The name of the folder which sits between the base url and the partial
        :return: The content with all relative urls converted to absolute urls
        """
        if not content:
            return ""

        regex_pattern = r'''(?:url\(|<(?:link|a|script|img)[^>]+(?:src|href)\s*=\s*)(?!['"]?(?:data|http|https))['"]?([^'"\)\s>#]+)'''
        return re.sub(regex_pattern,
                      lambda match: Helper.replace_relative_url_with_abs_url(match, base_url, folder_name), content)


class Post:
    def __init__(self, config: BlogConfig, paths: PathConfig, src_file_path: str):
        self.config = config
        self.paths = paths

        self.title = ""
        self.description = ""
        self.date = ""
        self.tags = []

        self.md_content = ""
        self.html_content = ""

        self.src_path = src_file_path
        self.dst_path = Helper.post_src_to_dst_path(self.src_path, self.paths.dst_posts_dir_path, ".html")
        self.remote_path = Helper.strip_top_directory_in_path(self.dst_path)
        self.filename = os.path.basename(self.dst_path)

    def validate_starting_marker(self, md_data: str) -> bool:
        """
        Validate the presence and correctness of the starting marker of a markdown post
        :param md_data: The full content of the markdown post file
        :return: True if the starting marker is valid, False otherwise
        """
        if md_data.strip() != "---":
            logger.error(
                f"Failed to validate header of {self.src_path} - the starting marker \"---\" is missing or incorrect.")
            return False
        return True

    def validate_title_field(self, md_data: str) -> bool:
        """
        Validate the presence and correctness of the title field of a markdown post
        :param md_data: The full content of the markdown post file
        :return: True if the title field is valid, False otherwise
        """
        if not re.match(r'^title:\s*(\S+)', md_data):
            logger.error(
                f"Failed to validate header of {self.src_path} - the title field is missing, empty, or incorrect.")
            return False
        self.title = re.search(r'^title:\s*(.*?)\s*$', md_data).group(1)
        return True

    def validate_description_field(self, md_data: str) -> bool:
        """
        Validate the presence and correctness of the description field of a markdown post
        :param md_data: The full content of the markdown post file
        :return: True if the description field is valid, False otherwise
        """
        if not re.match(r'^description:\s*(\S+)', md_data):
            logger.error(
                f"Failed to validate header of {self.src_path} - the description field is missing, empty, or incorrect.")
            return False
        self.description = re.search(r'^description:\s*(.*?)\s*$', md_data).group(1)
        return True

    def validate_date_field(self, md_data: str) -> bool:
        """
        Validate the presence and correctness of the date field of a markdown post.
        The date field must be in the format YYYY-MM-DD.
        :param md_data: The full content of the markdown post file
        :return: True if the date field is valid, False otherwise
        """
        if not re.match(r'^date:\s*([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)', md_data):
            logger.error(
                f"Failed to validate header of {self.src_path} - the date field is missing, empty, or incorrect.")
            return False
        self.date = re.search(r'([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)', md_data).group(1)
        return True

    def validate_tags_field(self, md_data: str) -> bool:
        """
        Validate the presence and correctness of the tags field of a markdown post
        The tags field must be a comma-separated list of strings.
        :param md_data: The full content of the markdown post file
        :return: True if the tags field is valid, False otherwise
        """
        if not re.match(r'^tags:\s*(\S+)', md_data):
            logger.error(
                f"Failed to validate header of {self.src_path} - the tags field is missing, empty, or incorrect.")
            return False
        tag_values = re.search(r'^tags:\s*(.*?)\s*$', md_data).group(1)
        self.tags = [tag for tag in re.findall(r'[^,\s][^,]*[^,\s]|[^,\s]', tag_values)]
        return True

    def validate_end_marker(self, md_data: str) -> bool:
        """
        Validate the presence and correctness of the end marker of a markdown post
        :param md_data: The full content of the markdown post file
        :return: True if the end marker is valid, False otherwise
        """
        if md_data.strip() != "---":
            logger.error(
                f"Failed to validate header of {self.src_path} - the end marker \"---\" is missing or incorrect.")
            return False
        return True

    def validate_header(self) -> bool:
        """
        Validates all fields in the header of a markdown post
        :return: True if the header is valid, False otherwise
        """
        logger.debug(f"Processing {self.src_path} ...")
        with open(self.src_path, mode="r", encoding="utf-8") as f:
            md_data = f.readlines()

        # Validate all fields in the header
        if not self.validate_starting_marker(md_data[0]) or not self.validate_title_field(md_data[1]) or \
                not self.validate_description_field(md_data[2]) or not self.validate_date_field(md_data[3]) or \
                not self.validate_tags_field(md_data[4]) or not self.validate_end_marker(md_data[5]):
            return False
        return True

    def get_tags_as_html(self) -> str:
        """
        Wraps the tags of the post in html divs
        :return: The tags wrapped in html divs
        """
        tags = []
        for tag in self.tags:
            tag_name = urllib.parse.urlencode({"tag": tag})
            tag_html = f"<div class=\"tag\" onclick=\"location.href='/articles.html?{tag_name}'\">"
            tag_html += "<div class=\"tag-text\">"
            tag_html += f"{tag}"
            tag_html += "</div>"
            tag_html += "</div>"
            tags.append(tag_html)
        return "<div class=\"tags\">\n" + "\n".join(tags) + "\n</div>"

    def get_theme_toggle_if_enabled(self):
        toggle_code = "<button class=\"theme_btn\" id=\"themeToggleBtn\"> \
                <svg height=\"100%\" viewBox=\"0 0 16 16\" width=\"100%\" xmlns=\"http://www.w3.org/2000/svg\"> \
                    <path d=\"m 8 0 c -4.40625 0 -8 3.59375 -8 8 s 3.59375 8 8 8 s 8 -3.59375 8 -8 s -3.59375 -8 -8 -8 z m 0 1.941406 c 3.359375 0 6.058594 2.699219 6.058594 6.058594 s -2.699219 6.058594 -6.058594 6.058594 z m 0 0\" /> \
                </svg> \
            </button>"
        if self.config.blog_theme_can_toggle == "true":
            return toggle_code
        return ""

    def get_tags_as_meta(self) -> str:
        """
        Wraps the tags of the post in header meta tags
        :return: The tags wrapped in header meta tags
        """
        tags = []
        for tag in self.tags:
            tag_html = f"<meta property=\"og:article:tag\" content=\"{tag}\"/>"
            tags.append(tag_html)
        return "".join(tags)

    def generate(self) -> str:
        """
        Converts the markdown post to html and generates and wraps the html content in the post template
        :return: The generated post in html format wrapped in the post template
        """

        # Convert post from markdown to html
        self.html_content = Helper.pandoc_md_to_html(self.src_path)

        # Load the post template and substitute the placeholders with the actual values
        with open(os.path.join(self.paths.src_templates_dir_path, "post.html.template"), mode="r",
                  encoding="utf-8") as f:
            post_template = f.read()

        substitutions = {
            "blog_title": self.config.blog_title,
            "blog_description": self.config.blog_description,
            "blog_url": self.config.blog_url,
            "blog_theme": self.config.blog_theme,
            "author_mail": self.config.blog_author_mail,
            "author_copyright": self.config.blog_author_copyright,
            "post_title": self.title,
            "post_description": self.description,
            "post_author": self.config.blog_author_name,
            "post_date": self.date,
            "post_content": self.html_content,
            "posts_url": self.config.blog_url + self.paths.post_dir_name,
            "post_tags": self.get_tags_as_html(),
            "post_meta_tags": self.get_tags_as_meta(),
            "assets_dir": Helper.strip_top_directory_in_path(self.paths.dst_assets_dir_path),
            "meta_dir": Helper.strip_top_directory_in_path(self.paths.dst_meta_dir_path),
            "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
            "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
            "theme_toggle": self.get_theme_toggle_if_enabled()
        }
        return Template(post_template).substitute(substitutions)


class Page:

    def __init__(self, config: BlogConfig, paths: PathConfig, src_page_path: str):
        self.config = config
        self.paths = paths
        self.src_path = src_page_path
        self.dst_path = Helper.post_src_to_dst_path(src_page_path, self.paths.dst_dir_path, ".html")
        self.page_title = os.path.basename(src_page_path).split('.')[0]
        self.html_content = ""

    def get_theme_toggle_if_enabled(self):
        toggle_code = "<button class=\"theme_btn\" id=\"themeToggleBtn\"> \
                <svg height=\"100%\" viewBox=\"0 0 16 16\" width=\"100%\" xmlns=\"http://www.w3.org/2000/svg\"> \
                    <path d=\"m 8 0 c -4.40625 0 -8 3.59375 -8 8 s 3.59375 8 8 8 s 8 -3.59375 8 -8 s -3.59375 -8 -8 -8 z m 0 1.941406 c 3.359375 0 6.058594 2.699219 6.058594 6.058594 s -2.699219 6.058594 -6.058594 6.058594 z m 0 0\" /> \
                </svg> \
            </button>"
        if self.config.blog_theme_can_toggle == "true":
            return toggle_code
        return ""

    def generate(self) -> str:
        """
        Converts the markdown page to html and generates and wraps the html content in the page template
        :return: The generated page in html format
        """

        # Convert page from markdown to html
        self.html_content = Helper.pandoc_md_to_html(self.src_path)

        # Load the page template and substitute the placeholders with the actual values
        with open(os.path.join(self.paths.src_templates_dir_path, "page.html.template"), mode="r",
                  encoding="utf-8") as f:
            page_template = f.read()

        substitutions = {
            "blog_title": self.config.blog_title,
            "blog_description": self.config.blog_description,
            "blog_url": self.config.blog_url,
            "blog_theme": self.config.blog_theme,
            "author_mail": self.config.blog_author_mail,
            "author_copyright": self.config.blog_author_copyright,
            "page_title": self.page_title,
            "page_content": self.html_content,
            "assets_dir": Helper.strip_top_directory_in_path(self.paths.dst_assets_dir_path),
            "meta_dir": Helper.strip_top_directory_in_path(self.paths.dst_meta_dir_path),
            "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
            "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
            "theme_toggle": self.get_theme_toggle_if_enabled()
        }
        return Template(page_template).substitute(substitutions)


class TagsPage(Page):

    def __init__(self, config: BlogConfig, paths: PathConfig, src_page_path: str, posts: list[Post]):
        super().__init__(config, paths, src_page_path)
        self.posts = posts

    def get_post_tags_with_count_as_html(self) -> str:
        """
        Obtains all unique tags from all posts, sorts them by count and wraps them in html divs
        :return: The unique, sorted tags wrapped in html divs
        """
        unique_tags = list(set(tag for post in self.posts for tag in post.tags))
        tag_counts = {tag: sum(tag in post.tags for post in self.posts) for tag in unique_tags}
        sorted_tags = sorted(unique_tags, key=lambda tag: tag_counts[tag], reverse=True)

        tags = "<div class=\"tags\">"
        for tag in sorted_tags:
            tag_count = tag_counts[tag]
            tag_param = urllib.parse.urlencode({"tag": tag})
            tags += f"<div class=\"tag\" onclick=\"location.href='articles.html?{tag_param}'\">"
            tags += f"<div class=\"tag-text\">{tag}</div>"
            tags += f"<div class=\"tag-count\">{tag_count}</div>"
            tags += "</div>"
        tags += "</div>"
        return tags

    def get_theme_toggle_if_enabled(self):
        toggle_code = "<button class=\"theme_btn\" id=\"themeToggleBtn\"> \
                <svg height=\"100%\" viewBox=\"0 0 16 16\" width=\"100%\" xmlns=\"http://www.w3.org/2000/svg\"> \
                    <path d=\"m 8 0 c -4.40625 0 -8 3.59375 -8 8 s 3.59375 8 8 8 s 8 -3.59375 8 -8 s -3.59375 -8 -8 -8 z m 0 1.941406 c 3.359375 0 6.058594 2.699219 6.058594 6.058594 s -2.699219 6.058594 -6.058594 6.058594 z m 0 0\" /> \
                </svg> \
            </button>"
        if self.config.blog_theme_can_toggle == "true":
            return toggle_code
        return ""

    def generate(self) -> str:
        """
        Converts the markdown tags-page to html and generates and wraps the html content in the page template
        :return: The generated page in html format
        """
        # Convert page from markdown to html
        self.html_content = Helper.pandoc_md_to_html(self.src_path)

        # Load the page template and substitute the placeholders with the actual values
        with open(os.path.join(self.paths.src_templates_dir_path, "page.html.template"), mode="r",
                  encoding="utf-8") as f:
            tags_page_template = f.read()

        # Get tags from posts, sorted by count and convert them to html
        tags_html = self.get_post_tags_with_count_as_html()

        substitutions = {
            "blog_title": self.config.blog_title,
            "blog_description": self.config.blog_description,
            "blog_url": self.config.blog_url,
            "blog_theme": self.config.blog_theme,
            "author_mail": self.config.blog_author_mail,
            "author_copyright": self.config.blog_author_copyright,
            "page_title": "Tags",
            "page_content": self.html_content + tags_html,
            "assets_dir": Helper.strip_top_directory_in_path(self.paths.dst_assets_dir_path),
            "meta_dir": Helper.strip_top_directory_in_path(self.paths.dst_meta_dir_path),
            "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
            "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
            "theme_toggle": self.get_theme_toggle_if_enabled()
        }
        return Template(tags_page_template).substitute(substitutions)


class ArticlesPage(Page):
    def __init__(self, config: BlogConfig, paths: PathConfig, src_page_path: str, posts: list[Post]):
        super().__init__(config, paths, src_page_path)
        self.posts = posts

    def get_article_listing_as_html(self) -> str:
        """
        Generates the html for the article listing of all posts
        :return: The html for the article listing
        """
        article_listing = "<article>"
        article_listing += "<ul class=\"articles\">"
        for post in sorted(self.posts, key=lambda x: x.date, reverse=True):
            article_listing += f'<li class=\"article-entry\" id=\"{post.filename}\">'
            article_listing += "<div class=\"article-date\">"
            article_listing += f'[{post.date}]'
            article_listing += "</div>"
            article_listing += "<div class=\"article-title\">"
            article_listing += f'<a href=\"{post.remote_path}\">{post.title}</a>'
            article_listing += "</div>"
            article_listing += f'</li>'
        article_listing += "</ul>"
        article_listing += "</article>"

        return article_listing

    def get_theme_toggle_if_enabled(self):
        toggle_code = "<button class=\"theme_btn\" id=\"themeToggleBtn\"> \
                <svg height=\"100%\" viewBox=\"0 0 16 16\" width=\"100%\" xmlns=\"http://www.w3.org/2000/svg\"> \
                    <path d=\"m 8 0 c -4.40625 0 -8 3.59375 -8 8 s 3.59375 8 8 8 s 8 -3.59375 8 -8 s -3.59375 -8 -8 -8 z m 0 1.941406 c 3.359375 0 6.058594 2.699219 6.058594 6.058594 s -2.699219 6.058594 -6.058594 6.058594 z m 0 0\" /> \
                </svg> \
            </button>"
        if self.config.blog_theme_can_toggle == "true":
            return toggle_code
        return ""

    def generate(self) -> str:
        """
        Converts the markdown articles-page to html and generates and wraps the html content in the page template
        :return: The generated page in html format
        """
        # Convert page from markdown to html
        self.html_content = Helper.pandoc_md_to_html(self.src_path)

        # Load the page template
        template_path = os.path.join(self.paths.src_templates_dir_path, "page.html.template")
        with open(template_path, mode="r", encoding="utf-8") as f:
            articles_page_template = f.read()

        # Write the page template with the actual values
        substitutions = {
            "blog_title": self.config.blog_title,
            "blog_description": self.config.blog_description,
            "blog_url": self.config.blog_url,
            "blog_theme": self.config.blog_theme,
            "author_mail": self.config.blog_author_mail,
            "author_copyright": self.config.blog_author_copyright,
            "page_title": "Articles",
            "page_content": self.html_content + self.get_article_listing_as_html(),
            "assets_dir": Helper.strip_top_directory_in_path(self.paths.dst_assets_dir_path),
            "meta_dir": Helper.strip_top_directory_in_path(self.paths.dst_meta_dir_path),
            "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
            "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
            "theme_toggle": self.get_theme_toggle_if_enabled()
        }
        return Template(articles_page_template).substitute(substitutions)


class RSSFeed:

    def __init__(self, config: BlogConfig, paths: PathConfig, posts: list[Post]):
        self.config = config
        self.paths = paths
        self.posts = posts
        self.feed_data = ""

    def generate(self) -> None:
        """
        Formats all posts as RSS feed entries and writes the feed to a file
        """

        # Load RSS template
        template_path = os.path.join(self.paths.src_templates_dir_path, "feed.xml.template")
        logger.debug(f"Processing {template_path} ...")
        with open(template_path, mode="r", encoding="utf-8") as f:
            rss_template = f.read()

        # Create a feed entry for each post
        for post in self.posts:
            post_title = html.escape(post.title)
            post_link = urljoin(self.config.blog_url, post.remote_path)
            post_content = html.escape(
                Helper.make_urls_absolute(post.html_content, self.config.blog_url, self.paths.post_dir_name))

            self.feed_data += f"<item>"
            self.feed_data += f"<title>{post_title}</title>"
            self.feed_data += f"<link>{post_link}</link>"
            self.feed_data += f"<description>{post_content}</description>"
            self.feed_data += f"</item>"

        # Substitute the placeholders with the actual values
        substitutions = {
            "blog_title": self.config.blog_title,
            "blog_url": self.config.blog_url,
            "blog_description": self.config.blog_description,
            "rss_items": self.feed_data,
        }
        rss_template = Template(rss_template).substitute(substitutions)

        # Write substituted template to file
        feed_path = os.path.join(self.paths.dst_dir_path, "feed.xml")
        with open(feed_path, mode="w", encoding="utf-8") as f:
            f.write(rss_template)


class Sitemap:

    def __init__(self, config: BlogConfig, paths: PathConfig, posts: list[Post]):
        self.config = config
        self.paths = paths
        self.posts = posts
        self.feed_data = ""

    def generate(self) -> None:
        """
        Formats all posts as Sitemap entries and writes the feed to a file
        """

        # Load Sitemap template
        template_path = os.path.join(self.paths.src_templates_dir_path, "sitemap.xml.template")
        logger.debug(f"Processing {template_path} ...")
        with open(template_path, mode="r", encoding="utf-8") as f:
            sitemap_template = f.read()

        lastmod = datetime.date.today().strftime("%Y-%m-%d")

        site_paths = [urljoin(self.config.blog_url, post.remote_path) for post in self.posts]
        site_paths.append(urljoin(self.config.blog_url, "index.html"))
        site_paths.append(urljoin(self.config.blog_url, "articles.html"))
        site_paths.append(urljoin(self.config.blog_url, "tags.html"))
        site_paths.append(urljoin(self.config.blog_url, "about.html"))

        # Create a feed entry for each post
        for site_path in site_paths:
            self.feed_data += f"<url>"
            self.feed_data += f"<loc>{site_path}</loc>"
            self.feed_data += f"<lastmod>{lastmod}</lastmod>"
            self.feed_data += f"</url>"

        # Substitute the placeholders with the actual values
        substitutions = {
            "sitemap_items": self.feed_data,
        }
        sitemap_template = Template(sitemap_template).substitute(substitutions)

        # Write substituted template to file
        sitemap_path = os.path.join(self.paths.dst_dir_path, "sitemap.xml")
        with open(sitemap_path, mode="w", encoding="utf-8") as f:
            f.write(sitemap_template)


class Robots:

    def __init__(self, config: BlogConfig, paths: PathConfig):
        self.config = config
        self.paths = paths

    def generate(self) -> None:
        """
        Generates a robots.txt file in the destination
        """

        # Load Robots template
        template_path = os.path.join(self.paths.src_templates_dir_path, "robots.txt.template")
        logger.debug(f"Processing {template_path} ...")
        with open(template_path, mode="r", encoding="utf-8") as f:
            robots_template = f.read()

        # Substitute the placeholders with the actual values
        substitutions = {
            "blog_url": self.config.blog_url,
        }
        robots_template = Template(robots_template).substitute(substitutions)

        # Write substituted template to file
        sitemap_path = os.path.join(self.paths.dst_dir_path, "robots.txt")
        with open(sitemap_path, mode="w", encoding="utf-8") as f:
            f.write(robots_template)


class Blog:

    def __init__(self, config: BlogConfig, paths: PathConfig):
        self.config = config
        self.paths = paths
        self.posts = []
        self.pages = []
        self.processed_posts = 0
        self.skipped_posts = 0

        if not shutil.which("pandoc"):
            logger.error("Pandoc is not installed. Exiting...")

    def generate(self) -> None:
        """
        Generates the blog, i.e. creates the build directory, copies all files to the build directory, processes all
        posts and pages and generates the rss feed
        """
        logger.debug("Loading configurations...")
        self.load_configuration()

        logger.debug("Creating build directories and copying files...")
        self.clean_build_directory()
        self.create_build_directories()
        self.copy_files_to_build_directories()
        logger.info("Processing posts...")
        self.process_posts()
        logger.info("Processing pages...")
        self.process_pages()
        logger.info("Processing scripts...")
        self.process_scripts()
        logger.info("Processing rss feed...")
        self.process_rss_feed()
        logger.info("Processing favicon...")
        self.process_favicon()
        logger.info("Processing manifest...")
        self.process_manifest()
        logger.info("Processing sitemap...")
        self.process_sitemap()
        logger.info("Processing robots...")
        self.process_robots()

    def load_configuration(self) -> None:
        path = "mublog.ini"

        parser = configparser.ConfigParser()
        _ = parser.read(path, encoding="utf-8")

        if len(parser.sections()) == 0:
            logger.error("No configuration sections were loaded")
            raise FileNotFoundError(path)

        if "mublog" not in parser:
            logger.error("mublog configuration section was not found")
            raise FileNotFoundError(path)

        section = parser["mublog"]

        self.config.blog_url = section["blog_url"]
        self.config.blog_title = section["blog_title"]
        self.config.blog_description = section["blog_description"]
        self.config.blog_author_name = section["blog_author_name"]
        self.config.blog_author_mail = section["blog_author_mail"]
        self.config.post_ignore_prefix = section["post_ignore_prefix"]
        self.config.blog_author_copyright = section["blog_author_copyright"]
        self.config.blog_theme = section["blog_theme"]
        self.config.blog_theme_can_toggle = section["blog_theme_can_toggle"]

    def clean_build_directory(self) -> None:
        """
        Removes the build directory and all its contents
        """
        try:
            shutil.rmtree(self.paths.dst_dir_path, ignore_errors=True)
        except Exception as e:
            logger.error(f"Failed to remove old build directory: {str(e)}")

    def create_build_directories(self) -> None:
        """
        Creates the build directory and all subdirectories
        """
        directories = [
            self.paths.dst_dir_path,
            self.paths.dst_posts_dir_path,
            self.paths.dst_css_dir_path,
            self.paths.dst_assets_dir_path,
            self.paths.dst_meta_dir_path,
            self.paths.dst_js_dir_path,
        ]
        for directory in directories:
            try:
                os.makedirs(directory, exist_ok=True)
            except Exception as e:
                logger.error(f"Failed to create directory: {str(e)}")
                exit(1)

    def copy_files_to_build_directories(self) -> None:
        """
        Copies css and assets from the src directory to the build directory
        """
        Helper.copy_files(self.paths.src_css_dir_path, self.paths.dst_css_dir_path)
        Helper.copy_files(self.paths.src_meta_dir_path, self.paths.dst_meta_dir_path)
        Helper.copy_files(self.paths.src_assets_dir_path, self.paths.dst_assets_dir_path)

    def process_posts(self) -> None:
        """
        Processes all posts, i.e. validates the post header, generates the post html and writes the post to a file
        """
        for file_path in glob.glob(os.path.join(self.paths.src_posts_dir_path, "*.md")):
            # Skip posts that start with the ignore prefix
            if os.path.basename(file_path).startswith(self.config.post_ignore_prefix):
                logger.warning(f"Skipping {file_path} ...")
                self.skipped_posts += 1
                continue

            # Validate and generate the post
            post = Post(self.config, self.paths, file_path)
            if post.validate_header():
                with open(post.dst_path, mode="w", encoding="utf-8") as f:
                    f.write(post.generate())
                self.processed_posts += 1
                self.posts.append(post)
            else:
                logger.error(f"Post validation failed. Exiting...")
                exit(1)

    def process_pages(self) -> None:
        """
        Processes all pages, generates the page html and writes the page to a file
        """
        for file_path in glob.glob(os.path.join(self.paths.src_dir_path, "*.md")):
            logger.debug(f"Processing {file_path} ...")
            # ToDo: Add header to pages, e.g. to set the title
            # ToDo: Add page header validation

            # Create page of the appropriate type
            if os.path.basename(file_path) == "articles.md":
                page = ArticlesPage(self.config, self.paths, file_path, self.posts)
            elif os.path.basename(file_path) == "tags.md":
                page = TagsPage(self.config, self.paths, file_path, self.posts)
            else:
                page = Page(self.config, self.paths, file_path)

            # Write the generated page to disk
            with open(page.dst_path, mode="w", encoding="utf-8") as f:
                f.write(page.generate())
            self.pages.append(page)

    def process_rss_feed(self) -> None:
        """
        Generates the rss feed
        """
        feed = RSSFeed(self.config, self.paths, self.posts)
        feed.generate()

    def process_scripts(self) -> None:
        """
        Processes all scripts, i.e. generates the tag mapping script
        """
        # Load the JavaScript template
        tags_template_path = os.path.join(self.paths.src_templates_dir_path, "mublog.js.template")
        logger.debug(f"Processing {tags_template_path} ...")
        with open(tags_template_path, mode="r", encoding="utf-8") as f:
            js_template = f.read()

        # Create a mapping of post filenames to tags and substitute the template placeholders with the actual values
        with open(os.path.join(self.paths.dst_js_dir_path, "tags.js"), mode="w", encoding="utf-8") as f:
            entries = [f'"{post.filename}": [{", ".join(map(repr, post.tags))}]' for post in self.posts]
            substitutions = {"tag_mapping": "\n" + ",\n".join(entries) + "\n",
                             "blog_theme": f"\"{self.config.blog_theme}\"",
                             "theme_can_toggle": f"\"{self.config.blog_theme_can_toggle}\""}
            f.write(Template(js_template).substitute(substitutions))

    def process_favicon(self) -> None:
        """
        Processes the site's Favicon, if present.
        """
        icon_path = os.path.join(self.paths.src_meta_dir_path, "favicon.ico")
        icon_exists = os.path.isfile(icon_path)

        if icon_exists:
            destination_path = os.path.join(self.paths.dst_dir_name, "favicon.ico")
            shutil.copy(icon_path, destination_path)

    def process_manifest(self) -> None:
        """
        Processes the site's manifest, if present.
        """

        manifest_path = os.path.join(self.paths.dst_assets_dir_path, "site.webmanifest")
        manifest_exists = os.path.isfile(manifest_path)

        if manifest_exists:
            destination_path = os.path.join(self.paths.dst_dir_name, "site.webmanifest")
            shutil.copy(manifest_path, destination_path)

    def process_sitemap(self) -> None:
        """
        Generates the Sitemap.xml file
        """
        sitemap = Sitemap(self.config, self.paths, self.posts)
        sitemap.generate()

    def process_robots(self) -> None:
        """
        Generates the Robots.txt file
        """
        sitemap = Robots(self.config, self.paths)
        sitemap.generate()


if __name__ == '__main__':
    # Configure logging
    start_time = time.time()
    logger = logging.getLogger('mublog')
    logger.setLevel(logging.DEBUG)
    ch = logging.StreamHandler()
    ch.setLevel(logging.DEBUG)
    ch.setFormatter(LogFormatter())
    logger.addHandler(ch)

    # Start blog generation
    blog_conf = BlogConfig()
    path_conf = PathConfig()
    blog = Blog(blog_conf, path_conf)
    blog.generate()

    # Build summary
    end_time = time.time()
    print("---------------------------------------------------------")
    logger.info(f"Posts Processed: {blog.processed_posts} | Posts Skipped: {blog.skipped_posts}")
    logger.info(f"Elapsed Time: {round(end_time - start_time, 1)} seconds.")
    logger.info("Blog generation complete.")
