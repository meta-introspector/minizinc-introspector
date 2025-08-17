# SOP: Constant Management and Deduplication in Rust Codebase

## 1. Purpose
This Standard Operating Procedure (SOP) defines a systematic process for identifying, analyzing, and refactoring constants (`const` and `static`) within the Rust codebase. The primary objectives are to enforce the "one constant, one declaration" rule, identify constants used more than once, determine their originating module, and plan for their relocation to dedicated constant modules (e.g., `constants.rs`, `config.rs`, `types.rs`) to improve modularity, maintainability, and code clarity.

## 2. Scope
This SOP applies to all Rust source files (`*.rs`) within the project's root directory `/data/data/com.termux/files/home/storage/github/libminizinc`.

## 3. Procedure

### Phase 1: Code Ingestion and Abstract Syntax Tree (AST) Parsing
**Objective:** To parse all Rust files into Abstract Syntax Trees (ASTs) to accurately extract constant declarations and track all identifier usages.

**Steps:**
1.  **Identify Rust Files:** Recursively traverse the project directory to locate all files with a `.rs` extension.
2.  **Read File Content:** For each identified Rust file, read its entire content into a string.
3.  **Parse to AST:** Use the `syn` crate to parse the file content into a Rust Abstract Syntax Tree (AST).
4.  **Extract Constant Declarations:** During AST traversal, identify all `const` and `static` item declarations. For each, record its name and the absolute path of the file where it is declared.
5.  **Collect All Identifiers:** During AST traversal, collect every identifier encountered. This comprehensive list will be used in Phase 2 to count constant usages.

### Phase 2: Constant Usage Analysis
**Objective:** To count the occurrences of each declared constant across the entire codebase and flag those used more than once.

**Steps:**
1.  **Map Declarations:** Create a central map (e.g., `HashMap`) where the key is the constant's name (as a string) and the value is a data structure containing its declaration path and an initial usage count of zero.
2.  **Count Usages:** Iterate through the comprehensive list of all identifiers collected in Phase 1. For each identifier, if it matches the name of a declared constant, increment that constant's usage count in the central map.
3.  **Flag Duplication:** After counting, iterate through the central map. Any constant with a `usage_count` greater than 1 is flagged as being reused.

### Phase 3: Constant Location and Refactoring Planning
**Objective:** To determine the defining module for each constant and formulate a plan for relocation if it does not adhere to the "one constant, one declaration" rule in a dedicated constants file.

**Steps:**
1.  **Analyze Declaration Path:** For each flagged constant, examine its `declaration_path` (identified in Phase 1).
2.  **Categorize Constants:**
    *   **Dedicated Constant File:** If the constant's declaration file name (e.g., `constants.rs`, `config.rs`, `types.rs`, or similar project-specific dedicated constant files) or its immediate parent directory clearly indicates it's a module specifically for constants, mark it as compliant.
    *   **Non-Dedicated File:** If the constant is declared in any other type of file (e.g., a file containing function implementations, struct definitions, etc.), mark it for potential refactoring.
3.  **Generate Report:** Produce a comprehensive report that includes:
    *   The name of each constant.
    *   Its absolute declaration path.
    *   Its total usage count across the project.
    *   A clear flag if it is used more than once.
    *   A recommendation to move the constant to a dedicated constants module if it is flagged for reuse and not already in a dedicated constant file.

## 4. Tools
*   **Rust Programming Language:** The primary language for implementing the analysis tool.
*   **`cargo`:** Rust's package manager and build system.
*   **`syn` crate:** For parsing Rust source code into Abstract Syntax Trees (ASTs).
*   **`walkdir` crate:** For efficiently traversing the file system to find Rust files.
*   **`std::collections::HashMap`:** For efficient storage and retrieval of constant information and usage counts.

## 5. Expected Outcome
*   A detailed, machine-generated report identifying all constants, their usage counts, and flagging those that violate the "one constant, one declaration" rule.
*   A clear plan for refactoring constants that are reused but not declared in dedicated constant modules.
*   Improved code organization, reduced technical debt, and enhanced maintainability of the Rust codebase.

---

# Analysis: `syn` Usage Contrast in `libminizinc`

This section provides a review and contrast of existing `syn` usage patterns within the `libminizinc` project, specifically within the `zos-bootstrap` crate, compared to the newly introduced `constant_analyzer` crate.

