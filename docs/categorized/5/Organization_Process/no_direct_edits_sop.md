# SOP: No Direct Edits - Composable and Incremental Development

## 1. Purpose

This Standard Operating Procedure (SOP) establishes a strict policy against direct modification of existing code files for feature development, bug fixes, or refactoring. Instead, all changes must be implemented through the creation of new, smaller, and functionally composable modules. This approach aims to enhance code quality, maintainability, traceability, and collaboration by ensuring that every "edit" is a clearly defined "semantic patch" or "vibe" that can be composed into the existing system.

## 2. Scope

This SOP applies to all code development activities within the project, including but not limited to MiniZinc models, shell scripts, and configuration files.

## 3. Principles

*   **Immutability of Existing Code (where possible):** Once a code module is stable and committed, direct modifications to its content should be avoided.
*   **Composability:** New features or changes should be implemented as independent modules that can be "composed" or "included" into the existing system.
*   **Incrementality:** Changes should be small, atomic, and verifiable at each step.
*   **Traceability:** Every change should be a distinct, documented commit, representing a clear "semantic patch" or "vibe."
*   **Readability & Maintainability:** Smaller, focused modules are easier to understand, test, and maintain.
*   **Reduced Risk:** Avoiding direct edits minimizes the risk of introducing regressions or unintended side effects in existing, stable code.

## 4. Procedure

### 4.1 Feature Development / Bug Fix / Refactoring

When a change is required:

1.  **Identify the Target Functionality:** Determine which part of the system needs modification or extension.
2.  **Do NOT Edit Existing Files:** Under no circumstances should an existing file be directly opened and modified for the purpose of introducing a new feature or fixing a bug within its existing lines.
3.  **Create a New Module:**
    *   If adding a new feature, create a new `.mzn` file (or script, etc.) that implements this feature.
    *   If modifying existing functionality, create a new `.mzn` file that *replaces* or *extends* the functionality of the old file. This new file should encapsulate the desired change.
4.  **Compose Functionally:**
    *   In the main model (or the relevant higher-level module), update the `include` statements to incorporate the new module.
    *   If the new module replaces an old one, update the `include` statement to point to the new module and remove the `include` for the old one.
    *   Ensure that the composition maintains the overall system's integrity and functionality.
5.  **Test Incrementally:** Verify the change by running relevant tests or the affected part of the system.
6.  **Document the "Vibe" / "Semantic Patch":**
    *   Each new module should have clear internal comments explaining its purpose, inputs, outputs, and how it contributes to the overall system.
    *   The commit message for the new module should clearly articulate the "vibe" or "semantic patch" it represents (e.g., "feat: Add X functionality," "refactor: Improve Y component," "fix: Resolve Z issue").

### 4.2 Handling "Broken" Files

If a file is identified as "broken" (e.g., due to a previous error, or it violates a principle):

1.  **Do NOT Edit the Broken File Directly:** The broken file should be considered immutable in its broken state for the purpose of fixing it.
2.  **Rewrite into Smaller Modules (if applicable):** If the broken file is monolithic or contains multiple responsibilities, break it down into smaller, functional modules (as per 4.1.3).
3.  **Replace via Composition:** Create a new, correct version of the file (or its constituent modules) and replace the old, broken file by updating the `include` statements in the higher-level modules.
4.  **Document the Fix:** The commit message should clearly state that the file was "rewritten" or "replaced via composition" to fix the identified issue.

## 5. Enforcement

Adherence to this SOP is mandatory for all contributors. Violations will be addressed through code reviews and, if necessary, re-education on the principles of composable and incremental development.

## 6. Review and Improvement

This SOP is a living document and will be reviewed periodically (e.g., quarterly) or as needed to incorporate lessons learned and improve the development process.
