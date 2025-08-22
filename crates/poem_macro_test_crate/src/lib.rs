// crates/poem_macro_test_crate/src/lib.rs
use poem_macros::poem_function;

use std::{
	  collections::HashMap
};

#[poem_function]
fn my_poem_function(
    _line: &str,
    _captures: Vec<String>,
    _fixed_fm: &mut HashMap<String, String>,
) -> Result<()> {
    Ok(())
}
