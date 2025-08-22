use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
use crate::manual_parser::manual_parse_poem_file;

#[test]
fn test_manual_parser_meme_handling() -> Result<()> {
    let poem_content = r#"---
title: Test Poem
memes:
  - description: This is a test meme description.
    template: test_template
---
Poem body content.
"#;

    let mut fixed_fm = FixedFrontMatter::default();
    manual_parse_poem_file(poem_content, &mut fixed_fm)?;

    // Assertions
    assert_eq!(fixed_fm.title, Some("Test Poem".to_string()));

    // Check meme parsing
    let memes = fixed_fm.memes.unwrap();
    assert_eq!(memes.len(), 1);
    assert_eq!(memes[0].description, "This is a test meme description.".to_string());
    assert_eq!(memes[0].template, "test_template".to_string());

    Ok(())
}
