## Change Request: Rust Code to Fix `keywords` field in Poem Front Matter

### Problem

The `keywords` field in the front matter of many poem Markdown files (e.g., `docs/poems/the_omniverse_deployers_ode.md`) is currently a comma-separated string, but it should be a YAML list of strings. This causes a YAML parsing error: `mapping values are not allowed in this context`.

### Manager's Lament (Poem)

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

I propose adding a new function to `crates/poem_yaml_fixer/src/functions/callbacks/` (e.g., `handle_comma_separated_keywords.rs`) and integrating it into the `poem_yaml_fixer` workflow. This function will take the comma-separated string and convert it into a `Vec<String>`.

**1. New Callback Function: `crates/poem_yaml_fixer/src/functions/callbacks/handle_comma_separated_keywords.rs`**

```rust
use anyhow::Result;
use crate::functions::types::FixedFrontMatter;
use poem_traits::CallbackFn;

pub fn handle_comma_separated_keywords(
    _line: &str, // The line that matched the regex (not directly used here, but part of signature)
    captures: Vec<String>, // The captured groups from the regex
    fixed_fm: &mut FixedFrontMatter, // The mutable FixedFrontMatter struct
) -> Result<()> {
    if let Some(keywords_str) = captures.get(1) { // Assuming the keywords string is the first captured group
        let keywords: Vec<String> = keywords_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        fixed_fm.keywords = Some(keywords);
    }
    Ok(())
}

// You would also need to add this to src/functions/callbacks/mod.rs:
// pub mod handle_comma_separated_keywords;
```

**2. Modify `crates/poem_yaml_fixer/src/functions/create_function_registry.rs`:**

This function is responsible for registering callback functions. You would need to add an entry for the new `handle_comma_separated_keywords` function.

```rust
// ... (existing code)

pub fn create_function_registry() -> PoemFunctionRegistry {
    let mut registry = PoemFunctionRegistry::new();

    // ... (existing registrations)

    // Register the new keyword handler
    let metadata_keywords = poem_traits::PoemFunctionMetadata {
        regex_entry: poem_traits::RegexEntry {
            name: "keywords_comma_separated".to_string(),
            pattern: "keywords: (.*)".to_string(), // This regex will capture the comma-separated string
            callback_function: "handle_comma_separated_keywords".to_string(),
        },
        title: None,
        summary: None,
        keywords: None,
        emojis: None,
        art_generator_instructions: None,
        pending_meme_description: None,
    };
    let callback_keywords: CallbackFn = Box::new(handle_comma_separated_keywords::handle_comma_separated_keywords);
    let static_poem_function_entry_keywords: &'static PoemFunctionEntry = Box::leak(Box::new((metadata_keywords, callback_keywords)));
    registry.insert("handle_comma_separated_keywords".to_string(), static_poem_function_entry_keywords);

    registry
}
```

**3. Modify `crates/poem_yaml_fixer/src/functions/types.rs`:**

Ensure `FixedFrontMatter` has a `keywords` field of type `Option<Vec<String>>`.

```rust
// ... (existing code)

#[derive(Debug, Default, Serialize, Deserialize, Clone)] // Add Default, Serialize, Deserialize, Clone
pub struct FixedFrontMatter {
    pub title: Option<String>,
    pub summary: Option<String>,
    pub keywords: Option<Vec<String>>, // Change to Vec<String>
    pub emojis: Option<String>,
    pub art_generator_instructions: Option<String>,
    pub memes: Option<Vec<Meme>>,
    pub poem_body: Option<String>,
    pub pending_meme_description: Option<String>,
}
```

### Rationale

This approach allows the `poem_yaml_fixer` to automatically correct the `keywords` format during processing. The `handle_comma_separated_keywords` callback will be triggered by a regex that matches the `keywords: ` line, and it will then parse the comma-separated string into a `Vec<String>`, which is the desired YAML list representation.

This adheres to the "manager of Rust edits" mandate by providing the Rust code changes necessary to implement the fix, rather than directly modifying the YAML file.

