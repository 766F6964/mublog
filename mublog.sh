#!/bin/bash

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

# Description:
#    Removes old build artefacts, and generates the build directories
#    The /dst directory is the root directory of the blog
#    The /dst/posts directory contains all the blog post files
#    The /dst/assets directory stores images, videos etc of posts
#    The /dst/css directory contains the style sheets of the blog
initialize_directories() {
    
    rm -rf "$dst_root_dir"

    # Create output directories
    if mkdir -p "$dst_root_dir" &&
        mkdir -p "$dst_posts_dir" &&
        mkdir -p "$dst_css_dir" &&
        mkdir -p "$dst_assets_dir" &&
        cp "$src_css_dir"/*.css "$dst_css_dir" &&
        cp -r "$src_assets_dir/." "$dst_assets_dir/"; then
        echo "Build directories initialized."
    else
        echo "Failed to create build directories. Aborting."
        exit 1
    fi

    
}

function validate_header() {
  # Verifies presence and validity of header fields line by line. If a field 
  # is not present, or its value is not valid, the variables will be set to empty. 
  # Leading and trailing whitespaces will be stripped, if present, except 
  # for the markers, where only trailing whitespace is stripped.
  #   Line 1: Check for --- marker
  #   Line 2: Check for title-field
  #   Line 3: Check for description-field
  #   Line 4: Check for date-field with valid date in YYYY-MM-DD format
  #   Line 5: Check for tags-field
  #   Line 6: Check for --- marker

  echo "Validating post $1 ..."
  marker1=$(sed -n '1p' "$1" | sed 's/^---[[:space:]]*$/---/; t; s/.*//')
  title=$(sed -n '2p' "$1" | sed -n 's/^title:\s*\(.*\)\s*$/\1/p')
  description=$(sed -n '3p' "$1" | sed -n 's/^description:\s*\(.*\)\s*$/\1/p')
  date=$(sed -n '4p' "$1" | sed -n 's/^date:\s*\(.*\)\s*$/\1/p')
  regex='^[0-9]{4}-(0[1-9]|1[0-2])-(0[1-9]|[1-2][0-9]|3[0-1])$'
  date=$(echo "$date" | grep -P "$regex" | awk '{print $1}')
  tags=$(sed -n '5p' "$1" | sed -n 's/^tags:\s*\(.*\)\s*$/\1/p')
  marker2=$(sed -n '6p' "$1" | sed 's/^---[[:space:]]*$/---/; t; s/.*//')
  
  # Check if the header is invalid (aka, non-empty fields)
  if [ -z "$marker1" ]; then
      echo "Invalid Header: Starting markers missing or incorrect" && exit 1
  elif [ -z "$title" ]; then
      echo "Invalid Header: Title field missing or incorrect" && exit 1
  elif [ -z "$description" ]; then
    echo "Invalid Header: Description field missing or incorrect" && exit 1
  elif [ -z "$date" ]; then
      echo "Invalid Header: Date field missing, incorrect or in wrong format." && exit 1
  elif [ -z "$tags" ]; then
      echo "Invalid Header: Tags field missing or incorrect" && exit 1
  elif [ -z "$marker2" ]; then
      echo "Invalid Header: Ending marker missing or incorrect" && exit 1
  fi
}

build_pages() {

    local header="
<html>
<meta charset="utf-8">
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
<link rel=\"stylesheet\" href=\"/css/normalize.css\" type=\"text/css\" media=\"all\">
<link rel=\"stylesheet\" href=\"/css/style.css\" type=\"text/css\" media=\"all\">
<nav>
<a href=\"/index.html\">home</a>
<a href=\"/articles.html\">articles</a>
<a href=\"mailto:johndoe@mail.com\">mail</a>
<a href=\"/about.html\">about</a>
</nav>
<hr>"
    local footer="
</main>
<footer>
<hr>
<p>
Copyright &copy; 2023 John Doe<br />
</p>
</footer>
</body>
</html>"

    pandoc "$1" -f markdown -t html | { echo -e "$header"; cat; echo -e "$footer"; } > "$2"
}



process_files() {
    local src_posts_dir="$1"

    # Find all .md posts in the post directory and extract their information
    while IFS= read -r -d '' src_post_path; do
        if validate_header "$src_post_path"; then
            local date=$(grep -oP "(?<=date: ).*" "$src_post_path")
            local title=$(grep -oP "(?<=title: ).*" "$src_post_path")

            base_name=$(basename "$src_post_path")
            local dst_post_path="${dst_posts_dir}/${base_name%.md}.html"

            posts+=("$date|$title|$src_post_path|$dst_post_path")
        fi
    done < <(find "$src_posts_dir" -name "*.md" -print0)
}

sort_posts() {
    IFS=$'\n' sorted_posts=($(sort -r <<<"${posts[*]}"))
    unset IFS
}

initialize_directories

echo "Building home page ..."
build_pages "$src_root_dir/about.md" "$dst_root_dir/about.html"
build_pages "$src_root_dir/index.md" "$dst_root_dir/index.html"
build_pages "$src_root_dir/articles.md" "$dst_root_dir/articles.html"
echo "Building posts ..."
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
    echo "Processing post: $src"
    echo "  title: $title"
    echo "  date: $date"
    echo "  output: $dst"
    echo "  dst_link: $dst_link"
    
    # Check if the file should be ignored (if it starts with the ignore delimter)
    filename=$(basename "$src")
    if [[ $filename == $post_ignore_delim* ]]; then
        posts_skipped=$(($posts_skipped+1))
        continue
    else
        # Build article list
        article_item="<li><b style=\"color: #58A6FF;\">"[${date}]"</b> <a href="\"/${dst_link}\"">${title}</a></li>"
        article_list=$article_list$article_item

        # Build post file
        build_pages "$src" "$dst"
        posts_processed=$(($posts_processed+1))
    fi
done

article_list=$article_list"</ul>"

echo "Generating article listing ..."
sed -i -e '/<article>/ {
    N
    s|<article>\(.*\)</article>|<article>\1\n'"$(sed 's/[&/\]/\\&/g' <<< "$article_list")"'\n</article>|
}' "$dst_root_dir/articles.html"

echo "Finished! (built: $posts_processed, skipped: $posts_skipped)"
