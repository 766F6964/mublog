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


class Helper:

	@staticmethod
	def generate_absolute_path(blog_url, relative_path):
		if relative_path.startswith('/'):
			# Absolute path from the web-root directory
			return urljoin(blog_url, relative_path.lstrip('/'))
		elif relative_path.startswith('../'):
			# Relative path from a post
			return urljoin(blog_url, relative_path)
		else:
			# Relative path within a post
			return urljoin(blog_url, os.path.join('posts/', relative_path))

	@staticmethod
	def make_relative_urls_absolute(conf: BlogConfig, paths: PathConfig, html_input: str):

		# Information: Matches relative URLs if they are in a link, a, script or img tag.
		# For a URL to be considered relative, it can't start with http:// or https:// or data://
		# It also excludes fragment identifiers (aka #)
		regex_pattern = r'''(?:url\(|<(?:link|a|script|img)[^>]+(?:src|href)\s*=\s*)(?!['"]?(?:data|http|https))['"]?([^'"\)\s>#]+)'''
		new_html = html_input
		offset = 0
		for match in re.finditer(regex_pattern, html_input):
			relative_url = match.group(1)
			absolute_url = Helper.generate_absolute_path(conf.blog_url, relative_url)  # Is this a good way ??

			Logger.log_info(f"Convert relative URL for RSS feed: {relative_url} => {absolute_url}")

			start_index = match.start(1) + offset
			end_index = match.end(1) + offset
			new_html = new_html[:start_index] + absolute_url + new_html[end_index:]
			offset += len(absolute_url) - len(relative_url)

		return new_html

	@staticmethod
	def generate_post_tags(post_tags: list[str]) -> str:
		tags = []
		for tag in post_tags:
			tag_name = urllib.parse.urlencode({"tag": tag})
			tag_html = f"<div class=\"tag-bubble\" onclick=\"location.href='/articles.html?{tag_name}'\">{tag}</div>"
			tags.append(tag_html)
		return "<div class=\"tags\">\n" + "\n".join(tags) + "\n</div>"

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
			Logger.log_fail(
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
			Logger.log_fail(
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
			Logger.log_fail(
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
			Logger.log_fail(
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
			Logger.log_fail(
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
			Logger.log_fail(
				f"Failed to validate header of {self.src_path} - the end marker \"---\" is missing or incorrect.")
			return False
		return True

	def validate_header(self) -> bool:
		"""
		Validates all fields in the header of a markdown post
		:return: True if the header is valid, False otherwise
		"""
		Logger.log_info(f"Processing {self.src_path} ...")
		with open(self.src_path, "r") as f:
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
			tag_html = f"<div class=\"tag-bubble\" onclick=\"location.href='/articles.html?{tag_name}'\">{tag}</div>"
			tags.append(tag_html)
		return "<div class=\"tags\">\n" + "\n".join(tags) + "\n</div>"

	def generate(self) -> str:
		"""
		Converts the markdown post to html and generates and wraps the html content in the post template
		:return: The generated post in html format wrapped in the post template
		"""

		# Convert post from markdown to html
		self.html_content = Helper.convert_md_to_html(self.src_path)

		# Load the post template and substitute the placeholders with the actual values
		with open(os.path.join(self.paths.src_templates_dir_path, "post.template"), encoding="utf-8") as f:
			post_template = f.read()

		substitutions = {
			"author_mail": self.config.blog_author_mail,
			"author_copyright": self.config.blog_author_copyright,
			"post_title": self.title,
			"post_content": self.html_content,
			"post_tags": self.get_tags_as_html(),
			"css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
			"js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
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

	def generate(self) -> str:
		# Convert page from markdown to html
		self.html_content = Helper.convert_md_to_html(self.src_path)

		# Load the page template and substitute the placeholders with the actual values
		with open(os.path.join(self.paths.src_templates_dir_path, "page.template"), encoding="utf-8") as f:
			page_template = f.read()

		substitutions = {
			"author_mail": self.config.blog_author_mail,
			"author_copyright": self.config.blog_author_copyright,
			"page_title": self.page_title,
			"page_content": self.html_content,
			"css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
			"js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
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
			tags += (f'<div class="tag-bubble" onclick="location.href=\'articles.html?{tag_param}\'">'
					 f"{tag}<span>{tag_count}</span></div>")
		tags += "</div>"
		return tags

	def generate(self) -> str:
		# Convert page from markdown to html
		self.html_content = Helper.convert_md_to_html(self.src_path)

		# Load the page template and substitute the placeholders with the actual values
		with open(os.path.join(self.paths.src_templates_dir_path, "page.template"), encoding="utf-8") as f:
			tags_page_template = f.read()

		# Get tags from posts, sorted by count and convert them to html
		tags_html = self.get_post_tags_with_count_as_html()

		substitutions = {
			"author_mail": self.config.blog_author_mail,
			"author_copyright": self.config.blog_author_copyright,
			"page_title": "Tags",
			"page_content": self.html_content + tags_html,
			"css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
			"js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
		}
		return Template(tags_page_template).substitute(substitutions)


class ArticlesPage(Page):
	def __init__(self, config: BlogConfig, paths: PathConfig, src_page_path: str, posts: list[Post]):
		super().__init__(config, paths, src_page_path)
		self.posts = posts

	def get_article_listing_as_html(self) -> str:
		article_listing = "<article>"
		article_listing += "<ul class=\"articles\">"
		for post in self.posts:
			article_listing += f'<li id=\"{post.filename}\">'
			article_listing += f'<b>[{post.date}]</b> '
			article_listing += f'<a href=\"{post.remote_path}\">{post.title}</a>'
			article_listing += f'</li>'
		article_listing += "</ul>"
		article_listing += "</article>"

		return article_listing

	def generate(self) -> str:
		# Convert page from markdown to html
		self.html_content = Helper.convert_md_to_html(self.src_path)

		# Load the page template and substitute the placeholders with the actual values
		with open(os.path.join(self.paths.src_templates_dir_path, "page.template"), encoding="utf-8") as f:
			articles_page_template = f.read()

		articles_html = self.get_article_listing_as_html()
		substitutions = {
			"author_mail": self.config.blog_author_mail,
			"author_copyright": self.config.blog_author_copyright,
			"page_title": "Articles",
			"page_content": self.html_content + articles_html,
			"css_dir": Helper.strip_top_directory_in_path(self.paths.dst_css_dir_path),
			"js_dir": Helper.strip_top_directory_in_path(self.paths.dst_js_dir_path),
		}
		return Template(articles_page_template).substitute(substitutions)


class Blog:

	def __init__(self, config: BlogConfig, paths: PathConfig):
		self.config = config
		self.paths = paths
		self.posts = []
		self.pages = []

		if not shutil.which("pandoc"):
			Logger.log_fail("Pandoc is not installed. Exiting...")

	def generate(self) -> None:
		self.clean_build_directory()
		self.create_build_directories()
		self.copy_files_to_build_directories()
		self.process_posts()
		self.process_pages()
		self.generate_js()
		self.generate_rss_feed()

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
			# Skip posts that start with the ignore prefix
			if os.path.basename(file_path).startswith(self.config.post_ignore_prefix):
				continue

			# Validate and generate the post
			post = Post(self.config, self.paths, file_path)
			if post.validate_header():
				with open(post.dst_path, "w", encoding="utf-8") as f:
					f.write(post.generate())
				self.posts.append(post)
			else:
				Logger.log_fail(f"Failed to process post {file_path}")

	def process_pages(self) -> None:
		# TODO: Add header to pages, e.g. to set the title
		for file_path in glob.glob(os.path.join(self.paths.src_dir_path, "*.md")):
			# ToDo: Move the logic to distinguish page types to the Page class
			# ToDo: Add page header validation

			# Create correct page type
			if os.path.basename(file_path) == "articles.md":
				page = ArticlesPage(self.config, self.paths, file_path, self.posts)
			elif os.path.basename(file_path) == "tags.md":
				page = TagsPage(self.config, self.paths, file_path, self.posts)
			else:
				page = Page(self.config, self.paths, file_path)

			# Generate page
			with open(page.dst_path, "w", encoding="utf-8") as f:
				f.write(page.generate())
			self.pages.append(page)

	def generate_js(self) -> None:
		# Load the rss template and substitute the placeholders with the actual values
		with open(os.path.join(self.paths.src_templates_dir_path, "tags.js.template"), encoding="utf-8") as f:
			js_template = f.read()

		entries = [f'"{post.filename}": [{", ".join(map(repr, post.tags))}]' for post in self.posts]
		mapping = "\n" + ",\n".join(entries) + "\n"
		substitutions = {"tag_mapping": mapping}
		js_data = Template(js_template).substitute(substitutions)
		Helper.writefile(os.path.join(self.paths.dst_js_dir_path, "tags.js"), js_data)
		Logger.log_pass(f"Processed JavaScript file.")

	def generate_rss_feed(self):
		# TODO: Implement rss class in a similar way as the page class

		# Load the rss template and substitute the placeholders with the actual values
		with open(os.path.join(self.paths.src_templates_dir_path, "rss.xml.template"), encoding="utf-8") as f:
			rss_template = f.read()

		rss_items = ""
		for post in self.posts:
			# Replace relative urls with absolute URLs
			abs_urls = Helper.make_relative_urls_absolute(self.config, self.paths, post.html_content)

			# Generate rss item
			rss_items += "<item>\n"
			rss_items += f"<title>{html.escape(post.title)}</title>\n"
			link = self.config.blog_url + Helper.strip_top_directory_in_path(
				self.paths.dst_posts_dir_path) + post.filename
			rss_items += f"<link>{link}</link>\n"
			rss_items += f"<description>{html.escape(abs_urls)}</description>\n"
			rss_items += "</item>\n"

		substitutions = {
			"blog_title": self.config.blog_title,
			"blog_url": self.config.blog_url,
			"blog_description": self.config.blog_description,
			"rss_items": rss_items
		}
		feed = Template(rss_template).substitute(substitutions)
		Helper.writefile(os.path.join(self.paths.dst_dir_path, "feed.xml"), feed)
		Logger.log_pass(f"Successfully generated rss feed")


if __name__ == '__main__':
	blog_conf = BlogConfig()
	path_conf = PathConfig()

	blog = Blog(blog_conf, path_conf)
	blog.generate()
