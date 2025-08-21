use anyhow::Result;
use syn::{parse_quote, ItemFn};
use poem_macro_impl::poem_function_impl;
//use std::collections::HashMap;

fn main() -> Result<()> {
    // Define a dummy function as if it were input to the macro
    let input_fn: ItemFn = parse_quote! {
        fn my_dummy_function(
            line: &str,
            captures: Vec<String>,
            fixed_fm: &mut std::collections::HashMap<String, String>,
        ) -> anyhow::Result<(), anyhow::Error> {
            // This is a dummy implementation
            println!("Dummy function executed: line={}, captures={{:?}}", line, captures);
            fixed_fm.insert("dummy_key".to_string(), "dummy_value".to_string());
            Ok(())
        }
    };

    // Call the macro implementation function directly
    let expanded_tokens = poem_function_impl(input_fn);

    // Print the generated code
    println!("Generated Code:\n{}", expanded_tokens);

    Ok(())
}

