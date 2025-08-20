use regex::Regex;

pub fn matches_ku(text: &str) -> bool {
    let pattern = r"^(kuadrant|kube|kubernetes_swagger|kubevirt_hostname|kubevirt_instance_id|kubevirt_instance_type|kuckeyactiondown|kuckeytranslatenodeadkeysmask|kudos|kudy68zonesmyfukbmpxitopwk6il7uuj51ecgjdot1y1fga6ueicub3znq8pqqf|kuin|kuka|kullback|kun|kunde|kunne|kunnen|kuqzl|kuvasz)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
