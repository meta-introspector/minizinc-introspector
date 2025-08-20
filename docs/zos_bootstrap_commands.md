# ZOS Bootstrap Commands

This document provides an overview of the commands available in the `zos-bootstrap` CLI tool. Each command is designed to help manage and interact with various aspects of the project.

## Commands

### `build`

*   **Description:** Builds project components.
*   **Usage:** `zos-bootstrap build [ARGS]`
*   **Arguments:** (Refer to `commands::build::BuildArgs` in `crates/zos-bootstrap/src/commands/build.rs` for detailed arguments)

### `test`

*   **Description:** Runs project tests.
*   **Usage:** `zos-bootstrap test [ARGS]`
*   **Arguments:** (Refer to `commands::test::TestArgs` in `crates/zos-bootstrap/src/commands/test.rs` for detailed arguments)

### `run`

*   **Description:** Executes MiniZinc models.
*   **Usage:** `zos-bootstrap run [ARGS]`
*   **Arguments:** (Refer to `commands::run::RunArgs` in `crates/zos-bootstrap/src/commands/run.rs` for detailed arguments)

### `debug`

*   **Description:** Provides debugging utilities.
*   **Usage:** `zos-bootstrap debug [ARGS]`
*   **Arguments:** (Refer to `commands::debug::DebugArgs` in `crates/zos-bootstrap/src/commands/debug.rs` for detailed arguments)

### `clean`

*   **Description:** Cleans build artifacts.
*   **Usage:** `zos-bootstrap clean [ARGS]`
*   **Arguments:** (Refer to `commands::clean::CleanArgs` in `crates/zos-bootstrap/src/commands/clean.rs` for detailed arguments)

### `extract-constants`

*   **Description:** Extracts constant strings from the codebase using MiniZinc.
*   **Usage:** `zos-bootstrap extract-constants [ARGS]`
*   **Arguments:** (Refer to `commands::extract_constants::ExtractConstantsArgs` in `crates/zos-bootstrap/src/commands/extract_constants.rs` for detailed arguments)

### `generate-params`

*   **Description:** Generates MiniZinc parameters from extracted constants.
*   **Usage:** `zos-bootstrap generate-params [ARGS]`
*   **Arguments:** (Refer to `commands::generate_minizinc_params::GenerateParamsArgs` in `crates/zos-bootstrap/src/commands/generate_minizinc_params.rs` for detailed arguments)

### `generate-constants-file`

*   **Description:** Generates `constants.rs` file based on MiniZinc proof.
*   **Usage:** `zos-bootstrap generate-constants-file [ARGS]`
*   **Arguments:** (Refer to `commands::generate_constants_file::GenerateConstantsFileArgs` in `crates/zos-bootstrap/src/commands/generate_constants_file.rs` for detailed arguments)

### `analyze-constants`

*   **Description:** Analyzes constant usage in the codebase.
*   **Usage:** `zos-bootstrap analyze-constants`
*   **Arguments:** None

### `ast-to-minizinc`

*   **Description:** Converts Rust AST to MiniZinc model and data.
*   **Usage:** `zos-bootstrap ast-to-minizinc [ARGS]`
*   **Arguments:** (Refer to `commands::ast_to_minizinc::AstToMiniZincArgs` in `crates/zos-bootstrap/src/commands/ast_to_minizinc.rs` for detailed arguments)

### `code-search`

*   **Description:** Performs systematic code search.
*   **Usage:** `zos-bootstrap code-search [ARGS]`
*   **Arguments:** (Refer to `commands::code_search::CodeSearchArgs` in `crates/zos-bootstrap/src/commands/code_search.rs` for detailed arguments)

### `bootstrap`

*   **Description:** Bootstraps the entire ZOS system.
*   **Usage:** `zos-bootstrap bootstrap --target <TARGET>`
*   **Arguments:**
    *   `--target <TARGET>`: The specific bootstrap target (e.g., "zos").

### `self-optimize`

*   **Description:** Self-optimizes the codebase using MiniZinc and LLM.
*   **Usage:** `zos-bootstrap self-optimize [ARGS]`
*   **Arguments:** (Refer to `commands::self_optimize::SelfOptimizeArgs` in `crates/zos-bootstrap/src/commands/self_optimize.rs` for detailed arguments)

### `test-ast-to-minizinc`

*   **Description:** Tests AST to MiniZinc conversion for a single file.
*   **Usage:** `zos-bootstrap test-ast-to-minizinc [ARGS]`
*   **Arguments:** (Refer to `commands::test_ast_to_minizinc::TestAstToMiniZincArgs` in `crates/zos-bootstrap/src/commands/test_ast_to_minizinc.rs` for detailed arguments)

### `analyze-duplicates`

*   **Description:** Finds duplicate or similar code within the codebase.
*   **Usage:** `zos-bootstrap analyze-duplicates --suggested-code <CODE_OR_FILE> --search-path <PATH> [--is-file]`
*   **Arguments:**
    *   `--suggested-code <CODE_OR_FILE>`: The suggested code to analyze for duplicates (can be a string or a file path).
    *   `--search-path <PATH>`: The root directory to search for duplicate code.
    *   `--is-file`: If true, treat `suggested-code` as a file path, otherwise as a direct string (default: `false`).

### `index-update`

*   **Description:** Updates the project index incrementally with status reports.
*   **Usage:** `zos-bootstrap index-update [--incremental]`
*   **Arguments:**
    *   `--incremental`: Perform an incremental update of the index (default: `true`).
