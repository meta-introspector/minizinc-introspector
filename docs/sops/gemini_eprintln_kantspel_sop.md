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
        *   "inspect" or `🔍` into `{:?}`
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

    *   **Message with debug formatting (inspect/🔍):**
        ```rust
        gemini_eprintln!("Debug value: inspect", my_variable = some_value);
        // This will translate to: eprintln!("Debug value: {:?}", some_value);
        ```

## Commit History

- [Commit fca2cfeefb4ae3c8dda99ce6b19318f27141eee6: docs(gemini_utils): Add n00b guide for enhanced gemini_eprintln features](docs/commits/fca2cfeefb4ae3c8dda99ce6b19318f27141eee6_docs_gemini_utils_Add_n00b_guide_for_enhanced_gemini_eprintln_features.md)
- [Commit ea2b6529e3b82cfb8bde45bb3f55e22c5ccef69f: feat(gemini_utils): Enhance gemini_eprintln with auto-emoji and dynamic naming](docs/commits/ea2b6529e3b82cfb8bde45bb3f55e22c5ccef69f_feat_gemini_utils_Enhance_gemini_eprintln_with_auto-emoji_and_dynamic_naming.md)
- [Commit a11299c7d5bbad2625f41c1014113c65eb037a45: refactor(gemini_eprintln): Refine argument parsing and add debug logging](docs/commits/a11299c7d5bbad2625f41c1014113c65eb037a45_refactor_gemini_eprintln_Refine_argument_parsing_and_add_debug_logging.md)
- [Commit 96a393a5b6ab10ef9182b781240a108af4fb7748: feat(gemini_eprintln): Add argument_mapper placeholder](docs/commits/96a393a5b6ab10ef9182b781240a108af4fb7748_feat_gemini_eprintln_Add_argument_mapper_placeholder.md)
- [Commit c3540f674e11d6c6cca5c236e3e17f1f5102e447: refactor(kantspel_macros): Modularize macro implementations](docs/commits/c3540f674e11d6c6cca5c236e3e17f1f5102e447_refactor_kantspel_macros_Modularize_macro_implementations.md)
- [Commit c05098c17f71d759b0929be74bbddb5f0467f45f: feat: Enhance MiniZinc FFI safety; modularize kantspel_macros; add MZN testing script](docs/commits/c05098c17f71d759b0929be74bbddb5f0467f45f_feat_Enhance_MiniZinc_FFI_safety_modularize_kantspel_macros_add_MZN_testing_script.md)
- [Commit 5413e67a5dabd3b4ca5a7f03c0b54361603cb63f: feat(asciicast_processor): Implement Rust code generation from asciicasts; refactor core logic](docs/commits/5413e67a5dabd3b4ca5a7f03c0b54361603cb63f_feat_asciicast_processor_Implement_Rust_code_generation_from_asciicasts_refactor_core_logic.md)
- [Commit cd18fb8540d1c9f08c6175fe98d5cd54e601b8c4: feat(gemini_eprintln): Implement named argument parsing; refactor asciicast_processor event handling](docs/commits/cd18fb8540d1c9f08c6175fe98d5cd54e601b8c4_feat_gemini_eprintln_Implement_named_argument_parsing_refactor_asciicast_processor_event_handling.md)