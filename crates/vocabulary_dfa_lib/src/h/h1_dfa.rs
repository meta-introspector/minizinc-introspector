use regex::Regex;

pub fn matches_h1(text: &str) -> bool {
    let pattern = r"^(h100|h11|h1_builder|h1_key|h1d|h1fqyhjxnovdibjfk1cfcfwsntn8|h1qiarigqcsylutx06audf9hg7|h1rt8kvxknhqextrfky8r9wjzbz8ycih6j4wq5fz9hgp|h1v1h)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
