use regex::Regex;

pub fn matches_fv(text: &str) -> bool {
    let pattern = r"^(fv_overflow|fvisibility|fvpy2ahzo0sdwzm6lvsoaqi7kpafue4wquaqewiltqv8imhehmpevxt|fvs_from_dependency|fvs_from_requested)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
