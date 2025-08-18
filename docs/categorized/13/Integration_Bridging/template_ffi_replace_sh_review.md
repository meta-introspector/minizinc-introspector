## Review of `template_ffi_replace.sh`

*   **Purpose:** This script is a *template* for performing a highly specific text replacement within a file, intended for FFI function declaration changes. It explicitly warns about its fragility and recommends extreme caution.
*   **Key Commands and Dependencies:**
    *   `sed -i "s|${OLD_DECL}|${NEW_DECL}|g" "$FILE"`: The core command, using `sed` for in-place substitution. It uses `|` as a delimiter, which is a good practice when the pattern might contain `/`.
*   **Relevance to FFI, MiniZinc, and the "Big Idea":**
    *   **FFI:** Directly related to FFI development, specifically for modifying FFI declarations.
    *   **MiniZinc:** Indirectly relevant, as it's a tool for managing the FFI that interacts with MiniZinc.
    *   **"Big Idea":**
        *   **Monotonic Epic Idea (Contradiction/Caveat):** This script directly contradicts the "add-only, never edit" philosophy. The script itself acknowledges its fragility and the need for caution. Its presence suggests that while the "add-only" philosophy is preferred, there might be practical scenarios (especially during refactoring or fixing deeply embedded issues) where direct text replacement is considered, albeit with extreme caution. This highlights a tension between the ideal philosophy and practical development needs.
        *   **Tooling for Refactoring:** It represents a type of tool that might be used in complex refactoring scenarios, even if it goes against the primary development philosophy.
        *   **LLM Interaction:** An LLM (like myself) would need to be extremely careful if asked to use such a script, always prioritizing the "add-only" approach unless explicitly overridden by the user with full understanding of the implications.

This script is a template for a powerful but dangerous operation, highlighting a potential tension with the project's core "add-only" philosophy.
