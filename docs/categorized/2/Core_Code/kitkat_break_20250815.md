# KitKat Break - August 15, 2025

## Current Status and Learnings

### C ABI Standalone Test
*   The C++ wrapper (`libminizinc_c_wrapper.so`) builds successfully.
*   The standalone C test program (`c_abi_test/test_c_abi.cpp`) compiles and runs successfully, confirming the basic functionality of the C ABI.

### Rust FFI Linking Issues
*   We've encountered persistent and challenging runtime linking errors (`library "libminizinc_c_wrapper.so" not found`) when running Rust tests on Termux/Android.
*   Attempts to resolve this using:
    *   `LD_LIBRARY_PATH` (both relative and absolute paths)
    *   `build.rs` with `cargo:rustc-link-search` and `cargo:rustc-link-lib`
    *   `build.rs` with `cargo:rustc-link-arg` (absolute path)
    *   Copying `libminizinc_c_wrapper.so` to the Rust target directory (`tools/minizinc_ffi/target/debug/`)
    *   Setting `RUSTFLAGS="-C link-arg=-Wl,-rpath=..."`
*   None of these approaches have reliably resolved the runtime linking issue across all test runs. The problem seems deeply rooted in the Android/Termux linker behavior.

### MiniZinc GC and Memory Management
*   Rust tests are causing segmentation faults and MiniZinc GC assertions (`GC::alloc()` and `ASTStringData::destroy()`).
*   This is likely due to the global nature of MiniZinc's garbage collector and string interner, which conflict when multiple `MiniZinc::Env` instances are created and destroyed by Rust's isolated test runner.
*   Adding `MiniZinc::GCLock` to individual C wrapper functions was not sufficient, as the issue appears to be at a higher level of `MiniZinc::Env` lifecycle management.
*   Exposing `GC::lock()` and `GC::unlock()` via FFI was a preparatory step for a more centralized GC management strategy in Rust.

## Plan for After the KitKat Break

### 1. Address Rust Linking Issue (LD_PRELOAD Strategy)
*   **LD_PRELOAD**: The preferred strategy for ensuring `libminizinc_c_wrapper.so` is found at runtime is to use `LD_PRELOAD`. This bypasses standard linker search paths and explicitly loads the shared library before program execution.
*   **Action**: Ensure `build_test_c_rust.sh` correctly sets `LD_PRELOAD` for Rust test execution. This will involve verifying the path to `libminizinc_c_wrapper.so` and setting the environment variable appropriately.
*   **Alternative strategies (if LD_PRELOAD fails)**: If `LD_PRELOAD` proves insufficient, we may need to:
    *   Investigate `dlopen`/`LoadLibrary` in Rust to explicitly load the shared library at runtime.
    *   Consider a custom test harness in Rust that manages the shared library loading.
    *   As a last resort, explore if MiniZinc can be built as a static library to eliminate dynamic linking entirely.

### 2. Implement Global `MiniZinc::Env` in Rust Tests
*   This is the primary solution for the GC and memory management issues.
*   **Action**: Modify `tools/minizinc_ffi/src/tests/mod.rs` to:
    *   Use `lazy_static!` (already added to `Cargo.toml`) to create a single, global `MiniZincEnvironment` instance.
    *   Modify each test to use this global instance, ensuring only one `MiniZinc::GC` and string interner are active throughout the test suite.
    *   This will involve removing the `setup()` function and its calls from individual tests.

### 3. Clean Up Unused Imports in Rust
*   **Action**: Run `cargo fix --lib -p minizinc_ffi --tests` to address the minor warnings about unused imports.

## Next Steps (Immediate)
*   Create this documentation file.
*   Commit the current state of the repository.
*   Take a well-deserved KitKat break.