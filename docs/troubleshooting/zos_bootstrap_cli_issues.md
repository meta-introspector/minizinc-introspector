# Troubleshooting `zos-bootstrap` CLI Issues

This document details common issues encountered when building and running the `zos-bootstrap` executable, along with their resolutions.

## 1. `libminizinc_c_wrapper.so` Not Found Error

### Problem Description

When attempting to run Rust executables that depend on the `libminizinc_c_wrapper.so` shared library (e.g., `zos-bootstrap`), you might encounter a runtime error indicating that the library cannot be found. This typically manifests as:

```
CANNOT LINK EXECUTABLE "/data/data/com.termux/files/home/storage/github/libminizinc/target/debug/zos-bootstrap": library "libminizinc_c_wrapper.so" not found: needed by main executable
```

This error occurs because the dynamic linker cannot locate the required C++ shared library at runtime.

### Root Cause

The `zos-bootstrap` crate (and potentially other Rust crates in this project) relies on a C++ shared library, `libminizinc_c_wrapper.so`, for Foreign Function Interface (FFI) calls to the underlying MiniZinc C++ library. If this shared library is not built or is not placed in a location where the system's dynamic linker can find it, the executable will fail to launch.

### Resolution Steps

To resolve this issue, you need to ensure the C++ wrapper is built and its location is added to the `LD_LIBRARY_PATH` environment variable before executing the Rust program.

1.  **Build the C++ Wrapper:**
    The `libminizinc_c_wrapper.so` is built as part of the main `libminizinc` C++ project. You can compile it by running the dedicated build script from the project root:
    ```bash
    ./scripts/build_libminizinc.sh
    ```
    This script will configure and build the `libminizinc` project using CMake and Make, placing the compiled `libminizinc_c_wrapper.so` file in the `build/` directory (relative to the project root).

2.  **Set `LD_LIBRARY_PATH`:**
    The `LD_LIBRARY_PATH` environment variable tells the dynamic linker where to look for shared libraries. Since `libminizinc_c_wrapper.so` is located in the `build/` directory (which is not a standard system library path), you must set `LD_LIBRARY_PATH` to include this directory before running any executable that depends on it.

    For example, when running `zos-bootstrap` using `cargo run`:
    ```bash
    LD_LIBRARY_PATH=/data/data/com.termux/files/home/storage/github/libminizinc/build cargo run -p zos-bootstrap -- <command>
    ```
    Replace `<command>` with the actual `zos-bootstrap` command or subcommand you intend to run (e.g., `help`, `build`, `test`).

## 2. `zos-bootstrap` CLI Argument Issues: "Argument names must be unique, but 'help' is in use"

### Problem Description

When attempting to run `zos-bootstrap`, you might encounter an error message from the `clap` argument parsing library similar to:

```
Command launch: Argument names must be unique, but 'help' is in use by more than one argument or group (call `cmd.disable_help_flag(true)` to remove the auto-generated `--help`)
```

This error indicates a conflict where a custom argument or subcommand named `help` is defined, clashing with `clap`'s automatically generated `--help` flag.

### Root Cause

The `zos-bootstrap` crate's command-line interface is defined using the `clap` crate. At some point, a custom `help` subcommand or argument was introduced that conflicted with `clap`'s default `--help` flag generation. To resolve this, the `zos-bootstrap` CLI configuration was modified to explicitly disable the auto-generated `--help` flag.

### Resolution Steps

1.  **Ensure `zos-bootstrap` Crate is Updated:**
    The fix for this issue involves a change in the `zos-bootstrap` crate's source code. Specifically, the `Cli` struct in `crates/zos-bootstrap/src/cli.rs` has been updated to include `disable_help_flag = true` in its `#[command(...)]` attribute:
    ```rust
    #[derive(Parser)]
    #[command(author, version, about, long_about = None, disable_help_flag = true)]
    pub struct Cli {
        #[command(subcommand)]
        pub command: Option<Commands>,
    }
    ```
    Ensure your local copy of the `zos-bootstrap` crate is up-to-date and has this change applied.

2.  **Use `help` Subcommand Instead of `--help` Flag:**
    After the `disable_help_flag` is set to `true`, the `--help` flag will no longer function as expected. Instead, you must use the `help` subcommand to display help information for `zos-bootstrap` or its subcommands.

    To get general help for `zos-bootstrap`:
    ```bash
    LD_LIBRARY_PATH=/data/data/com.termux/files/home/storage/github/libminizinc/build cargo run -p zos-bootstrap -- help
    ```

    To get help for a specific subcommand (e.g., `build`):
    ```bash
    LD_LIBRARY_PATH=/data/data/com.termux/files/home/storage/github/libminizinc/build cargo run -p zos-bootstrap -- help build
    ```

By following these steps, you should be able to successfully build and interact with the `zos-bootstrap` CLI.

## Commit History

- [Commit 19bbe4f5ee5368d5c239e894df678af8b5541c49: feat: Document troubleshooting for zos-bootstrap CLI and FFI linking](docs/commits/19bbe4f5ee5368d5c239e894df678af8b5541c49_feat_Document_troubleshooting_for_zos-bootstrap_CLI_and_FFI_linking.md)