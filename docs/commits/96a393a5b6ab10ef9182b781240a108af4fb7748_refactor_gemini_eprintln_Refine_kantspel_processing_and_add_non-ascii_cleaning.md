refactor(gemini_eprintln): Refine kantspel processing and add non-ascii cleaning

This commit refines the `gemini_eprintln!` macro's `kantspel` processing logic and introduces a utility function for cleaning non-ASCII characters.

Key changes include:
- Improved handling of `::brick::` (mapping to `{}`) and `::inspect::` (mapping to `{:?}`) placeholders within `gemini_eprintln!`.
- Addition of `clean_non_ascii` function in `gemini_utils::string_processor` to filter out non-ASCII characters, enhancing data cleanliness.
- Minor adjustments to `gemini_eprintln!`'s internal argument handling and debugging output.

This work further strengthens the `kantspel` principles applied in `gemini_eprintln!` and improves the robustness of string processing within the project.