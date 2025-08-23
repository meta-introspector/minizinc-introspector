use anyhow::Result;
//use std::collections::HashMap;
use std::path::PathBuf;
//use regex::Regex;
use std::io::Write;

// gemini wanted to remove this...
//  │    10 - pub fn generate_and_save_new_regex_templates(                                                              │
//  │    11 -     unmatched_lines: &[String],                                                                            │
//  │    12 -     current_dir: &PathBuf,                                                                                 │
//  │    13 - ) -> Result<()> {                                                                                          │
//  │    14 -     let regex_templates_dir = current_dir.join("crates/poem_yaml_fixer/src/regex_templates");              │
//  │    15 -     std::fs::create_dir_all(&regex_templates_dir)?;                                                        │
//  │    10                                                                                                              │
//  │    ══════════════════════════════════════════════════════════════════════════════════════════════════════════════  │
//  │    17 -     let mut new_module_declarations: Vec<String> = Vec::new();                                             │
//  │    18 -                                                                                                            │
//  │    19 -     if !unmatched_lines.is_empty() {                                                                       │
//  │    20 -         generate_and_save_generalized_regex(unmatched_lines, current_dir)?;                                │
//  │    21 -     }                                                                                                      │
//  │    22 -                                                                                                            │
//  │    23 -     // Update the regex_templates/mod.rs file                                                              │
//  │    24 -     let mod_rs_path = regex_templates_dir.join("mod.rs");                                                  │
//  │    25 -     let mut mod_rs_file = std::fs::OpenOptions::new()                                                      │
//  │    26 -         .append(true)                                                                                      │
//  │    27 -         .create(true)                                                                                      │
//  │    28 -         .open(&mod_rs_path)?;                                                                              │
//  │    29 -                                                                                                            │
//  │    30 -     for declaration in new_module_declarations {                                                           │
//  │    31 -         writeln!(mod_rs_file, "{}", declaration)?;                                                         │
//  │    32 -     }                                                                                                      │
//  │    33 -     println!("Updated {:?} with new module declarations.", mod_rs_path);                                   │
//  │    34 -                                                                                                            │
//  │    35 -     Ok(())                                                                                                 │
//  │    36 - }



