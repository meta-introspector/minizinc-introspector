use regex::Regex;

pub fn matches_eg(text: &str) -> bool {
    let pattern = r"^(eg01|eg02|eg03|eg04|eg09|eg10|eg11|eg12|eg13|eg14|eg15|eg16|eg17|eg18|eg19|eg20|eg21|eg22|eg23|egde|egg_counts|egg_mode|egg_patterns|egg_type|eggnog|eggrpah|egis|egmontkob|egr|egr0da1dvngwhrs|egra|egrav|egrave|egrep|egress|egress_llm_traffic|egress_traffic|egret|egretta|egs|egsd|egsdo|egsdot|egui|egy|egyes|egyetlen|egyik|egypt|egyptian|egyre|egyéb|egész)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
