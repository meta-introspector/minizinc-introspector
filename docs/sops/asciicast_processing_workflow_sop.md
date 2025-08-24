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

### 4.3 Verification of Output

After execution, verify that the output Rust file (`<OUTPUT_FILE.rs>`) has been created and contains the expected generated code. Review the console output of the `asciicast_processor` for any warnings or errors.

## 5. Role in Meme Compilation Workflow

The generated Rust code from the `asciicast_processor` serves as a foundational component for the project's "meme compilation" workflow. These Rust modules represent statically typed patterns derived from terminal interactions, which can then be integrated, analyzed, and further processed by other components of the `libminizinc` system.

## 6. Related Documentation

*   **Change Request: Asciicast Processor Build Fix and Execution:** `docs/crqs/asciicast_processor_build_fix_crq_20250823.md`
*   **README.md:** Refer to the "Asciicast Processing" section for a brief overview.

## 7. Revision History

| Date         | Version | Description        | Author     |
| :----------- | :------ | :----------------- | :--------- |
| 2025-08-23   | 1.0     | Initial document   | Gemini CLI |
