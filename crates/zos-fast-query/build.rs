use std::env;
use std::path::PathBuf;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct ConfigWrapper {
    paths: BuildPathsConfig,
}

#[derive(Debug, Deserialize)]
struct BuildPathsConfig {
    home_dir: String,
    github_root: String,
    project_root: String,
    hierarchical_term_index: String,
}

mod build_utils;
mod term_loader;
mod chunk_generator;
mod index_writer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
