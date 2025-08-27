# Standard Operating Procedure: Asciicast Processing Workflow

**Date:** August 23, 2025
**Author:** Gemini CLI

## 1. Purpose

This SOP outlines the standardized procedure for processing asciinema recordings using the `asciicast_processor` tool within the `libminizinc` project. This process is integral to the "meme compilation" workflow, enabling the transformation of terminal interaction patterns into statically typed Rust code.

## 2. Scope

This SOP applies to all developers and automated agents involved in the generation, processing, and integration of asciinema recordings into the project's codebase.

## 3. Prerequisites

*   A functional `libminizinc` development environment with all dependencies resolved.
*   The `asciicast_processor` tool successfully built (located at `target/debug/asciicast_processor` or `target/release/asciicast_processor`).
*   An asciinema recording file (`.cast` extension) to be processed.

## 4. Procedure

### 4.1 Locating the Asciinema Recording

Identify the `.cast` file you wish to process. Typically, these files are stored in the `docs/` directory or its subdirectories.

*   **To find the newest `.cast` file:**
    ```bash
    ls -latr docs/*.cast
    ```
    (This command will list `.cast` files in `docs/` sorted by modification time, newest last.)

### 4.2 Executing the `asciicast_processor`

The `asciicast_processor` tool requires an input `.cast` file and an output `.rs` file for the generated Rust code.

*   **Command Syntax:**
    ```bash
    target/debug/asciicast_processor --rust-output-file <OUTPUT_FILE.rs> <INPUT_FILE.cast>
    ```
    *   Replace `<OUTPUT_FILE.rs>` with the desired absolute or relative path for the generated Rust code file (e.g., `docs/my_recording_processed.rs`).
    *   Replace `<INPUT_FILE.cast>` with the absolute or relative path to the asciinema recording file (e.g., `docs/asciicast11.cast`).

*   **Example:**
    ```bash
    target/debug/asciicast_processor --rust-output-file docs/asciicast11_processed.rs docs/asciicast11.cast
    ```

#### 4.2.1 `asciicast_processor` Arguments

The `asciicast_processor` accepts the following command-line arguments:

*   `--input-file <FILE>`: **(Positional Argument)** Path to the asciicast input file (`.cast`). This is the primary source of terminal interaction data.
*   `--rust-output-file <FILE>`: **(Required)** Path to the output `.rs` file where the generated Rust code (containing `#[poem_function]` macros) will be saved.
*   `--limit <NUMBER>`: **(Optional)** Limits the number of events to process from the beginning of the asciicast. Useful for quick previews or debugging. Defaults to 10. Conflicts with `--tail`.
*   `--tail <NUMBER>`: **(Optional)** Processes only the last N events from the asciicast. Useful for reviewing recent interactions. Conflicts with `--limit`.
*   `--steps <NUM1,NUM2,...>`: **(Optional)** A comma-separated list of numbers defining the prefix lengths for hierarchical grouping of lines. This influences how `grex` generates regexes and the structure of the `poem_function` hierarchy. Defaults to `5,3,1`.
*   `--ascii-names`: **(Optional Flag)** When present, enables the conversion of specific Unicode characters (like block elements) in the terminal output to their ASCII names (e.g., `█` becomes `BLOCK`). This ensures cleaner text for regex generation and avoids issues with non-ASCII characters in generated code.

#### 4.2.2 Internal Workflow

The `asciicast_processor` operates through the following key steps:

1.  **Argument Parsing:** Uses the `clap` crate to parse command-line arguments, configuring the processing behavior.
2.  **Asciicast Reading & Deserialization:** Opens the specified `.cast` file, reads its content, and deserializes the JSON data into an `asciicast::Header` and a stream of `asciicast::Entry` events.
3.  **Event Collection & Filtering:** Collects all `Output` events from the asciicast. It respects the `--limit` or `--tail` arguments to process a subset of events if specified. `Input` events are currently ignored.
4.  **Output Cleaning:** For each collected output line:
    *   ANSI escape codes are stripped using `strip-ansi-escapes` to obtain raw, readable text.
    *   If `--ascii-names` is enabled, specific Unicode characters are replaced with their ASCII equivalents.
5.  **Hierarchical Regex Generation (`build_hierarchy`):**
    *   The cleaned output lines are fed into the `build_hierarchy` function.
    *   This function recursively groups similar lines based on common prefixes, determined by the `--steps` argument.
    *   For each group, `grex::RegExpBuilder` is used to generate a concise regular expression that matches the common patterns within that group. This creates a tree-like structure of `RegexHierarchyNode`s.
6.  **Poem Function Generation (`generate_poem_functions`):**
    *   The `generate_poem_functions` function traverses the `RegexHierarchyNode` tree.
    *   For each node that has a generated regex, it constructs a `proc_macro2::TokenStream` representing a `#[poem_function]` macro.
    *   These macros are populated with metadata such as `name`, `pattern` (the generated regex), `title`, `summary`, `keywords`, `emojis`, `art_generator_instructions`, and `pending_meme_description`.
7.  **Rust Code Output:** The accumulated `TokenStream` (representing the entire generated Rust code) is written to the specified `--rust-output-file`.

### 4.3 Verification of Output

After execution, verify that the output Rust file (`<OUTPUT_FILE.rs>`) has been created and contains the expected generated code. Review the console output of the `asciicast_processor` for any warnings or errors.

## 5. Role in Meme Compilation Workflow

The generated Rust code from the `asciicast_processor` serves as a foundational component for the project's "meme compilation" workflow. These Rust modules represent statically typed patterns derived from terminal interactions, which can then be integrated, analyzed, and further processed by other components of the `libminizinc` system. The `#[poem_function]` macros encapsulate these patterns, making them discoverable and usable within the Rust codebase for various purposes, including:

*   **Pattern Recognition:** Identifying recurring sequences or structures in terminal output.
*   **Automated Testing:** Creating assertions based on expected terminal behavior.
*   **Semantic Analysis:** Associating terminal patterns with higher-level concepts or "memes" through the rich metadata provided in the `poem_function` attributes.
*   **Code Generation:** Serving as building blocks for other code generation processes that rely on understanding terminal interactions.

## 6. Related Documentation

*   **Change Request: Asciicast Processor Build Fix and Execution:** `docs/crqs/asciicast_processor_build_fix_crq_20250823.md`
*   **README.md:** Refer to the "Asciicast Processing" section for a brief overview.

## 7. Revision History

| Date         | Version | Description        | Author     |
| :----------- | :------ | :----------------- | :--------- |
| 2025-08-23   | 1.0     | Initial document   | Gemini CLI |
| 2025-08-24   | 1.1     | Detailed arguments and internal workflow of `asciicast_processor`. Expanded on role in meme compilation. | Gemini CLI |
