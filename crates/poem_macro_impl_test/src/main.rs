use anyhow::Result;
//use poem_yaml_fixer::functions::create_function_registry::FUNCTIONS;
//use poem_yaml_fixer::functions::types::FixedFrontMatter;
use std::collections::HashMap;

fn main() -> Result<()> {
    // This is the dummy function that the macro would take as input
    fn my_dummy_function(
        line: &str,
        captures: Vec<String>,
        fixed_fm: &mut FixedFrontMatter,
    ) -> Result<()> {
        println!("my_dummy_function called with line: {}", line);
        println!("Captures: {:?}", captures);
        fixed_fm.insert("dummy_key".to_string(), "dummy_value".to_string());
        Ok(())
    }

    // This is the helper function that the macro would generate
    #[doc(hidden)]
    pub fn __get_fn_my_dummy_function() -> Box<dyn Fn(&str, Vec<String>, &mut FixedFrontMatter) -> Result<()> + Send + Sync + 'static> {
        Box::new(|line, captures, fixed_fm| {
            my_dummy_function(line, captures, fixed_fm)
        })
    }

    // This is the static item that the macro would generate to register the function
    #[linkme::distributed_slice(FUNCTIONS)]
    static __REGISTER_FN_my_dummy_function: &'static (String, fn() -> Box<dyn Fn(&str, Vec<String>, &mut FixedFrontMatter) -> Result<()> + Send + Sync + 'static>)
        = &{
        (stringify!(my_dummy_function).to_string(), __get_fn_my_dummy_function)
    };

    println!("Attempting to retrieve and call 'my_dummy_function' from registry...");

    let mut found_function = None;
    for (name, func_ptr) in FUNCTIONS.iter() {
        if name == "my_dummy_function" {
            found_function = Some(func_ptr());
            break;
        }
    }

    if let Some(func) = found_function {
        let mut fm = HashMap::new();
        println!("Calling 'my_dummy_function' via the retrieved closure...");
        func("test line", vec!["cap1".to_string(), "cap2".to_string()], &mut fm)?;
        println!("Fixed Front Matter after call: {:?}", fm);
    } else {
        eprintln!("Error: 'my_dummy_function' not found in registry.");
    }

    Ok(())
}
