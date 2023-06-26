import glob
import os
import shutil
import subprocess
import sys
import re

class Mublog():

    dst_root_dir = "dst"
    dst_posts_dir = f"{dst_root_dir}/posts"
    dst_css_dir = f"{dst_root_dir}/css"
    dst_assets_dir = f"{dst_root_dir}/assets"
    dst_js_dir = f"{dst_root_dir}/js"
    src_root_dir = "src"
    src_posts_dir = f"{src_root_dir}/posts"
    src_css_dir = f"{src_root_dir}/css"
    src_assets_dir = f"{src_root_dir}/assets"
    src_js_dir = f"{src_root_dir}/js"
    post_ignore_delim = "_"

    posts = []

    author_name = "John Doe"
    author_mail = "johndoe@example.com"
    author_copyright = f"Copyright 2023 {author_name}"

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
            shutil.rmtree(self.dst_root_dir, ignore_errors=True)
        except Exception as e:
            Utils.log_fail(f"Failed to remove old build directory: {str(e)}")
            sys.exit(1)

        # Create output directories
        try:
            for d in (self.dst_root_dir, self.dst_posts_dir, self.dst_css_dir, self.dst_assets_dir, self.dst_js_dir):
                os.makedirs(d, exist_ok=True)
        except Exception as e:
            Utils.log_fail(f"Failed to create output directories: {str(e)}")
            sys.exit(1)

        # Copy files to dst directories
        try:
            for f in glob.glob(f"{self.src_css_dir}/*.css"):
                shutil.copy(f, self.dst_css_dir)
            for f in glob.glob(f"{self.src_js_dir}/*.js"):
                shutil.copy(f, self.dst_js_dir)
            shutil.copytree(self.src_assets_dir, self.dst_assets_dir, dirs_exist_ok=True)
        except Exception as e:
            Utils.log_fail(f"Failed to copy css/js/asset files: {str(e)}")
            sys.exit(1)

        Utils.log_pass("Environment set up successfully.")

    def process_posts(self):
        # Obtain and analyze all posts
        for file_path in glob.glob(self.src_posts_dir + '/*.md'):
            if not os.path.basename(file_path).startswith(self.post_ignore_delim):
                Utils.log_info(f"Processing {file_path} ...")
                self.posts.append(Post(file_path))

        # TODO: Convert posts to html
        for post in self.posts:
            print(post.title)
            
    def process_pages(self):
        pass


class Post:

    src_path = ""
    dst_path = ""
    title = ""
    description = ""
    date = ""
    tags = []
    raw_file_contents = ""

    def __init__(self, src_file_path):
        self.validate_post(src_file_path)
        self.src_path = src_file_path
        self.dst_path = Utils.src_to_dst_path(src_file_path, "dst/posts/", ".html")

    def get_src_path(self):
        return self.src_path

    def get_dst_path(self):
        return self.dst_path

    def get_title(self):
        return self.title

    def get_description(self):
        return self.description

    def get_date(self):
        return self.date

    def get_tags(self):
        return self.tags

    def validate_post(self, src_file_path):
        self.raw_file_contents = Utils.read_file_contents(src_file_path)
        # Check that file is long enough to accomodate header
        if (len(self.raw_file_contents) < 6):
            Utils.log_fail(f"Failed to validate header of {src_file_path}.")
            exit(1)
        # Validation line 1: Starting marker
        if (self.raw_file_contents[0].strip() != "---"):
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
        self.date = re.search(r'([0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$)', self.raw_file_contents[3]).group(1)
        # Validation line 5: tags field
        if not re.match(r'^tags:\s*(\S+)', self.raw_file_contents[4]):
            Utils.log_fail(f'Failed to validate header of {src_file_path}')
            Utils.log_fail(f'The tags field is missing, empty or incorrect.')
        tag_values = re.search(r'^tags:\s*(.*?)\s*$', self.raw_file_contents[4]).group(1)
        for tag in re.findall(r'[^,\s][^,]*[^,\s]|[^,\s]', tag_values):
            self.tags.append(tag)
        # Validation line 6: Ending marker
        if (self.raw_file_contents[5].strip() != "---"):
            Utils.log_fail(f"Failed to validate header of {src_file_path}")
            Utils.log_fail(f"The ending marker \"---\" is missing or incorrect")
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


blog = Mublog()
blog.generate_blog()
