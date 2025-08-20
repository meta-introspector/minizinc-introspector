use regex::Regex;

pub fn matches_eu(text: &str) -> bool {
    let pattern = r"^(euarctos|euch|euclidean_distance_8d|euclidean_norm|euclideancodebook|eue|euer|eues|euler_ancestral_discrete|euler_angles|euler_angles_to_matrix|euler_class|euler_elegance|euler_gauss_analysis|euler_sample|euler_totient|eulerancestral|eulerancestraldiscretescheduler|eulerancestraldiscreteschedulerconfig|eulergamma|eulerian|eulerian_number|eulerreflection|eull|eum|euml|eupleridae|eur|eure|eurem|euren|eurent|eurer|eures|euro2004|européen|européens|euros|eus|eusse|eussent|eusses|eussiez|eussions|eut|euterpe|euv|eux|euybckqyaijavbjnboh7hqidcmwwamhwqnyoe4g2xhrprk8icm8g|euybckqyaijavbjnboh7hqidcmwwamhwqnyoe4g2xhrprk8icm8gzzu16i7se4eiebmlkqnh1gtwcx1bmk6ilu8bxwn5wpvifbimnptdlval7zx5inpfggwwjx)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
