import glob
import os
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
        self.js_dir_name = "js"
        self.css_dir_name = "css"
        self.templates_dir_name = "templates"

        # Construct local src directory paths
        self.src_dir_path = self.src_dir_name
        self.src_posts_dir_path = os.path.join(self.src_dir_path, self.post_dir_name)
        self.src_assets_dir_path = os.path.join(self.src_dir_path, self.assets_dir_name)
        self.src_css_dir_path = os.path.join(self.src_dir_path, self.css_dir_name)
        self.src_templates_dir_path = os.path.join(self.src_dir_path, self.templates_dir_name)

        # Construct local dst directory paths
        self.dst_dir_path = self.dst_dir_name
        self.dst_posts_dir_path = os.path.join(self.dst_dir_path, self.post_dir_name)
        self.dst_assets_dir_path = os.path.join(self.dst_dir_path, self.assets_dir_name)
        self.dst_css_dir_path = os.path.join(self.dst_dir_path, self.css_dir_name)
        self.dst_js_dir_path = os.path.join(self.dst_dir_path, self.js_dir_name)

        # Construct path used by webserver
        self.web_root_dir = "/"


class BlogConfig:
    def __init__(self):
        self.blog_url = "https://my-blog.com/"
        self.blog_title = "John's Awesome Blog"
        self.blog_description = "Short description what the blog is about"
        self.blog_author_name = "John Doe"
        self.blog_author_mail = "johndoe@example.com"
        self.blog_author_copyright = f"Copyright 2023 {self.blog_author_name}"
        self.post_ignore_prefix = "_"


class PathHandler:

    def __init__(self, dst_root_dir, src_root_dir, blog_url):
        self.dst_root_dir = dst_root_dir
        self.src_root_dir = src_root_dir
        self.blog_url = blog_url

    def generate_absolute_path(self, relative_path):
        if relative_path.startswith('/'):
            # Absolute path from the web-root directory
            return urljoin(self.blog_url, relative_path.lstrip('/'))
        elif relative_path.startswith('../'):
            # Relative path from a post
            return urljoin(self.blog_url, relative_path)
        else:
            # Relative path within a post
            return urljoin(self.blog_url, 'posts/' + relative_path)

    def is_text_link(self, text):
        return text.startswith('"') or text.startswith("'")

    def is_relative_path(self, path):
        absolute_prefixes = ["http://", "https://", "ftp://", "sftp://"]
        for prefix in absolute_prefixes:
            if path.startswith(prefix):
                return False
        return True

    def convert_relative_urls(self, ref_location):
        print(f"Relative URL found at: {ref_location}")

    def make_relative_urls_absolute(self, html_input: str):
        # Information: Matches relative URLs if they are in a link, a, script or img tag.
        # For a URL to be considered relative, it can't start with http:// or https:// or data://
        # It also excludes fragment identifiers (aka #)
        regex_pattern = r'''(?:url\(|<(?:link|a|script|img)[^>]+(?:src|href)\s*=\s*)(?!['"]?(?:data|http|https))['"]?([^'"\)\s>#]+)'''
        new_html = html_input
        offset = 0
        for match in re.finditer(regex_pattern, html_input):
            relative_url = match.group(1)
            absolute_url = self.generate_absolute_path(relative_url)

            Logger.log_info(f"Convert relative URL for RSS feed: {relative_url} => {absolute_url}")

            start_index = match.start(1) + offset
            end_index = match.end(1) + offset
            new_html = new_html[:start_index] + absolute_url + new_html[end_index:]
            offset += len(absolute_url) - len(relative_url)

        return new_html


