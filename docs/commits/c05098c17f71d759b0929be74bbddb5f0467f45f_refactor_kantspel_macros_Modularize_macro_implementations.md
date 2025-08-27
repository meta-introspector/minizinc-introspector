refactor(kantspel_macros): Modularize macro implementations

This commit refactors the `kantspel_macros` crate to improve modularity and organization of its macro implementations.

Key changes include:
- Delegation of `kantspel_regex` and `kantspel_transform` macro implementations to dedicated modules (`macros/kantspel_regex.rs` and `macros/kantspel_transform.rs`).
- Extraction of `REGEX_EMOJIS` and `REGEX_ALIASES` HashMaps into a new `regex_maps.rs` module for better separation of concerns.

This refactoring enhances the maintainability and clarity of the `kantspel` implementation, aligning with the project's principles of structured development.