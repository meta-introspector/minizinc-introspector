# SOP: Onboarding N00b Interns for MiniZinc FFI Development

## 1. Purpose
This Standard Operating Procedure (SOP) provides a guide for new interns joining the MiniZinc FFI development team. It outlines the current state of the project, details the immediate tasks related to code refactoring and documentation, and provides essential tools and resources for successful onboarding.

## 2. Scope
This SOP applies to all new interns assigned to the `libminizinc` project, specifically focusing on the Rust Foreign Function Interface (FFI) development and documentation.

## 3. Current Project State

### 3.1. Documentation Restructuring
The project documentation (`docs/` directory) is currently undergoing a major restructuring to improve organization and readability. New subdirectories have been created for different documentation categories (e.g., `architecture/`, `development/`, `ffi/`, `minizinc/`, `sops/`, `glossary/`, `tutorials/`, `reference/`). Existing documents have been moved to their respective new locations.

### 3.2. MiniZinc FFI Refactoring
The Rust FFI for MiniZinc (`tools/minizinc_ffi/src/lib.rs`) is being refactored to adhere to the "one declaration per file" principle. The goal is to make `lib.rs` solely a module declaration and re-export file, with all actual code residing in smaller, logically grouped files within submodules.

**Current Progress on FFI Refactoring:**
*   **Opaque Type Declarations:** All unique `pub struct` declarations have been extracted from `lib.rs` and moved to `tools/minizinc_ffi/src/types/mod.rs`.
*   **Individual `impl` Blocks:** Several `impl` blocks have been extracted into separate files (e.g., `float_lit.rs`, `set_lit.rs`, `bool_lit.rs`, `string_lit.rs`, `id.rs`, `environment.rs`, `model.rs`, `item.rs`, `vardecl.rs`, `typeinst.rs`, `expression.rs`, `array_lit.rs`).

## 4. Intern Responsibilities and Immediate Tasks

Your primary responsibility will be to continue the `lib.rs` splitting task and ensure the project remains functional and well-documented throughout the process.

### 4.1. Familiarization
*   **Project Structure:** Explore the `libminizinc` repository, paying close attention to the `tools/minizinc_ffi/` directory and the newly restructured `docs/` directory.
*   **Documentation:** Read through the existing documentation, especially:
    *   `docs/index.md` (once created)
    *   `docs/ffi/ffi_overview.md` (if exists)
    *   `docs/ffi/minizinc_ffi_enhancements.md`
    *   `docs/sops/debugging_sop.md`
*   **Rust Module System:** Ensure you have a solid understanding of Rust's module system, `mod` declarations, and `pub use` statements.

### 4.2. Continuing `lib.rs` Splitting

The remaining `impl` blocks and the `unsafe extern "C"` block in `tools/minizinc_ffi/src/lib.rs` need to be moved to separate files.

**Detailed Steps:**

1.  **Identify Remaining Blocks:** Open `tools/minizinc_ffi/src/lib.rs`. Identify any `impl` blocks that have not yet been moved, and the main `unsafe extern "C"` block.
2.  **Create New Files:** For each identified `impl` block, create a new `.rs` file in the `tools/minizinc_ffi/src/` directory (or a new subdirectory if logical grouping is needed, e.g., `tools/minizinc_ffi/src/expression_impls/`).
    *   For the `unsafe extern "C"` block, create `tools/minizinc_ffi/src/ffi_bindings.rs`.
    *   For the `#[cfg(test)] mod tests` block, create `tools/minizinc_ffi/src/tests/mod.rs` (and create `tools/minizinc_ffi/src/tests/` directory if it doesn't exist).
3.  **Move Content:** Cut the content of the `impl` block or `unsafe extern "C"` block from `lib.rs` and paste it into its new file.
4.  **Update `lib.rs`:** In `tools/minizinc_ffi/src/lib.rs`, replace the moved content with appropriate `mod` declarations and `pub use` statements.
    *   For example, if you move `impl MiniZincModel { ... }` to `tools/minizinc_ffi/src/model.rs`, you would add `mod model;` and `pub use model::MiniZincModel;` (or `pub use model::*;` if all items in `model.rs` should be public).
5.  **Update `use` statements in new files:** Ensure that any `use` statements within the newly created files are correct and point to the right locations (e.g., `use super::*` or `use crate::types::MiniZincModel;`).
6.  **Iterate and Verify:** Repeat this process for all remaining blocks. After each significant move, attempt to build the project (`cargo build`) to catch any errors early.

### 4.3. Generating `pub use` Statements (Script Assistance)

To assist with updating `lib.rs` with `pub use` statements, you can use a bash script.

**Script: `scripts/editor/generate_pub_uses.sh` (To be created)**

This script will read the `tools/minizinc_ffi/src/` directory, identify all `.rs` files (excluding `lib.rs` and `mod.rs` files within subdirectories), and generate the corresponding `pub use` statements.

```bash
#!/bin/bash

# This script generates pub use statements for lib.rs based on the file structure.
# It should be run from the project root.

LIB_RS_DIR="tools/minizinc_ffi/src"

echo "// Generated by generate_pub_uses.sh - DO NOT EDIT MANUALLY"
echo ""

# Find all .rs files directly under LIB_RS_DIR, excluding lib.rs
find "$LIB_RS_DIR" -maxdepth 1 -type f -name "*.rs" ! -name "lib.rs" | sort | while read -r file; do
    module_name=$(basename "$file" .rs)
    echo "pub mod ${module_name};"
    echo "pub use ${module_name}::*;"
    echo ""
done

# Find all subdirectories under LIB_RS_DIR, excluding feature_tests
find "$LIB_RS_DIR" -maxdepth 1 -type d ! -name "$(basename "$LIB_RS_DIR")" ! -name "feature_tests" | sort | while read -r dir; do
    module_name=$(basename "$dir")
    echo "pub mod ${module_name};"
    echo "pub use ${module_name}::*;"
    echo ""
done
```

**Usage:**
1.  Run the script: `./scripts/editor/generate_pub_uses.sh > tools/minizinc_ffi/src/lib.rs.new`
2.  Review `lib.rs.new` and manually merge it into `tools/minizinc_ffi/src/lib.rs`, adding necessary `use` statements at the top.

## 5. Tools and Resources
*   **Git:** For version control, committing changes in small logical chunks.
*   **`cargo`:** For building and testing Rust code (`cargo build`, `cargo test`).
*   **`sed` (with caution):** For simple, non-contextual text replacements.
*   **`scripts/editor/generate_pub_uses.sh`:** To assist in generating `pub use` statements.
*   **MiniZinc Documentation:** Refer to the official MiniZinc documentation for language specifics.
*   **Rust Documentation:** Refer to the official Rust documentation for language features and best practices.

## 6. Communication and Collaboration
*   **Regular Updates:** Provide regular updates on your progress.
*   **Ask Questions:** Do not hesitate to ask questions if you are unsure about any task or encounter difficulties.
*   **Small Commits:** Commit your changes frequently in small, logical chunks with clear commit messages.
*   **Branching:** Work on a dedicated feature branch for your tasks.

## 7. Context Break
As requested, we will now take a context break. This SOP provides a clear roadmap for the next phase of development.
