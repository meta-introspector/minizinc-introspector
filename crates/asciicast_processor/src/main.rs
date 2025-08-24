use asciicast::{Entry, EventType, Header};
use std::fs::File;
use std::io::{
    //self,
    BufReader, Write};
use serde_json::Deserializer;
use grex::RegExpBuilder;
use anyhow::{Result, anyhow};
use clap::Parser;
use strip_ansi_escapes::strip;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;
use std::path::PathBuf;

use quote::quote;
use syn::{Ident, LitStr};
use proc_macro2::TokenStream;
use regex::Regex;

use gemini_utils::gemini_eprintln;

// Re-export the macro for use in generated code
#[allow(unused_imports)]
use poem_macros::poem_function;

use clap::Subcommand;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the asciicast input file
    input_file: PathBuf, // Added input file argument

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Generates Rust code from an asciicast file
    Generate {
        /// Limit the number of events to process from the beginning
        #[arg(long, default_value_t = 10, conflicts_with = "tail")]
        limit: usize,
        /// Process only the last N events
        #[arg(long, conflicts_with = "limit")]
        tail: Option<usize>,
        /// Steps for hierarchical grouping (e.g., --steps 5,3,1)
        #[arg(long, value_delimiter = ',', default_values_t = [5, 3, 1])]
        steps: Vec<usize>,
        /// Output file to save the generated Rust code
        #[arg(long)]
        rust_output_file: PathBuf,
        /// Enable ASCII naming for Unicode characters and ANSI sequences
        #[arg(long)]
        ascii_names: bool,
    },
    /// Analyzes an asciicast file using previously generated Rust code
    Analyze {
        /// Path to the previously generated Rust code file (.rs)
        #[arg(long)]
        generated_rust_file: PathBuf,
    },
}

// This struct is no longer directly serialized/deserialized, but its structure is used to generate code
#[derive(Debug)]
struct RegexHierarchyNode {
    prefix_regex: Option<String>,
    children: Vec<RegexHierarchyNode>,
    lines: Vec<String>,
}

fn map_to_ascii_names(input: &str) -> String {
    input.replace("█", "BLOCK").replace("░", "LIGHT_SHADE")
}

fn build_hierarchy(lines: Vec<String>, steps: &[usize]) -> RegexHierarchyNode {
    if lines.is_empty() {
        return RegexHierarchyNode { prefix_regex: None, children: Vec::new(), lines: Vec::new() };
    }

    if steps.is_empty() || lines.len() == 1 {
        let regex = RegExpBuilder::from(&lines.iter().map(|s| s.as_str()).collect::<Vec<&str>>()).build();
        return RegexHierarchyNode { prefix_regex: Some(regex), children: Vec::new(), lines };
    }

    let current_prefix_len = steps[0];
    let remaining_steps = &steps[1..];

    let mut prefix_groups: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let graphemes: Vec<&str> = line.graphemes(true).collect();
        let prefix = graphemes.iter().take(current_prefix_len).map(|&s| s).collect::<String>();
        
        prefix_groups.entry(prefix).or_default().push(line);
    }

    let mut children = Vec::new();
    for (prefix, group_lines) in prefix_groups {
        let common_prefix_regex = RegExpBuilder::from(&[&prefix]).build();
        let mut meaningful_remaining_lines: Vec<String> = Vec::new();

        for line in group_lines {
            let graphemes: Vec<&str> = line.graphemes(true).collect();
            let remaining_part = if graphemes.len() > prefix.len() {
                graphemes.iter().skip(prefix.len()).map(|&s| s).collect::<String>()
            } else {
                "".to_string()
            };
            let trimmed_remaining_part = remaining_part.trim().to_string();
            if !trimmed_remaining_part.is_empty() {
                meaningful_remaining_lines.push(trimmed_remaining_part);
            }
        }

        if !meaningful_remaining_lines.is_empty() {
            let child_node = build_hierarchy(meaningful_remaining_lines, remaining_steps);
            children.push(RegexHierarchyNode {
                prefix_regex: Some(common_prefix_regex),
                children: child_node.children,
                lines: child_node.lines,
            });
        } else { // If no meaningful remaining lines, this node is a leaf with its prefix regex
            children.push(RegexHierarchyNode {
                prefix_regex: Some(common_prefix_regex),
                children: Vec::new(),
                lines: Vec::new(), // No remaining lines to store here
            });
        }
    }

    RegexHierarchyNode { prefix_regex: None, children, lines: Vec::new() }
}

