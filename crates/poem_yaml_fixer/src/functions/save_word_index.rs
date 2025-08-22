// This file contains the WordIndex struct and the save_word_index function.
// It is responsible for managing the word index for poems.

use std::{
    //collections::HashMap,
    path::PathBuf};
use anyhow::Result;
use serde_yaml;
//use serde::{Deserialize, Serialize};
use crate::functions::types::WordIndex; // Import WordIndex from types module

// Function to save the word index to a file
#[allow(dead_code)]
pub fn save_word_index(word_index: &WordIndex, path: &PathBuf) -> Result<()> {
    let yaml_content = serde_yaml::to_string(word_index)?;
    std::fs::write(path, yaml_content)?;
    Ok(())
}
