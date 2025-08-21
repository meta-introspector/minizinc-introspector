# SOP: Poem Formatter Usage

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the steps for using the `poem_formatter` Rust tool to standardize the format of Markdown poem files within the `docs/poems/` directory. The tool moves the main poem content into the YAML front matter, making it suitable for static site generators and other metadata-driven processes.

## 2. Scope
This SOP applies to all Markdown files located in the `/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems/` directory, excluding `index.md`.

## 3. Prerequisites
*   Rust toolchain installed.
*   The `poem_formatter` crate built and available in `./target/release/`.

## 4. Procedure

### 4.1. Building the `poem_formatter`
If the `poem_formatter` executable is not yet built or needs to be rebuilt after code changes:
1.  Navigate to the project root directory: `/data/data/com.termux/files/home/storage/github/libminizinc/`
2.  Execute the following command:
    ```bash
    cargo build --package poem_formatter --release
    ```
    This will compile the tool and place the executable at `./target/release/poem_formatter`.

### 4.2. Running the `poem_formatter`
To process the poem files:
1.  Navigate to the project root directory: `/data/data/com.termux/files/home/storage/github/libminizinc/`
2.  Execute the following command:
    ```bash
    ./target/release/poem_formatter
    ```
    The tool will iterate through all `.md` files in `docs/poems/` (excluding `index.md`), read their content, move the poem body into the `poem_body` field of the YAML front matter, and overwrite the original file.

## 5. Verification
After running the tool, you can verify its operation by:
*   Inspecting a few modified poem files (e.g., `docs/poems/some_poem.md`) to confirm that the poem content is now within the `poem_body` field in the YAML front matter.
*   Checking `git status` to see the modified files.

## 6. Troubleshooting
*   **YAML Parsing Errors:** If the tool reports YAML parsing errors, inspect the specified file and line number. Ensure that the YAML front matter is correctly formatted, especially regarding quoting of strings and proper indentation.
*   **File Not Found Errors:** Ensure the `docs/poems/` directory exists and contains Markdown files.
*   **Build Errors:** If `cargo build` fails, check the error messages for Rust compilation issues.

## 7. Related Documents
*   `docs/poems/index.md`
*   `crates/poem_formatter/src/main.rs`
*   `crates/poem_formatter/Cargo.toml`
