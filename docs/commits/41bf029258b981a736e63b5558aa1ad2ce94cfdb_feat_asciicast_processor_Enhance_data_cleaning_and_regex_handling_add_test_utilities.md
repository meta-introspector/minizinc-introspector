feat(asciicast_processor): Enhance data cleaning and regex handling; add test utilities

This commit enhances the `asciicast_processor` crate with improved data cleaning, testing utilities, and refined regex handling, along with integrating `serde` and `serde_yaml` for potential data serialization.

Key changes include:
- Integration of `clean_non_ascii` from `gemini_utils::string_processor` for robust output data cleaning.
- Addition of new binaries `test_strip` and `test_cleaning` for focused testing of data processing functionalities.
- Refined regex handling in `src/main.rs` for the `CountRaw` command, utilizing `regex::escape` for safer pattern matching.
- Integration of `serde` and `serde_yaml` dependencies, laying the groundwork for structured data processing within the `asciicast_processor`.
- The commit message "wip broken" suggests this was a work in progress that might have introduced temporary issues, which were likely resolved in subsequent commits.

This work contributes to the robustness and testability of the `asciicast_processor`, a key component for processing asciinema recordings.