use regex::Regex;

pub fn matches_qd(text: &str) -> bool {
    let pattern = r"^(qd|qdev|qdevbext|qdeveloper_input|qdeveloper_minimal|qdeveloperchatresponsestream|qdeveloperchatresponsestreamerror|qdevelopersendmessage|qdevelopersendmessageerror|qdeveloperstandalone|qdeveloperstandalonefree|qdeveloperstandalonepower|qdeveloperstandalonepro|qdeveloperstandaloneproplus|qdeveloperstreamingclient|qdisc|qdrant_client|qdrant_port|qdrant_port_secondary|qdrantvectorstore)$";
    let re = Regex::new(pattern).unwrap();
    re.is_match(text)
}
