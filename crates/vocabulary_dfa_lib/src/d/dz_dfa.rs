use regex::Regex;

pub fn matches_dz(text: &str) -> bool {
    let pattern = r"^(dzc|dzcy|dzefghi|dzi|dzig|dzigr|dzigra|dzigrar|dzigrarr|dzn_data_generator|dzn_filename|dzn_output|dzngen|dzngenrust|dzvvxt4sasgh1cwgrzbcx2sq5yswcurqgx1y1zehtwt6)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
