# SOP: Extending the Rust FFI for MiniZinc

This Standard Operating Procedure (SOP) outlines the process for extending the existing Rust FFI to MiniZinc by adding new functionality. This involves modifying the C wrapper and updating the Rust bindings.

## 1. Understanding the Architecture

The current Rust FFI to MiniZinc relies on a C wrapper (`minizinc_c_wrapper`) that acts as an intermediary between Rust and the core MiniZinc C++ library (`libmzn.so`).

*   **Rust (`minizinc_ffi` crate)**: Declares C functions using `unsafe extern "C"` and provides safe Rust wrappers.
*   **C Wrapper (`minizinc_c_wrapper.h`, `minizinc_c_wrapper.cpp`)**: Exposes a clean C API. Implements these C functions by calling the underlying MiniZinc C++ API.
*   **MiniZinc C++ Library (`libmzn.so`)**: The core MiniZinc functionality.

## 2. Adding New Functionality

To expose new MiniZinc functionality to Rust, follow these steps:

### 2.1 Modify the C Wrapper (`minizinc_c_wrapper`)

1.  **Identify MiniZinc C++ Functionality**: Determine the specific MiniZinc C++ classes, methods, or functions you want to expose. Consult the MiniZinc C++ source code (e.g., `model.hh`, `parser.hh`, `env.hh`, `values.hh`).

2.  **Declare C Function in `minizinc_c_wrapper.h`**:
    *   Add a new function declaration within the `extern "C"` block in `tools/minizinc_c_wrapper/minizinc_c_wrapper.h`.
    *   **C ABI Compatibility**: Ensure the function signature uses C-compatible types (e.g., `const char*` for strings, `void*` for opaque C++ objects, `int` for booleans/integers).
    *   **Memory Management**: Define clear ownership rules for memory allocated by C++ functions and passed to Rust. If Rust takes ownership, ensure a corresponding `_free` function is provided in the C wrapper.
    *   **Error Handling**: Decide how errors from the C++ layer will be propagated to Rust (e.g., return codes, error messages as C strings).

    **Example (conceptual):**
    ```c
    // In minizinc_c_wrapper.h
    MiniZincModel* minizinc_parse_model_from_string(MiniZincEnv* env, const char* model_str, const char* filename);
    ```

3.  **Implement C Function in `minizinc_c_wrapper.cpp`**:
    *   Implement the declared C function in `tools/minizinc_c_wrapper/minizinc_c_wrapper.cpp`.
    *   Use `reinterpret_cast` to convert between opaque `void*` pointers (from C) and MiniZinc C++ object pointers.
    *   Call the appropriate MiniZinc C++ methods.
    *   Handle C++ exceptions and convert them into C-compatible error indicators (e.g., return `nullptr` or an error code).
    *   Ensure proper memory management (e.g., `new` and `delete` calls are balanced).

    **Example (conceptual):**
    ```cpp
    // In minizinc_c_wrapper.cpp
    MiniZincModel* minizinc_parse_model_from_string(MiniZincEnv* env_ptr, const char* model_str, const char* filename) {
        MiniZinc::Env* env = reinterpret_cast<MiniZinc::Env*>(env_ptr);
        // ... implementation using MiniZinc::parse_from_string ...
        return reinterpret_cast<MiniZincModel*>(model);
    }
    ```

4.  **Compile the C Wrapper**:
    *   Navigate to the `libminizinc` project's root.
    *   Compile `minizinc_c_wrapper.cpp` to an object file:
        ```bash
        g++ -c -fPIC -I/data/data/com.termux/files/home/storage/github/libminizinc/include \
            -I/data/data/com.termux/files/home/storage/github/libminizinc/build/include \
            -I/data/data/com.termux/files/usr/include/c++/v1 \
            -I/data/data/com.termux/files/usr/include \
            -o /data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper/minizinc_c_wrapper.o \
            /data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper/minizinc_c_wrapper.cpp
        ```
    *   Link the object file into the shared library:
        ```bash
        g++ -shared -o /data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper/libminizinc_c_wrapper.so \
            /data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper/minizinc_c_wrapper.o \
            -L/data/data/com.termux/files/home/storage/github/libminizinc/build -lmzn
        ```

### 2.2 Update Rust Bindings (`minizinc_ffi`)

1.  **Declare FFI Function in `minizinc_ffi/src/lib.rs`**:
    *   Add the new C function declaration within the `unsafe extern "C" { ... }` block in `tools/minizinc_ffi/src/lib.rs`.
    *   Ensure correct type mapping from C types to Rust FFI types (e.g., `*const c_char`, `*mut c_void`).

    **Example (conceptual):**
    ```rust
    // In minizinc_ffi/src/lib.rs
    unsafe extern "C" {
        fn minizinc_parse_model_from_string(
            env: *mut MiniZincEnv,
            model_str: *const c_char,
            filename: *const c_char,
        ) -> *mut MiniZincModel;
    }
    ```

2.  **Create Safe Rust Wrapper (Optional but Recommended)**:
    *   For better ergonomics and safety, create a safe Rust function that wraps the `unsafe` FFI call. This function should handle:
        *   Conversion of Rust types to C-compatible types (e.g., `&str` to `*const c_char`).
        *   Error handling (e.g., checking return codes, converting C error messages to Rust `Result` types).
        *   Memory management (e.g., ensuring C strings are properly freed if Rust takes ownership).

    **Example (conceptual):**
    ```rust
    // In minizinc_ffi/src/lib.rs (outside extern "C" block)
    pub fn parse_model(env: &mut MiniZincEnv, model_code: &str, filename: &str) -> Result<MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let model_ptr = unsafe {
            minizinc_parse_model_from_string(
                env as *mut MiniZincEnv, // Cast mutable reference to raw pointer
                model_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if model_ptr.is_null() {
            Err("Failed to parse MiniZinc model".to_string()) // More detailed error handling needed
        } else {
            Ok(model_ptr)
        }
    }
    ```

3.  **Write Tests**:
    *   Add unit tests in `minizinc_ffi/src/lib.rs` to verify the new FFI function.
    *   Ensure `LD_LIBRARY_PATH` is set correctly when running tests.

    **Example (conceptual):**
    ```rust
    // In minizinc_ffi/src/lib.rs (within tests module)
    #[test]
    fn test_parse_model() {
        let mut env = unsafe { minizinc_env_new() };
        assert!(!env.is_null());

        let model_code = "int: x; solve satisfy;";
        let filename = "test_model.mzn";

        let model = parse_model(&mut env, model_code, filename).unwrap();
        assert!(!model.is_null());

        unsafe { minizinc_model_free(model) };
        unsafe { minizinc_env_free(env) };
    }
    ```

## 3. Best Practices for FFI Extension

*   **Start Small**: Expose minimal functionality first and expand incrementally.
*   **Clear Ownership**: Be explicit about who owns memory across the FFI boundary. Rust's ownership system needs careful consideration when dealing with raw pointers from C.
*   **Error Propagation**: Design a robust error handling mechanism. Avoid panicking in FFI functions; return error codes or `Result` types.
*   **Documentation**: Document each FFI function and its safe Rust wrapper thoroughly, including expected inputs, outputs, and any ownership rules.
*   **Testing**: Write comprehensive tests for all FFI functions to ensure correctness and prevent regressions.
*   **C++ ABI Stability**: Be aware that direct FFI to C++ can be fragile due to name mangling and ABI changes between compiler versions. The C wrapper mitigates this by providing a stable C API.
