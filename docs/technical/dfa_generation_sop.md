# Standard Operating Procedure: DFA Module Generation

## 1. Purpose
This SOP describes the process for generating Deterministic Finite Automata (DFA) modules from a comprehensive list of terms. These modules are used for efficient term recognition within the `ragit` project. The generation process ensures terms are split into manageable files and filenames are sanitized to prevent issues with non-ASCII characters.

## 2. Scope
This SOP applies to the `crates/vocabulary_dfa_generator` crate and its interaction with the `all_terms.txt` input file and the `crates/vocabulary_dfa_lib/src` output directory.

## 3. Input
The primary input is the `all_terms.txt` file, located at `/data/data/com.termux/files/home/storage/github/libminizinc/all_terms.txt`. This file contains a newline-separated list of terms.

## 4. Output
Generated DFA modules are Rust source files (`.rs`) located in subdirectories within `/data/data/com.termux/files/home/storage/github/libminizinc/crates/vocabulary_dfa_lib/src`.

## 5. Generation Process

The `crates/vocabulary_dfa_generator/src/main.rs` script performs the following steps:

### 5.1. Term Filtering
Terms from `all_terms.txt` are filtered based on these criteria:
*   Must not be empty.
*   Must start with an ASCII alphabetic character.
*   Must not be purely numeric or purely hexadecimal.

### 5.2. Term Splitting and Directory Structure
Filtered terms are organized into a hierarchical directory and file structure:
*   **First-Level Split (by First Character):** Terms are initially grouped by their first character (converted to lowercase ASCII). A subdirectory is created for each unique sanitized first character (e.g., `a/`, `b/`).
*   **Second-Level Split (by Second Character, if needed):** If a group of terms sharing the same first character exceeds `MAX_TERMS_PER_FILE` (currently 1000 terms), it is further split by its second character (converted to lowercase ASCII).
    *   Files for these groups are named `[sanitized_first_char][sanitized_second_char]_dfa.rs` (e.g., `ab_dfa.rs` within the `a/` subdirectory).
*   **Single-Level Split:** If a group of terms sharing the same first character does not exceed `MAX_TERMS_PER_FILE`, all terms in that group are written to a single file named `[sanitized_first_char]_dfa.rs` (e.g., `a_dfa.rs` within the `a/` subdirectory).

### 5.3. Filename Sanitization
To ensure compatibility and prevent issues with non-ASCII characters in filenames, all characters used in directory and file names are sanitized:
*   ASCII alphanumeric characters remain unchanged.
*   Non-ASCII alphanumeric characters are converted to their Unicode hexadecimal representation, prefixed with `U` (e.g., `U02C7` for `ˇ`). This ensures uniqueness and avoids collisions.

### 5.4. Module Content
Each generated `.rs` file contains:
*   `use regex::Regex;`
*   A public function `matches_X_Y(text: &str) -> bool` (where `X` and `Y` are the sanitized first and optional second characters).
*   A `regex::Regex` pattern that is a logical OR (`|`) concatenation of all terms belonging to that specific module.

## 6. Execution
To regenerate the DFA modules, run the `crates/vocabulary_dfa_generator` crate:
```bash
cargo run --package vocabulary_dfa_generator
```

## 7. Maintenance
*   The `MAX_TERMS_PER_FILE` constant in `src/main.rs` can be adjusted to control the granularity of splitting.
*   The `is_purely_numeric` and `is_purely_hex` functions can be refined if additional filtering criteria are needed.
*   The `sanitize_filename_char` function ensures filename compatibility. If new character handling is required, this function should be updated.
