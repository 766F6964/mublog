import glob
import os
import shutil

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

    def __init__(self):
        pass

    def generate_blog(self):
        self.initialize_environment()

    def initialize_environment(self):

        # Remove dst directory, to ensure clean rebuild
        shutil.rmtree(self.dst_root_dir, ignore_errors=True)

        # Create output directories
        for d in (self.dst_root_dir, self.dst_posts_dir, self.dst_css_dir, self.dst_assets_dir, self.dst_js_dir):
            os.makedirs(d, exist_ok=True)

        # Copy files to dst directories
        for f in glob.glob(f"{self.src_css_dir}/*.css"):
            shutil.copy(f, self.dst_css_dir)
        for f in glob.glob(f"{self.src_js_dir}/*.js"):
            shutil.copy(f, self.dst_js_dir)
        shutil.copytree(self.src_assets_dir, self.dst_assets_dir, dirs_exist_ok=True)

class Utils():

    def log_info():
        pass


blog = Mublog()
blog.generate_blog()
