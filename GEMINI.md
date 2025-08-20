# Gemini CLI Guidelines for `libminizinc` Project

This document outlines the guidelines and best practices for the Gemini CLI agent when interacting with the `libminizinc` project. Adhering to these guidelines will ensure efficient, safe, and consistent development.

## 1. Project Overview and Philosophy

*   **Purpose:** The `libminizinc` project aims to integrate MiniZinc models with Rust applications, enabling advanced constraint programming and optimization. It serves as a foundational layer for building intelligent systems.
*   **Modularity:** The project emphasizes a highly modular structure, often adhering to a "one declaration per file" principle where applicable.
*   **Language Focus:** Primarily Rust, with interactions with MiniZinc. Avoid introducing Python, Golang, or TypeScript.
*   **Build System:** Uses Cargo for Rust projects.
*   **Testing:** Unit tests are crucial for new features and bug fixes.

## 2. General Interaction Guidelines

*   **Verbosity:** Be verbose in explanations, especially for complex changes or when introducing new concepts. Add comments to code where clarity is enhanced.
*   **Confirmation:** Seek confirmation for significant actions, especially those that modify the file system or codebase extensively, unless explicitly told to "yolo" or proceed without waiting.
*   **Error Handling:** Prioritize resolving compilation errors before addressing warnings. Address warnings systematically.
*   **Commit Messages:** Provide clear, concise, and informative commit messages. Use `git commit -F <file>` for multi-line messages.

## 3. Tool Usage

### 3.1. `cargo` Commands

*   **`cargo build --workspace`:** Use this command to build all crates in the workspace. This is the primary command for checking compilation status.
*   **`cargo test --workspace`:** Use this command to run all tests in the workspace.
*   **Avoiding `cargo clean` and `cargo update`:** Unless absolutely necessary (e.g., to resolve persistent build issues that cannot be fixed otherwise), avoid `cargo clean` and `cargo update` due to potentially long build times. Trust incremental builds.

### 3.2. `git` Commands

*   **`git status`:** Always check `git status` before and after making changes to understand the state of the repository.
*   **`git add <file>`:** Stage specific files for commit.
*   **`git commit -F <file>`:** Commit staged changes using a message from a file. Always write the commit message to a temporary file first.
*   **Submodules:** Be aware of submodules (e.g., `minizinc_data/huggingface`, `vendor/Ipopt`). Changes within submodules require specific handling and should generally be managed by the user unless explicit instructions are given.

### 3.3. File System Tools (`read_file`, `write_file`, `replace`)

*   **Absolute Paths:** Always use absolute paths for `file_path` arguments.
*   **`read_file`:** Use to inspect file content before making changes.
*   **`write_file`:** Use to create new files or overwrite existing ones. Be cautious when overwriting.
*   **`replace`:** Use with extreme caution. Ensure `old_string` is precise and includes sufficient context (at least 3 lines before and after) to avoid unintended replacements. For multi-line replacements, ensure exact matching of whitespace and indentation. If a complex multi-line replacement is needed, consider a two-step approach: delete old content, then write new content.

### 3.4. Shell Commands (`run_shell_command`)

*   **Purpose:** Use for executing shell commands like `cargo` and `git` commands.
*   **Security:** Be mindful of security implications when executing shell commands. Explain the purpose and potential impact of commands that modify the file system or system state.

## 4. Specific Project Conventions and Quirks

*   **`rust_lattice_project`:** This crate is primarily for demonstrating Rust language features and testing macros. Warnings related to unused code within this crate (e.g., `unused_variables`, `unused_macros`) can often be suppressed with `#[allow(dead_code)]` or by adding example usages, as they are part of the learning/testing material.
*   **Documentation Location:** All project documentation should reside in the `docs/` directory.
*   **Error Handling in Rust:** Prefer `Result` and `Box<dyn std::error::Error>` for error propagation.

## 5. Current State and Next Steps

*   **Build Status:** The project currently builds cleanly with no errors or warnings.
*   **Documentation:** Initial documentation has been generated for `project_overview.md`, `zos_bootstrap_commands.md`, and `ragit_string_utils.md`.
*   **Future Tasks:** Be prepared to generate more documentation, QA procedures, or assist with further development tasks as requested by the user.
