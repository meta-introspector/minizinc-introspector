feat(asciicast_processor): Enhance data cleaning, regex, and testing; add documentation

This commit significantly enhances the `asciicast_processor` crate with improved data cleaning, refined regex handling, and new testing utilities. It also introduces new documentation files outlining future tasks and summarizing recent build fixes.

Key changes include:
- Integration of `clean_non_ascii` from `gemini_utils::string_processor` for robust output data cleaning.
- Addition of new binaries `test_strip` and `test_cleaning` for focused testing of data processing functionalities.
- Refined regex handling in `src/main.rs` for the `CountRaw` command, utilizing `regex::escape` for safer pattern matching.
- Integration of `serde` and `serde_yaml` dependencies, laying the groundwork for structured data processing within the `asciicast_processor`.
- Creation of `docs/next_task_asciicast_processor_fix.md` to outline the next steps for fixing filtering logic.
- Creation of `docs/recent_build_fixes_and_gemini_utils_refinement.md` to summarize recent build fixes and `gemini_utils` refinement.

This work contributes to the robustness, testability, and documentation of the `asciicast_processor`, a key component for processing asciinema recordings.