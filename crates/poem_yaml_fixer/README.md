# `poem_yaml_fixer`

A Rust command-line tool designed to process Markdown files containing YAML front matter, primarily focusing on standardizing the front matter structure and providing basic YAML error patching.

## Overview

This tool reads Markdown files (specifically those in `docs/poems/` or a specified file path) that are expected to contain YAML front matter delimited by `---`. It parses this front matter, extracts recognized fields, and then re-serializes it into a standardized YAML format based on the `FixedFrontMatter` schema. The tool aims to ensure consistency in the structure of the YAML front matter across poem files. It also includes functionality to attempt to patch common YAML parsing errors.

## Current Functionality

*   **YAML Front Matter Standardization:** The core function is to standardize the YAML front matter. It expects a `---` delimited YAML block at the very beginning of the Markdown file. Recognized fields are `title`, `summary`, `keywords`, `emojis`, `art_generator_instructions`, `memes`, `poem_body`, and `pending_meme_description`. Fields not part of this schema will be discarded during re-serialization.
*   **`poem_body` Extraction:** If a `poem_body` field is found within the YAML front matter, its content is extracted and handled separately, then re-inserted into the main Markdown content.
*   **YAML Error Patching:** The tool can attempt to fix common YAML parsing errors by applying predefined patch functions. Currently, the `quote_line` patch function is available to automatically quote problematic lines in the YAML front matter.
*   **Content Change Safety:** The tool calculates the percentage of content change after processing. If this change exceeds a user-defined maximum (defaulting to 1%), the processing is aborted to prevent unintended significant modifications.
*   **Debug Mode:** A debug flag allows for printing the standardized YAML front matter to the console.

## Limitations / Future Potential

The tool's architecture supports more advanced regex-based transformations and additional callback functions for processing `memes` within the front matter. While the `quote_line` patch is active, other such features are not yet fully integrated into the main processing pipeline. The `load_regex_config` function is currently a dummy implementation, and the `process_memes_with_workflow` function is not fully utilized for complex transformations.

## Usage

```bash
# Process all Markdown files in docs/poems/
cargo run --package poem_yaml_fixer

# Process a specific file
cargo run --package poem_yaml_fixer -- --file /path/to/your/poem.md

# Process with a higher allowed change percentage (e.g., 5%)
cargo run --package poem_yaml_fixer -- --max-change-percentage 5.0

# Enable debug output
cargo run --package poem_yaml_fixer -- --debug

# Process with the 'quote_line' patch function to fix YAML errors
cargo run --package poem_yaml_fixer -- --patch-function quote_line
```

## Expected Input Format

Markdown files should start with YAML front matter delimited by `---` on separate lines:

```markdown
---
title: "My Awesome Poem"
summary: "A brief summary of the poem."
keywords: "poem, awesome, markdown"
emojis: "✨📝"
art_generator_instructions: "Generate an image of a starry night."
memes:
  - name: "meme_one"
    description: "Description for meme one."
  - name: "meme_two"
    description: "Description for meme two."
poem_body: |
  This is the first line of the poem.
  This is the second line.
pending_meme_description: "A description for a pending meme."
---

This is the main body of the Markdown file,
which will follow the YAML front matter.
```

## Development

To build and run the tool:

```bash
cargo build --package poem_yaml_fixer
cargo run --package poem_yaml_fixer
```