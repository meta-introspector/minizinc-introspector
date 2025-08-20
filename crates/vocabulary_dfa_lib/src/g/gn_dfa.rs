use regex::Regex;

pub fn matches_gn(text: &str) -> bool {
    let pattern = r"^(gn3|gna|gnap|gnapp|gnappr|gnappro|gnapprox|gnarly|gne|gneq|gneqq|gniz4mq886btndt3pijgsu2gbw6it7sqrwncro45useb|gnlu8tah8bus|gnquad|gnquads|gns|gnsi|gnsim|gnullvm|gnunullexpr|gnusparse)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
