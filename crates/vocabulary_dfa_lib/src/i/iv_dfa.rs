use regex::Regex;

pub fn matches_iv(text: &str) -> bool {
    let pattern = r"^(iv1|ivborw0kggoaaaansuheugaaaaeaaaabcayaaaaffcsjaaaaduleqvr42mnk|ivborw0kggoaaaansuheugaaaaeaaaabcayaaaaffcsjaaaaduleqvr42mnkyphfdwachwga60e6kgaaaabjru5erkjggg|ivborw0kggoaaaansuheugaaaaeaaaabcayaaaaffcsjaaaaduleqvr42mp8|ivfpq|ivfpqindexbuilder|ivg37|ivwpi5o)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
