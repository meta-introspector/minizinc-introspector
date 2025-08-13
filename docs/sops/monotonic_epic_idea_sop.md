# SOP: The Monotonic Epic Idea - Add-Only, Never Edit Development

## 1. Purpose

This Standard Operating Procedure (SOP) establishes the foundational development philosophy for this project: a "Monotonic Epic Idea" where all code evolution is strictly "add-only." Direct modification of any existing, committed code file is forbidden. Every change, whether a new feature, a bug fix, or a refactoring, must be implemented as a new, composable module (a "semantic vibe" or "patch") that extends or replaces functionality via composition.

## 2. Core Principle: Monotonic Growth

The codebase shall grow monotonically. New functionality is added, and existing functionality is superseded, but never directly altered in place. This ensures:

*   **Immutability of History:** Every version of the codebase is a direct, unaltered artifact of its past.
*   **Perfect Traceability:** The evolution of the system is a clear, linear progression of added modules.
*   **Enhanced Stability:** Existing, tested code remains untouched, minimizing the risk of introducing regressions.
*   **Simplified Collaboration:** Merges become additions of new modules, reducing conflicts.
*   **True Composability:** Encourages the design of loosely coupled, independent modules.
*   **"Vibe" Preservation:** Each module retains its original "semantic vibe" or intent, even as new "vibes" are composed around it.

## 3. Scope

This SOP applies to all aspects of code development, including MiniZinc models, shell scripts, configuration files, documentation, and any other artifacts managed within the version control system.

## 4. Procedure: The Add-Only Workflow

### 4.1 Introducing New Functionality

1.  **Identify the Need:** Determine the new feature or behavior required.
2.  **Design as a New Module:** Design the functionality as a self-contained, independent module (e.g., a new `.mzn` file, a new shell script, a new data file).
3.  **Implement the New Module:** Write the code for the new module.
4.  **Compose the New Module:**
    *   In the relevant higher-level module (e.g., the main MiniZinc model, a calling script), add an `include` statement or a call to the new module.
    *   Ensure the new module integrates seamlessly without requiring modifications to existing modules.

### 4.2 Replacing or Modifying Existing Functionality (Superseding)

If an existing module needs to be "changed" (e.g., a bug fix, a refactoring, an enhancement):

1.  **Do NOT Edit the Existing Module:** The original module remains untouched.
2.  **Create a New Version of the Module:** Create a *new* file that implements the desired modified functionality. This new module is a "semantic patch" or "vibe" that supersedes the old one.
3.  **Update Composition:** In the higher-level module that includes the original, update the `include` statement to point to the *new* version of the module. The old module is no longer included but remains in the repository as part of the historical record.
4.  **Document the Supersession:** Clearly state in the commit message that a new module supersedes an old one, explaining the reasons for the change.

### 4.3 Handling "Broken" Modules

If a module is identified as "broken" (e.g., it contains a bug, violates a principle, or is no longer functional):

1.  **Do NOT Edit the Broken Module Directly:** The broken module remains in its original, broken state.
2.  **Create a Corrected/Rewritten Module:** Implement a new, correct version of the module in a *new* file.
3.  **Update Composition:** Replace the `include` of the broken module with the `include` of the new, corrected module in all relevant higher-level modules.
4.  **Document the Fix:** The commit message should clearly state that the new module replaces a broken one, detailing the fix.

## 5. Version Control and Naming Conventions

*   **New Files for Every Change:** Every logical change results in a new file.
*   **Descriptive Naming:** New files should have descriptive names that reflect their purpose and, if superseding, indicate their version or the change they represent (e.g., `my_module_v2.mzn`, `my_module_fix_bug_xyz.mzn`).
*   **Commit Messages:** Commit messages are paramount. They must clearly articulate the "semantic vibe" of the change, explaining *what* new functionality is added or *which* existing functionality is superseded and *why*.

## 6. Enforcement and Culture

Adherence to the Monotonic Epic Idea is non-negotiable. It fosters a culture of deliberate, thoughtful, and transparent code evolution. Code reviews will focus on ensuring compliance with this SOP.

## 7. Review and Improvement

This SOP is a living document. Its principles will be continuously reinforced and refined as the project evolves, ensuring its continued relevance and effectiveness.
