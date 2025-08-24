# Change Request: Raw Match Counter and Processing Consistency Feature in `asciicast_processor`

**Date:** August 24, 2025
**Author:** Gemini CLI

## Description

This Change Request (CRQ) documents significant enhancements to the `asciicast_processor` crate, introducing a dedicated `count-raw` subcommand for raw input analysis and an implicit consistency check within the `Filter` subcommand.

The primary goal is to provide robust tools for verifying the presence of specific regex patterns in asciicast data at different processing stages:
*   **`count-raw` subcommand:** Allows users to count occurrences of a specified regex pattern directly within the raw input stream of an asciicast file. This provides immediate feedback on the presence of patterns before any processing or stripping of ANSI escape codes.
*   **Implicit Panic in `Filter` subcommand:** A critical new feature that automatically triggers a panic if a regex pattern, found in the raw asciicast input, is *not* found in the processed (cleaned) output of the `Filter` subcommand. This ensures that patterns expected to persist through processing are indeed present, aiding in debugging and data integrity verification.

## Changes Implemented

### 1. `crates/asciicast_processor/src/cli.rs`

*   **`CountRaw` subcommand name:** The `CountRaw` subcommand was updated to use `#[command(name = "count-raw")]` for kebab-case CLI usage.
*   **No new arguments:** No new arguments were added to existing subcommands, adhering to the principle of leveraging existing inputs.

### 2. `crates/asciicast_processor/src/main.rs`

*   **`CountRaw` subcommand re-implementation:** The logic for the `count-raw` subcommand was re-implemented to directly use `regex::Regex` for pattern matching and `std::io::{BufReader, BufRead}` for reading the input file line by line. This provides a direct raw match counting and printing functionality.
*   **Implicit Panic Logic in `Filter` subcommand:**
    *   The `Filter` subcommand now performs an internal check using `raw_parser::check_raw_matches` to determine if the provided `regex` is present in the raw asciicast input.
    *   After processing the asciicast events and cleaning the output lines (stripping ANSI escape codes, etc.), the `Filter` subcommand checks if the `regex` is present in the `cleaned_output_lines`.
    *   If the `regex` was found in the raw input *and* was *not* found in the processed output, the application will `panic!("String found in raw input but not in processed output!")`. This behavior is implicit and does not require any additional CLI arguments.

### 3. `crates/asciicast_processor/src/raw_parser.rs`

*   **Function Renaming and Signature Change:** The `count_and_print_raw_matches` function was renamed to `check_raw_matches`. Its signature was changed to `pub fn check_raw_matches(file_path: &std::path::PathBuf, regex_pattern: &str) -> Result<bool>`, making it a utility function that returns `true` if a match is found in the raw input, and `false` otherwise. All printing functionality was removed from this function.

## Rationale and Impact

These enhancements provide more robust debugging and verification capabilities for asciicast processing. The `count-raw` subcommand offers a direct way to inspect raw input, while the implicit panic in the `Filter` subcommand acts as a critical safeguard, immediately alerting developers to discrepancies between raw and processed data. This is particularly useful for identifying issues where formatting or escape codes might be inadvertently stripped, leading to loss of critical information. The changes adhere to the principle of not adding unnecessary CLI arguments, integrating the new logic seamlessly into existing commands.

## Verification

The features have been verified through internal testing during development.

*   **`count-raw` subcommand:**
    ```bash
    cargo run -p asciicast_processor --release docs/asciicast21.cast count-raw --regex "mini-act"
    ```
    (Expected output: Raw matches found for "mini-act" in docs/asciicast21.cast, along with the matching lines.)

*   **Implicit Panic in `Filter` subcommand:**
    To trigger the panic, use a regex that is present in the raw asciicast input (e.g., an ANSI escape code like `^\\e`) but is stripped during processing.
    ```bash
    cargo run -p asciicast_processor --release docs/asciicast21.cast Filter --regex "^\\\e"
    ```
    (Expected behavior: The application should panic with the message "String found in raw input but not in processed output!")

## Future Considerations

*   Further refinement of panic messages to provide more context.
*   Potential for configurable panic behavior (e.g., warning instead of panic) if user feedback indicates a need for less strict enforcement.