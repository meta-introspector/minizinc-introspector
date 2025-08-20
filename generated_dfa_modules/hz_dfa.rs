use regex::Regex;

pub fn matches_hz(text: &str) -> bool {
    let pattern = r"^(hz|hz9nydgn1k15wnwffkx7csmzp4vftntwlxaedomfgnxy|hza7zvu|hzfgnywmnztxmkrj5bf6h9ftotbtmna|hzofipbw3gmyqqhjx2ldvirxji7blhhhtunwx5f10lfayolqa9j859ab1w7nd09|hzwmbuensfkrpimdzem6zstdu5czevgkvdcwbaptogc9)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
