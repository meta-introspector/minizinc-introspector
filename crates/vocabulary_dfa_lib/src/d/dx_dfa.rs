use regex::Regex;

pub fn matches_dx(text: &str) -> bool {
    let pattern = r"^(dx2m8ldu4gogqae9ibytusqo|dxgi|dxgi_format|dxgi_format_a8_unorm|dxgi_format_b8g8r8a8_unorm|dxm2yvsousg1twmqghlkosreqxhturoehwxrtgpmmfwi|dxqtnuksm0sgnadfplpqcebddupmb09ny8oht8bx8wqhv9eyoblrprd7dvhousio02gbshe3p2wj7)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
