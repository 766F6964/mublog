# μblog

## Overview
The goal of μblog (pronounced /myuːblɒɡ/) is to provide a single script that allows you to generate and manage a tiny static site blog.
The μblog tool is written in shell script. It generates posts written in markdown in a beautiful, but minimalistic blog site.
Create your blog now, without having to leverage large frameworks and software stacks.

## Features
All the usual constructs, that you find in markdown:

**Tables:**

```
| Name     | Age | City      |
|----------|-----|-----------|
| John     | 25  | New York  |
| Sarah    | 30  | London    |
| Michael  | 28  | San Francisco |
```

**Text Formatting:**

Italic Text: `_italic_`

Bold Text: `**bold**`

Strike-through Text: `~~strikethrough~~`

**Lists:**
```
- Entry 1
  - Subentry 1
  - Subentry 2
- Entry 2
  - Subentry 1
    - Subsubentry 1
  - Subentry 2
- Entry 3
```

**Quotes:**

```
> This is a quote from someone.
```

**Headings:**

```
# Heading
## Subheading
### Subsubheading
```

**And more ...** 

You can also directly write HTML/CSS in the markdown file, for some more advanced features.
And obviously, you can edit the CSS file to adjust colors, look and feel of the blog.

## Building:

This blog includes some example posts, so you can get an idea about the look and feel.
To build the page simply run `./mublog.sh` in the root directory. When the script is finished, the generated files
can be found in the `dst` directory. To view the blog, you can spin up a webserver in that directory, e.g. `python3 -m http.server 8000`.
Then visit `http://0.0.0.0:9000/` in your browser.

## Structure Explanation:

- `src/` This is the folder in which you should work
- `src/*.md` The .md files in the src directory resemble the navigation tabs of your blog.
- `src/posts/` This is where all your posts go (Markdown files)
- `src/css` This contains the CSS file(s) that style the blog
- `src/assets` This is where assets that you want to reference in your blog go. (E.g. images)
- `dst/` This is the output directory, when your blog got built. Start the webserver in this directory.

Every blog post file must have a header at the very top, that specifies some metadata.
This metadata is validated during the build process, and used to ensure all posts show up with 
the correct title, and ordered by date in the article tab.
The tag and description field is currently unused.

If a post is a draft, and you dont want to include it in your build, prefix the markdown filename with an underscore.

## Dependencies:

- pandoc
