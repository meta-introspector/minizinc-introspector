# Standard Operating Procedure: Constant Deduplication

## 1. Purpose

This SOP outlines the process for identifying and centralizing duplicate literal values used in constant declarations within the codebase. The rule enforced is "one constant, one declaration per unique literal value." This practice improves code maintainability, reduces errors, and enhances readability by ensuring a single source of truth for common literal values.

## 2. Scope

This SOP applies to all Rust source code (`*.rs`) within the project that declares constants (`const` and `static`).

## 3. Procedure

### 3.1. Running the Constant Analyzer

1.  Navigate to the root directory of the project.
2.  Execute the `constant_analyzer` program using Cargo:

    ```bash
    cargo run --package constant_analyzer
    ```

    This command will analyze the Rust source code, extract literal values of constants, count their occurrences, and generate a report.

### 3.2. Interpreting the Report

1.  The analysis report is generated at `./reports/constant_analysis_report.txt`.
2.  Open the `constant_analysis_report.txt` file.
3.  The report lists constants grouped by their literal value. For each unique literal value, it provides:
    *   **Constant:** The literal value itself.
    *   **Declared in:** The file path where one of the constants holding this literal value is declared.
    *   **Usage Count:** The number of times this exact literal value is declared across the codebase.
    *   **STATUS:**
        *   `FLAGGED (Used more than once)`: Indicates that this literal value is declared in multiple constants. This is a violation of the "one constant, one declaration per unique literal value" rule.
        *   `OK (Used once or not at all)`: Indicates that this literal value is declared only once (or not at all, if it's a constant that was found but not used in any other constant declaration).
    *   **PLAN:** For `FLAGGED` constants, a suggestion to centralize its definition is provided.
    *   **NOTE:** Information about whether the constant is already in a dedicated constants file (e.g., `constants.rs`, `config.rs`, `types.rs`) or a directory named `constants` or `config` or `types`.

### 3.3. Refactoring Steps

For each constant flagged with `FLAGGED (Used more than once)`:

1.  **Identify the primary declaration:** Choose one of the declarations of the duplicated literal value to be the canonical source.
2.  **Centralize the definition:** If the primary declaration is not already in a dedicated constants file (as indicated by the `NOTE`), consider moving it to a suitable `constants.rs`, `config.rs`, or `types.rs` file.
3.  **Replace duplicates:** In all other locations where the same literal value is declared, replace the constant declaration with a reference to the centralized constant.
    *   Example: If `const FOO: &str = "hello";` and `const BAR: &str = "hello";` are flagged, choose `FOO` as primary. Then, change `const BAR: &str = "hello";` to `const BAR: &str = FOO;` (or remove `BAR` if its usage can be replaced directly with `FOO`).
4.  **Verify:** After refactoring, re-run the `constant_analyzer` to ensure the duplication is resolved and no new issues are introduced.

## 4. Tools

*   `constant_analyzer` (Rust program)
*   `cargo` (Rust package manager)

## 5. Expected Outcome

*   Reduced duplication of literal values in constant declarations.
*   Improved code maintainability and readability.
*   A clearer understanding of the codebase's constant landscape.
*   Adherence to the "one constant, one declaration per unique literal value" rule.
