use regex::Regex;

pub fn matches_vu(text: &str) -> bool {
    let pattern = r"^(vu|vuestra|vuestras|vuestro|vuestros|vulgaris|vulkaninfo|vulpes|vulputate|vultr_hostname|vultr_instance_id|vultr_region_code|vulture)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
