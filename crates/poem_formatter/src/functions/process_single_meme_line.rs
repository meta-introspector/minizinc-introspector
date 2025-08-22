use anyhow::Result;
use poem_traits::PoemFrontMatterTrait; // Import the trait

#[poem_macros::poem_function]
#[allow(dead_code)]
pub fn process_single_meme_line(
    line: &str,
    _captures: Vec<String>, // We might not use captures directly here, but the macro expects it
    fixed_fm: &mut dyn PoemFrontMatterTrait, // Changed type to trait object
) -> Result<()> { // Changed return type to anyhow::Result
    // This function will be transformed by the macro.
    // For now, it will just print the line and return Ok.
    println!("Processing meme line: {}", line);
    // Example: Store something in fixed_fm if the macro needs to interact with it
    if let Some(s) = fixed_fm.get_pending_meme_description_mut() {
        *s = line.to_string();
    }
    Ok(())
}
