# Change Request: Implementation of `kantspel_replace` Tool

*   **CRQ ID:** CRQ-20250823-001 (Auto-generated based on date and sequence)
*   **Date:** 2025-08-23
*   **Status:** Deferred
*   **Priority:** Medium (Addresses recurring pain points with `replace` tool)
*   **Requester:** User (via Gemini CLI interaction)
*   **Approver:** [To be determined by project governance]

## 1. Executive Summary

This Change Request proposes the development and integration of a new `kantspel_replace` tool within the `libminizinc` project. This tool aims to enhance the reliability and usability of string replacement operations, particularly when dealing with special characters such as backslashes (`\`) and curly braces (`{}`), by incorporating "kantspel auto-fixes." The implementation of this tool is currently deferred but formally documented for future consideration.

## 2. Problem Statement

The existing string replacement mechanisms (e.g., the `replace` tool within the Gemini CLI) frequently encounter issues when handling special characters in `old_string` and `new_string` arguments. These issues often stem from:
*   Misinterpretation of escape sequences (e.g., `\`).
*   Ambiguity with formatting placeholders (e.g., `{}`).
*   Inconsistent handling of literal special characters across different contexts.

This leads to:
*   Frequent failures of replacement operations.
*   Increased debugging time for the agent and user.
*   Reduced efficiency and reliability of automated code modifications.
*   User frustration due to the need for manual intervention and precise string literal matching.

## 3. Proposed Solution: `kantspel_replace` Tool

A new tool, `kantspel_replace`, will be developed to address the identified problems. Its core features will include:

*   **Intelligent String Processing:** The tool will internally process `old_string` and `new_string` to apply "kantspel auto-fixes." This involves normalizing problematic character representations based on the `kantspel.rs` definitions (e.g., automatically converting `\` to `\\` for literal backslashes, or handling `{{` for literal curly braces).
*   **Enhanced Robustness:** By standardizing the interpretation of special characters, the tool will significantly reduce the likelihood of replacement failures.
*   **Improved Usability:** Users will be able to specify `old_string` and `new_string` in a more intuitive manner, with the tool handling the underlying complexities of character escaping and interpretation.

### 3.1. Tool API (Proposed)

*   **Name:** `kantspel_replace`
*   **Parameters:**
    *   `file_path`: Absolute path to the file to modify (string, required).
    *   `old_string`: The string to be replaced (string, required).
    *   `new_string`: The string to replace `old_string` with (string, required).
    *   `apply_kantspel_fixes`: Boolean flag (optional, default `true`). If `true`, automatic kantspel fixes are applied.
    *   `expected_replacements`: Integer (optional). Number of expected replacements.

### 3.2. Technical Implementation Notes (High-Level)

*   **New Rust Crate:** `crates/kantspel_replace`.
*   **Dependency:** Will depend on `crates/gemini_utils` to leverage `kantspel` constants and existing string processing utilities.
*   **Core Logic:** Will involve parsing and transforming `old_string` and `new_string` based on predefined kantspel rules before performing the actual file content replacement.

## 4. Benefits

*   **Increased Reliability:** Fewer failed replacement operations.
*   **Improved Efficiency:** Reduced time spent on debugging and manual string manipulation.
*   **Enhanced User Experience:** More intuitive and less error-prone interaction with string replacement tasks.
*   **Adherence to Standards:** Formalizes the handling of problematic characters in line with project conventions.

## 5. Risks

*   **Complexity:** Implementing robust kantspel auto-fixes can be complex and may introduce new edge cases if not thoroughly tested.
*   **Performance Overhead:** Internal string processing might introduce a minor performance overhead, though likely negligible for typical use cases.
*   **Backward Compatibility:** Careful design is needed to ensure the new tool integrates smoothly without negatively impacting existing workflows.

## 6. Deferral Justification

The implementation of the `kantspel_replace` tool is currently deferred due to ongoing critical path tasks. This CRQ serves to formally document the identified need and the proposed solution for future prioritization and development.

## 7. Approval

*   **Requested By:** Gemini CLI Agent on behalf of User
*   **Date:** 2025-08-23

---