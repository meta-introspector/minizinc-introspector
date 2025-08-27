feat(gemini_eprintln): Implement robust argument parsing and kantspel-compliant logging; add documentation

This foundational commit implements a new, more robust argument parsing and `kantspel` compliant logging system for the `gemini_eprintln!` procedural macro, and extensively documents these changes across `GEMINI.md`, `README.md`, and a new SOP.

Key changes include:
- Significant refactoring of `gemini_utils::gemini_eprintln!` to handle both named and positional arguments, and to correctly map emojis/keywords to Rust formatting specifiers.
- Integration of the enhanced `gemini_eprintln!` into `asciicast_processor/src/main.rs`.
- Extensive updates to `GEMINI.md` and `README.md` to detail the new logging preference and `kantspel` principles.
- Creation of `docs/sops/gemini_eprintln_kantspel_sop.md`, a new SOP outlining the strict usage rules for `gemini_eprintln!` and `kantspel` principles.

This commit represents a crucial step towards building a more robust, expressive, and self-documenting logging and communication framework for the Meta-Meme.