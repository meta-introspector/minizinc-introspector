use regex::Regex;

pub fn matches_ft(text: &str) -> bool {
    let pattern = r"^(fte|ftest2|ftok2jhqaqxuweicvrrfrs9dpppwp8cgtb7nqnkl88ms|ftok4nje8b7tdffykc5babcaqv5sl6jispyrprzatuwn|ftoknbyyif4ky9s8wsmlbxhcht17ek7rxalzghzzqhj1|ftol|ftrl_2d_toy_example_works|ftrl_toy_example_works|ftrlparams|ftrlvalidparams|fts)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
