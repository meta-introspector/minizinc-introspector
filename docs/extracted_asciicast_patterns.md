# Extracted Regex Patterns from Asciicast Data

This document outlines the process of extracting regex patterns from the `docs/asciicast21.cast` file using the `asciicast_processor` tool, and lists the extracted patterns.

## Process:

1.  **Asciicast Processing:** The `asciicast_processor` tool was used to process the raw asciicast data from `docs/asciicast21.cast`. This tool is designed to parse asciicast files and extract meaningful patterns from the terminal output.

    The command executed was:
    ```bash
    cargo run -p asciicast_processor --release docs/asciicast21.cast generate --rust-output-file docs/main_patterns.rs --limit 10
    ```
    This command instructed the `asciicast_processor` to:
    *   Run in release mode (`--release`).
    *   Process `docs/asciicast21.cast` as the input file.
    *   Use the `generate` subcommand to create Rust code representing the extracted patterns.
    *   Output the generated Rust code to `docs/main_patterns.rs`.
    *   Limit the processing to the first 10 events (`--limit 10`) for initial analysis.

2.  **Pattern Extraction:** After the `asciicast_processor` generated the `docs/main_patterns.rs` file (which contains Rust code with `#[poem_function(...)]` attributes embedding the patterns), the `grep` command was used to extract only the regex patterns from this generated file.

    The command executed was:
    ```bash
    grep -oP 'pattern = "\K[^"+' docs/main_patterns.rs
    ```
    This command:
    *   Used Perl-compatible regular expressions (`-P`).
    *   Printed only the matched parts (`-o`).
    *   Extracted the content within `pattern = "..."` by using `\K` to reset the match start, effectively capturing only the regex string itself.

## Extracted Patterns (from the first 10 events):

```
^
^e': g$
^ascii$
^8$
^\\(reve$
^7$
^19$
^g': g$
^~/\\.\.\.$
```

These patterns represent the regular expressions identified by the `asciicast_processor` from the initial events of the asciicast stream. Further analysis would involve processing more events and potentially refining these patterns.
