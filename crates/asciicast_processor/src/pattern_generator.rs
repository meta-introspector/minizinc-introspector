use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;


use quote::quote;
use syn::{Ident, LitStr};
use proc_macro2::TokenStream;

use grex::RegExpBuilder;



#[derive(Debug)]
pub struct RegexHierarchyNode {
    pub prefix_regex: Option<String>,
    pub children: Vec<RegexHierarchyNode>,
    pub lines: Vec<String>,
}

pub fn map_to_ascii_names(input: &str) -> String {
    input.replace("█", "BLOCK").replace("░", "LIGHT_SHADE")
}

pub fn build_hierarchy(lines: Vec<String>, steps: &[usize]) -> RegexHierarchyNode {
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

pub fn generate_poem_functions(node: &RegexHierarchyNode, parent_name: &str, level: usize) -> TokenStream {
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
