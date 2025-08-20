use regex::Regex;

pub fn matches_ez(text: &str) -> bool {
    let pattern = r"^(ez|ez_quad|ez_quads|ezek|ezen|ezivyi3sv5kjwxmu77pnbrt8jmkvuqwdifllzzplven7|ezneyo9iwepnbeh22euro|ezt|ezzel|ezért)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
