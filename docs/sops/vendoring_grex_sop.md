# SOP: Vendoring `grex` (Regex Generator)

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the process for vendoring the `grex` (Generate Regular Expressions from Test Cases) tool into the project. Vendoring `grex` ensures that the project has a self-contained and reproducible dependency for generating regular expressions, which can be used for dynamic regex suggestion and patching within our Rust tools.

## 2. Scope
This SOP applies to the `grex` repository (https://github.com/pemistahl/grex) and its integration into the `vendor/` directory of this project.

## 3. Prerequisites
*   Git installed and configured.
*   Internet connectivity to clone the `grex` repository.

## 4. Procedure

### 4.1. Adding `grex` as a Git Submodule
To add `grex` as a submodule:
1.  Navigate to the root directory of the project: `/data/data/com.termux/files/home/storage/github/libminizinc/`
2.  Execute the following command:
    ```bash
    git submodule add https://github.com/pemistahl/grex vendor/grex
    ```
    This command clones the `grex` repository into the `vendor/grex` directory and adds it as a submodule entry in `.gitmodules`.

### 4.2. Initializing and Updating Submodules
If you are cloning the main repository for the first time or need to update submodules:
1.  Navigate to the project root directory.
2.  Execute the following commands:
    ```bash
    git submodule init
    git submodule update
    ```

### 4.3. Integrating `grex` as a Rust Dependency (if needed)
If `grex` is to be used as a library within a Rust crate (e.g., `fix_meme_yaml` for dynamic regex generation):
1.  Open the `Cargo.toml` file of the Rust crate that will depend on `grex` (e.g., `crates/fix_meme_yaml/Cargo.toml`).
2.  Add the following line under the `[dependencies]` section, pointing to the vendored path:
    ```toml
    grex = { path = "../../vendor/grex" }
    ```
    Adjust the `path` accordingly based on the relative location of the `vendor/grex` directory from your crate's `Cargo.toml`.

## 5. Verification
*   Confirm that the `vendor/grex` directory exists and contains the `grex` repository files.
*   Verify that `.gitmodules` contains an entry for `grex`.
*   If integrated as a Rust dependency, ensure the dependent crate builds successfully.

## 6. Troubleshooting
*   **Submodule Cloning Issues:** Ensure proper network connectivity and Git configuration.
*   **Dependency Resolution Errors:** If Rust compilation fails after adding `grex` as a dependency, check the `path` in `Cargo.toml` and ensure the `grex` repository is correctly cloned and updated.

## 7. Related Documents
*   `.gitmodules`
*   `vendor/grex/`
*   `crates/fix_meme_yaml/Cargo.toml` (example usage)
