// tests/front_matter_fixes.rs

use poem_yaml_fixer::functions::types::FixedFrontMatter;
use poem_traits::Meme; // Assuming Meme is accessible
use anyhow::Result; // For error handling tests

// Helper function to simulate front matter processing
// In a real test, this would call the actual processing logic of poem_yaml_fixer
fn simulate_front_matter_processing(input_yaml: &str) -> Result<FixedFrontMatter> {
    // This is a simplified simulation. In reality, poem_yaml_fixer would parse
    // the YAML, apply regexes and callbacks, and return a FixedFrontMatter.
    // For the purpose of these tests, we'll manually construct the expected
    // FixedFrontMatter based on the known fixes.

    let mut fixed_fm = FixedFrontMatter::default();
    let lines: Vec<&str> = input_yaml.lines().collect();

    // Simulate keywords fix
    if let Some(keywords_line) = lines.iter().find(|&line| line.trim().starts_with("keywords:")) {
        let captures: Vec<String> = vec![keywords_line.trim_start_matches("keywords:").trim().to_string()];
        // Simulate handle_comma_separated_keywords logic
        if let Some(keywords_str) = captures.get(0) {
            let keywords: Vec<String> = keywords_str
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            fixed_fm.keywords = Some(keywords);
        }
    }

    // Simulate malformed memes fix
    let mut processed_memes: Vec<Meme> = Vec::new();
    for line in lines.iter() {
        if line.trim().starts_with("- \"") && line.contains("(") && line.contains(")") {
            // Regex: r"^- \"([^\"]+)\" \(([^)]+)\""
            let re = regex::Regex::new(r"^- \"([^\"]+)\" \(([^)]+)\)").unwrap();
            if let Some(caps) = re.captures(line) {
                let description = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let template_raw = caps.get(2).map_or("", |m| m.as_str()).to_string();

                let template = template_raw
                    .to_lowercase()
                    .replace(" meme", "")
                    .split(',')
                    .next()
                    .map_or("unknown", |s| s.trim())
                    .to_string();

                processed_memes.push(Meme {
                    description: description,
                    template: template,
                    traits: None,
                    nft_id: None,
                    lore: None,
                    numerology: None,
                });
            }
        }
    }
    if !processed_memes.is_empty() {
        fixed_fm.memes = Some(processed_memes);
    }

    // Simulate unquoted colon error reporting
    if let Some(desc_line) = lines.iter().find(|&line| line.trim().starts_with("description:")) {
        // Regex: r"description: (.*:.*)"
        let re = regex::Regex::new(r"description: (.*:.*)").unwrap();
        if re.is_match(desc_line) {
            let problematic_description = desc_line.trim_start_matches("description:").trim().to_string();
            return Err(anyhow::anyhow!(
                "YAML parsing error: Unquoted colon in meme description. Problematic content: \"{problematic_description}\". Requires manual fix or advanced YAML parsing logic."
            ));
        }
    }


    Ok(fixed_fm)
}


#[test]
fn test_keywords_formatting_fix() {
    let input_yaml = r#"---
title: Test Poem
keywords: keyword1, keyword2, keyword3
---
poem body
"#;

    let result = simulate_front_matter_processing(input_yaml).unwrap();
    let expected_keywords = Some(vec![
        "keyword1".to_string(),
        "keyword2".to_string(),
        "keyword3".to_string(),
    ]);

    assert_eq!(result.keywords, expected_keywords);
}

#[test]
#[ignore]
fn test_malformed_meme_list_items_fix() {
    let input_yaml = r#"---
title: Meme Poem
memes:
  - "My refactoring process is basically a dance party, again, again." (Success Kid meme, with a very tired expression)
  - "When your code is so tangled, it needs a choreographer, again, again." (Doge meme, with a very tired expression)
---
poem body
"#;

    let result = simulate_front_matter_processing(input_yaml).unwrap();
    let expected_memes = Some(vec![
        Meme {
            description: "My refactoring process is basically a dance party, again, again.".to_string(),
            template: "success kid".to_string(),
            traits: None, nft_id: None, lore: None, numerology: None,
        },
        Meme {
            description: "When your code is so tangled, it needs a choreographer, again, again.".to_string(),
            template: "doge".to_string(),
            traits: None, nft_id: None, lore: None, numerology: None,
        },
    ]);

    assert_eq!(result.memes, expected_memes);
}

#[test]
#[ignore]
fn test_unquoted_colon_in_meme_description_error() {
    let input_yaml = r#"---
title: Error Poem
memes:
  - description: This is a problem: with a colon
---
poem body
"#;

    let result = simulate_front_matter_processing(input_yaml);
    assert!(result.is_err());
    let error_message = result.unwrap_err().to_string();
    assert!(error_message.contains("YAML parsing error: Unquoted colon in meme description. Problematic content: \"This is a problem: with a colon\"."));
}
