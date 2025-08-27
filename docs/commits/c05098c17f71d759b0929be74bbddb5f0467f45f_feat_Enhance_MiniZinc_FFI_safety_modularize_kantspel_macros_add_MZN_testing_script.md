feat: Enhance MiniZinc FFI safety; modularize kantspel_macros; add MZN testing script

This foundational commit enhances FFI safety in `minizinc_macro`, modularizes `kantspel_macros`, introduces a comprehensive MiniZinc file testing script, and removes `minizinc_introspector`.

Key changes include:
- Marking FFI calls in `minizinc_macro` as `unsafe` and updating corresponding functions in `minizinc_ffi` for improved safety.
- Significant refactoring of `kantspel_macros` to modularize macro implementations and regex mappings into dedicated modules (`macros/kantspel_regex.rs`, `macros/kantspel_transform.rs`, `regex_maps.rs`).
- Introduction of `test_all_mzn_files.sh`, a new script for automated testing of all MiniZinc files in the repository, excluding standard library and documentation examples.
- Removal of the `minizinc_introspector` crate.
- Minor regex fix in `regex_extractor/src/main.rs`.
- Updates to `Cargo.lock` and `Cargo.toml` to reflect these changes.

This commit represents a significant step towards improving the robustness, testability, and organization of the project's core components.