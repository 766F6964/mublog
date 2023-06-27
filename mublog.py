import glob
import os
import shutil
import subprocess
import re
import urllib.parse
from string import Template


class Config:
    def __init__(self):
        self.dst_root_dir = "dst"
        self.dst_posts_dir = f"{self.dst_root_dir}/posts"
        self.dst_css_dir = f"{self.dst_root_dir}/css"
        self.dst_assets_dir = f"{self.dst_root_dir}/assets"
        self.dst_js_dir = f"{self.dst_root_dir}/js"
        self.src_root_dir = "src"
        self.src_posts_dir = f"{self.src_root_dir}/posts"
        self.src_css_dir = f"{self.src_root_dir}/css"
        self.src_assets_dir = f"{self.src_root_dir}/assets"
        self.src_templates_dir = f"{self.src_root_dir}/templates"
        self.post_ignore_delim = "_"
        self.author_name = "John Doe"
        self.author_mail = "johndoe@example.com"
        self.author_copyright = f"Copyright 2023 {self.author_name}"
        self.posts = []
        self.pages = []


class Helper:
    @staticmethod
    def check_pandoc_installed():
        if not shutil.which("pandoc"):
            Logger.log_fail("Pandoc is not installed. Please install Pandoc before continuing.")

    @staticmethod
    def clean_build_directory(directory):
        try:
            shutil.rmtree(directory, ignore_errors=True)
        except Exception as e:
            Logger.log_fail(f"Failed to remove old build directory: {str(e)}")

    @staticmethod
    def create_directory(directory):
        try:
            os.makedirs(directory, exist_ok=True)
        except Exception as e:
            Logger.log_fail(f"Failed to create directory: {str(e)}")

    @staticmethod
    def copy_files(source, destination):
        try:
            for f in glob.glob(f"{source}/*"):
                shutil.copy(f, destination)
        except Exception as e:
            Logger.log_fail(f"Failed to copy files: {str(e)}")

    @staticmethod
    def read_file_contents(file_path):
        try:
            with open(file_path, 'r') as file:
                return file.readlines()
        except FileNotFoundError:
            Logger.log_fail(f"Failed to load file '{file_path}'.")

    @staticmethod
    def src_to_dst_path(src_file_path: str, dst_dir: str, dst_ext: str) -> str:
        file_name = os.path.basename(src_file_path)
        base_name, _ = os.path.splitext(file_name)
        return os.path.join(dst_dir, base_name + dst_ext)

    @staticmethod
    def writefile(path: str, contents: str):
        with open(path, "w", encoding="utf-8") as f:
            f.write(contents)

    @staticmethod
    def substitute(mapping: dict[str, str], in_path: str, out_path: str = None):
        if not out_path:
            out_path = in_path

        template_text = Helper.read_file_contents(in_path)
        template = Template("".join(template_text))
        output = template.substitute(mapping)
        Helper.writefile(out_path, output)


class Blog:

    def __init__(self, config):
        self.config = config

    def generate(self):
        Helper.check_pandoc_installed()
        self.clean_build_directory()
        self.create_output_directories()
        self.copy_files_to_dst()
        self.process_posts()
        self.process_pages()

    def clean_build_directory(self):
        Helper.clean_build_directory(self.config.dst_root_dir)

    def create_output_directories(self):
        directories = [
            self.config.dst_root_dir,
            self.config.dst_posts_dir,
            self.config.dst_css_dir,
            self.config.dst_assets_dir,
            self.config.dst_js_dir,
        ]
        for directory in directories:
            Helper.create_directory(directory)

    def copy_files_to_dst(self):
        Helper.copy_files(self.config.src_css_dir, self.config.dst_css_dir)
        Helper.copy_files(self.config.src_assets_dir, self.config.dst_assets_dir)

    def process_posts(self):
        builder = SiteBuilder(self.config)
        for file_path in glob.glob(self.config.src_posts_dir + "/*.md"):
            if not os.path.basename(file_path).startswith(self.config.post_ignore_delim):
                post = Post(self.config, file_path)
                self.config.posts.append(post)
                builder.generate_post(post)

        builder.generate_js()

    def process_pages(self):
        builder = SiteBuilder(self.config)
        for file_path in glob.glob(self.config.src_root_dir + "/*.md"):
            page = Page(self.config, file_path)
            self.config.pages.append(page)
            if page.src_path.endswith("tags.md"):
                builder.generate_tags_page(page)
            elif page.src_path.endswith("articles.md"):
                builder.generate_articles_page(page)
            else:
                builder.generate_page(page)


