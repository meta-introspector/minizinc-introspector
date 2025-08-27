# SOP: External Rust Crate Integration

## Purpose

This Standard Operating Procedure (SOP) outlines the standardized process for integrating external Rust crates into the project. The goal is to ensure consistency, maintainability, and loose coupling with third-party dependencies, facilitating future modifications or replacements.

## Scope

This SOP applies to all new and existing external Rust crates intended for integration into the `libminizinc` project.

## Procedure

### 1. Search for Crates

*   **Objective:** Identify suitable Rust crates that provide the required functionality.
*   **Action:**
    *   Utilize `crates.io` as the primary source for discovering Rust crates.
    *   Evaluate crates based on:
        *   Functionality and feature set.
        *   License compatibility (ensure it aligns with project licensing).
        *   Community support and maintenance activity (e.g., recent commits, open issues).
        *   Documentation quality and examples.
        *   Code quality and adherence to Rust best practices.
        *   Performance characteristics (if critical for the use case).

### 2. Find the GitHub Repository

*   **Objective:** Locate the official GitHub (or other Git-based forge) repository for the selected crate.
*   **Action:**
    *   Navigate to the crate's page on `crates.io` and look for the "Repository" link.
    *   Verify that the repository is the official source and actively maintained.

### 3. Vendorize the Crate (Git Submodule)

*   **Objective:** Integrate the external crate's source code directly into our project's `vendor` directory to ensure build reproducibility, offline compilation, and controlled dependency management.
*   **Action:**
    *   Use `git submodule add <repository_url> vendor/<crate_name>` to add the external crate as a Git submodule.
        *   **Example:** `git submodule add https://github.com/example/some-crate.git vendor/some-crate`
    *   Update the `Cargo.toml` of the dependent crate(s) to reference the vendorized path.
        *   **Example:** Change `some-crate = "1.0.0"` to `some-crate = { path = "../../vendor/some-crate" }` (adjust path as necessary based on project structure).
    *   Run `cargo build` to ensure the vendorized crate compiles correctly within the project context.

### 4. Write SOPs (Documentation)

*   **Objective:** Document the integration process, any specific configurations, known issues, and future considerations related to the vendorized crate.
*   **Action:**
    *   Create a dedicated markdown file (e.g., `docs/sops/crate_name_integration_notes.md`) or update an existing relevant SOP.
    *   Include:
        *   **Crate Purpose:** A brief description of what the crate does and why it was chosen.
        *   **Version:** The specific version or commit hash of the vendorized crate.
        *   **Integration Steps:** Detailed steps taken during vendorization and configuration.
        *   **API Notes:** Any specific API quirks, common usage patterns, or important considerations for developers using the wrapped functionality.
        *   **Known Issues:** Document any identified bugs, limitations, or warnings (e.g., lifetime warnings from `octocrab`).
        *   **Future Work:** Outline potential future improvements, upgrades, or alternative crates to consider.

### 5. Wrap and Integrate Loosely

*   **Objective:** Create an abstraction layer or wrapper around the vendorized crate to decouple its specific implementation details from our core application logic. This promotes loose coupling, simplifies future dependency swaps, and allows for custom extensions.
*   **Action:**
    *   Create a new module or crate within our project (e.g., `crates/our_crate_wrapper`) that acts as an interface to the vendorized crate.
    *   Define traits or public functions in this wrapper that expose only the necessary functionality from the vendorized crate.
    *   Implement these traits/functions by calling the underlying vendorized crate's APIs.
    *   Ensure that our core application code interacts only with this wrapper, never directly with the vendorized crate.
    *   **Example:** If integrating a logging crate, define our own `log` trait and implement it using the vendorized logger. If we later switch logging crates, only the wrapper implementation needs to change.

## Quality Control

*   All steps must be followed for each new crate integration.
*   Documentation (SOPs) must be kept up-to-date with any changes to the integrated crates or the integration process.
*   Regularly review vendorized dependencies for security vulnerabilities and updates.

## Related Documents

*   `docs/sops/code_doc_update_sop.md` (General Code and Documentation Update Procedure)
*   `docs/sops/git_submodule_management.md` (if a more detailed submodule SOP is needed)
