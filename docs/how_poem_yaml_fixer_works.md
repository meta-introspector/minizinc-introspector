# How `poem_yaml_fixer` Works (Current State)

The `poem_yaml_fixer` is a Rust CLI tool designed to process Markdown files, primarily focusing on their YAML front matter. Its core functionalities include:

## 1. Configuration Loading

Upon startup, `poem_yaml_fixer` loads its regex patterns and associated callback functions from `crates/poem_yaml_fixer/src/regex_patterns.toml`. These patterns are compiled into `regex::Regex` objects and stored in a `FunctionRegistry` (a `HashMap` mapping callback names to `(PoemFunctionMetadata, Box<dyn Fn(...)>)`).

## 2. Main Processing Flow (Default Behavior)

When run without specific test flags, the tool processes Markdown files (e.g., from `docs/poems/`). For each file:
*   It reads the file content.
*   It attempts to extract and process YAML front matter and poem body.
*   It applies registered regex patterns to the content, triggering associated callback functions for matching lines.
*   It reconstructs and writes the fixed content back to the file (or performs a dry run).

## 3. `--generate-regex-report`

This flag triggers a standalone report generation. It reads `regex_patterns.toml`, compiles each regex, and prints a detailed report to `stdout`. The report includes:
*   Regex `name`
*   Regex `pattern`
*   Associated `callback_function`
*   `Compilation Status` (SUCCESS/FAILED)

This feature is used for verifying the syntax and compilation of all configured regex patterns.

## 4. `--test-yaml <FILE_PATH>`

This flag enables a testing mode for specific YAML input. It takes a YAML file (e.g., `test.yml`) as input, which is expected to contain a `file_path` and an `unmatched_lines` array.
*   It parses the input YAML file.
*   It iterates through each `line` in the `unmatched_lines` array.
*   For each `line`, it attempts to match it against *all* currently loaded regex patterns in the `FunctionRegistry`.
*   If a `line` is matched by an existing regex, it reports the match.
*   If a `line` is *not* matched by any existing regex, it then uses the `grex` library to generate a new, precise regex pattern for that specific `line`.
*   The generated `grex` regex, along with a suggested TOML entry (including `name`, `pattern`, and `callback_function`), is printed to `stdout`. This output is designed to be "cherry-picked" by the user into `regex_patterns.toml` for future integration.

This `--test-yaml` feature serves as a powerful debugging and development aid for identifying missing regex patterns and generating new ones for previously unhandled input lines. It does *not* modify any files or `mod.rs` automatically; it only provides suggestions.
