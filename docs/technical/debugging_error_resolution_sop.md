# Standard Operating Procedure: Debugging and Error Resolution (Under "No Direct Edits" Philosophy)

## 1. Purpose
This SOP provides a structured approach to debugging and resolving compilation or runtime errors within the `libminizinc` project, specifically adhering to the "Monotonic Epic Idea" philosophy where direct modification of existing, committed code is generally avoided. The goal is to identify root causes and implement solutions that are additive, composable, and maintain the integrity of the project's historical record.

## 2. Scope
This SOP applies to all errors encountered during development, testing, or deployment of `libminizinc` components, particularly when the resolution requires careful consideration of the "no direct edits" principle.

## 3. Principles
*   **Preservation of History:** Prioritize additive solutions over in-place modifications.
*   **Root Cause Analysis:** Thoroughly investigate errors to understand their fundamental origin, not just their symptoms.
*   **Minimal Intervention:** Seek the least intrusive solution that resolves the issue.
*   **Documentation:** Document the error, the investigation process, and the chosen resolution.
*   **Collaboration:** Engage with team members (or the user, in the case of an AI agent) when faced with ambiguous or challenging errors, especially when core principles might be tested.

## 4. Procedure

### 4.1. Observe and Document the Error
1.  **Capture Full Output:** Record the complete error message, including stack traces, warnings, and any associated context (e.g., `cargo build` output, test failures, runtime logs).
2.  **Identify Error Type:** Determine if it's a compilation error (syntax, type mismatch, unresolved symbols), a linker error, or a runtime error (panic, unexpected behavior).
3.  **Note Environment:** Document the operating system, Rust toolchain version, and any relevant environment variables or build flags.

### 4.2. Initial Triage and Context Gathering
1.  **Consult Documentation:** Review relevant project documentation, SOPs, and `README.md` files for known issues or established patterns related to the error.
2.  **Review Recent Changes:** Use `git log --patch -3 --all` to inspect recent commits across all branches. Look for changes that might have introduced the error, especially in related modules or dependencies.
3.  **Check Dependencies:** If the error points to a third-party crate, consult its documentation, changelogs, and `crates.io` for known issues or compatibility notes. Verify the version being used.

### 4.3. Investigation and Root Cause Analysis
1.  **Isolate the Problem:** Narrow down the problematic code section. For compilation errors, focus on the line numbers indicated. For runtime errors, use debugging tools (e.g., `rust-gdb`, `lldb`, `eprintln!` debugging).
2.  **Understand the "Why":** Beyond *what* the error is, understand *why* it's occurring. Is it a type mismatch due to an API change? A missing function due to a dependency update? A logical flaw?
3.  **Consider "No Direct Edits" Implications:** Before formulating a solution, evaluate how it aligns with the "Monotonic Epic Idea":
    *   Can the problem be solved by adding a new module or function that wraps or supersedes the problematic logic?
    *   Can a feature flag be used to conditionally include/exclude the problematic code or a new, corrected version?
    *   Is it a dependency versioning issue that can be resolved by updating `Cargo.toml` (an additive configuration change)?
    *   Is the error in a vendored crate? If so, can a minimal, documented patch be applied within the `vendor/` directory, with a clear rationale and a plan for upstream contribution?

### 4.4. Formulating and Implementing the Solution
1.  **Propose Additive Solution:** Based on the root cause and adherence to principles, formulate a solution that is as additive as possible.
    *   **Preferred:** Update `Cargo.toml` for dependency versioning, add new functions/modules, or use feature flags.
    *   **Last Resort (for critical fixes):** If a direct modification to existing code is unavoidable to resolve a critical, blocking error, it *must* be explicitly approved by the user/team lead, and documented with a clear rationale for violating the "no direct edits" principle in that specific instance. The change should be minimal and immediately followed by a plan to refactor it into an additive solution.
2.  **Implement the Solution:** Apply the chosen solution.
3.  **Test:** Run all relevant tests (unit, integration, system) to verify the fix and ensure no regressions are introduced.

### 4.5. Documentation and Communication
1.  **Record Resolution:** Document the error, the investigation steps, the chosen solution, and the justification for any deviations from the "no direct edits" principle.
2.  **Update SOPs:** If the error highlights a gap in existing SOPs or suggests a new best practice, propose updates or new SOPs.
3.  **Communicate:** Inform relevant stakeholders (user, team) about the error and its resolution.

## 5. Expected Outcome
*   Efficient and systematic resolution of errors.
*   Maintenance of code quality and historical integrity.
*   Continuous improvement of development procedures.
*   A robust and reliable `libminizinc` codebase.
