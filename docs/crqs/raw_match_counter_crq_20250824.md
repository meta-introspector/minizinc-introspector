# Change Request: Raw Match Counter Feature in `asciicast_processor`

**Date:** August 24, 2025
**Author:** Gemini CLI

## Description

This Change Request (CRQ) documents the addition of a new `CountRaw` subcommand to the `asciicast_processor` crate. This feature allows users to count occurrences of a specified regex pattern directly within the raw input stream of an asciicast file, providing immediate feedback on the presence of patterns before full parsing.

## Changes Implemented

### 1. `crates/asciicast_processor/src/cli.rs`

*   **Removed `count_raw_matches` from `Args` struct:** The `count_raw_matches` argument was moved from a top-level optional argument in the `Args` struct to a dedicated subcommand.
*   **Added `CountRaw` subcommand to `Commands` enum:** A new subcommand `CountRaw` was introduced, which takes a `regex` argument.

### 2. `crates/asciicast_processor/src/main.rs`

*   **Removed `if let Some(raw_regex) = &args.count_raw_matches` block:** The logic for handling raw match counting was removed from the main function's initial execution block.
*   **Added new arm to `match args.command` for `Commands::CountRaw`:** A new arm was added to the `match args.command` statement to handle the `CountRaw` subcommand. This arm calls the `count_and_print_raw_matches` function from the `raw_parser` module.

### 3. `crates/asciicast_processor/src/raw_parser.rs`

*   **New file created:** A new module `raw_parser.rs` was created.
*   **`count_and_print_raw_matches` function:** This function was implemented to read the input file line by line, apply a regex filter, and print matching lines along with a total count.

## Rationale and Impact

This feature provides a quick and efficient way to check for the presence of specific patterns in raw asciicast data without requiring full parsing and processing of the entire stream. This is particularly useful for debugging, data exploration, and quickly verifying assumptions about the content of asciicast files. By moving this functionality to a dedicated subcommand, the CLI interface remains clean and intuitive.

## Verification

The feature has been verified by successfully building the `asciicast_processor` crate and running the `CountRaw` subcommand with a test regex.

```bash
cargo run -p asciicast_processor --release docs/asciicast21.cast CountRaw --regex "mini-act"
```
(Expected output: Raw matches found for "mini-act" in docs/asciicast21.cast)

## Future Considerations

*   Adding options for context lines in `CountRaw` subcommand.
*   Integrating `CountRaw` output with other tools or reports.