class Page:

    def __init__(self, config, src_file_path):
        self.config = config
        self.src_path = src_file_path
        self.dst_path = Helper.src_to_dst_path(src_file_path, "dst/", ".html")


class Post:
    def __init__(self, config, src_file_path):
        self.config = config
        self.title = ""
        self.description = ""
        self.date = ""
        self.tags = []
        self.src_path = ""
        self.dst_path = ""
        self.dst_path_remote = ""
        self.raw_file_contents = ""

        self.validate_post(src_file_path)
        self.src_path = src_file_path
        self.dst_path = Helper.src_to_dst_path(src_file_path, "dst/posts/", ".html")
        self.dst_path_remote = Helper.src_to_dst_path(src_file_path, "posts/", ".html")

    def validate_post(self, src_file_path):
        Logger.log_info(f"Processing {src_file_path} ...")
        self.raw_file_contents = Helper.read_file_contents(src_file_path)

        # Check that file is long enough to accommodate header
        if len(self.raw_file_contents) < 6:
            Logger.log_fail(f"Failed to validate header of {src_file_path}.")

        # Validation line 1: Starting marker
        if self.raw_file_contents[0].strip() != "---":
            Logger.log_fail(f"Failed to validate header of {src_file_path}")
            Logger.log_fail(f"The starting marker \"---\" is missing or incorrect")

        # Validation line 2: title field
        if not re.match(r'^title:\s*(\S+)', self.raw_file_contents[1]):
            Logger.log_fail(f'Failed to validate header of {src_file_path}')
            Logger.log_fail(f'The title field is missing, empty, or incorrect.')
        self.title = re.search(r'^title:\s*(.*?)\s*$', self.raw_file_contents[1]).group(1)

        # Validation line 3: description field
        if not re.match(r'^description:\s*(\S+)', self.raw_file_contents[2]):
            Logger.log_fail(f'Failed to validate header of {src_file_path}')
            Logger.log_fail(f'The description field is missing, empty, or incorrect.')
        self.description = re.search(r'^description:\s*(.*?)\s*$', self.raw_file_contents[2]).group(1)

        # Validation line 4: date field
        if not re.match(r'^date:\s*([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)', self.raw_file_contents[3]):
            Logger.log_fail(f'Failed to validate header of {src_file_path}')
            Logger.log_fail(f'The date field is missing, empty, or not in the correct format (YYYY-MM-DD)')
        self.date = re.search(r'([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)',
                              self.raw_file_contents[3]).group(1)

        # Validation line 5: tags field
        if not re.match(r'^tags:\s*(\S+)', self.raw_file_contents[4]):
            Logger.log_fail(f'Failed to validate header of {src_file_path}')
            Logger.log_fail(f'The tags field is missing, empty, or incorrect.')
        tag_values = re.search(r'^tags:\s*(.*?)\s*$', self.raw_file_contents[4]).group(1)
        self.tags = [tag for tag in re.findall(r'[^,\s][^,]*[^,\s]|[^,\s]', tag_values)]

        # Validation line 6: Ending marker
        if self.raw_file_contents[5].strip() != "---":
            Logger.log_fail(f"Failed to validate header of {src_file_path}")
            Logger.log_fail(f"The ending marker \"---\" is missing or incorrect")


