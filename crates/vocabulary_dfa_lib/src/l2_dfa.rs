use regex::Regex;

pub fn matches_l2(text: &str) -> bool {
    let pattern = r"^(l201|l204|l206|l210|l2123|l2130|l2137|l21_norm|l221|l224|l226|l228|l231|l2337|l2357|l2361c10|l2361c122|l239|l2394|l2403|l2414|l243|l245|l254|l262|l265|l267|l268|l270|l274|l275|l28|l283|l284|l288|l289|l290|l2_dist|l2_memberships|l2_normalize|l2_reg|l2r_l1loss_svc_dual|l2r_l2loss_svc|l2r_l2loss_svc_dual|l2r_lr|l2r_lr_dual|l2regularizedl1losssvcdual|l2regularizedl2losssvc|l2regularizedl2losssvcdual|l2regularizedlogistic|l2regularizedlogisticdual)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
