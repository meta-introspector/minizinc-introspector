use regex::Regex;

pub fn matches_ih(text: &str) -> bool {
    let pattern = r"^(ihier_part|ihm|ihn|ihnen|ihost|ihr|ihre|ihrem|ihren|ihrer|ihres|ihrnqmdv7oqcpgq7oejrzj4bxvy2qojnziqqdoc7vp3itw1)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
