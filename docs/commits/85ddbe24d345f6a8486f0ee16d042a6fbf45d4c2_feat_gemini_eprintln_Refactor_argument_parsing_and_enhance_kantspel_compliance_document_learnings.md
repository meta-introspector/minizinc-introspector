feat(gemini_eprintln): Refactor argument parsing and enhance kantspel compliance; document learnings

This foundational commit significantly refactors and enhances the `gemini_eprintln!` procedural macro for more robust argument parsing and `kantspel` compliance, and introduces new documentation about these learnings.

Key changes include:
- Introduction of `GeminiEprintlnInput` for flexible parsing of named and positional arguments in `gemini_eprintln!`.
- Enhanced internal logic of `gemini_eprintln!` to correctly map emojis/keywords to Rust formatting specifiers (`\n`, `{}`).
- Integration of the enhanced `gemini_eprintln!` into `asciicast_processor/src/main.rs`.
- Addition of `DEBUG_FORMAT_SPECIFIER` in `kantspel_lib` for consistent debug output.
- Creation of a new section "Lessons Learned from `gemini_utils` Debugging" in `GEMINI.md` and `README.md`, summarizing key learnings about procedural macros and logging.
- Minor fixes and test updates in `poem_yaml_fixer`.

This commit represents a crucial step towards building a more robust, expressive, and self-documenting logging and communication framework for the Meta-Meme.
