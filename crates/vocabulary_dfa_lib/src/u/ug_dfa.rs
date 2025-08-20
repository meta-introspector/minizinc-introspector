use regex::Regex;

pub fn matches_ug(text: &str) -> bool {
    let pattern = r"^(ug_cuda|ug_metal|ug_op|ugelu|ugelu_erf|uglier|uglifiers|ugr|ugra|ugrav|ugrave|ugw9bzbiuhnrzjjr8ltx4cagmfubevd4sx32q8btqiau4zngtdho16pwwrsq1f6|ugyanis)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
