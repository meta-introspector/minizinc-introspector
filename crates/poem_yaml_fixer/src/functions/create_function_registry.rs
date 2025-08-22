use anyhow::Result;
use linkme::distributed_slice;
use poem_traits::{PoemFrontMatterTrait, Meme}; // Import Meme from poem_traits

// Define the distributed slice where functions will register themselves.
// This static is populated by functions annotated with #[poem_macros::poem_function]
// and is used by the poem_header! macro to create the function registry.
#[distributed_slice]
#[allow(non_upper_case_globals)] // Suppress warnings for macro-generated static variables
pub static FUNCTIONS2: [&'static (String, fn() -> Box<dyn Fn(&str, Vec<String>, &mut dyn PoemFrontMatterTrait) -> Result<(), anyhow::Error> + Send + Sync + 'static>)];

// Removed: pub mod callbacks;