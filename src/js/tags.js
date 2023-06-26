var tag_mapping = {
    "posts/inland_taipan.html" : ["venomous snakes","inland taipan","reptiles","australian wildlife","venomous animals"],
    "posts/giant_pacific_octopus.html" : ["marine predators","giant pacific octopus","aquatic wildlife"],
    "posts/great_white_shark.html" : ["marine predators" ,"great white shark", "aquatic wildlife","shark facts"],
    "posts/komodo_dragon.htm" : ["reptiles","monitor lizards","komodo dragon","wildlife conservation","venomous animals"],
    "posts/black_mamba.html" : ["venomous snakes","black mamba","reptiles","african wildlife","venomous animals"]
};

function get_tag_parameter() {
    const urlParams = new URLSearchParams(window.location.search);
    const tag = urlParams.get('tag');
    if (tag) {
        console.log(`Tag: ${tag}`);
        filter_by_tag(tag);
    } else {
        console.log('Tag parameter not found in the URL');
    }
}

function hide_article_listing(post_url) {
    article_list = document.getElementsByClassName("articles")[0];
    list_entries = article_list.getElementsByTagName('li');

    for (var i = 0; i < list_entries.length; i++) {
        path = list_entries[i].children[1].pathname;
        console.log("path: " + path);
        console.log("post_url: " + post_url);
        if (path.includes(post_url) == false) {
            console.log("Hiding: " + post_url);
            list_entries[i].style.display = 'none';
            break;
        }
    }
}

function filter_by_tag(tag) {
    for (var item in tag_mapping) {
      if (tag_mapping.hasOwnProperty(item)) {
        var tags = tag_mapping[item];
        for (var i = 0; i < tags.length; i++) {
          var current_tag = tags[i];
          if (current_tag == tag) {
            hide_article_listing(item);
            break;
          }
        }
      }
    }
    console.log(article_list);
}

document.addEventListener("DOMContentLoaded", get_tag_parameter);