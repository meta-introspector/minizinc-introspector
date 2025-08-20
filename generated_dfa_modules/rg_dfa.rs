use regex::Regex;

pub fn matches_rg(text: &str) -> bool {
    let pattern = r"^(rg8unorm|rg_lru|rgb16|rgb24|rgb_color|rgb_data|rgb_for_index|rgb_idx|rgb_or_hsl|rgb_slice|rgb_values|rgba16|rgba32|rgba8|rgba8unorm|rgbaimage|rgbavisitor|rgbe|rgbimage|rgbx|rglru|rgrc|rgs|rgs_|rgwood|rgx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
