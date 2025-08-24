# Change Request: Escape Regexes with Emojis

## 1. Objective

To enhance the readability and maintainability of regular expressions within the project by introducing a system for emoji-based escapes. This aligns with the existing `kantspel` philosophy of using expressive symbols for complex or special characters.

We will create a new procedural macro, `kantspel_regex!`, within the `kantspel_macros` crate. This macro will transpile a string literal containing emojis into a standard, valid regular expression string that can be used by the `regex` crate.

## 2. Plan

1.  **Create `kantspel_regex!` Macro:**
    *   Implement a new function-like procedural macro `kantspel_regex!` in `crates/kantspel_macros/src/lib.rs`.
    *   The macro will accept a single string literal as input.

2.  **Define Emoji-to-Regex Mapping:**
    *   Create a static `HashMap` to define the translation from specific emojis to their corresponding regex components (e.g., `🤸` -> `\s`, `🔢` -> `\d`).
    *   This mapping will be comprehensive enough to cover common regex patterns.

3.  **Implement Transformation Logic:**
    *   The macro will parse the input string and replace each recognized emoji with its regex equivalent.
    *   Careful attention will be paid to proper escaping of characters, especially backslashes, for the final regex string.

4.  **Refactor Existing Code:**
    *   Identify all instances where `regex::Regex::new()` is used in the codebase.
    *   Replace the existing raw string regex patterns with calls to the new `kantspel_regex!` macro, using the new emoji-based syntax.

5.  **Build, Test, and Verify:**
    *   Compile the entire project using `cargo check --all-targets` to identify any issues with the transpiled regexes.
    *   Fix any invalid regex patterns that arise from the transformation.
    *   Run tests to ensure that the regex-dependent logic still functions as expected.

6.  **Documentation:**
    *   Create a new Standard Operating Procedure (SOP) document: `docs/sops/emoji_regex_sop.md`.
    *   This document will detail the purpose of the `kantspel_regex!` macro, provide a complete list of the emoji-to-regex mappings, and show clear usage examples.
    *   Add comprehensive doc comments to the `kantspel_regex!` macro implementation.

## 3. Rationale

This change will:
*   Improve the developer experience by making complex regexes more intuitive and less error-prone.
*   Enforce consistency with the project's `kantspel` principles.
*   Centralize the definition of regex patterns, making them easier to manage and update.
