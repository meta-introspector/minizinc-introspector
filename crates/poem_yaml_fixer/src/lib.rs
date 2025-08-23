use std::path::{Path, PathBuf};
use regex::Regex;
use crate::functions::types::{FixedFrontMatter, PoemFunctionRegistry};
use crate::functions::utils::option_vec_helpers::{is_option_vec_empty, extend_option_vec};
use std::io::Write; // Add this
//use std::fs::File; // Add this

poem_macros::poem_header!();

pub mod functions;
pub mod manual_parser;

// Helper function to write to a log file
fn write_to_log_file(log_dir: &PathBuf, poem_path: &Path, message: &str) -> anyhow::Result<()> {
    std::fs::create_dir_all(log_dir)?; // Ensure directory exists
    let file_name = poem_path.file_name().unwrap_or_default().to_string_lossy().replace(".md", ".log");
    let log_file_path = log_dir.join(file_name);
    let mut file = std::fs::File::create(&log_file_path)?;
    file.write_all(message.as_bytes())?;
    Ok(())
}

pub fn process_file(path: &Path, regex_config: &RegexConfig, function_registry: &PoemFunctionRegistry, report_entries: &mut Vec<functions::report_generator::PoemReportEntry>, debug: bool, dry_run: bool, log_dir: &PathBuf) -> anyhow::Result<()> {
    write_to_log_file(log_dir, path, &format!("Processing file: {:?}\n", path))?;
    let content = std::fs::read_to_string(path)?;

    let mut stack: Vec<FixedFrontMatter> = vec![FixedFrontMatter::default()];
    let mut matched_lines_by_regex_for_report: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut unmatched_lines_for_report: Vec<String> = Vec::new();

    let compiled_regexes: HashMap<String, Regex> = regex_config
        .regexes
        .iter()
        .map(|entry| (entry.name.clone(), Regex::new(&entry.pattern).unwrap()))
        .collect();

    for line in content.lines() {
        let mut matched = false;
        for regex_entry in &regex_config.regexes {
            if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                if regex.is_match(line) {
                    matched_lines_by_regex_for_report
                        .entry(regex_entry.name.clone())
                        .and_modify(|lines_map| {
                            *lines_map.entry(line.to_string()).or_insert(0) += 1;
                        })
                        .or_insert_with(|| {
                            let mut lines_map = HashMap::new();
                            lines_map.insert(line.to_string(), 1);
                            lines_map
                        });
                    let captures: Vec<String> = regex.captures(line).map_or(vec![], |caps| {
                        caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                    });

                    if regex_entry.callback_function == "handle_new_document" {
                        stack.push(FixedFrontMatter::default());
                    } else {
                        if let Some(current_fm) = stack.last_mut() {
                            if let Some((_metadata, callback)) = function_registry.get(&regex_entry.callback_function) {
                                (*callback)(line, captures, current_fm)?;
                            } else {
                                // eprintln!("Warning: Callback function '{}' not found for regex '{}'", regex_entry.callback_function, regex_entry.name);
                            }
                        }
                    }
                    matched = true;
                    break;
                }
            }
        }
        if !matched {
            if let Some(current_fm) = stack.last_mut() {
                let mut body = current_fm.poem_body.take().unwrap_or_default();
                body.push_str(line);
                body.push('\n');
                current_fm.poem_body = Some(body);
            }
            unmatched_lines_for_report.push(line.to_string());
        }
    }

    let archeology_file_path = path.with_extension("md.archeology.md");
    if archeology_file_path.exists() {
        let archeology_content = std::fs::read_to_string(&archeology_file_path)?;
        let mut archeology_stack: Vec<FixedFrontMatter> = vec![FixedFrontMatter::default()];
        for line in archeology_content.lines() {
            let mut matched = false;
            for regex_entry in &regex_config.regexes {
                if let Some(regex) = compiled_regexes.get(&regex_entry.name) {
                    if regex.is_match(line) {
                        matched_lines_by_regex_for_report
                            .entry(regex_entry.name.clone())
                            .and_modify(|lines_map| {
                                *lines_map.entry(line.to_string()).or_insert(0) += 1;
                            })
                            .or_insert_with(|| {
                                let mut lines_map = HashMap::new();
                                lines_map.insert(line.to_string(), 1);
                                lines_map
                            });
                        let captures: Vec<String> = regex.captures(line).map_or(vec![], |caps| {
                            caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                        });

                        if regex_entry.callback_function == "handle_new_document" {
                            archeology_stack.push(FixedFrontMatter::default());
                        } else {
                            if let Some(current_fm) = archeology_stack.last_mut() {
                                if let Some((_metadata, callback)) = function_registry.get(&regex_entry.callback_function) {
                                    (*callback)(line, captures, current_fm)?;
                                } else {
                                    // eprintln!("Warning: Callback function '{}' not found for regex '{}'", regex_entry.callback_function, regex_entry.name);
                                }
                            }
                        }
                        matched = true;
                        break;
                    }
                }
            }
            if !matched {
                if let Some(current_fm) = archeology_stack.last_mut() {
                    let mut body = current_fm.poem_body.take().unwrap_or_default();
                    body.push_str(line);
                    body.push('\n');
                    current_fm.poem_body = Some(body);
                }
                unmatched_lines_for_report.push(line.to_string());
            }
        }
        

        if let Some(main_fm) = stack.first_mut() {
            for mut recovered_fm in archeology_stack.into_iter() {
                if main_fm.title.is_none() {
                    main_fm.title = recovered_fm.title;
                }
                if main_fm.summary.is_none() {
                    main_fm.summary = recovered_fm.summary;
                }
                if main_fm.keywords.is_none() {
                    main_fm.keywords = recovered_fm.keywords;
                }
                if main_fm.emojis.is_none() {
                    main_fm.emojis = recovered_fm.emojis;
                }
                if main_fm.art_generator_instructions.is_none() {
                    main_fm.art_generator_instructions = recovered_fm.art_generator_instructions;
                }
                if !is_option_vec_empty(&recovered_fm.memes) {
                    extend_option_vec(&mut main_fm.memes, recovered_fm.memes);
                }
                if let Some(pb) = recovered_fm.poem_body.take() {
                    let mut body = main_fm.poem_body.take().unwrap_or_default();
                    body.push_str("\n");
                    body.push_str(&pb);
                    main_fm.poem_body = Some(body);
                }
            }
        }
    }

    let mut final_content = String::new();
    for fm in stack {
        final_content.push_str("---");
        final_content.push_str(&serde_yaml::to_string(&fm)?);
        if let Some(body) = fm.poem_body {
            final_content.push_str(&body);
        }
    }

    if dry_run {
        write_to_log_file(log_dir, path, &format!("Dry run: Would write to {:?}\n", path))?;
        if debug {
            write_to_log_file(log_dir, path, &format!("--- New Content ---\n{}\n", final_content))?;
        }
    } else {
        std::fs::write(path, &final_content)?;
        write_to_log_file(log_dir, path, &format!("Applied changes to: {:?}\n", path))?;
    }

    report_entries.push(functions::report_generator::PoemReportEntry {
        file_path: path.to_string_lossy().into_owned(),
        status: "Success".to_string(),
        matched_lines_by_regex: Some(matched_lines_by_regex_for_report),
        error_message: None,
        extracted_words_count: None,
        dry_run_changes_applied: dry_run,
        unmatched_lines: Some(unmatched_lines_for_report),
    });

    Ok(())
}

