// crates/poem_macro_test_crate/src/lib.rs
use poem_macros::poem_function;
use poem_traits::PoemFrontMatterTrait;


#[poem_function]
fn my_poem_function(
    _line: &str,
    _captures: Vec<String>,
    _fixed_fm: &mut dyn PoemFrontMatterTrait,
) -> anyhow::Result<()> { // Explicitly use anyhow::Result
    Ok(())
}