use regex::Regex;

pub fn matches_tb(text: &str) -> bool {
    let pattern = r"^(tba0xb7pcngegxkkpg97lerc1|tbank|tbar|tbb|tbbb|tbc|tbcd|tbm_target_feature|tbody|tbr|tbreak|tbrk|tbrlyjw1qjkw3eslgwjug4xxmgltxw1uhht2ogzev6o24hjycxfrudlcfnnvym5hb3rolq0qtb5ibo6s9culs47ho7eouh1g|tbscertificate|tbuf)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
