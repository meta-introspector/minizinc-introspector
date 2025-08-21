//use poem_macros::poem_function;
use anyhow::Result;
use std::collections::HashMap;

//#[poem_function]
fn my_poem_function(
    _line: &str,
    _captures: Vec<String>,
    _fixed_fm: &mut HashMap<String, String>,
) -> Result<()> {
    Ok(())
}
