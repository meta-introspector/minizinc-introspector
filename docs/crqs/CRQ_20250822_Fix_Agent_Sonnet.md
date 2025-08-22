## Change Request: Fix Malformed Front Matter in `agent_sonnet.md`

**Date:** 2025-08-22

**Author:** Gemini CLI Agent (Rust Edit Manager)

**Problem Description:**
The file `/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems/agent_sonnet.md` is currently malformed. It contains multiple, incomplete, or incorrectly formatted YAML front matter blocks, likely due to previous merge conflicts or manual edits. This prevents automated processing by tools like `fix_meme_yaml`, which expect a single, well-formed YAML front matter block at the very beginning of the file (starting at line 0).

**Impact:**
- Prevents `fix_meme_yaml` from correctly parsing and cleaning the poem.
- Hinders automated processing and analysis of poem metadata.
- Introduces inconsistencies in the project's documentation.

**Proposed Solution (Rust Code):**
To address this, I propose the following Rust code snippet. This function will read the `agent_sonnet.md` file, identify the last valid YAML front matter block, extract the corresponding poem body, and then rewrite the file with a single, correctly formatted front matter and the poem content.

```rust
use std::{fs, path::Path};
use anyhow::{Result, anyhow};
use serde_yaml;

/// Fixes the malformed YAML front matter in a given poem file.
/// It identifies the last valid YAML block and reconstructs the file
/// with only that block and the subsequent poem body.
fn fix_agent_sonnet_poem(file_path: &Path) -> Result<()> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Find all YAML front matter delimiters (---)
    let mut delimiter_indices = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.trim() == "---" {
            delimiter_indices.push(i);
        }
    }

    // Assuming the last complete YAML block is the desired one.
    // A valid YAML block needs at least two '---' delimiters.
    // We'll look for the last pair that forms a complete block.
    let mut target_fm_start = None;
    let mut target_fm_end = None;

    for i in (0..delimiter_indices.len()).rev() {
        if i > 0 {
            let end_idx = delimiter_indices[i];
            let start_idx = delimiter_indices[i-1];

            // Check if this looks like a valid YAML block (e.g., not just a single ---)
            // And if the content between them can be parsed as YAML
            let potential_fm_lines = &lines[start_idx + 1 .. end_idx];
            let potential_fm_str = potential_fm_lines.join("\n");

            if serde_yaml::from_str::<serde_yaml::Value>(&potential_fm_str).is_ok() {
                target_fm_start = Some(start_idx);
                target_fm_end = Some(end_idx);
                break; // Found the last valid block
            }
        }
    }

    let (fm_start, fm_end) = match (target_fm_start, target_fm_end) {
        (Some(start), Some(end)) => (start, end),
        _ => return Err(anyhow!("Could not find a valid YAML front matter block in {:?}", file_path)),
    };

    let front_matter_lines = &lines[fm_start + 1 .. fm_end];
    let poem_body_lines = &lines[fm_end + 1 ..];

    let new_content = format!(
        "---\n{}\n---\n{}",
        front_matter_lines.join("\n"),
        poem_body_lines.join("\n")
    );

    fs::write(file_path, new_content)?;

    Ok(())
}

// Example usage (to be integrated into a main function or test):
// fn main() -> Result<()> {
//     let poem_path = Path::new("/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems/agent_sonnet.md");
//     fix_agent_sonnet_poem(poem_path)
// }
```

**Rationale:**
This approach adheres to the new operational guidelines by:
- **Identifying the issue:** The Rust code uses logic and parsing attempts to identify the correct front matter.
- **Proposing Rust code:** The solution is provided as a Rust function that can be integrated into an existing utility or a new script.
- **Avoiding direct edits:** I am not performing the file modification myself, but providing the means for it to be done programmatically.

**Recommended Action:**
Integrate the `fix_agent_sonnet_poem` function into a suitable Rust utility within the project (e.g., a new binary in `crates/` or an extension to `fix_meme_yaml` if appropriate) and execute it to clean `agent_sonnet.md`.

**Verification:**
After applying the fix, run `cargo run --package fix_meme_yaml` again to ensure that `agent_sonnet.md` is now correctly processed without errors.