class SiteBuilder:
    def __init__(self, config):
        self.config = config

    def load_template(self, template_path: str):
        with open(template_path, encoding="utf-8") as f:
            return f.read()

    def generate_js(self):
        js_template = self.load_template(self.config.src_templates_dir + "/tags.js.template")
        entries = [
            f'    "{post.dst_path_remote}": [{", ".join([f"{tag!r}" for tag in post.tags])}]'
            for post in self.config.posts
        ]
        mapping = "\n" + ",\n".join(entries) + "\n"
        substitutions = {"tag_mapping": mapping}
        js_data = Template(js_template).substitute(substitutions)
        Helper.writefile(self.config.dst_js_dir + "/tags.js", js_data)
        Logger.log_pass(f"Processed JS file.")

    def generate_post(self, post):
        content = self.convert_md_html_with_pandoc(post.src_path)
        post_template = self.load_template(self.config.src_templates_dir + "/post.template")
        substitutions = {
            "author_mail": self.config.author_mail,
            "author_copyright": self.config.author_copyright,
            "title": post.title,
            "content": content,
        }
        post_data = Template(post_template).substitute(substitutions)
        Helper.writefile(post.dst_path, post_data)
        Logger.log_pass(f"Successfully processed {post.src_path}")

    def generate_tags_page(self, page):
        unique_tags = list(set(tag for post in self.config.posts for tag in post.tags))
        tag_counts = {tag: sum(tag in post.tags for post in self.config.posts) for tag in unique_tags}
        sorted_tags = sorted(unique_tags, key=lambda tag: tag_counts[tag], reverse=True)

        content = "<div class=\"tags\">"
        for tag in sorted_tags:
            tag_count = tag_counts[tag]
            tag_param = urllib.parse.urlencode({"tag": tag})
            content += (
                f'<div class="tag-bubble" onclick="location.href=\'articles.html?{tag_param}\'">'
                f"{tag}<span>{tag_count}</span></div>"
            )
        content += "</div>"

        post_template = self.load_template(self.config.src_templates_dir + "/page.template")
        substitutions = {
            "author_mail": self.config.author_mail,
            "author_copyright": self.config.author_copyright,
            "title": "Blog",
            "content": content,
        }
        page_data = Template(post_template).substitute(substitutions)
        Helper.writefile(page.dst_path, page_data)
        Logger.log_pass(f"Successfully processed {page.src_path}")

    def generate_articles_page(self, page):
        content = "<article>"
        content += "<ul class=\"articles\">"
        for post in self.config.posts:
            content += (
                f'<li><b>[{post.date}]</b> <a href="{post.dst_path_remote}">{post.title}</a></li>'
            )
        content += "</ul>"
        content += "</article>"

        post_template = self.load_template(self.config.src_templates_dir + "/page.template")
        substitutions = {
            "author_mail": self.config.author_mail,
            "author_copyright": self.config.author_copyright,
            "title": "Blog",
            "content": content,
        }
        page_data = Template(post_template).substitute(substitutions)
        Helper.writefile(page.dst_path, page_data)
        Logger.log_pass(f"Successfully processed {page.src_path}")

    def generate_page(self, page):
        content = self.convert_md_html_with_pandoc(page.src_path)
        post_template = self.load_template(self.config.src_templates_dir + "/page.template")
        substitutions = {
            "author_mail": self.config.author_mail,
            "author_copyright": self.config.author_copyright,
            "title": "Blog",
            "content": content,
        }
        page_data = Template(post_template).substitute(substitutions)
        Helper.writefile(page.dst_path, page_data)
        Logger.log_pass(f"Successfully processed {page.src_path}")

    @staticmethod
    def convert_md_html_with_pandoc(src_path):
        command = ["pandoc", src_path, "-f", "markdown", "-t", "html"]
        try:
            result = subprocess.run(command, check=True, capture_output=True, text=True)
            return result.stdout
        except subprocess.CalledProcessError:
            Logger.log_fail(f"Pandoc failed while processing {src_path}")


class Logger:
    PASS = "\033[32m[PASS]\033[0m"
    FAIL = "\033[31m[FAIL]\033[0m"
    INFO = "\033[34m[INFO]\033[0m"
    WARN = "\033[33m[WARN]\033[0m"

    @staticmethod
    def log_info(message):
        print(f"{Logger.INFO} {message}")

    @staticmethod
    def log_fail(message):
        print(f"{Logger.FAIL} {message}")
        exit(1)

    @staticmethod
    def log_warn(message):
        print(f"{Logger.WARN} {message}")

    @staticmethod
    def log_pass(message):
        print(f"{Logger.PASS} {message}")


cfg = Config()
blog = Blog(cfg)
blog.generate()
