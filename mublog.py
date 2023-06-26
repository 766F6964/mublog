import glob
import os
import shutil
import subprocess
import sys
import re
from string import Template

dst_root_dir = "dst"
dst_posts_dir = f"{dst_root_dir}/posts"
dst_css_dir = f"{dst_root_dir}/css"
dst_assets_dir = f"{dst_root_dir}/assets"
dst_js_dir = f"{dst_root_dir}/js"
src_root_dir = "src"
src_posts_dir = f"{src_root_dir}/posts"
src_css_dir = f"{src_root_dir}/css"
src_assets_dir = f"{src_root_dir}/assets"
src_templates_dir = f"{src_root_dir}/templates"
src_js_dir = f"{src_root_dir}/js"
post_ignore_delim = "_"
author_name = "John Doe"
author_mail = "johndoe@example.com"
author_copyright = f"Copyright 2023 {author_name}"
posts = []
pages = []


class Mublog():

    def __init__(self):
        pass

    def generate_blog(self):
        self.initialize_environment()
        self.process_posts()
        self.process_pages()

    def initialize_environment(self):
        Utils.log_info("Setting up environment...")

        # Check if pandoc is installed
        if not shutil.which("pandoc"):
            Utils.log_fail("Pandoc is not installed. Please install Pandoc before continuing.")
            sys.exit(1)

        # Remove dst directory, to ensure clean rebuild
        try:
            shutil.rmtree(dst_root_dir, ignore_errors=True)
        except Exception as e:
            Utils.log_fail(f"Failed to remove old build directory: {str(e)}")
            sys.exit(1)

        # Create output directories
        try:
            for d in (dst_root_dir, dst_posts_dir, dst_css_dir, dst_assets_dir, dst_js_dir):
                os.makedirs(d, exist_ok=True)
        except Exception as e:
            Utils.log_fail(f"Failed to create output directories: {str(e)}")
            sys.exit(1)

        # Copy files to dst directories
        try:
            for f in glob.glob(f"{src_css_dir}/*.css"):
                shutil.copy(f, dst_css_dir)
            for f in glob.glob(f"{src_js_dir}/*.js"):
                shutil.copy(f, dst_js_dir)
            shutil.copytree(src_assets_dir, dst_assets_dir, dirs_exist_ok=True)
        except Exception as e:
            Utils.log_fail(f"Failed to copy css/js/asset files: {str(e)}")
            sys.exit(1)

        Utils.log_pass("Environment set up successfully.")

    def process_posts(self):
        # Obtain and analyze all posts
        builder = SiteBuilder()
        for file_path in glob.glob(src_posts_dir + '/*.md'):
            if not os.path.basename(file_path).startswith(post_ignore_delim):
                # Validate and convert post
                post = Post(file_path)
                posts.append(post)

                # Substitute content into template with header, footer etc
                builder.generate_post(post)

    def process_pages(self):
        builder = SiteBuilder()
        for file_path in glob.glob(src_root_dir + '/*.md'):
            page = Page(file_path)
            pages.append(page)
            if page.get_src_path().endswith("tags.md"):
                builder.generate_tags_page(page)
            elif page.get_src_path().endswith("articles.md"):
                builder.generate_articles_page(page)
            else:
                builder.generate_page(page)


class Page:

    def __init__(self, src_file_path):
        self.src_path = src_file_path
        self.dst_path = Utils.src_to_dst_path(src_file_path, "dst/", ".html")

    def get_src_path(self):
        return self.src_path

    def get_dst_path(self):
        return self.dst_path