class Helper:

    @staticmethod
    def generate_post_tags(post) -> str:
        tags = "<div class=\"tags\">\n"
        for tag in post.tags:
            tag_name = urllib.parse.urlencode({"tag": tag})
            tags += (
                f'<div class="tag-bubble" onclick="location.href=\'/articles.html?{tag_name}\'">'
                f"{tag}</div>\n"
            )
        tags += "</div>"
        return tags

    @staticmethod
    def convert_md_to_html(src_path: str) -> str:
        command = ["pandoc", src_path, "-f", "markdown", "-t", "html"]
        try:
            result = subprocess.run(command, check=True, capture_output=True, text=True)
            return result.stdout
        except subprocess.CalledProcessError:
            Logger.log_fail(f"Pandoc failed while processing {src_path}")

    @staticmethod
    def strip_top_directory_in_path(path: str) -> str:
        parts = path.split(os.sep)
        return os.sep.join(parts[1:]) if len(parts) > 1 else path

    @staticmethod
    def clean_build_directory(directory: str) -> None:
        try:
            shutil.rmtree(directory, ignore_errors=True)
        except Exception as e:
            Logger.log_fail(f"Failed to remove old build directory: {str(e)}")

    @staticmethod
    def create_directory(directory: str) -> None:
        try:
            os.makedirs(directory, exist_ok=True)
        except Exception as e:
            Logger.log_fail(f"Failed to create directory: {str(e)}")

    @staticmethod
    def copy_files(src_path: str, dst_path: str) -> None:
        try:
            for f in glob.glob(f"{src_path}/*"):
                shutil.copy(f, dst_path)
        except Exception as e:
            Logger.log_fail(f"Failed to copy files: {str(e)}")

    @staticmethod
    def read_file_contents(file_path: str) -> list[str]:
        try:
            with open(file_path, 'r') as file:
                return file.readlines()
        except FileNotFoundError:
            Logger.log_fail(f"Failed to load file '{file_path}'.")

    @staticmethod
    def post_src_to_dst_path(src_file_path: str, dst_dir: str, dst_ext: str) -> str:
        file_name = os.path.basename(src_file_path)
        base_name, _ = os.path.splitext(file_name)
        return os.path.join(dst_dir, base_name + dst_ext)

    @staticmethod
    def replace_file_extension(file_path: str, ext: str) -> str:
        root, old_extension = os.path.splitext(file_path)

        if not old_extension:
            Logger.log_fail(f"The file path '{file_path}' does not have an extension to replace.")

        return root + '.' + ext.strip('.')

    @staticmethod
    def writefile(path: str, contents: str) -> None:
        with open(path, "w", encoding="utf-8") as f:
            f.write(contents)

    @staticmethod
    def substitute(mapping: dict[str, str], in_path: str, out_path: str = None) -> None:
        if not out_path:
            out_path = in_path

        template_text = Helper.read_file_contents(in_path)
        template = Template("".join(template_text))
        output = template.substitute(mapping)
        Helper.writefile(out_path, output)


class Page:

    def __init__(self, config: BlogConfig, paths: PathConfig, src_page_path: str):
        self.config = config
        self.paths = paths
        self.src_path = src_page_path
        self.dst_path = Helper.post_src_to_dst_path(src_page_path, self.paths.dst_dir_path, ".html")


class Post:
    def __init__(self, config: BlogConfig, paths: PathConfig, src_file_path: str):
        self.config = config
        self.paths = paths

        self.title = ""
        self.description = ""
        self.date = ""
        self.tags = []
        self.raw_md_contents = ""
        self.raw_html_content = ""

        self.src_path = src_file_path
        self.dst_path = Helper.post_src_to_dst_path(self.src_path, self.paths.dst_posts_dir_path, ".html")
        self.remote_path = Helper.strip_top_directory_in_path(self.dst_path)
        self.filename = os.path.basename(self.dst_path)

        self.validate_post(src_file_path)

    def validate_post(self, src_file_path: str) -> None:
        Logger.log_info(f"Processing {src_file_path} ...")
        self.raw_md_contents = Helper.read_file_contents(src_file_path)

        # Check that file is long enough to accommodate header
        if len(self.raw_md_contents) < 6:
            Logger.log_fail(f"Failed to validate header of {src_file_path}.")

        # Validation line 1: Starting marker
        if self.raw_md_contents[0].strip() != "---":
            Logger.log_fail(f"Failed to validate header of {src_file_path}\n"
                            f"       The starting marker \"---\" is missing or incorrect.")

        # Validation line 2: title field
        if not re.match(r'^title:\s*(\S+)', self.raw_md_contents[1]):
            Logger.log_fail(f"Failed to validate header of {src_file_path}\n"
                            f"       The title field is missing, empty, or incorrect.")
        self.title = re.search(r'^title:\s*(.*?)\s*$', self.raw_md_contents[1]).group(1)

        # Validation line 3: description field
        if not re.match(r'^description:\s*(\S+)', self.raw_md_contents[2]):
            Logger.log_fail(f"Failed to validate header of {src_file_path}\n"
                            f"       The description field is missing, empty, or incorrect.")
        self.description = re.search(r'^description:\s*(.*?)\s*$', self.raw_md_contents[2]).group(1)

        # Validation line 4: date field
        if not re.match(r'^date:\s*([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)', self.raw_md_contents[3]):
            Logger.log_fail(f"Failed to validate header of {src_file_path}\n"
                            f"       The date field is missing, empty, or not in the expected format (YYYY-MM-DD).")
        self.date = re.search(r'([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)',
                              self.raw_md_contents[3]).group(1)

        # Validation line 5: tags field
        if not re.match(r'^tags:\s*(\S+)', self.raw_md_contents[4]):
            Logger.log_fail(f"Failed to validate header of {src_file_path}\n"
                            f"       The tags field is missing, empty, or incorrect.")
        tag_values = re.search(r'^tags:\s*(.*?)\s*$', self.raw_md_contents[4]).group(1)
        self.tags = [tag for tag in re.findall(r'[^,\s][^,]*[^,\s]|[^,\s]', tag_values)]

        # Validation line 6: Ending marker
        if self.raw_md_contents[5].strip() != "---":
            Logger.log_fail(f"Failed to validate header of {src_file_path}\n"
                            f"       The ending marker \"---\" is missing or incorrect.")


