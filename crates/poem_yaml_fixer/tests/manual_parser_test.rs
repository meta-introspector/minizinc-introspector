use anyhow::Result;
use poem_yaml_fixer::functions::types::FixedFrontMatter;
use poem_yaml_fixer::manual_parser::manual_parse_poem_file;

#[test]
fn test_manual_parser_simple_fields() -> Result<()> {
    let poem_content = r#"---
title: Test Poem
summary: A test summary.
keywords: keyword1, keyword2
emojis: ✨📝
art_generator_instructions: Generate a test image.
---
Poem body content.
"#;

    let mut fixed_fm = FixedFrontMatter::default();
    manual_parse_poem_file(poem_content, &mut fixed_fm)?;

    // Assertions for simple fields
    assert_eq!(fixed_fm.title, Some("Test Poem".to_string()));
    assert_eq!(fixed_fm.summary, Some("A test summary.".to_string()));
    assert_eq!(fixed_fm.keywords, Some(vec!["keyword1".to_string(), "keyword2".to_string()]));
    assert_eq!(fixed_fm.emojis, Some("✨📝".to_string()));
    assert_eq!(fixed_fm.art_generator_instructions, Some("Generate a test image.".to_string()));

    // Ensure memes and poem_body are not populated by this simplified parser
    assert!(fixed_fm.memes.is_none() || fixed_fm.memes.as_ref().unwrap().is_empty());
    assert!(fixed_fm.poem_body.is_none());

    Ok(())
}