class Post:
    def __init__(self, src_file_path):

        self.src_path = ""
        self.dst_path = ""
        self.title = ""
        self.description = ""
        self.date = ""
        self.tags = []
        self.raw_file_contents = ""

        self.validate_post(src_file_path)
        self.src_path = src_file_path
        self.dst_path = Utils.src_to_dst_path(src_file_path, "dst/posts/", ".html")
        self.dst_path_remote = Utils.src_to_dst_path(src_file_path, "posts/", ".html")

    def get_src_path(self):
        return self.src_path

    def get_dst_path(self):
        return self.dst_path

    def get_dst_path_remote(self):
        return self.dst_path_remote

    def get_title(self):
        return self.title

    def get_description(self):
        return self.description

    def get_date(self):
        return self.date

    def get_tags(self):
        return self.tags

    def validate_post(self, src_file_path):
        Utils.log_info(f"Processing {src_file_path} ...")
        self.raw_file_contents = Utils.read_file_contents(src_file_path)
        # Check that file is long enough to accomodate header
        if len(self.raw_file_contents) < 6:
            Utils.log_fail(f"Failed to validate header of {src_file_path}.")
            exit(1)
        # Validation line 1: Starting marker
        if self.raw_file_contents[0].strip() != "---":
            Utils.log_fail(f"Failed to validate header of {src_file_path}")
            Utils.log_fail(f"The starting marker \"---\" is missing or incorrect")
            exit(1)
        # Validation line 2: title field
        if not re.match(r'^title:\s*(\S+)', self.raw_file_contents[1]):
            Utils.log_fail(f'Failed to validate header of {src_file_path}')
            Utils.log_fail(f'The title field is missing, empty or incorrect.')
            exit(1)
        self.title = re.search(r'^title:\s*(.*?)\s*$', self.raw_file_contents[1]).group(1)
        # Validation line 3: description field
        if not re.match(r'^description:\s*(\S+)', self.raw_file_contents[2]):
            Utils.log_fail(f'Failed to validate header of {src_file_path}')
            Utils.log_fail(f'The description field is missing, empty or incorrect.')
            exit(1)
        self.description = re.search(r'^description:\s*(.*?)\s*$', self.raw_file_contents[2]).group(1)
        # Validation line 4: date field:
        if not re.match(r'^date:\s*([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)', self.raw_file_contents[3]):
            Utils.log_fail(f'Failed to validate header of {src_file_path}')
            Utils.log_fail(f'The date field is missing, empty or not in the correct format (YYYY-MM-DD)')
            exit(1)
        self.date = re.search(r'([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)',
                              self.raw_file_contents[3]).group(1)
        # Validation line 5: tags field
        if not re.match(r'^tags:\s*(\S+)', self.raw_file_contents[4]):
            Utils.log_fail(f'Failed to validate header of {src_file_path}')
            Utils.log_fail(f'The tags field is missing, empty or incorrect.')
        tag_values = re.search(r'^tags:\s*(.*?)\s*$', self.raw_file_contents[4]).group(1)
        for tag in re.findall(r'[^,\s][^,]*[^,\s]|[^,\s]', tag_values):
            self.tags.append(tag)
        # Validation line 6: Ending marker
        if self.raw_file_contents[5].strip() != "---":
            Utils.log_fail(f"Failed to validate header of {src_file_path}")
            Utils.log_fail(f"The ending marker \"---\" is missing or incorrect")
            exit(1)

