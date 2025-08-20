use regex::Regex;

pub fn matches_qo(text: &str) -> bool {
    let pattern = r"^(qocnc1amruffrifu|qop|qopf|qos_class_background|qos_class_default|qos_class_raw|qos_class_t|qos_class_unspecified|qos_class_user_initiated|qos_class_user_interactive|qos_class_utility|qos_cost_results|qos_default|qosclass|qospolicies|qospolicybuilder|qosservicemetrics|qosservicemetricserrors|qosservicemetricsstats|qov4nthw3dftkmztsqw6pbckpsroegkdjpazzta01og9a7vk1atbeadjecelmepsz1eqcejfniycuvlkojfof29v)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
