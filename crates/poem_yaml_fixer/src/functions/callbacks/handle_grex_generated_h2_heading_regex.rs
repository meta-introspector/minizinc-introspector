use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
use poem_macros::poem_function;
use poem_traits::PoemFrontMatterTrait;

#[poem_function(
    name = "grex_generated_h2_heading",
    pattern = r"^##\s+.*$",
    callback_function = "handle_grex_generated_h2_heading_regex",
)]
pub fn handle_grex_generated_h2_heading_regex(
    _line: &str,
    _captures: Vec<String>,
    _fixed_front_matter: &mut dyn PoemFrontMatterTrait,
) -> Result<()>
{
    // TODO: Implement logic for this regex
    Ok(())
}

