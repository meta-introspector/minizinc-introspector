use anyhow::Result;
use crate::functions::types::FixedFrontMatter;

pub fn handle_memes_start_regex(
    _line: &str,
    _captures: Vec<String>,
    fixed_front_matter: &mut FixedFrontMatter,
) -> Result<()> {
    // Initialize memes as an empty vector if it's None
    if fixed_front_matter.memes.is_none() {
        fixed_front_matter.memes = Some(Vec::new());
    }
    Ok(())
}