### Next Steps for You (the User)

1.  Implement the changes described above in your Rust codebase.
2.  Rebuild and run `poem_yaml_fixer`.
3.  Observe the updated `poem_processing_report.yaml` to see if `the_omniverse_deployers_ode.md` now processes successfully.


--- Implementation Plan ---

**Phase 1: Implement `keywords` field fix**

This phase focuses on converting comma-separated `keywords` strings into proper YAML lists.

1.  **Create New Callback Function File:**
    *   Create a new file: `/data/data/com.termux/files/home/storage/github/libminizinc/crates/poem_yaml_fixer/src/functions/callbacks/handle_comma_separated_keywords.rs`
    *   Add the following Rust code to this file:
        ```rust
        use anyhow::Result;
        use crate::functions::types::FixedFrontMatter;
        use poem_traits::CallbackFn;

        pub fn handle_comma_separated_keywords(
            _line: &str, // The line that matched the regex (not directly used here, but part of signature)
            captures: Vec<String>, // The captured groups from the regex
            fixed_fm: &mut FixedFrontMatter, // The mutable FixedFrontMatter struct
        ) -> Result<()> {
            if let Some(keywords_str) = captures.get(1) { // Assuming the keywords string is the first captured group
                let keywords: Vec<String> = keywords_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                fixed_fm.keywords = Some(keywords);
            }
            Ok(())
        }
        ```

2.  **Register the New Module:**
    *   Open the file: `/data/data/com.termux/files/home/storage/github/libminizinc/crates/poem_yaml_fixer/src/functions/callbacks/mod.rs`
    *   Add the following line to declare the new module:
        ```rust
        pub mod handle_comma_separated_keywords;
        ```

3.  **Update `FixedFrontMatter` Structure:**
    *   Open the file: `/data/data/com.termux/files/home/storage/github/libminizinc/crates/poem_yaml_fixer/src/functions/types.rs`
    *   Ensure the `FixedFrontMatter` struct has a `keywords` field of type `Option<Vec<String>>` and derives `Default`, `Serialize`, `Deserialize`, and `Clone`. It should look like this:
        ```rust
        #[derive(Debug, Default, Serialize, Deserialize, Clone)]
        pub struct FixedFrontMatter {
            pub title: Option<String>,
            pub summary: Option<String>,
            pub keywords: Option<Vec<String>>, // Ensure this line is correct
            pub emojis: Option<String>,
            pub art_generator_instructions: Option<String>,
            pub memes: Option<Vec<Meme>>,
            pub poem_body: Option<String>,
            pub pending_meme_description: Option<String>,
        }
        ```

4.  **Register Callback in Function Registry:**
    *   Open the file: `/data/data/com.termux/files/home/storage/github/libminizinc/crates/poem_yaml_fixer/src/functions/create_function_registry.rs`
    *   Add the following code within the `create_function_registry()` function to register the new callback. Place it alongside existing registrations:
        ```rust
        // Register the new keyword handler
        let metadata_keywords = poem_traits::PoemFunctionMetadata {
            regex_entry: poem_traits::RegexEntry {
                name: "keywords_comma_separated".to_string(),
                pattern: "keywords: (.*)".to_string(), // This regex will capture the comma-separated string
                callback_function: "handle_comma_separated_keywords".to_string(),
            },
            title: None,
            summary: None,
            keywords: None,
            emojis: None,
            art_generator_instructions: None,
            pending_meme_description: None,
        };
        let callback_keywords: CallbackFn = Box::new(handle_comma_separated_keywords::handle_comma_separated_keywords);
        let static_poem_function_entry_keywords: &'static PoemFunctionEntry = Box::leak(Box::new((metadata_keywords, callback_keywords)));
        registry.insert("handle_comma_separated_keywords".to_string(), static_poem_function_entry_keywords);
        ```

---
**Implementation Status:** Implemented in commit `1903c331be7903ead77c74f5d3fb15ea7dc049ac`.
