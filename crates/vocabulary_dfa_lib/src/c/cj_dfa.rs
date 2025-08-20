use regex::Regex;

pub fn matches_cj(text: &str) -> bool {
    let pattern = r"^(cjlin|cjs|cjsx|cjwvc3znpg|cjzy83ggjhqpgdq8visv3u91jdjlueaalzoobrxtnnlu)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
