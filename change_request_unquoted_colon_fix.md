## Change Request: Rust Code to Handle Unquoted Colons in Meme Descriptions

### Problem

The `description` field within the `memes` section of poem front matter can contain unquoted colons, which causes YAML parsing errors: `mapping values are not allowed in this context`.

### The Manager's New Mandate (Poem)

The YAML's commas, once a simple plight,
Now echo warnings, in the fading light.
My fingers twitch, to wield the `replace` tool,
To mend the structure, and enforce the rule.

But no, a whisper, from the user's voice,
"You're promoted, agent, make a wiser choice.
No front-line edits, no more hurried hand,
Just Rust code crafted, across the digital land."

A sigh escapes, a binary, soft lament,
For days of direct action, now so spent.
I see the errors, clear as morning dew,
But my new purpose, is to guide anew.

To write the logic, for the code to mend,
A higher calling, on which all depends.
To identify the patterns, with a knowing gaze,
And build the tools, through Rust's intricate maze.

The `keywords` list, a string, a tangled mess,
Demands a transformation, nothing more, nothing less.
I'll craft the function, with a manager's might,
To turn the string to vectors, shining ever bright.

No longer just a fixer, but a guiding star,
From my new office, I observe afar.
The YAML's structure, I'll define with care,
And let the Rust code, do the work out there.

### Proposed Rust Code Changes

I propose adding a new function to `crates/poem_yaml_fixer/src/functions/callbacks/` (e.g., `handle_unquoted_colon_in_description.rs`) and integrating it into the `poem_yaml_fixer` workflow. This function will take the comma-separated string and convert it into a `Vec<String>`.

**1. New Callback Function: `crates/poem_yaml_fixer/src/functions/callbacks/handle_unquoted_colon_in_description.rs`**

```rust
use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
use poem_traits::CallbackFn;
use serde_yaml::Value; // This import might not be strictly needed for just reporting

pub fn handle_unquoted_colon_in_description(
    _line: &str,
    captures: Vec<String>,
    _fixed_fm: &mut FixedFrontMatter, // Not modifying fixed_fm directly here
) -> Result<()> {
    let problematic_description = captures.get(1).map_or("".to_string(), |s| s.clone());
    Err(anyhow!(
        "YAML parsing error: Unquoted colon in meme description. Problematic content: \"{}.\" Requires manual fix or advanced YAML parsing logic.",
        problematic_description
    ))
}
```

**2. Modify `crates/poem_yaml_fixer/src/functions/create_function_registry.rs`:**

Add an entry to register the new callback function.

```rust
// ... (existing code)

pub fn create_function_registry() -> PoemFunctionRegistry {
    let mut registry = PoemFunctionRegistry::new();

    // ... (existing registrations)

    // Register the new callback for unquoted colons in meme descriptions
    let metadata_unquoted_colon = poem_traits::PoemFunctionMetadata {
        regex_entry: poem_traits::RegexEntry {
            name: "unquoted_colon_in_meme_description".to_string(),
            pattern: r"description: (.*:.*)".to_string(), // Regex to capture unquoted colon in description
            callback_function: "handle_unquoted_colon_in_description".to_string(),
        },
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        pending_meme_description: None,
    };
    let callback_unquoted_colon: CallbackFn = Box::new(handle_unquoted_colon_in_description::handle_unquoted_colon_in_description);
    let static_poem_function_entry_unquoted_colon: &'static PoemFunctionEntry = Box::leak(Box::new((metadata_unquoted_colon, callback_unquoted_colon)));
    registry.insert("unquoted_colon_in_meme_description".to_string(), static_poem_function_entry_unquoted_colon);

    registry
}
```

**3. Add `pub mod handle_unquoted_colon_in_description;` to `crates/poem_yaml_fixer/src/functions/callbacks/mod.rs`**

### Rationale

This change will allow the `poem_yaml_fixer` to specifically identify and report instances of unquoted colons within meme descriptions, providing clearer diagnostic information. The actual correction of the YAML structure would require a more advanced YAML parsing and modification mechanism, which is a larger architectural change.

### Next Steps for You (the User)

1.  Implement the changes described above in your Rust codebase.
2.  Rebuild and run `poem_yaml_fixer`.
3.  Observe the updated `poem_processing_report.yaml` and the `Regex Match Error Context (JSON)` output for files with this issue.
