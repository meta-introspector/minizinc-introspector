use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct RegexEntry {
    pub name: String,
    pub pattern: String,
    pub callback_function: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RegexConfig {
    pub regexes: Vec<RegexEntry>,
}
