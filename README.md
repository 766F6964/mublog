# μblog

## Overview

The goal of μblog (pronounced /myuːblɒɡ/) is to provide a single script that allows you to generate a tiny static blog site.
The μblog tool is written in Python. It generates posts, written in Markdown, in a beautiful, but minimalistic blog site.
The generates files can directly be pushed in the webroot of your server, and your blog will be up and running.
Create your blog now, without having to leverage large web-frameworks/software stacks.

![ezgif com-video-to-gif](https://github.com/766F6964/mublog/assets/34845270/01b27f1d-7ee8-4a66-bb93-4a329fe4695d)

## Features

With μblog, you get all the essential features, you need in a blog:

- **Markdown Writing:** Writing blog posts means, simply writing markdown. This means, all the usual Markdown features
work like you would expect (Images, Tables, Listings, Text-Formatting, Quotes, Headings, Links, Anchors ...)
- **Minimal Design:** By default, μblog comes with a minimalist aesthetic. Adjusting the colors or layout is very simple, 
and can be done by editing the `style.css` file, that defines the look and feel of μblog.
- **Article Listing:** The article page of μblog gives list of all your blog posts, sorted by date.
- **Tag-Filtering:** It is possible to filter your blog posts by tags. The tags page lists all tags, of all posts,
but you can also filter by the tags that are shown at the bottom of each blog post.
- **Simple Deployment:** Deploying your blog is as simple as it can get. Just upload the files in the `dst` build directory
to your server, and your blog will be up and running.
- **And more to come:** More features will be added in the future, to make μblog even better.

## Usage

Out of the box, μblog comes with some example posts, so you can get an idea how a blog site can look like.
To build the blog files, first, make sure you have python and pandoc installed. 
Then simply run `python mublog.py` in the root directory of the repository. 
When the script is finished, the generated files can be found in the `dst` directory. 
To test the blog locally, you can spin up a webserver in that directory, 
e.g. by running `python3 -m http.server 8000`. 
Then visit `http://0.0.0.0:8000/` in your browser.

If you want to deploy the blog to your server, all you have to do is upload the generated files in the `dst` directory
to the webroot of your server.

## Structure Explanation:

To get a good grasp, which directories and files are relevant for your blog site creation, we have provided a 
comprehensive table below that explains everything.

<table>
    <thead>
        <tr>
            <th>Directory</th>
            <th>Explanation</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td><code>src/</code></td>
            <td>
                This <code>src/</code> directory is the root directory of your blog.
                It contains the main pages of your blog, such as home, articles, tags and about.
                You can modify these, if you want, or add more pages.
                Just make sure that you reference new pages in the header of the template files, otherwise they wont
                show up.
            </td>
        </tr>
        <tr>
            <td><code>src/assets/</code></td>
            <td>The <code>src/assets/</code> directory is where all the files go, you might want to reference in your blog
                posts,
                e.g. images, videos, code-snippets etc.
            </td>
        </tr>
        <tr>
            <td><code>src/css/</code></td>
            <td>
                The <code>src/css/</code> directory, contains <code>style.css</code>, the css file that defines the colors and the
                layout of the blog.
                It also contains <code>normalize.css</code>, a file that ensures that the blog will be shown correctly in all
                browsers and on mobile devices.
            </td>
        </tr>
        <tr>
            <td><code>src/meta/</code></td>
            <td>
                The <code>src/meta/</code> directory, contains all files that compose some kind of metacharacteristic for the site such as icons.
            </td>
        </tr>
        <tr>
            <td><code>src/posts/</code></td>
            <td>
                The <code>src/posts/</code> directory contains all your blog posts.
                This is probably where you will spend most of your time.
                To write a new post, refer to the next section.
            </td>
        </tr>
        <tr>
            <td><code>src/templates/</code></td>
            <td>
                The <code>src/templates/</code> directory holds templates for pages, posts, and generated JavaScript
                files.
                These templates are used to incorporate header and footer information into your content, as well as to
                insert article listings and other elements.
                Be careful when modifying these files to avoid potential issues that could break the blog site.
            </td>
        </tr>
        <tr>
            <td><code>dst/</code></td>
            <td>
                The <code>dst/</code> directory serves as the output location for all the compiled blog files.
                These files should be uploaded to your webroot in order to successfully deploy your blog.
            </td>
        </tr>
    </tbody>
</table>

## Blog configurations

The author, e-mail address, link, description and any other configuration data are stored in the `mublog.ini` file.
The script reads from this file to generate the blog so it is a good idea to update this file before you publish.

## Writing a Post

To write a new blog post, simply create a new markdown file in `src/posts/`.
Every blog post **must** start with the metadata header, formatted like so:
```
---
title: Blog Title
description: A short description of this post
date: 2023-05-30
tags: blogging,web development,writing,creativity 
---
```

After the header, you can write your post in normal Markdown.
It is also possible to inline HTML, if you prefer that, e.g. to link images or for tables.

For more information, refer to the example posts in the repo.

## Dependencies:

- python
- pandoc
