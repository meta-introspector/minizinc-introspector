use regex::Regex;

pub fn matches_e3(text: &str) -> bool {
    let pattern = r"^(e3m5hm8yaeb7iphqxfypaklqxnezctugbdma8jdrghoo|e3php7w8kb7np3ctq1qq2tw3kctjrsxbqgw9vm2mwv2y)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
