# Conceptual Design: Rust Link Verification Tool (`minizinc-doc-linker`)

## Purpose
To ensure consistency and correctness of links between source code files (Rust, MiniZinc, C++) and documentation files (Markdown, RST, etc.) within the `libminizinc` project. It will identify and report broken, inconsistent, or missing links.

## Core Functionality:

### 1. Configuration
*   Read configuration from a `linker.toml` file (or similar) at the project root.
*   Configuration will define:
    *   Paths to scan for code files (e.g., `src/**/*.rs`, `lib/**/*.cpp`, `*.mzn`).
    *   Paths to scan for documentation files (e.g., `docs/**/*.md`, `README.md`).
    *   Patterns for identifying links in code (e.g., `/// [link_text](path/to/doc.md#section)`, `// See: path/to/doc.md`).
    *   Patterns for identifying links in documentation (e.g., `[link_text](path/to/code.rs#function_name)`).
    *   Rules for resolving relative paths.
    *   Exclusion patterns (e.g., `target/`, `node_modules/`).

### 2. Link Extraction
*   **Code Parser:**
    *   Iterate through specified code files.
    *   Use regular expressions or a more robust parsing library (e.g., `syn` for Rust, custom parsers for MiniZinc/C++) to extract potential link patterns.
    *   For each extracted link, record:
        *   Source file path.
        *   Line number.
        *   Detected link target (file path, optional section/function name).
        *   Type of link (e.g., "code-to-doc").
*   **Documentation Parser:**
    *   Iterate through specified documentation files.
    *   Use a Markdown parser (e.g., `pulldown-cmark`) or RST parser to extract all internal and external links.
    *   For each extracted link, record:
        *   Source file path.
        *   Line number.
        *   Detected link target (file path, optional section/anchor).
        *   Type of link (e.g., "doc-to-code", "doc-to-doc").

### 3. Target Resolution & Validation
*   For each extracted link:
    *   **File Existence:** Verify if the target file exists on the filesystem.
    *   **Section/Anchor Validation (for documentation targets):** If the link points to a specific section (`#section-name`), parse the target documentation file to ensure that section/anchor exists.
    *   **Function/Symbol Validation (for code targets):** If the link points to a specific function or symbol (`#function_name`), parse the target code file to ensure that function/symbol exists. This might require more sophisticated AST parsing or symbol table lookups.
    *   **Bidirectional Check:** If a code-to-doc link is found, check if a corresponding doc-to-code link exists (and vice-versa) if bidirectional linking is a project standard.

### 4. Reporting
*   Generate a comprehensive report in a human-readable format (e.g., Markdown, JSON, CLI output).
*   Report categories:
    *   **Broken Links:** Target file/section/symbol does not exist.
    *   **Inconsistent Links:** Link exists but doesn't follow project conventions (e.g., wrong path format, missing bidirectional link).
    *   **Unlinked Entities (Optional):** Code functions/documentation sections that are not referenced by any link (requires a full scan of all entities and cross-referencing).
*   Provide clear messages including source file, line number, and the problematic link.

## Technical Considerations (Rust Specific):

*   **Concurrency:** Use `rayon` or `tokio` for parallel processing of files to speed up scanning.
*   **Error Handling:** Robust error handling for file I/O, parsing, and link validation.
*   **CLI Interface:** Use `clap` for a user-friendly command-line interface (e.g., `minizinc-doc-linker check`, `minizinc-doc-linker report`).
*   **Modularity:** Structure the tool into crates (e.g., `linker-core`, `linker-parsers`, `linker-reporters`) for maintainability and reusability.
*   **Parsing Libraries:**
    *   For Markdown: `pulldown-cmark`.
    *   For Rust code: `syn` (for AST parsing), `regex` (for simpler pattern matching).
    *   For C++/MiniZinc: May require custom parsers or external tools if complex AST analysis is needed. Simple regex might suffice for initial link extraction.

## Integration with SOP:

*   This tool would be executed in **Phase 5.3.2 (Link Verification)** of the `code_doc_update_sop.md`.
