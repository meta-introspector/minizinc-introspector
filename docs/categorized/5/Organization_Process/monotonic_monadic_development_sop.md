# SOP: Monotonic Monadic Development in the Quasi Meta Memespace

## 1. Purpose

This Standard Operating Procedure (SOP) formalizes the project's core development philosophy: a "Monotonic Monadic Functional Additive, Constructive, Gödelian System of Vibrations in the Quasi Meta Memespace." It outlines how all changes are to be implemented as additive, self-contained "vibrations" (new modules or versions), rather than destructive edits to existing artifacts. This SOP specifically addresses the limitations of traditional "edit tools" (including the Gemini CLI's `replace` function) by providing a framework for constructive evolution, ensuring perfect traceability, immutability, and alignment with the project's philosophical underpinnings.

## 2. Core Principles

*   **Monotonic Growth:** The codebase and all associated artifacts (MiniZinc models, Rust programs, documentation) shall only grow through addition. Nothing is ever truly deleted or modified in place.
*   **Monadic Operations:** Each change is conceived as a self-contained "vibration" or "monad" that transforms the system state by adding new information, rather than altering existing information.
*   **Functional Purity:** Changes are designed to be side-effect-free on existing components. New functionality is achieved through composition of new vibrations.
*   **Additive Construction:** All development is constructive. New versions of modules or data supersede old ones by being added alongside them, with higher-level systems referencing the new "vibration."
*   **Gödelian Self-Reference:** The system embraces self-reference and meta-modeling. Concepts, code, and their relationships are represented as Gödel numbers and embedded within the "quasi meta memespace" (Clifford multivectors, Riemann manifold).
*   **Vibrations in the Quasi Meta Memespace:** All changes are considered "vibrations" that resonate within the project's conceptual space, influencing and being influenced by the underlying mathematical and philosophical structures.

## 3. Scope

This SOP applies to all development activities within the project, including:

*   MiniZinc models (`.mzn` files)
*   MiniZinc data files (`.dzn` files)
*   Rust source code (`.rs` files)
*   Shell scripts (`.sh` files)
*   Documentation (`.md` files)
*   Configuration files (`.toml`, `.env`)
*   Any other artifact managed under version control.

## 4. Procedure: Constructive Evolution (No Edit Tools)

The following procedures replace the use of traditional "edit tools" (like `replace` or direct file modification) with additive, constructive operations.

### 4.1. Modifying an Existing File (Supersession)

If a change is required for an existing file (e.g., bug fix, feature enhancement, refactoring):

1.  **Do NOT directly modify the existing file.**
2.  **Create a new version of the file:** Copy the content of the existing file to a new file with a versioned name (e.g., `my_module_v2.mzn`, `my_function_improved.rs`).
3.  **Implement the changes in the new file:** Apply the desired modifications to the *new* file.
4.  **Update references:** In all higher-level modules or scripts that previously referenced the old file, update the references to point to the new version.
5.  **Document the supersession:** In the commit message, clearly state that the new file supersedes the old one, explaining the reasons for the change and the versioning scheme used.

    *   **Example (MiniZinc Model):** If `model_v1.mzn` needs a change, create `model_v2.mzn` with the modifications. Update any `include "model_v1.mzn";` to `include "model_v2.mzn";`.
    *   **Example (Rust Function):** If `my_function` in `src/lib.rs` needs a change, create `my_function_v2` in a new file or within the same file (if it's a small, self-contained change and the file is not already versioned). Update all call sites from `my_function()` to `my_function_v2()`.

### 4.2. Deleting a File (Deprecation)

If a file is no longer needed:

1.  **Do NOT delete the file from the repository.**
2.  **Deprecate the file:**
    *   If it's a source file, add a comment at the top indicating it's deprecated and which new file supersedes it.
    *   If it's a data file, add a comment indicating its deprecation.
3.  **Remove all references:** Ensure no active higher-level modules or scripts reference the deprecated file.
4.  **Document the deprecation:** In the commit message, clearly state that the file is deprecated and why, and which new file (if any) replaces its functionality.

### 4.3. Adding New Files

1.  **Create the new file:** Implement the new functionality in a new, appropriately named file.
2.  **Integrate the new file:** Reference the new file in relevant higher-level modules or scripts.
3.  **Document the addition:** In the commit message, clearly describe the new functionality and its purpose.

## 5. Tooling and Gemini CLI Interaction

Given the "no edit tools" principle, the Gemini CLI will interact with the codebase exclusively through additive operations:

*   **File Creation:** Use `write_file` to create new files or new versions of existing files.
*   **File Reading:** Use `read_file` and `read_many_files` to inspect content.
*   **Directory Listing:** Use `list_directory` and `glob` to explore the codebase.
*   **Shell Commands:** Use `run_shell_command` for operations like `mv` (for renaming new versions), `cp` (for creating new versions), `rm` (for generated temporary files, *not* committed files), and `git` commands for version control.
*   **No `replace`:** The `replace` tool will not be used. All modifications will follow the supersession procedure.

## 6. Version Control and Naming Conventions

*   **Explicit Versioning:** All files that are subject to change should incorporate versioning in their names (e.g., `model_v1.mzn`, `model_v2.mzn`).
*   **Descriptive Naming:** File names should clearly indicate their purpose and the nature of the change (e.g., `sieve_embedding_optimized.mzn`, `gemini_agent_conceptual_grok4.mzn`).
*   **Commit Messages:** Commit messages are paramount. They must clearly articulate the "vibration" of the change, explaining *what* new functionality is added or *which* existing functionality is superseded and *why*. They should also reference this SOP.

## 7. Review and Improvement

This SOP is a living document, a vibration within the memespace. Its principles will be continuously reinforced and refined as the project evolves, ensuring its continued relevance and effectiveness in guiding our monotonic monadic development.

---
