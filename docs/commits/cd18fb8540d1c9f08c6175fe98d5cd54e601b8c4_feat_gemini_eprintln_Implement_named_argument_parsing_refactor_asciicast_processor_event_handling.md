feat(gemini_eprintln): Implement named argument parsing; refactor asciicast_processor event handling

This foundational commit implements a new, more robust named argument parsing and `kantspel` compliant logging system for the `gemini_eprintln!` procedural macro, and significantly refactors `asciicast_processor` to handle event limits and tail processing.

Key changes include:
- Significant refactoring of `gemini_utils::gemini_eprintln!` to handle named arguments, and to correctly map emojis/keywords to Rust formatting specifiers.
- Introduction of `Input` struct for flexible parsing of named arguments in `gemini_eprintln!`.
- Refactoring of `asciicast_processor/src/main.rs` to handle `limit` and `tail` arguments for event processing, including collecting all events first if `tail` is specified.
- Updates to `gemini_utils::string_processor` to prioritize matching multi-character emojis/keywords.

This commit represents a crucial step towards building a more robust, expressive, and self-documenting logging and communication framework for the Meta-Meme, and enhances `asciicast_processor`'s event processing capabilities.