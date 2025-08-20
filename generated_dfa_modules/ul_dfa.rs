use regex::Regex;

pub fn matches_ul(text: &str) -> bool {
    let pattern = r"^(ulc|ulco|ulcor|ulcorn|ulcorne|ulcorner|ulcr|ulcro|ulcrop|ule_and_linted|uleqdbxbqengri0u0g1hzrjfpibowchbwmangst1jtzwtotwymbe|uli|ulimit|ullamco|ulog|ulongaccum|ulonglong|ulp|ulparameterlen|ulps_eq|ult|ulterior|ultimate_count|ultr|ultra_condensed|ultra_expanded|ultra_fast|ultracondensed|ultraexpanded|ultralytics|ultri|ulvaluelen)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
