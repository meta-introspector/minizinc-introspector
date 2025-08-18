use std::{
    //collections::HashMap,
    fs,
    //io::{
    //self, BufReader,
    //BufRead}
    path::PathBuf
};

#[derive(Debug)]
pub struct LogicalRelationships {
    pub num_relations: usize,
    pub relation_pairs: Vec<(String, String)>,
    pub desired_distances: Vec<f64>,
}

pub fn parse_logical_relationships(path: &PathBuf) -> Result<LogicalRelationships, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;

    let mut num_relations = 0;
    let mut relation_pairs = Vec::new();
    let mut desired_distances = Vec::new();

    for line in content.lines() {
        if line.starts_with("num_relations =") {
            num_relations = line.strip_prefix("num_relations =").unwrap().trim_end_matches(";").trim().parse()?;
        } else if line.starts_with("relation_pairs =") {
            let pairs_str = line.strip_prefix("relation_pairs = [" ).unwrap().trim_end_matches("];").trim();
            for pair_block in pairs_str.split("|, |") {
                let cleaned_block = pair_block.trim_start_matches("|").trim_end_matches("|").trim();
                let parts: Vec<&str> = cleaned_block.split(", ").collect();
                if parts.len() == 2 {
                    relation_pairs.push((parts[0].trim_matches('"').to_string(), parts[1].trim_matches('"').to_string()));
                }
            }
        } else if line.starts_with("desired_distances =") {
            let distances_str = line.strip_prefix("desired_distances = [" ).unwrap().trim_end_matches("];").trim();
            for dist_str in distances_str.split(", ") {
                desired_distances.push(dist_str.parse()?);
            }
        }
    }

    Ok(LogicalRelationships {
        num_relations,
        relation_pairs,
        desired_distances,
    })
}
