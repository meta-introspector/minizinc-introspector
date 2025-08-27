feat(gemini_utils): Enhance kantspel emoji mapping and add token generator

This commit enhances the `kantspel` emoji/keyword mapping within `gemini_utils` and introduces a token generator for the `gemini_eprintln!` macro.

Key changes include:
- Significant updates to the `EMOJIS` HashMap in `gemini_utils/src/string_processor/mod.rs` for more comprehensive `kantspel` translations.
- Creation of `gemini_utils/src/token_generator/generate_eprintln_tokens.rs` and `gemini_utils/src/token_generator/mod.rs` to handle the generation of the final `eprintln!` TokenStream.

This work further refines the `gemini_utils` crate's ability to provide expressive and `kantspel`-compliant logging.