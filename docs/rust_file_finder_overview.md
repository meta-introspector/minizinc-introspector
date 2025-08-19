# `rust_file_finder` Overview

The `rust_file_finder` is a Rust utility designed to traverse a specified directory, identify Rust source files (`.rs`), and determine the Cargo package to which each file belongs.

## Functionality

- **File Traversal:** Uses the `walkdir` crate to recursively search for files within a predefined root directory (currently `/data/data/com.termux/files/home/storage/github/`).
- **Rust File Identification:** Filters for files with the `.rs` extension.
- **Cargo.toml Discovery:** For each identified Rust file, it searches upwards through the directory hierarchy to locate the nearest `Cargo.toml` file.
- **Package Information Extraction:** Parses the found `Cargo.toml` to extract the `package.name` and `package.version`.
- **Output:** Prints information about each found Rust file, including its path, the associated Cargo package name, version, and the path to its `Cargo.toml`. If a Rust file does not appear to belong to a Cargo package (no `Cargo.toml` found), it reports that as well.

## Purpose

This tool is useful for:
- Gaining an understanding of the Rust project structure within a large repository.
- Identifying orphaned Rust files that are not part of any defined Cargo package.
- Assisting in code analysis and dependency mapping by linking source files to their containing packages.
