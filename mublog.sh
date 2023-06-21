#!/bin/bash


out_root_dir="dst"
out_posts_dir="${out_root_dir}/posts"
out_css_dir="${out_root_dir}/css"
out_assets_dir="${out_root_dir}/assets"
src_root_dir="src"
src_posts_dir="${src_root_dir}/posts"
src_css_dir="${src_root_dir}/css"
src_assets_dir="${src_root_dir}/assets"

# Description: 
#     Reads the config file and extracts key-value entries into variables.
#     Each extracted variable is accessible as with the key name (lowercase). 
# Parameters: 
#     $1: Path to the configuration file
load_config() {
    while IFS='=' read -r key value
    do
        key=$(echo "$key" | sed 's/^[ \t]*//;s/[ \t]*$//')
        value=$(echo "$value" | sed 's/^[ \t]*//;s/[ \t]*$//')
        if [[ -z $key || $key == \#* ]]; then
            continue
        fi
        key_var=$(echo "$key" | tr '[:upper:]' '[:lower:]' | sed 's/[^[:alnum:]]/_/g')
        eval "$key_var='$value'"
    done < "mublog.conf"
}

# Description:
#    Removes old build artefacts, and generates the build directories
#    The /dst directory is the root directory of the blog
#    The /dst/posts directory contains all the blog post files
#    The /dst/assets directory stores images, videos etc of posts
#    The /dst/css directory contains the style sheets of the blog
initialize_directories() {
    
    rm -rf "$out_root_dir"

    # Create output directories
    if mkdir -p "$out_root_dir" &&
        mkdir -p "$out_posts_dir" &&
        mkdir -p "$out_css_dir" &&
        mkdir -p "$out_assets_dir"; then
        echo "Build directories initialized."
        sleep 1
    else
        echo "Failed to create build directories. Aborting."
        exit 1
    fi
}

build_pages() {
    local header="HEADER DATA"
    local footer="FOOTER DATA"

    pandoc "$1" -f markdown -t html | { echo -e "$header"; cat; echo -e "$footer"; } > "$2"
}

load_config
initialize_directories

build_pages "$src_posts_dir/komodo_dragon.md" "$out_posts_dir/komodo_dragon.html" 
build_pages "$src_posts_dir/great_white_shark.md" "$out_posts_dir/great_white_shark.html" 
build_pages "$src_posts_dir/black_mamba.md" "$out_posts_dir/black_mamba.html" 

echo "Done"