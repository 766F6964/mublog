var tag_mapping = {};

function select_tag(tag) {

    console.log("Tag %s matches these posts:", tag);
    for (var path in tag_mapping) {
        if (tag_mapping[path].includes(tag)) {
            console.log(path);
        }
    }
}