class Blog:

    def __init__(self, config: BlogConfig, paths: PathConfig):
        self.config = config
        self.paths = paths
        self.posts = []
        self.pages = []

        if not shutil.which("pandoc"):
            Logger.log_fail("Pandoc is not installed. Exiting...")

    def generate(self) -> None:
        # builder = SiteBuilder(self.config, self.paths)

        self.clean_build_directory()
        self.create_build_directories()
        self.copy_files_to_build_directories()

        self.process_posts()
        self.generate_js()
        # self.generate_rss_feed()
        self.process_pages()

    def clean_build_directory(self) -> None:
        Helper.clean_build_directory(self.paths.dst_dir_path)

    def create_build_directories(self) -> None:
        directories = [
            self.paths.dst_dir_path,
            self.paths.dst_posts_dir_path,
            self.paths.dst_css_dir_path,
            self.paths.dst_assets_dir_path,
            self.paths.dst_js_dir_path,
        ]
        for directory in directories:
            Helper.create_directory(directory)

    def copy_files_to_build_directories(self) -> None:
        Helper.copy_files(self.paths.src_css_dir_path, self.paths.dst_css_dir_path)
        Helper.copy_files(self.paths.src_assets_dir_path, self.paths.dst_assets_dir_path)

    def process_posts(self) -> None:
        for file_path in glob.glob(os.path.join(self.paths.src_posts_dir_path, "*.md")):
            if os.path.basename(file_path).startswith(self.config.post_ignore_prefix):
                continue

            post = Post(self.config, self.paths, file_path)
            post.raw_html_content = Helper.convert_md_to_html(post.src_path)
            post_template = self.load_template(os.path.join(self.paths.src_templates_dir_path, "post.template"))

            substitutions = {
                "author_mail": self.config.blog_author_mail,
                "author_copyright": self.config.blog_author_copyright,
                "post_title": post.title,
                "post_content": post.raw_html_content,
                "post_tags": Helper.generate_post_tags(post),
                "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
                "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
            }
            post_data = Template(post_template).substitute(substitutions)
            Helper.writefile(post.dst_path, post_data)
            Logger.log_pass(f"Successfully processed {post.src_path}")
            self.posts.append(post)

    def process_pages(self) -> None:
        for file_path in glob.glob(os.path.join(self.paths.src_dir_path, "*.md")):
            page = Page(self.config, self.paths, file_path)
            self.pages.append(page)
            if page.src_path.endswith("tags.md"):
                self.generate_tags_page(page)
            elif page.src_path.endswith("articles.md"):
                self.generate_articles_page(page)
            else:
                self.generate_page(page)

    def load_template(self, template_path: str) -> str:
        try:
            with open(template_path, encoding="utf-8") as f:
                return f.read()
        except FileNotFoundError:
            Logger.log_fail(f"Template file {template_path} not found.")
        except IOError:
            Logger.log_fail(f"Failed to open template file {template_path}.")

    def generate_js(self) -> None:
        js_template = self.load_template(os.path.join(self.paths.src_templates_dir_path, "tags.js.template"))
        entries = [f'"{post.filename}": [{", ".join(map(repr, post.tags))}]' for post in self.posts]
        mapping = "\n" + ",\n".join(entries) + "\n"
        substitutions = {"tag_mapping": mapping}
        js_data = Template(js_template).substitute(substitutions)
        Helper.writefile(os.path.join(self.paths.dst_js_dir_path, "tags.js"), js_data)
        Logger.log_pass(f"Processed JavaScript file.")

    def generate_rss_feed(self):
        rss_template = self.load_template(os.path.join(self.paths.src_templates_dir_path, "rss.xml.template"))
        rss_items = ""
        for post in self.posts:
            # Replace relative urls with absolute URLs
            edited = pm.make_relative_urls_absolute(post.raw_html_content)

            rss_items += "<item>\n"
            rss_items += f"<title>{html.escape(post.title)}</title>\n"
            link = self.config.blog_url + Helper.strip_top_directory_in_path(self.config.dst_posts_dir) + post.filename
            rss_items += f"<link>{link}</link>\n"
            rss_items += f"<description>{html.escape(edited)}</description>\n"
            rss_items += "</item>\n"

        substitutions = {
            "blog_title": self.config.blog_title,
            "blog_url": self.config.blog_url,
            "blog_description": self.config.blog_description,
            "rss_items": rss_items
        }
        feed = Template(rss_template).substitute(substitutions)
        Helper.writefile(self.config.dst_root_dir + "feed.xml", feed)
        Logger.log_pass(f"Successfully generated rss feed")

    def generate_tags_page(self, page: Page) -> None:
        unique_tags = list(set(tag for post in self.posts for tag in post.tags))
        tag_counts = {tag: sum(tag in post.tags for post in self.posts) for tag in unique_tags}
        sorted_tags = sorted(unique_tags, key=lambda tag: tag_counts[tag], reverse=True)

        content = Helper.convert_md_to_html(page.src_path)
        content += "<div class=\"tags\">"
        for tag in sorted_tags:
            tag_count = tag_counts[tag]
            tag_param = urllib.parse.urlencode({"tag": tag})
            content += (
                f'<div class="tag-bubble" onclick="location.href=\'articles.html?{tag_param}\'">'
                f"{tag}<span>{tag_count}</span></div>"
            )
        content += "</div>"

        post_template = self.load_template(os.path.join(self.paths.src_templates_dir_path, "page.template"))
        substitutions = {
            "author_mail": self.config.blog_author_mail,
            "author_copyright": self.config.blog_author_copyright,
            "page_title": "Blog",
            "page_content": content,
            "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
            "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
        }
        page_data = Template(post_template).substitute(substitutions)
        Helper.writefile(page.dst_path, page_data)
        Logger.log_pass(f"Successfully processed {page.src_path}")

    def generate_articles_page(self, page: Page) -> None:
        content = Helper.convert_md_to_html(page.src_path)
        content += "<article>\n"
        content += "<ul class=\"articles\">\n"
        for post in self.posts:
            content += (
                f'<li id=\"{post.filename}\"><b>[{post.date}]</b> <a href="{post.remote_path}">{post.title}</a></li>\n'
            )
        content += "</ul>\n"
        content += "</article>"

        post_template = self.load_template(os.path.join(self.paths.src_templates_dir_path, "page.template"))
        substitutions = {
            "author_mail": self.config.blog_author_mail,
            "author_copyright": self.config.blog_author_copyright,
            "page_title": "Blog",
            "page_content": content,
            "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
            "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
        }
        page_data = Template(post_template).substitute(substitutions)
        Helper.writefile(page.dst_path, page_data)
        Logger.log_pass(f"Successfully processed {page.src_path}")

    def generate_page(self, page: Page) -> None:
        Logger.log_info(f"Generating page: {page.src_path} ...")
        content = Helper.convert_md_to_html(page.src_path)
        post_template = self.load_template(os.path.join(self.paths.src_templates_dir_path, "page.template"))
        substitutions = {
            "author_mail": self.config.blog_author_mail,
            "author_copyright": self.config.blog_author_copyright,
            "page_title": self.config.blog_title,
            "page_content": content,
            "css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
            "js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
        }
        page_data = Template(post_template).substitute(substitutions)
        Helper.writefile(page.dst_path, page_data)
        Logger.log_pass(f"Successfully processed {page.src_path}")


class SiteBuilder:
    def __init__(self, config: BlogConfig, paths: PathConfig, posts):
        self.config = config
        self.paths = paths
        self.posts = posts


class Logger:
    PASS = "\033[32m[PASS]\033[0m"
    FAIL = "\033[31m[FAIL]\033[0m"
    INFO = "\033[34m[INFO]\033[0m"
    WARN = "\033[33m[WARN]\033[0m"

    @staticmethod
    def log_info(message: str) -> None:
        print(f"{Logger.INFO} {message}")

    @staticmethod
    def log_fail(message: str) -> None:
        print(f"{Logger.FAIL} {message}")
        exit(1)

    @staticmethod
    def log_warn(message: str) -> None:
        print(f"{Logger.WARN} {message}")

    @staticmethod
    def log_pass(message: str) -> None:
        print(f"{Logger.PASS} {message}")


blog_conf = BlogConfig()
path_conf = PathConfig()

blog = Blog(blog_conf, path_conf)
blog.generate()

# print(paths.src_posts_dir_path)
# print(paths.src_assets_dir_path)
# pm = PathHandler(cfg.dst_root_dir, cfg.src_root_dir, cfg.blog_url)
