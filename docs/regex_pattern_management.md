# Regex Pattern Management in `poem_yaml_fixer`: TOML vs. `poem_function` Macro

The `poem_yaml_fixer` tool utilizes regular expressions to identify and process specific patterns within Markdown files, particularly within their YAML front matter. Currently, there are two primary mechanisms for defining and managing these regex patterns: a TOML configuration file (`regex_patterns.toml`) and the `#[poem_function]` procedural macro.

## 1. `regex_patterns.toml`

**How it works:**
*   This file (`crates/poem_yaml_fixer/src/regex_patterns.toml`) is a static configuration file that lists regex patterns, their associated names, and the callback functions designed to handle matches for those patterns.
*   During application startup, the `initialize_config` function reads and parses this TOML file.
*   Each `[[regexes]]` entry defines a `name`, `pattern` (the regular expression string), and `callback_function` (the name of the Rust function to be executed when the pattern matches).
*   The patterns are compiled into `regex::Regex` objects at runtime.

**Why it exists:**
*   **Flexibility:** Allows for easy modification and addition of regex patterns without requiring recompilation of the Rust code. This is useful for quick adjustments or for patterns that might be user-configurable in the future.
*   **Separation of Concerns:** Keeps regex definitions separate from the core Rust logic, making the patterns more discoverable and manageable for non-developers or for quick reviews.

## 2. `#[poem_function]` Procedural Macro

**How it works:**
*   This is a custom procedural macro (`poem_macros`) applied directly to Rust functions (e.g., `handle_old_meme_regex`).
*   The macro takes arguments such as `name`, `pattern`, and `callback_function` (which is typically the name of the function it's applied to).
*   At compile time, this macro generates code that associates the regex pattern with the function it decorates. It also generates metadata (like `POEM_FUNCTION_METADATA`) that can be collected into a `FunctionRegistry`.

**Why it exists:**
*   **Compile-time Safety:** Ensures that the `callback_function` actually exists and has the correct signature, catching errors at compile time rather than runtime.
*   **Code Colocation:** Keeps the regex pattern definition directly alongside the Rust function that implements its logic, improving code readability and maintainability for developers.
*   **Automatic Registration:** Simplifies the process of registering new regex-callback pairs, as the macro handles the boilerplate.

## 3. The Problem: Two Sources of Truth

The existence of both `regex_patterns.toml` and the `#[poem_function]` macro as sources for regex patterns introduces several challenges:

*   **Duplication:** The same regex pattern and its associated callback function might be defined in both the TOML file and the macro attributes, leading to redundancy.
*   **Synchronization Issues:** Changes to a regex pattern or its callback might need to be updated in two different places, increasing the risk of inconsistencies and bugs if one source is updated but the other is not.
*   **Increased Error Proneness:** Developers must remember which source to use for which type of pattern, and ensure consistency between the two. Errors in TOML parsing (as we've seen) or incorrect macro usage can lead to runtime failures or unexpected behavior.
*   **Ambiguity:** It's not immediately clear which source takes precedence if a pattern is defined in both, or how the `FunctionRegistry` is ultimately populated from these potentially conflicting sources.

## 4. Proposed Future State

The ideal future state is to streamline regex pattern management, primarily relying on the `#[poem_function]` macro as the authoritative source for most patterns.

*   **`#[poem_function]` as Primary:** Most regex patterns and their associated callback functions would be defined directly via the `#[poem_function]` macro. This leverages compile-time checks and code colocation benefits.
*   **TOML as Optional/Dynamic:** The `regex_patterns.toml` file would become optional and serve a more specialized role:
    *   **Dynamic Patterns:** For patterns that need to be configurable by end-users without recompilation.
    *   **Override Mechanism:** To temporarily override patterns defined by the macro for debugging or specific deployments.
    *   **External Integration:** For patterns loaded from external sources.
*   **Unified Registration:** The `initialize_config` function would be refactored to first collect all patterns registered by the `#[poem_function]` macro, and then optionally load patterns from the TOML file, applying them as overrides or additions.

This approach aims to reduce duplication, improve consistency, and simplify the overall regex pattern management workflow, making `poem_yaml_fixer` more robust and easier to maintain.
