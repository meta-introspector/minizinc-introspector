use regex::Regex;

pub fn matches_ny(text: &str) -> bool {
    let pattern = r"^(ny|nyan|nydogs|nyefghiz|nyjkl|nynorsk|nyrphzda0thrxf0loqi0|nyström|nyu|nyy|nyyyy|nyz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
