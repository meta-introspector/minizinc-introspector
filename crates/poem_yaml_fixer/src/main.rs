use std::path::{Path, PathBuf};
use clap::Parser;
use walkdir::WalkDir;
use serde::{Serialize, Deserialize};
use crate::functions::types::{FixedFrontMatter, PoemFunctionRegistry};
use regex::Regex;

poem_macros::poem_header!();

mod functions;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional path to a single poem file to process. If not provided, processes all .md files in docs/poems/.
    #[arg(short, long, value_name = "FILE_PATH")]
    file: Option<PathBuf>,

    /// Maximum allowed percentage of content reduction. Aborts if reduction exceeds this. Defaults to 1.0.
    #[arg(long, value_name = "PERCENTAGE")]
    max_change_percentage: Option<f64>,

    /// Enable debug output, dumping findings in YAML format.
    #[arg(long)]
    debug: bool,

    /// Perform a dry run, showing changes without writing to disk.
    #[arg(long)]
    dry_run: bool,

    /// Use direct YAML parsing fast path.
    #[arg(long)]
    fast_parse: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PoemReportEntry {
    file_path: String,
    status: String,
    matched_patterns: Option<Vec<String>>,
    error_message: Option<String>,
    extracted_words_count: Option<usize>,
    dry_run_changes_applied: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct PoemReport {
    files: Vec<PoemReportEntry>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    let mut regex_config = get_default_regex_config();

    let external_config_path = current_dir.join("regex_config.toml");
    if external_config_path.exists() {
        println!("Loading external regex config from: {:?}", external_config_path);
        let external_config = functions::load_regex_config::load_regex_config(&external_config_path)?;

        for external_entry in external_config.regexes {
            if let Some(default_entry) = regex_config.regexes.iter_mut().find(|e| e.name == external_entry.name) {
                *default_entry = external_entry;
            } else {
                regex_config.regexes.push(external_entry);
            }
        }
    }

    let function_registry: PoemFunctionRegistry = create_function_registry();

    let mut report_entries: Vec<PoemReportEntry> = Vec::new();

    if let Some(file_path) = cli.file {
        process_file(&file_path, &regex_config, &function_registry, &mut report_entries, cli.debug, cli.dry_run)?;
    } else {
        for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
                if path.file_name().is_some_and(|name| name.to_str().unwrap_or("").ends_with(".archeology.md")) {
                    continue;
                }
                process_file(path, &regex_config, &function_registry, &mut report_entries, cli.debug, cli.dry_run)?;
            }
        }
    }

    let report = PoemReport { files: report_entries };
    let report_yaml = serde_yaml::to_string(&report)?;
    let report_path = current_dir.join("poem_processing_report.yaml");
    std::fs::write(&report_path, report_yaml)?;

    println!("Report generated at: {:?}", report_path);

    Ok(())
}

fn process_file(path: &Path, regex_config: &RegexConfig, function_registry: &PoemFunctionRegistry, report_entries: &mut Vec<PoemReportEntry>, debug: bool, dry_run: bool) -> Result<()> {
    println!("Processing file: {:?}", path);
    let content = std::fs::read_to_string(path)?;

    let mut stack: Vec<FixedFrontMatter> = vec![FixedFrontMatter::default()];

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
                    let captures: Vec<String> = regex.captures(line).map_or(vec![], |caps| {
                        caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                    });

                    if regex_entry.callback_function == "handle_new_document" {
                        stack.push(FixedFrontMatter::default());
                    } else {
                        if let Some(current_fm) = stack.last_mut() {
                            if let Some((_metadata, callback)) = function_registry.get(&regex_entry.callback_function) {
                                (*callback)(line, captures, current_fm)?;
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
                        let captures: Vec<String> = regex.captures(line).map_or(vec![], |caps| {
                            caps.iter().map(|m| m.map_or("".to_string(), |m| m.as_str().to_string())).collect()
                        });

                        if regex_entry.callback_function == "handle_new_document" {
                            archeology_stack.push(FixedFrontMatter::default());
                        } else {
                            if let Some(current_fm) = archeology_stack.last_mut() {
                                if let Some((_metadata, callback)) = function_registry.get(&regex_entry.callback_function) {
                                    (*callback)(line, captures, current_fm)?;
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
                if !recovered_fm.memes.is_empty() {
                    main_fm.memes.extend(recovered_fm.memes);
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
        println!("Dry run: Would write to {:?}", path);
        if debug {
            println!("--- New Content ---");
            println!("{}", final_content);
        }
    } else {
        std::fs::write(path, &final_content)?;
        println!("Applied changes to: {:?}", path);
    }

    report_entries.push(PoemReportEntry {
        file_path: path.to_string_lossy().into_owned(),
        status: "Success".to_string(),
        matched_patterns: None,
        error_message: None,
        extracted_words_count: None,
        dry_run_changes_applied: dry_run,
    });

    Ok(())
}