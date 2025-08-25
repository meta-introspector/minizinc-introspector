# Experiment: Fake Chroot Build using `mini-act`

## Objective
To demonstrate the capability of the `mini-act` crate to create a temporary, isolated build environment (a "fake chroot"), copy its own source code into it, and then successfully build itself within that environment. This experiment also implicitly tested `cargo`'s ability to resolve and fetch dependencies in such an isolated setup.

## Procedure
1.  **Modified `crates/mini-act/Cargo.toml`**: Added the `tempfile` crate as a dependency to facilitate the creation and management of temporary directories.
2.  **Modified `crates/mini-act/src/main.rs`**: Implemented logic to:
    *   Create a temporary directory using `tempfile`.
    *   Determine the `mini-act` crate's root directory using `env!("CARGO_MANIFEST_DIR")`.
    *   Copy `mini-act`'s `Cargo.toml` and `src` directory into the temporary directory.
    *   Change the current working directory of the `mini-act` process to the temporary directory.
    *   Execute `cargo build` within this temporary environment using `std::process::Command`.
    *   Print the build status and output (stdout/stderr).
3.  **Dependency Resolution and Debugging**: Encountered and resolved compilation issues related to the `tempfile` crate's versioning and its transitive dependencies (`libc`, `rand`). This involved updating `tempfile` to a compatible version (`3.21.0`).
4.  **Execution**: Built the modified `mini-act` crate and then executed it from the project root.
5.  **Reversion**: After successful demonstration, the modifications to `crates/mini-act/Cargo.toml` and `crates/mini-act/src/main.rs` were reverted to restore the original functionality of `mini-act` as a workflow interpreter.

## Outcome
The experiment was successful. The `mini-act` executable, when run, successfully:
*   Created a temporary directory.
*   Copied its own source files into this temporary, isolated environment.
*   Changed its working directory to this temporary location.
*   Executed `cargo build`, which successfully compiled `mini-act` and its dependencies within the isolated environment.

This demonstrates the feasibility of using `mini-act` (or similar Rust utilities) to manage isolated build environments and perform builds, implicitly handling dependency fetching and resolution as part of the `cargo build` process.

## Note on Git Checkout and Submodules
This experiment focused on `mini-act`'s ability to build itself in an isolated environment. It did *not* involve cloning the entire `libminizinc` repository or its submodules into the temporary chroot, nor did it verify specific Git versions or the presence of particular commits. Testing `git checkout` of the main repository and its submodules would require a separate, dedicated procedure involving explicit `git clone`, `git submodule update --init --recursive` commands within the isolated environment.
