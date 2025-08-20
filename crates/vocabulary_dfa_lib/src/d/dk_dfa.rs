use regex::Regex;

pub fn matches_dk(text: &str) -> bool {
    let pattern = r"^(dk8u9jdvuve9a8x1tqoptwbt71nqmv4cre6nib6ttn2nrgy4bl3u2tjxlvigy7j8spbubsmrddbdhlj6dubhw3fotnmnq15jcvphbzrobqlgvrjjwmzoah59gjy721dkkomtmlabongo|dkbl4lsqwa)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
