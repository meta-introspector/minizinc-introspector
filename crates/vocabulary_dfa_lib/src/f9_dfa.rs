use regex::Regex;

pub fn matches_f9(text: &str) -> bool {
    let pattern = r"^(f9dmtjjpi8vflu1ejn4kkyogdxgmvfsahxz35qo9rdcj|f9mwfw8cnyvwsrq8am1pgffl3cquzv37mbgoxzftzljn)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
