#!/usr/bin/env bash

declare -A post_info
declare -a posts

dst_root_dir="dst"
dst_posts_dir="${dst_root_dir}/posts"
dst_css_dir="${dst_root_dir}/css"
dst_assets_dir="${dst_root_dir}/assets"
src_root_dir="src"
src_posts_dir="${src_root_dir}/posts"
src_css_dir="${src_root_dir}/css"
src_assets_dir="${src_root_dir}/assets"
post_ignore_delim="_"
author_name="John Doe"
author_mail="johndoe@example.com"
footer_copyright="Copyright 2023 $author_name"

PASS="\e[32m[PASS]\e[0m"
FAIL="\e[31m[FAIL]\e[0m"
INFO="\e[34m[INFO]\e[0m"
WARN="\e[33m[WARN]\e[0m"

# Description:
#    Checks if pandoc is installed on the system.
#    Removes old build artefacts, and generates the build directories
#    The /dst directory is the root directory of the blog
#    The /dst/posts directory contains all the blog post files
#    The /dst/assets directory stores images, videos etc of posts
#    The /dst/css directory contains the style sheets of the blog
initialize() {

    # Make sure pandoc is installed on the system
    if ! command -v pandoc &> /dev/null; then
        echo -e "$FAIL Pandoc is not installed. Please install Pandoc before continuing." && exit 1
    fi

    echo -e "$INFO Initializing build directories ..."
    rm -rf "$dst_root_dir"

    # Create output directories
    if mkdir -p "$dst_root_dir" &&
        mkdir -p "$dst_posts_dir" &&
        mkdir -p "$dst_css_dir" &&
        mkdir -p "$dst_assets_dir" &&
        cp "$src_css_dir"/*.css "$dst_css_dir" &&
        cp -r "$src_assets_dir/." "$dst_assets_dir/"; then
        echo -e "$PASS Build directories initialized."
    else
        echo -e "$FAIL Failed to create build directories. Aborting."
        exit 1
    fi
}


# Description:
#     Checks that the start-marker "---" is present in the 1st line of the src post file metadata.
# Paramters:
#     $1: The path to the src post file to validate
function validate_start_marker() {
    local marker1_line=$(sed -n '1p' "$1")
    marker1=$(echo "$marker1_line" | sed 's/^---[[:space:]]*$/---/; t; s/.*//')
    if [ -z "$marker1" ]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The starting marker \"---\" is missing or incorrect" && exit 1
    fi
}

# Description:
#     Checks that the end-marker "---" is present in the 6th line of the src post file metadata.
# Paramters:
#     $1: The path to the src post file to validate
function validate_end_marker() {
    local marker2_line=$(sed -n '6p' "$1")
    marker2=$(echo "$marker2_line" | sed 's/^---[[:space:]]*$/---/; t; s/.*//')
    if [ -z "$marker2" ]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The ending marker \"---\" is missing or incorrect" && exit 1
    fi
}

# Description:
#     Checks that the title field is present in the 2nd line of the src post file metadata.
#     Also validates that the title field does not contain illegal characters.
# Paramters:
#     $1: The path to the src post file to validate
function validate_title() {
    local title_line=$(sed -n '2p' "$1")
    title=$(echo "$title_line" | sed -n 's/^title:\s*\(.*\)\s*$/\1/p')
    if [[ $title == *"|"* ]]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The \"|\" character is not allowed in the title field." && exit 1
    elif [ -z "$title" ]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The title field is missing or incorrect." && exit 1
    fi
}

# Description:
#     Checks that the description field is present in the 3rd line of the src post file metadata.
#     Also validates that the description field does not contain illegal characters.
# Paramters:
#     $1: The path to the src post file to validate
function validate_description() {
    local description_line=$(sed -n '3p' "$1")
    description=$(echo "$description_line" | sed -n 's/^description:\s*\(.*\)\s*$/\1/p')
    if [[ $description == *"|"* ]]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The \"|\" character is not allowed in the description field." && exit 1
    elif [ -z "$description" ]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The description field missing or incorrect" && exit 1
    fi
}

# Function to validate the date field
# Description:
#     Checks that the date field is present in the 4th line of the src post file metadata.
#     Also validates that the date is a valid date in the YYYY-MM-DD format.
# Paramters:
#     $1: The path to the src post file to validate
function validate_date() {
    local date_line=$(sed -n '4p' "$1")
    date=$(echo "$date_line" | sed -n 's/^date:\s*\(.*\)\s*$/\1/p')
    local regex='^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$'
    date=$(echo "$date" | grep -P "$regex" | awk '{print $1}')
    if [ -z "$date" ]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The date field is missing, incorrect, or in the wrong format (required: YYYY-MM-DD)." && exit 1
    fi
}

