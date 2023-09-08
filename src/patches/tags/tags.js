function get_tag_parameter() {
	const urlParams = new URLSearchParams(window.location.search);
	const tag = urlParams.get('tag');

	if (tag) {
		for (const item of Object.keys(tag_mapping)) {
			const tags = tag_mapping[item];

			if (!tags.includes(tag)) {
				hide_article_listing(item);
			}
		}
	}
}

function hide_article_listing(filename) {
	elem = document.getElementById(filename);
	if (elem) {
		elem.style.display = 'none';
	}
}