#!/bin/bash

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
#    Generates the build directories for the blog.
#    The /dst directory is the root directory of the blog
#    The /dst/posts directory contains all the blog post files
#    The /dst/assets directory stores images, videos etc of posts
#    The /dst/css directory contains the style sheets of the blog
initialize_directories() {
    out_root_dir="dst"
    out_posts_dir="${out_root_dir}/posts"
    out_css_dir="${out_root_dir}/css"
    out_assets_dir="${out_root_dir}/assets"
    
    if mkdir -p "$out_root_dir" &&
        mkdir -p "$out_posts_dir" &&
        mkdir -p "$out_css_dir" &&
        mkdir -p "$out_assets_dir"; then
        echo "Directories initialized."
    else
        echo "Failed to create build directories. Aborting."
        exit 1
    fi
}

load_config
initialize_directories