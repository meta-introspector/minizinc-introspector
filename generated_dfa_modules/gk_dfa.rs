use regex::Regex;

pub fn matches_gk(text: &str) -> bool {
    let pattern = r"^(gk|gk2zqssxla2rwvzk347ryhh6jjprsca69fjlw93zgi3b|gk8r4uumrawcrez5xjy5dazvv5v7afvyg77id37pvtk|gkhb0o3d22xcucql5henf4l1sjwg1vpgiw2rdyqxoxy|gkjz7qkcjq5x|gkot5hbsd81kmupncxhaqbhv3huebxafmlnpcx2hniwn)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