fn generate_poem_functions(node: &RegexHierarchyNode, parent_name: &str, level: usize) -> TokenStream {
    let mut functions = TokenStream::new();

    if let Some(ref regex_str) = node.prefix_regex {
                let fn_name_str = format!("{}_level{}", parent_name, level);
        let fn_name = Ident::new(&fn_name_str, proc_macro2::Span::call_site());
        let pattern_lit = LitStr::new(regex_str, proc_macro2::Span::call_site());

        let title_str = format!("Meme for pattern: {}", regex_str);
        let title_lit = LitStr::new(&title_str, proc_macro2::Span::call_site());

        let summary_str = format!("Generated from asciicast output at level {}", level);
        let summary_lit = LitStr::new(&summary_str, proc_macro2::Span::call_site());

        let keywords_str = format!("asciicast,regex,meme,level{}", level);
        let keywords_lit = LitStr::new(&keywords_str, proc_macro2::Span::call_site());

        let emojis_str = "🔍🌲🔄";
        let emojis_lit = LitStr::new(emojis_str, proc_macro2::Span::call_site());

        let art_generator_instructions_str = format!("Generate an image for: {}", regex_str);
        let art_generator_instructions_lit = LitStr::new(&art_generator_instructions_str, proc_macro2::Span::call_site());

        let pending_meme_description_str = format!("This meme represents the pattern: {}", regex_str);
        let pending_meme_description_lit = LitStr::new(&pending_meme_description_str, proc_macro2::Span::call_site());

        let function_code = quote! {
            #[poem_function(
                name = #fn_name_str,
                pattern = #pattern_lit,
                title = #title_lit,
                summary = #summary_lit,
                keywords = #keywords_lit,
                emojis = #emojis_lit,
                art_generator_instructions = #art_generator_instructions_lit,
                pending_meme_description = #pending_meme_description_lit
            )]
            pub fn #fn_name(line: &str, captures: &regex::Captures, fixed_fm: &mut std::collections::HashMap<String, String>) -> anyhow::Result<()> {
                eprintln!("Matched meme: {{}}", #fn_name_str);
                // Add logic here to process the matched line
                Ok(())
            }
        };
        functions.extend(function_code);
    }

    for child in &node.children {
        functions.extend(generate_poem_functions(child, parent_name, level + 1));
    }

    functions
}

