use regex::Regex;

pub fn matches_d3(text: &str) -> bool {
    let pattern = r"^(d31efnlgdiysi84woo3of4jmu7vmasus3z7j9hyxcely|d37n3bsg71ouwcwjbz37jzp7ufsxg2qmkeualj1pyvm6|d39vuspvfhjpvd7etmjzra5j1tsmp4lxfb43nxumgdht|d3d9caps|d3dcontentprotectioncaps|d3dvshadercaps2_0)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
