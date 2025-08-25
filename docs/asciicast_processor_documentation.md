# Asciicast Processor Tool Documentation

## Overview

The `asciicast_processor` is a Rust-based command-line tool designed to process asciinema recordings (`.cast` files). It provides functionalities to extract, analyze, and transform terminal session data, primarily for integration with the `libminizinc` project's "meme compilation" workflow.

## Features

Currently, the `asciicast_processor` supports the following subcommands:

*   **`generate`**: Generates Rust code containing `#[poem_function]` macros from asciicast output, based on hierarchical regex patterns.
*   **`analyze`**: Analyzes previously generated Rust code to extract patterns.
*   **`filter`**: Filters asciicast output by a given regex pattern and shows context around matches.
*   **`count-raw`**: Counts raw matches of a regex pattern within the input asciicast file.
*   **`extract-lines`**: Extracts all cleaned output lines from an asciicast file into a plain text file.

## Installation

The `asciicast_processor` is part of the `libminizinc` workspace. To build it, navigate to the project root and run:

```bash
cargo build --package asciicast_processor
```

This will compile the executable to `target/debug/asciicast_processor` (or `target/release/asciicast_processor` if built with `--release`).

## Usage

The `asciicast_processor` is invoked with the path to the asciicast input file, followed by a subcommand and its specific arguments.

```bash
target/debug/asciicast_processor <INPUT_FILE.cast> <SUBCOMMAND> [OPTIONS]
```

### Common Arguments

*   `<INPUT_FILE.cast>`: The absolute or relative path to the asciinema recording file.

### Subcommands and Arguments

#### `generate`

Generates Rust code from an asciicast file.

```bash
target/debug/asciicast_processor <INPUT_FILE.cast> generate [OPTIONS]
```

**Options:**

*   `--limit <NUMBER>`: Limit the number of events to process from the beginning. (Default: 10, conflicts with `--tail`)
*   `--tail <NUMBER>`: Process only the last N events. (Conflicts with `--limit`)
*   `--steps <NUM1,NUM2,...>`: Steps for hierarchical grouping (e.g., `--steps 5,3,1`). (Default: `5,3,1`)
*   `--rust-output-file <FILE>`: Output file to save the generated Rust code. (Required)
*   `--ascii-names`: Enable ASCII naming for Unicode characters and ANSI sequences.

#### `analyze`

Analyzes an asciicast file using previously generated Rust code.

```bash
target/debug/asciicast_processor <INPUT_FILE.cast> analyze [OPTIONS]
```

**Options:**

*   `--generated-rust-file <FILE>`: Path to the previously generated Rust code file (`.rs`). (Required)

#### `filter`

Filters asciicast output by regex and shows context.

```bash
target/debug/asciicast_processor <INPUT_FILE.cast> filter [OPTIONS]
```

**Options:**

*   `--limit <NUMBER>`: Limit the number of events to process from the beginning. (Default: 10)
*   `--regex <PATTERN>`: Regex pattern to filter lines. (Required)
*   `-C, --context <NUMBER>`: Show N lines of context around matching lines. (Default: 0)
*   `--occurrences <NUMBER>`: Limit the number of matching occurrences to find.

#### `count-raw`

Counts raw matches in the input file.

```bash
target/debug/asciicast_processor <INPUT_FILE.cast> count-raw [OPTIONS]
```

**Options:**

*   `--regex <PATTERN>`: Regex pattern to count raw matches. (Required)

#### `extract-lines`

Extracts all cleaned output lines from an asciicast file to a plain text file.

```bash
target/debug/asciicast_processor <INPUT_FILE.cast> extract-lines [OPTIONS]
```

**Options:**

*   `--output-file <FILE>`: Output file to save the extracted lines. (Required)

## Example: Extracting Lines

To extract all output lines from `docs/asciicast21.cast` to `docs/asciicast21_extracted_lines.txt`:

```bash
target/debug/asciicast_processor /data/data/com.termux/files/home/storage/github/libminizinc/docs/asciicast21.cast extract-lines --output-file /data/data/com.termux/files/home/storage/github/libminizinc/docs/asciicast21_extracted_lines.txt
```
