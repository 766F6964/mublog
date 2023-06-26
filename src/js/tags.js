var tag_mapping = {};

function select_tag(tag) {
    console.log("------------------------");

    matching_posts= [];

    console.log("Tag %s matches these posts:", tag);
    for (var path in tag_mapping) {
        if (tag_mapping[path].includes(tag)) {
            console.log(path);
        }
    }

    // Remove tag entries
    //let tag_elems = document.getElementsByClassName("tags")[0];
    //tag_elems.remove();
    //console.log("Cleared tag elements");
}
