# Strict `gemini_eprintln!` Usage and `kantspel` Principles for Team T33M

**Date:** Saturday, August 23, 2025

**Author:** Gemini CLI Agent

**Purpose:** This Standard Operating Procedure (SOP) clarifies the strict usage rules for the `gemini_utils::gemini_eprintln!` macro, emphasizing adherence to the project's `kantspel` principles, particularly regarding the handling of special characters, named arguments, and placeholder translation for LLM readability.

**Core Principle:** The `gemini_eprintln!` macro is designed for visually expressive output with **automatic translation** of specific keywords/emojis into standard Rust formatting characters (`\n`, `{}`). It supports named arguments for clear and structured output.

**Rules for `gemini_eprintln!`:**

1.  **Input Format:**
    *   The macro accepts a format string followed by an optional dictionary-like structure for named arguments.
    *   **Format String:** Should contain keywords (e.g., "sparkles", "brickwall", "building_construction") or their corresponding emojis (`✨`, `🧱`, `🏗️`) for special characters. It should **NOT** contain literal `\n`, `{}`, or `{{}}` as these will be handled by the macro's internal translation. Named placeholders for arguments should use the `:key:` syntax (e.g., `:version:`).
    *   **Named Arguments:** Provided as `key = value` pairs, separated by commas, enclosed in curly braces (e.g., `{ version = header.version, limit = args.limit }`).

2.  **Internal Translation (Macro Behavior):**
    *   The macro will automatically translate the following in the format string:
        *   "sparkles" or `✨` into `\n`
        *   "brickwall" or `🧱` into `{}`
        *   "building_construction" or `🏗️` into `{{}}`
        *   `:key:` placeholders into `{key}`
    *   This translation happens *before* the string is passed to the underlying `eprintln!` macro.

3.  **Example Usage:**

    *   **Simple message:**
        ```rust
        gemini_eprintln!("Asciicast Header:");
        ```

    *   **Message with named arguments:**
        ```rust
        gemini_eprintln!(" Version: :version:, Width: :width:, Height: :height:", version = header.version, width = header.width, height = header.height);
        ```

    *   **Message with special characters (emojis/keywords):**
        ```rust
        gemini_eprintln!("Processing events and collecting cleaned output (limited to :limit:).sparkles", limit = args.limit);
        // This will translate to: eprintln!("Processing events and collecting cleaned output (limited to {}.\n", args.limit);
        ```

    *   **Message with literal curly braces (building_construction):**
        ```rust
        gemini_eprintln!("This is a building_construction block.", some_var = value);
        // This will translate to: eprintln!("This is a {{}} block.", some_var = value);
        ```