fn extract_patterns_from_rust_file(file_path: &PathBuf) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(file_path)?;
    let syntax_tree = syn::parse_file(&content)?;

    let mut patterns = Vec::new();

    for item in syntax_tree.items {
        if let syn::Item::Fn(item_fn) = item {
            for attr in item_fn.attrs {
                if attr.path().is_ident("poem_function") {
                    if let Some(meta_list) = attr.meta.as_list() { // Use as_list() to get MetaList
                        for nested_meta in meta_list.nested.iter() { // Iterate over nested
                            if let syn::Meta::NameValue(name_value) = nested_meta { // Directly match NameValue
                                if name_value.path.is_ident("pattern") {
                                    if let syn::Expr::Lit(expr_lit) = name_value.value {
                                        if let syn::Lit::Str(lit_str) = expr_lit.lit {
                                            patterns.push(lit_str.value());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(patterns)
}

fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.input_file)?;
    let reader = BufReader::new(file);

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Read the header
    let header_value = de.next().ok_or_else(|| anyhow!("Missing header"))?;
    let header: Header = serde_json::from_value(header_value.map_err(|e| anyhow!(e))?)?;

    gemini_eprintln!("Asciicast Header:");
    gemini_eprintln!("  Version: :version:", version = header.version);
    gemini_eprintln!("  Width: :width:", width = header.width);
    gemini_eprintln!("  Height: :height:", height = header.height);
    if let Some(timestamp) = header.timestamp {
        gemini_eprintln!("  Timestamp: :timestamp:", timestamp = timestamp);
    }
    if let Some(duration) = header.duration {
        gemini_eprintln!("  Duration: :duration:", duration = duration);
    }
    if let Some(title) = header.title {
        gemini_eprintln!("  Title: :title:", title = title);
    }

    let mut all_events: Vec<Entry> = Vec::new();
    let mut event_count = 0;

    // Collect all events if --tail is specified, otherwise process with --limit
    if let Some(tail_count) = args.tail {
        gemini_eprintln!("Collecting all events to process last :tail_count: events.", tail_count = tail_count);
        for value in de {
            gemini_eprintln!("DEBUG: Processing value: :value_str:", value_str = format!("{:?}", value));
            let entry: Entry = serde_json::from_value(value.map_err(|e| anyhow!(e))?)?;
            all_events.push(entry);
        }
        event_count = all_events.len();
    } else {
        // Existing limit logic
        fn main() -> Result<()> {
    let args = Args::parse();

    let file = File::open(&args.input_file)?;
    let reader = BufReader::new(file);

    let mut de = Deserializer::from_reader(reader).into_iter::<serde_json::Value>();

    // Read the header
    let header_value = de.next().ok_or_else(|| anyhow!("Missing header"))?;
    let header: Header = serde_json::from_value(header_value.map_err(|e| anyhow!(e))?)?;

    gemini_eprintln!("Asciicast Header:");
    gemini_eprintln!("  Version: :version:", version = header.version);
    gemini_eprintln!("  Width: :width:", width = header.width);
    gemini_eprintln!("  Height: :height:", height = header.height);
    if let Some(timestamp) = header.timestamp {
        gemini_eprintln!("  Timestamp: :timestamp:", timestamp = timestamp);
    }
    if let Some(duration) = header.duration {
        gemini_eprintln!("  Duration: :duration:", duration = duration);
    }
    if let Some(title) = header.title {
        gemini_eprintln!("  Title: :title:", title = title);
    }

    let mut all_events: Vec<Entry> = Vec::new();
    let mut event_count = 0;

    // Collect all events
    for value in de {
        let entry: Entry = serde_json::from_value(value.map_err(|e| anyhow!(e))?)?;
        all_events.push(entry);
        event_count += 1;
    }

    match args.command {
        Commands::Generate { limit, tail, steps, rust_output_file, ascii_names } => {
            let start_index = if let Some(tail_count) = tail {
                if tail_count >= all_events.len() {
                    0 // Process all if tail_count is greater than or equal to total events
                } else {
                    all_events.len() - tail_count
                }
            } else {
                0 // Start from beginning for --limit
            };

            let end_index = if let Some(tail_count) = tail {
                event_count // Process up to the end of collected events
            } else {
                limit.min(event_count) // Process up to limit or collected events
            };

                        gemini_eprintln!("sparklesProcessing events and collecting cleaned output (limited to brickwall)...sparkles", limit = limit);

            let mut cleaned_output_lines: Vec<String> = Vec::new();
            for i in start_index..end_index {
                let entry = &all_events[i];
                match entry.event_type {
                    EventType::Output => {
                        let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();
                        let processed_data = if ascii_names {
                            map_to_ascii_names(&cleaned_data)
                        } else {
                            cleaned_data
                        };
                        cleaned_output_lines.push(processed_data);
                    },
                    EventType::Input => {
                        // Ignore input events for now
                    },
                }
            }

            gemini_eprintln!("Total number of events processed: :event_count:", event_count = event_count);

            let hierarchy = build_hierarchy(cleaned_output_lines, &steps);

            let generated_code = generate_poem_functions(&hierarchy, "root", 0);

            let mut output_file = File::create(&rust_output_file)?;
            output_file.write_all(generated_code.to_string().as_bytes())?;

            gemini_eprintln!("Generated Rust code written to: :file_path:", file_path = format!("{:?}", rust_output_file));
        },
        Commands::Analyze { generated_rust_file } => {
            gemini_eprintln!("Entering Analyze mode with generated Rust file: :file_path:", file_path = format!("{:?}", generated_rust_file));

            let patterns = extract_patterns_from_rust_file(&generated_rust_file)?;
            gemini_eprintln!("Extracted :count: patterns from :file_path:", count = patterns.len(), file_path = format!("{:?}", generated_rust_file));
            for pattern in patterns {
                gemini_eprintln!("  - :pattern:", pattern = pattern);
            }
            // TODO: Implement analysis logic here
            // 2. Match against all_events output lines
            // 3. Calculate match percentage
            // 4. Identify unmatched lines
            // 5. Perform bag-of-words and cosine similarity on unmatched lines
            // 6. Generate new regexes for unmatched lines
        },
    }

    Ok(())
}
        for value in de {
            gemini_eprintln!("DEBUG: Processing value: :value_str:", value_str = format!("{:?}", value));
            if event_count >= args.limit {
                gemini_eprintln!("Reached event processing limit of :limit:. Stopping.", limit = limit);
                break;
            }
            let entry: Entry = serde_json::from_value(value.map_err(|e| anyhow!(e))?)?;
            all_events.push(entry); // Store events even with limit, to simplify later processing
            event_count += 1;
        }
    }

    let mut cleaned_output_lines: Vec<String> = Vec::new();
    let start_index = if let Some(tail_count) = args.tail {
        if tail_count >= all_events.len() {
            0 // Process all if tail_count is greater than or equal to total events
        } else {
            all_events.len() - tail_count
        }
    } else {
        0 // Start from beginning for --limit
    };

    let end_index = if let Some(tail_count) = args.tail {
        event_count // Process up to the end of collected events
    } else {
        args.limit.min(event_count) // Process up to limit or collected events
    };

    for i in start_index..end_index {
        let entry = &all_events[i];
        match entry.event_type {
            EventType::Output => {
                let cleaned_data = String::from_utf8_lossy(&strip(entry.event_data.as_bytes())?).to_string();
                let processed_data = if args.ascii_names {
                    map_to_ascii_names(&cleaned_data)
                } else {
                    cleaned_data
                };
                cleaned_output_lines.push(processed_data);
            },
            EventType::Input => {
                // Ignore input events for now
            },
        }
    }

    gemini_eprintln!("Total number of events processed: :event_count:", event_count = event_count);

    let hierarchy = build_hierarchy(cleaned_output_lines, &args.steps);
    
    let generated_code = generate_poem_functions(&hierarchy, "root", 0);

    let mut output_file = File::create(&args.rust_output_file)?;
    output_file.write_all(generated_code.to_string().as_bytes())?;

    gemini_eprintln!("Generated Rust code written to: :file_path:", file_path = format!("{:?}", args.rust_output_file));

    Ok(())
}