class SiteBuilder:

    def __init__(self):
        pass

    def load_template(self, template_path: str):
        with open(template_path, encoding="utf-8") as f:
            return f.read()

    def generate_post(self, post):
        # Convert markdown to html
        content = self.convert_md_html_with_pandoc(post.get_src_path())

        # Substitute html content into post template
        post_template = self.load_template(src_templates_dir + "/post.template")
        substitutions = {
            "author_mail": author_mail,
            "author_copyright": author_copyright,
            "title": post.title,
            "content": content,
        }
        post_data = Template(post_template).substitute(substitutions)
        Utils.writefile(post.dst_path, post_data)
        Utils.log_pass(f"Successfully processed {post.get_src_path()}")

    def generate_tags_page(self, page):
        # Get unique tags, and their occurrence count
        unique_tags = list(set(tag for post in posts for tag in post.get_tags()))
        tag_counts = {tag: sum(tag in post.get_tags() for post in posts) for tag in unique_tags}
        sorted_tags = sorted(unique_tags, key=lambda tag: tag_counts[tag], reverse=True)

        # Generate article list
        content = "<div class=\"tags\">"
        for tag in sorted_tags:
            tag_count = tag_counts[tag]
            # TODO: Url-encode tag value to prevent possible command-injection vulns!
            content += f"<div class=\"tag-bubble\" onclick=\"location.href='articles.html?tag={tag}'\">{tag}<span>{tag_count}</span></div>"
        content += "</div>"

        # Substitute html content into page template
        post_template = self.load_template(src_templates_dir + "/page.template")
        substitutions = {
            "author_mail": author_mail,
            "author_copyright": author_copyright,
            "title": "Blog",
            "content": content,
        }

        page_data = Template(post_template).substitute(substitutions)
        Utils.writefile(page.get_dst_path(), page_data)
        Utils.log_pass(f"Successfully processed {page.get_src_path()}")

    def generate_articles_page(self, page):

        # Generate article list
        content = "<article>"
        content += "<ul class=\"articles\">"
        for post in posts:
            content += f"<li><b>[{post.get_date()}]</b> <a href=\"{post.get_dst_path_remote()}\">{post.get_title()}</a></li>"
        content += "</ul>"
        content += "</article>"

        # Substitute html content into page template
        post_template = self.load_template(src_templates_dir + "/page.template")
        substitutions = {
            "author_mail": author_mail,
            "author_copyright": author_copyright,
            "title": "Blog",
            "content": content,
        }

        page_data = Template(post_template).substitute(substitutions)
        Utils.writefile(page.get_dst_path(), page_data)
        Utils.log_pass(f"Successfully processed {page.get_src_path()}")

    def convert_md_html_with_pandoc(self, src_path):
        command = ["pandoc", src_path, "-f", "markdown", "-t", "html"]
        try:
            result = subprocess.run(command, check=True, capture_output=True, text=True)
            return result.stdout
        except:
            Utils.log_fail(f"Pandoc failed while processing {src_path}")
            exit(1)

    def generate_page(self, page):
        # Convert markdown to html
        content = self.convert_md_html_with_pandoc(page.get_src_path())

        # Substitute html content into page template
        post_template = self.load_template(src_templates_dir + "/page.template")
        substitutions = {
            "author_mail": author_mail,
            "author_copyright": author_copyright,
            "title": "Blog",
            "content": content,
        }

        page_data = Template(post_template).substitute(substitutions)
        Utils.writefile(page.get_dst_path(), page_data)
        Utils.log_pass(f"Successfully processed {page.get_src_path()}")

    def convert_md_html_with_pandoc(self, src_path):
        command = ["pandoc", src_path, "-f", "markdown", "-t", "html"]
        try:
            result = subprocess.run(command, check=True, capture_output=True, text=True)
            return result.stdout
        except:
            Utils.log_fail(f"Pandoc failed while processing {src_path}")
            exit(1)


class Utils():
    PASS = "\033[32m[PASS]\033[0m"
    FAIL = "\033[31m[FAIL]\033[0m"
    INFO = "\033[34m[INFO]\033[0m"
    WARN = "\033[33m[WARN]\033[0m"

    @staticmethod
    def log_info(message):
        print(f"{Utils.INFO} {message}")

    @staticmethod
    def log_fail(message):
        print(f"{Utils.FAIL} {message}")

    @staticmethod
    def log_warn(message):
        print(f"{Utils.WARN} {message}")

    @staticmethod
    def log_pass(message):
        print(f"{Utils.PASS} {message}")

    @staticmethod
    def read_file_contents(file_path):
        try:
            with open(file_path, 'r') as file:
                return file.readlines()
        except:
            Utils.log_fail(f"Failed to load file '{file_path}'.")
            exit(1)

    @staticmethod
    def src_to_dst_path(src_file_path, dst_dir, dst_ext):
        # Note: This is kinda hacky, ideally replace all the path stuff
        # with a dedicated conf class
        file_name = os.path.basename(src_file_path)
        base_name, extension = os.path.splitext(file_name)
        return dst_dir + base_name + dst_ext

    @staticmethod
    def writefile(path: str, contents: str):
        with open(path, "w", encoding="utf-8") as f:
            f.write(contents)

    @staticmethod
    def substitute(mapping: dict[str, str], in_path: str, out_path: str = None):
        if not out_path:
            out_path = in_path

        template_text = readfile(in_path)
        template = Template(template_text)
        output = template.substitute(mapping)
        writefile(out_path, output)


blog = Mublog()
blog.generate_blog()
