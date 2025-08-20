use regex::Regex;

pub fn matches_d0(text: &str) -> bool {
    let pattern = r"^(d0dlwlikvh3ypudlenvxe9pmkxdd0xczo6qcfyk50cpnv|d0jzbdjek5qidkbuwhax8wdjjvo0wl6r)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
