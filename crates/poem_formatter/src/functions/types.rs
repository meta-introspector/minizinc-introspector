use serde::{Deserialize
	    //, Serialize
};

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct RegexEntry {
    pub name: String,
    pub pattern: String,
    pub callback_function: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, Clone)]
pub struct RegexConfig {
    pub regexes: Vec<RegexEntry>,
}
