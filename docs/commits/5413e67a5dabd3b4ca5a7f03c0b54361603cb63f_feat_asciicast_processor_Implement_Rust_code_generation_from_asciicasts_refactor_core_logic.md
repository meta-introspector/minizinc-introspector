feat(asciicast_processor): Implement Rust code generation from asciicasts; refactor core logic

This foundational commit introduces significant refactoring and new features to the `asciicast_processor` crate, enabling the generation of Rust code from asciicast recordings.

Key changes include:
- Extensive refactoring of `asciicast_processor/src/main.rs` to act as a CLI dispatcher, delegating core logic to new modules.
- Introduction of `src/cli.rs`, `src/pattern_generator.rs`, and `src/rust_parser.rs` to handle CLI argument parsing, hierarchical pattern generation, and Rust code parsing for patterns, respectively.
- Addition of new binaries `test_strip` and `test_cleaning` for testing data processing.
- Minor refinements to `gemini_utils` and `kantspel_macros` related to `gemini_eprintln!` argument handling.
- Update of the `grex` submodule.

This work is a crucial step towards the project's "meme compilation" workflow, allowing for the static typing and compilation of patterns derived from terminal interactions.