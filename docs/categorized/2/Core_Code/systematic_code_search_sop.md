## 1. Purpose
This plan outlines a systematic approach to search and discover existing solutions, patterns, and implementations within the entire codebase (our own and vendored dependencies). This process aims to enhance code understanding, promote reuse, and inform future development efforts.

## 2. Scope
This plan applies to all source code files (`.rs`, `.cpp`, `.h`, `.hpp`, `.mzn`, etc.), configuration files (`.toml`, `.dzn`), and documentation files (`.md`) within the `libminizinc` project directory, including all vendored dependencies.

## 3. Tools
*   `default_api.glob`: For efficiently finding files matching specific patterns.
*   `default_api.search_file_content`: For searching for regular expression patterns within file contents.
*   `default_api.read_file`: For reading the content of specific files for detailed analysis.
*   `default_api.read_many_files`: For reading multiple files to get broader context.

## 4. Procedure

### Phase 1: Define Search Objectives
**Objective:** Clearly articulate what constitutes a "solution" for the current task.

1.  **Identify Keywords/Patterns:** Brainstorm specific keywords, function names, class names, API calls, or regular expressions relevant to the solution being sought.
    *   *Example:* If looking for MiniZinc model parsing, keywords might include `parse_model`, `CXTranslationUnit`, `parser.cpp`, `ast.hpp`.
2.  **Define Contextual Clues:** Consider the surrounding code, file types, or directory structures that might indicate a relevant solution.

### Phase 2: Broad Codebase Scan (Glob & Search)
**Objective:** Perform initial broad scans to identify potentially relevant files and code snippets.

1.  **File Type Targeting (Glob):**
    *   Use `glob` to find all files of relevant types across the entire codebase.
        *   *Example:* `glob(pattern="**/*.rs")`, `glob(pattern="**/*.{cpp,h,hpp}")`, `glob(pattern="**/*.mzn")`.
    *   This helps in understanding the overall file structure and where different types of code reside.
2.  **Keyword/Pattern Search (Search File Content):**
    *   Use `search_file_content` with the identified keywords/patterns across the entire codebase.
        *   *Example:* `search_file_content(pattern="parse_model", include="**/*.{rs,cpp,h,hpp}")`.
    *   Start with broad searches and progressively narrow down the scope if too many results are returned.

### Phase 3: Focused Analysis (Read & Analyze)
**Objective:** Deep dive into identified files and code snippets to understand the "solution."

1.  **Read Relevant Files:** Use `read_file` or `read_many_files` on files identified in Phase 2.
    *   Prioritize files with high relevance scores from `search_file_content` or those in key directories.
2.  **Contextual Reading:** Read surrounding code, comments, and related files to understand the full context of the solution.
3.  **Identify Dependencies:** Note any external libraries, internal modules, or specific functions that the solution relies on.
4.  **Trace Execution Flow:** (Conceptual) Mentally or diagrammatically trace how the solution is invoked and executed.

### Phase 4: Documentation and Archiving
**Objective:** Document the discovered solutions for future reference and knowledge sharing.

1.  **Summarize Findings:** Create concise summaries of the discovered solutions, including their purpose, implementation details, and any relevant caveats.
2.  **Update Relevant Documentation:** Integrate findings into existing SOPs, design documents, or create new documentation as needed.
3.  **Cross-reference:** Link discovered solutions to related concepts, issues, or other documentation.

## 5. Best Practices
*   **Iterative Refinement:** Start with broad searches and progressively refine queries based on initial results.
*   **Parallel Execution:** Utilize `glob` and `search_file_content` in parallel when independent.
*   **Error Handling:** Be mindful of tool limitations (e.g., truncation in `read_file`) and adjust queries accordingly.
*   **Version Control:** Ensure all documentation and code changes related to solution discovery are properly version-controlled.
*   **Collaboration:** Share findings with other team members (or the user) to foster collective understanding.

## 6. Example Application (Searching for MiniZinc AST Parsing in Rust)

1.  **Define Search Objectives:**
    *   Keywords: `TranslationUnit`, `parse`, `AST`, `clang_sys`, `clang-rs`, `libclang`.
    *   Context: Rust files (`.rs`), C++ files (`.cpp`, `.h`, `.hpp`).

2.  **Broad Codebase Scan:**
    *   `glob(pattern="**/*.rs")`
    *   `search_file_content(pattern="TranslationUnit::parse|TranslationUnit::from_ast|clang_parseTranslationUnit", include="**/*.rs")`

3.  **Focused Analysis:**
    *   `read_file` on `crates/minizinc_introspector/src/main.rs` (our current implementation).
    *   `read_file` on `vendor/clang-rs/src/lib.rs` (to understand `clang-rs` API).
    *   `read_file` on `vendor/clang-rs/src/translation_unit.rs` (if it exists).
