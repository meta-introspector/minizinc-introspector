# CRQ: `poem_yaml_fixer` Refactoring and Quality Improvement Plan

**Date:** August 22, 2025

## 1. Current State Overview

The `poem_yaml_fixer` tool has been enhanced with a `--dry-run` flag and the ability to load regex configurations from a TOML file, enabling its "awk-like" functionality for processing `memes` within YAML front matter. New poems have been added to `docs/poems/`, and a `README.md` has been created for the crate. A checkpoint commit has been made.

## 2. Identified Issues (Warnings, Unused Code, TODOs/FIXMEs)

During a recent code review, several areas for improvement were identified, categorized by priority:

### High Priority (Addressed in Checkpoint Commit)

*   Removed `#[allow(dead_code)]` from `extract_front_matter.rs`, `types.rs` (for `RegexEntry`, `RegexConfig`), `process_memes_with_workflow.rs`, `handle_unmatched_regex_error.rs`.
*   Removed commented-out `//Captures` from `process_memes_with_workflow.rs`.
*   Updated `process_single_file.rs` to accept and pass the `dry_run` parameter.
*   Cleaned up `create_function_registry.rs` by removing commented-out blocks, renaming callback functions to `snake_case`, removing leading underscores, and removing `#[allow(non_snake_case)]`.
*   Updated hardcoded path suggestion in `handle_unmatched_regex_error.rs` to be more generic.

### Medium Priority (To be addressed)

1.  **`process_poem_file.rs` - Unused Imports/Functions:** `parse_front_matter_fields`, `extract_words_from_text`, `save_word_index`, and `WordIndex` are imported but not used.
    *   **Plan:** Investigate the intended use of these components. If they are part of a planned future feature (e.g., word indexing), add explicit `TODO` comments explaining their purpose and future integration. Otherwise, remove them to reduce code bloat and warning noise.
2.  **`types.rs` - `WordIndex` unused:** This struct is defined but not used.
    *   **Plan:** Same as above.
3.  **`process_word_indexing.rs` - Not called:** This function is implemented but not integrated into the main processing flow.
    *   **Plan:** If word indexing is a desired feature, integrate this function into `process_poem_file.rs` or a new dedicated processing step.
4.  **`process_word_indexing.rs` - Type Mismatch:** The `WordIndex` struct expects `HashMap<String, HashMap<String, usize>>` but `process_word_indexing` tries to insert `Vec<String>`. This is a **critical type mismatch**.
    *   **Plan:** Correct the `WordIndex` struct definition or the usage in `process_word_indexing.rs` to ensure type consistency.
5.  **`process_word_indexing.rs` - Hardcoded path:** The hardcoded path `docs/word_index.yaml` should be made configurable.
    *   **Plan:** Introduce a CLI argument or configuration option for the word index path.
6.  **`save_word_index.rs` - `#[allow(dead_code)]`:** This attribute is still present.
    *   **Plan:** Remove this attribute once `save_word_index` is actually used (after `process_word_indexing` is integrated).
7.  **`process_all_files.rs` and `process_single_file.rs` - Not called:** These functions are implemented but not called from `main.rs`. `main.rs` directly implements their logic.
    *   **Plan:** Refactor `main.rs` to use `process_all_files` and `process_single_file` for better modularity and error handling.

### Low Priority (Minor improvements)

1.  **`save_word_index.rs` - Commented-out `use` statements:** Remove these.
2.  **`process_all_files.rs` and `process_single_file.rs` - Long `function_registry` type signature:**
    *   **Plan:** Consider introducing a `type alias` for `CallbackFn` or the full function signature to improve readability.

## 3. Action Plan

The immediate next steps are to:

1.  **Run `cargo fix --allow-dirty`:** Attempt to automatically fix some of the remaining warnings.
2.  **Manually address remaining warnings:** Go through the compiler warnings and fix them one by one, focusing on:
    *   Unused imports.
    *   Unused functions (deciding whether to integrate or remove).
    *   Type mismatches (especially `WordIndex`).
    *   Hardcoded paths.
    *   Refactoring `main.rs` to use `process_all_files` and `process_single_file`.
3.  **Verify:** After each set of fixes, run `cargo build` to ensure no new errors are introduced and warnings are reduced.
4.  **Update `README.md`:** Reflect any significant changes in functionality or usage.
5.  **QA:** Re-run the QA plan for `poem_yaml_fixer` to ensure all functionality (especially the new "awk-like" features) works as expected.

This plan aims to systematically improve the code quality, reduce warning noise, and ensure the `poem_yaml_fixer` tool is robust and well-documented.
