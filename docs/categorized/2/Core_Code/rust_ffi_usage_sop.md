# SOP: Using the Rust FFI for MiniZinc

This Standard Operating Procedure (SOP) outlines the steps required to use the established Rust FFI for MiniZinc in a Rust project.

## 1. Prerequisites

Before using the Rust FFI, ensure the following components are correctly set up:

*   **MiniZinc C++ Library (`libmzn.so`)**: The core MiniZinc C++ library must be built as a shared library.
    *   It should be located at: `/data/data/com.termux/files/home/storage/github/libminizinc/build/libmzn.so`
    *   If not present, follow the steps in `docs/rust_ffi_process.md` under "2.1 Building `libmzn.so` as a Shared Library".

*   **MiniZinc C Wrapper (`libminizinc_c_wrapper.so`)**: The C wrapper library that exposes a C API to Rust.
    *   It should be located at: `/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper/libminizinc_c_wrapper.so`
    *   If not present, follow the steps in `docs/rust_ffi_process.md` under "2.2 Creating the C Wrapper (`minizinc_c_wrapper`)".

## 2. Including `minizinc_ffi` in Your Rust Project

To use the FFI, add `minizinc_ffi` as a dependency in your Rust project's `Cargo.toml`.

```toml
# In your Rust project's Cargo.toml
[dependencies]
minizinc_ffi = { path = "/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_ffi" }
```

## 3. Setting `LD_LIBRARY_PATH` for Runtime Linking

The Rust executable needs to know where to find `libminizinc_c_wrapper.so` and `libmzn.so` at runtime. This is achieved by setting the `LD_LIBRARY_PATH` environment variable.

**For development and testing (before running `cargo run` or `cargo test`):**

```bash
export LD_LIBRARY_PATH=/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper:/data/data/com.termux/files/home/storage/github/libminizinc/build:$LD_LIBRARY_PATH
```

**Note:** This environment variable setting is session-specific. For persistent setup, consider adding it to your shell's profile (`.bashrc`, `.zshrc`) or using a build script that manages it.

## 4. Basic Usage Example (Calling `minizinc_get_version_string`)

Once the `minizinc_ffi` crate is included and `LD_LIBRARY_PATH` is set, you can call the FFI functions from your Rust code.

```rust
// In your Rust project's src/main.rs or src/lib.rs

use std::ffi::CStr;

// Import the FFI functions from the minizinc_ffi crate
// The actual extern "C" block is defined within minizinc_ffi/src/lib.rs
// You would typically use a 'use' statement to bring the functions into scope.
// For example, if minizinc_ffi exposes a module 'bindings' with the extern "C" block:
// use minizinc_ffi::bindings::minizinc_get_version_string;

// For simplicity, assuming minizinc_get_version_string is directly exposed by the crate root
// (as it is in our current minizinc_ffi/src/lib.rs for testing purposes)
extern "C" {
    fn minizinc_get_version_string() -> *const std::os::raw::c_char;
}

fn main() {
    // Call the FFI function (unsafe because it's an FFI call)
    let version_cstr = unsafe { minizinc_get_version_string() };

    // Convert the C string to a Rust string
    let version = unsafe {
        CStr::from_ptr(version_cstr)
            .to_str()
            .expect("Invalid UTF-8 string from C")
    };

    println!("MiniZinc Version: {}", version);
}
```

## 5. Troubleshooting

*   **`CANNOT LINK EXECUTABLE ... library "libminizinc_c_wrapper.so" not found`**: Ensure `LD_LIBRARY_PATH` is correctly set and points to the directory containing `libminizinc_c_wrapper.so`.
*   **`CANNOT LINK EXECUTABLE ... library "libmzn.so" not found`**: Ensure `LD_LIBRARY_PATH` also includes the directory containing `libmzn.so`.
*   **Compilation errors in `minizinc_ffi`**: Refer to `docs/rust_ffi_process.md` for details on how the C wrapper was built and its current limitations (e.g., hardcoded version string, commented-out includes).