# Description:
#     Checks that the description field is present in the 3rd line of the src post file metadata.
#     Also validates that the description field does not contain illegal characters.
# Paramters:
#     $1: The path to the src post file to validate
function validate_tags() {
    local tags_line=$(sed -n '5p' "$1")
    tags=$(echo "$tags_line" | sed -n 's/^tags:\s*\(.*\)\s*$/\1/p')
    if [[ $tags == *"|"* ]]; then
        echo -e "$FAIL Failed to validate header of $1"
        echo -e "$FAIL The \"|\" character is not allowed in the tags field." && exit 1
    elif [ -z "$tags" ]; then
        echo -e "$FAIL Failed to validate $1"
        echo -e "$FAIL The tags field is missing or incorrect" && exit 1
    fi
}

# Description:
#     Verifies presence and validity of the header fields line by line.
#     If a field is not present, the validation fails.
#     Leading and trailing whitespaces will be stripped, if present.
# Parameters:
#     $1: The path to the src post file to validate
function validate_header() {
    echo -e "$INFO Validating post $1 ..."
    validate_start_marker "$1"
    validate_end_marker "$1"
    validate_title "$1"
    validate_description "$1"
    validate_date "$1"
    validate_tags "$1"
    echo -e "$PASS Validated post $1"
}

# Description:
#     Converts the markdown post or page into html format using pandoc.
#     During this process, the header is prepended and the footer appended to the post.
# Parameters:
#     $1: The source path to the markdown post/page file
#     $2: The destination path where the converted html file will be saved.
build_pages() {

    local header=$(
        cat <<HERE
<!DOCTYPE html>
<html>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<link rel="stylesheet" href="/css/normalize.css" type="text/css" media="all">
<link rel="stylesheet" href="/css/style.css" type="text/css" media="all">
<nav>
<a href="/index.html">home</a>
<a href="/articles.html">articles</a>
<a href="mailto:$author_mail">mail</a>
<a href="/about.html">about</a>
</nav>
<hr>
HERE
    )

    local footer=$(
        cat <<HERE
<footer>
<hr>
<p>
$footer_copyright
<br/>
</p>
</footer>
</body>
</html>
HERE
    )

    pandoc "$1" -f markdown -t html | {
        echo -e "$header"
        cat
        echo -e "$footer"
    } >"$2"
}

# Description:
#     Iterate through all source post files, and extract values stored in their headers
#     such as date, title, but also stores source path and destination path.
# Parameters:
#     $1: The path to the source directory of the posts
process_files() {
    local src_posts_dir="$1"

    # Find all .md posts in the post directory and extract info from the headers
    while IFS= read -r -d '' src_post_path; do
        if validate_header "$src_post_path"; then
            local date
            local title
            date=$(grep -oP "(?<=date: ).*" "$src_post_path")
            title=$(grep -oP "(?<=title: ).*" "$src_post_path")

            base_name=$(basename "$src_post_path")
            local dst_post_path="${dst_posts_dir}/${base_name%.md}.html"

            posts+=("$date|$title|$src_post_path|$dst_post_path")
        fi
    done < <(find "$src_posts_dir" -name "*.md" -print0)
}

# Description:
#     Sorts posts in reverse chronological order, based on the extracted date
sort_posts() {
    IFS=$'\n' read -r -d '' -a sorted_posts < <(printf '%s\n' "${posts[@]}" | sort -r)
}

initialize
build_pages "$src_root_dir/about.md" "$dst_root_dir/about.html"
build_pages "$src_root_dir/index.md" "$dst_root_dir/index.html"
build_pages "$src_root_dir/articles.md" "$dst_root_dir/articles.html"
process_files "$src_posts_dir"
sort_posts

posts_processed=0
posts_skipped=0

article_list="<ul class=\"articles\">"
for post_info in "${sorted_posts[@]}"; do
    date=$(cut -d '|' -f 1 <<<"$post_info")
    title=$(cut -d '|' -f 2 <<<"$post_info")
    src=$(cut -d '|' -f 3 <<<"$post_info")
    dst=$(cut -d '|' -f 4 <<<"$post_info")
    dst_link=${dst#*/}
    echo -e "$INFO Processing post: $src"

    # Check if the file should be ignored (if it starts with the ignore delimter)
    filename=$(basename "$src")
    if [[ $filename == $post_ignore_delim* ]]; then
        posts_skipped=$((posts_skipped + 1))
        echo -e "$WARN Skipped post: $src"
        continue
    else
        # Build article list
        article_item="<li><b style=\"color: #14263b;\">"[${date}]"</b> <a href="\"/${dst_link}\"">${title}</a></li>"
        article_list=$article_list$article_item

        # Build post file
        build_pages "$src" "$dst"
        posts_processed=$((posts_processed + 1))

        echo -e "$PASS Processed post: $src"
    fi
done

article_list=$article_list"</ul>"

echo -e "$INFO Generating article listing ..."

# Replace article tags in the article.html file with the generated article list
sed -i -e '/<article>/ {
    N
    s|<article>\(.*\)</article>|<article>\1\n'"$(sed 's/[&/\]/\\&/g' <<<"$article_list")"'\n</article>|
}' "$dst_root_dir/articles.html"

echo "-----------------------------------------"
echo -e "$PASS Finished! (built: $posts_processed, skipped: $posts_skipped)"
