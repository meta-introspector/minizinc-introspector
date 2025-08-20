use regex::Regex;

pub fn matches_nˇ(text: &str) -> bool {
    let pattern = r"^(nˇ2|nˇ4|nˇ7|nˇ9|nˇaa|nˇabc|nˇbb|nˇccccc|nˇdd|nˇgh|nˇhi|nˇhigh|nˇijk|nˇk|nˇline|nˇllo|nˇrld|nˇth|nˇtwo|nˇworld)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