## 1. Files Reviewed
The following files were reviewed for their `syn` usage:
*   `/data/data/com.termux/files/home/storage/github/libminizinc/crates/zos-bootstrap/src/utils/error.rs`
*   `/data/data/com.termux/files/home/storage/github/libminizinc/crates/zos-bootstrap/src/commands/extract_constants.rs`
*   `/data/data/com.termux/files/home/storage/github/libminizinc/crates/zos-bootstrap/src/code_analysis/string_extractor.rs`
*   `/data/data/com.termux/files/home/storage/github/libminizinc/crates/zos-bootstrap/src/code_analysis/constant_usage_proof.rs`

## 2. Contrast Analysis

### 2.1. `crates/zos-bootstrap/src/utils/error.rs`
*   **Existing Usage:** This file defines `ZosError` and includes an `impl From<syn::Error> for ZosError`. This is a standard and robust pattern for converting `syn`'s parsing errors into the project's custom error type, ensuring consistent error handling.
*   **`constant_analyzer` Contrast:** The `constant_analyzer` crate currently propagates `syn::Error` directly via `Box<dyn std::error::Error>`. While functional, adopting a custom error type and `From` implementation would align `constant_analyzer` with the project's established error handling conventions if its scope expands.

### 2.2. `crates/zos-bootstrap/src/commands/extract_constants.rs`
*   **Existing Usage:** This file acts as an orchestrator. It does not directly use `syn` but calls functions from `string_extractor` and `constant_usage_proof` that do. It handles file globbing and overall command execution flow.
*   **`constant_analyzer` Contrast:** The `constant_analyzer`'s `main` function currently combines the orchestration, file globbing, and parsing logic into a single, self-contained unit. This simpler approach is suitable for its current focused utility.

### 2.3. `crates/zos-bootstrap/src/code_analysis/string_extractor.rs`
*   **Existing Usage:** This is a direct and significant user of `syn`. It defines `StringExtractor`, which implements `syn::visit::Visit` to traverse the AST. Its primary goal is to extract *string literals* from `const` items, `static` items, and general expressions. It also attempts to track module and function context during traversal.
*   **`constant_analyzer` Contrast:**
    *   **Similarities:** Both `string_extractor` and `constant_analyzer` utilize the `syn::visit::Visit` trait for AST traversal and `syn::parse_file` for parsing. Both are interested in `const` and `static` items.
    *   **Differences:** `string_extractor` is specialized in extracting *string literals*, while `constant_analyzer` is focused on identifying *all `const` and `static` declarations* (regardless of their type) and counting their overall usage across the codebase. `string_extractor`'s efforts to capture module/function context are more granular than `constant_analyzer`'s current approach, which only records the declaration file path.

### 2.4. `crates/zos-bootstrap/src/code_analysis/constant_usage_proof.rs`
*   **Existing Usage:** This file also uses `syn::visit::Visit` (via `ConstantUsageVisitor`) to find *usages* of constants, specifically those accessed through a `constants::` path (e.g., `constants::MY_CONSTANT`). It then compares these usages against a set of constants defined in a specific `constants.rs` file to identify unused constants.
*   **`constant_analyzer` Contrast:**
    *   **Similarities:** Both are concerned with constant usage analysis, employ `syn::visit::Visit` and `syn::parse_file`, and iterate through Rust files.
    *   **Differences:** `constant_usage_proof`'s usage detection is more targeted, looking for constants accessed via a specific module path. `constant_analyzer` takes a broader approach, collecting all identifiers and then counting occurrences of any declared `const`/`static` name, regardless of its access path. `constant_usage_proof` aims to "prove" usage for a predefined set of constants, whereas `constant_analyzer` aims to identify and flag *any* reused constant for potential refactoring.

## 3. Conclusion
The existing `syn` implementations within the `zos-bootstrap` crate demonstrate a solid and idiomatic use of `syn` for various code analysis tasks. The `constant_analyzer` crate's approach to using `syn` (parsing files, employing the visitor pattern, and traversing the AST) is consistent with these established patterns within the project.

This review confirms that the chosen methodology for `constant_analyzer` is well-aligned with the project's existing code analysis practices. As `constant_analyzer` evolves, especially if it expands to tasks like "similar code" detection, it could benefit from adopting the project's custom error handling and potentially a more modular internal structure similar to `zos-bootstrap`'s `code_analysis` sub-modules.
