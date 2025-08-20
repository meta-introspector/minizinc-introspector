use regex::Regex;

pub fn matches_mx(text: &str) -> bool {
    let pattern = r"^(mx_00|mx_01|mx_10|mx_11|mx_1p5|mx_4|mx_5|mxbaiembedlarge|mxbaiembedlargev1|mxbaiembedlargev1q|mxn|mxssyjfd3oza3cwdb0jepqf|mxtrf|mxuzjdcye44unrqmbskwt86cc5sejgjnhfwvq971rra6i)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
