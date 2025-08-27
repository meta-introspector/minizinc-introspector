feat: Integrate gemini_eprintln into asciicast_processor; enhance kantspel emoji mapping

This commit integrates the `gemini_eprintln!` procedural macro into the `asciicast_processor` crate for standardized logging and significantly enhances the `kantspel` emoji/keyword mapping within `gemini_utils`.

Key changes include:
- Replacement of standard `eprintln!` calls with `gemini_eprintln!` in `asciicast_processor/src/main.rs`, ensuring all logging adheres to project standards.
- Extensive modifications to the `EMOJIS` HashMap in `gemini_utils/src/string_processor/mod.rs`, introducing new mappings for improved `kantspel` compliance and expressive logging.
- Updates to `gemini_utils_test/src/lib.rs` to reflect the new `kantspel` syntax in `gemini_eprintln!` calls.

This work is a crucial step towards consistent and semantically rich communication within the project's tools.