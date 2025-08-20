use regex::Regex;

pub fn matches_dd(text: &str) -> bool {
    let pattern = r"^(dd_hh|ddag|ddagg|ddagge|ddagger|ddar|ddarr|ddddˇ|dddˇ|ddim|ddimscheduler|ddimschedulerconfig|ddlwvyuvdz26johmgsba7mjpjfgx5zp2dkp8qsf2c33v|ddo|ddot|ddotr|ddotra|ddotrah|ddotrahd|ddots|ddotse|ddotseq|ddpmscheduler|ddpmschedulerconfig|ddpmvariancetype|ddpmwschedulerconfig|ddscvzlh|ddsketch|ddsketch_config|ddthh|ddtwt|ddvhmk1ifzatsznskvufsk12uuj5owkoio|ddˇ|ddˇdd)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
