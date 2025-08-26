use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RunOptions {
    pub envs: HashMap<String, String>,
    pub current_dir: PathBuf,
}
