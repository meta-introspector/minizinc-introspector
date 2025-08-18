# Debugging Plan: Bisecting SIGSEGV in MiniZinc C Wrapper

## 1. Problem Statement
The Rust FFI for MiniZinc (`minizinc_ffi`) consistently encounters a `SIGSEGV` (segmentation fault) during the "Flattening" phase of MiniZinc model processing when the standard library path is provided to the `Flattener` constructor. This occurs even after integrating the C wrapper (`minizinc_c_wrapper`) into the main MiniZinc CMake build system, suggesting a deeper incompatibility or issue within the MiniZinc library itself on the Android environment.

The `SIGSEGV` is accompanied by a generic "MiniZinc parsing error (captured): | Exception: type error" message, indicating an exception is thrown but the process crashes before it can be fully handled.

## 2. Current State of Relevant Files

### 2.1. `tools/minizinc_c_wrapper_refactored/minizinc_c_wrapper.cpp`
```cpp
#include "minizinc_c_wrapper.h"
#include <minizinc/model.hh>
#include <minizinc/parser.hh>
#include <minizinc/flattener.hh> // Include Flattener header
#include <minizinc/solver.hh> // Include Solver header for SolverInitialiser

#include <iostream> // For debugging
#include <fstream>  // For temporary file
#include <cstdio>   // For remove (C-style file deletion)
#include <sstream>  // For std::stringstream

// Function to create a new MiniZinc environment
Flattener* minizinc_env_new() {
    // Create a static SolverInitialiser to ensure global MiniZinc setup
    static MiniZinc::SolverInitialiser solver_initialiser;

    // MiniZinc::Flattener constructor takes ostream& os, ostream& log, string stdlibDir
    // The stdlibDir is crucial for parsing.
    std::string stdlib_path = "/data/data/com.termux/files/home/storage/github/libminizinc/install/share/minizinc";
    MiniZinc::Flattener* new_flattener = new MiniZinc::Flattener(std::cout, std::cerr, stdlib_path);

    std::cerr << "DEBUG: minizinc_env_new - Created Flattener at: " << new_flattener << std::endl;
    // Removed: if (new_flattener) { std::cerr << "DEBUG: minizinc_env_new - Flattener->getEnv() returns: " << new_flattener->getEnv() << std::endl; } else { std::cerr << "DEBUG: minizinc_env_new - Flattener creation failed (nullptr)." << std::endl; }

    return reinterpret_cast<Flattener*>(new_flattener);
}

// Function to free a MiniZinc environment
void minizinc_env_free(Flattener* env) {
    delete reinterpret_cast<MiniZinc::Flattener*>(env);
}

// Function to parse a MiniZinc model from a string
MiniZincModel* minizinc_parse_model(Flattener* env_ptr, const char* model_str, const char* filename) {
    MiniZinc::Flattener* flattener = reinterpret_cast<MiniZinc::Flattener*>(env_ptr);
    
    std::string model_s(model_str);
    std::string filename_s = "/tmp/" + std::string(filename); // Prepend dummy absolute path
    std::vector<std::string> include_paths;
    // The standard library path is now handled by the Flattener constructor
    bool is_flatzinc = false;
    bool ignore_stdlib = false;
    bool parse_doc_comments = false;
    bool verbose = true; // Changed to true

    std::stringstream ss_err; // Create a stringstream for error capture
    std::ostream& err = ss_err; // Redirect error output to stringstream

    try {
        // Call flatten to initialize the Env
        flattener->flatten(model_s, filename_s);
        MiniZinc::Env* mzn_env_ptr = flattener->getEnv(); // Get the Env pointer from Flattener
        if (!mzn_env_ptr) {
            std::cerr << "Error: Flattener->getEnv() returned nullptr after flatten in minizinc_parse_model." << std::endl;
            return nullptr;
        }
        MiniZinc::Env& env = *mzn_env_ptr; // Dereference the pointer

        // After flattening, the Flattener's internal Env should contain the parsed model.
        // We retrieve the model directly from the Env.
        MiniZinc::Model* model = env.model();
        if (!model) {
            std::cerr << "Error: Env->model() returned nullptr after flatten in minizinc_parse_model." << std::endl;
            return nullptr;
        }
        return reinterpret_cast<MiniZincModel*>(model);
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc parsing error (captured): " << e.what() << std::endl;
        return nullptr;
    } catch (const std::exception& e) {
        std::cerr << "Standard exception (captured): " << e.what() << std::endl;
        return nullptr;
    } catch (...) {
        std::cerr << "Unknown exception during parsing (captured)." << std::endl;
        return nullptr;
    }
}

// Function to parse DZN data into a MiniZinc model
int minizinc_parse_data_from_string(Flattener* env_ptr, MiniZincModel* model_ptr, const char* data_str,
       const char* filename) {
    MiniZinc::Flattener* flattener = reinterpret_cast<MiniZinc::Flattener*>(env_ptr);
    // Call flatten to initialize the Env
    flattener->flatten(); // Call flatten without arguments to initialize Env
    MiniZinc::Env* mzn_env_ptr = flattener->getEnv(); // Get the Env pointer from Flattener
    if (!mzn_env_ptr) {
        std::cerr << "Error: Flattener->getEnv() returned nullptr after flatten in minizinc_parse_data_from_string." << std::endl;
        return -1;
    }
    MiniZinc::Env& env = *mzn_env_ptr; // Dereference the pointer
    MiniZinc::Model* model = reinterpret_cast<MiniZinc::Model*>(model_ptr);
    std::string data_s(data_str);
    std::string filename_s = "/tmp/" + std::string(filename); // Prepend dummy absolute path
    std::vector<std::string> include_paths;
    // The standard library path is now handled by the Flattener constructor
    bool is_flatzinc = false;
    bool ignore_stdlib = false;
    bool parse_doc_comments = false;
    bool verbose = true; // Changed to true

    std::stringstream ss_err; // Create a stringstream for error capture
    std::ostream& err = ss_err; // Redirect error output to stringstream

    // Workaround: MiniZinc::parse_data expects a vector of filenames, not a string directly.
    // Write the data string to a temporary file and pass its name.
    std::string temp_data_filename = filename_s + "_temp_data.dzn";
    std::ofstream temp_data_file(temp_data_filename);
    if (!temp_data_file.is_open()) {
        std::cerr << "Error: Could not open temporary data file for writing: " << temp_data_filename <<
        std::endl;
        return -1;
    }
    temp_data_file << data_s;
    temp_data_file.close();

    std::vector<std::string> data_files;
    data_files.push_back(temp_data_filename);

    try {
        MiniZinc::parse_data(env, model, data_files, include_paths, is_flatzinc,
                              ignore_stdlib, parse_doc_comments, verbose, err);

        // Clean up temporary file
        std::remove(temp_data_filename.c_str()); // Use C-style remove for broader compatibility

        return 0; // Success
    } catch (const MiniZinc::Exception& e) {
        std::cerr << "MiniZinc data parsing error (captured): " << e.what() << std::endl;
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (const std::exception& e) {
        std::cerr << "Standard exception (captured): " << e.what() << std::endl;
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    } catch (...) {
        std::cerr << "Unknown exception during data parsing (captured)." << std::endl;
        std::remove(temp_data_filename.c_str());
        return -1; // Failure
    }
}

// Function to free a MiniZinc model
void minizinc_model_free(MiniZincModel* model) {
    delete reinterpret_cast<MiniZinc::Model*>(model);
}

// Function to get version string (for testing FFI)
const char* minizinc_get_version_string() {
    return "2.9.4-introspector";
}
```

### 2.2. `tools/minizinc_ffi/src/lib.rs`
```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Opaque types for MiniZincModel and Flattener
type MiniZincModel = std::os::raw::c_void;
type Flattener = std::os::raw::c_void; // Changed from MiniZincEnv

unsafe extern "C" {
    fn minizinc_env_new() -> *mut Flattener; // Updated return type
    fn minizinc_env_free(env: *mut Flattener); // Updated parameter type
    fn minizinc_parse_model( // Updated function name
        env: *mut Flattener, // Updated parameter type
        model_str: *const c_char,
        filename: *const c_char,
    ) -> *mut MiniZincModel;
    fn minizinc_parse_data_from_string(
        env: *mut Flattener, // Updated parameter type
        model: *mut MiniZincModel,
        data_str: *const c_char,
        filename: *const c_char,
    ) -> std::os::raw::c_int;
    fn minizinc_model_free(model: *mut MiniZincModel);
    fn minizinc_get_version_string() -> *const c_char;
}

// Safe Rust wrappers for FFI functions
pub struct MiniZincEnvironment(*mut Flattener); // Updated to hold Flattener

impl MiniZincEnvironment {
    pub fn new() -> Result<Self, String> {
        let env_ptr = unsafe { minizinc_env_new() };
        if env_ptr.is_null() {
            Err("Failed to create MiniZinc environment".to_string())
        } else {
            Ok(MiniZincEnvironment(env_ptr))
        }
    }

    pub fn parse_model(&self, model_code: &str, filename: &str) -> Result<*mut MiniZincModel, String> {
        let model_cstr = CString::new(model_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let model_ptr = unsafe {
            minizinc_parse_model( // Updated function call
                self.0,
                model_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if model_ptr.is_null() {
            Err("Failed to parse MiniZinc model".to_string())
        } else {
            Ok(model_ptr)
        }
    }

    pub fn parse_data(&self, model: *mut MiniZincModel, data_code: &str, filename: &str) -> Result<(), String> {
        let data_cstr = CString::new(data_code).expect("CString::new failed");
        let filename_cstr = CString::new(filename).expect("CString::new failed");
        let result = unsafe {
            minizinc_parse_data_from_string(
                self.0, // Pass Flattener*
                model,
                data_cstr.as_ptr(),
                filename_cstr.as_ptr(),
            )
        };
        if result != 0 {
            Err("Failed to parse MiniZinc data".to_string())
        } else {
            Ok(())
        }
    }

    pub fn get_version_string(&self) -> String {
        let version_cstr = unsafe { minizinc_get_version_string() };
        unsafe { CStr::from_ptr(version_cstr).to_str().unwrap().to_string() }
    }
}

impl Drop for MiniZincEnvironment {
    fn drop(&mut self) {
        unsafe { minizinc_env_free(self.0) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_string() {
        let env = MiniZincEnvironment::new().unwrap();
        let version = env.get_version_string();
        println!("MiniZinc Version: {}", version);
        assert_eq!(version, "2.9.4-introspector");
    }

    #[test]
    fn test_env_creation_and_free() {
        let env = MiniZincEnvironment::new();
        assert!(env.is_ok());
        // Drop will be called automatically when env goes out of scope
    }

    #[test]
    fn test_parse_model_from_string() {
        let env = MiniZincEnvironment::new().unwrap();
        // Model with x defined
        let model_code = "int: x = 1; solve satisfy;";
        let filename = "test_model.mzn";
        let model_ptr = env.parse_model(model_code, filename);
        assert!(model_ptr.is_ok());
        let model = model_ptr.unwrap();
        assert!(!model.is_null());
        unsafe { minizinc_model_free(model) }; // Manually free the model
    }

    #[test]
    fn test_parse_data_from_string() {
        let env = MiniZincEnvironment::new().unwrap();
        // Model with x as a parameter (to be defined by data)
        let model_code = "int: x; solve satisfy;";
        let model_filename = "test_model_for_data.mzn";
        let model_ptr = env.parse_model(model_code, model_filename);
        assert!(model_ptr.is_ok()); // Ensure model parsing itself is successful
        let model = model_ptr.unwrap();
        assert!(!model.is_null());

        let data_code = "x = 10;";
        let data_filename = "test_data.dzn";
        let result = env.parse_data(model, data_code, data_filename); // Pass the model pointer
        assert!(result.is_ok());

        unsafe { minizinc_model_free(model) }; // Manually free the model
    }
}
```

### 2.3. `CMakeLists.txt` (relevant changes)
```cmake
# Libraries
include(cmake/targets/libmzn.cmake)
include(cmake/targets/minizinc_c_wrapper.cmake)
```

### 2.4. `cmake/targets/minizinc_c_wrapper.cmake`
```cmake
# Define the minizinc_c_wrapper library
add_library(minizinc_c_wrapper SHARED
    ${PROJECT_SOURCE_DIR}/tools/minizinc_c_wrapper_refactored/minizinc_c_wrapper.cpp
)

# Link against libmzn (the main MiniZinc library)
target_link_libraries(minizinc_c_wrapper PRIVATE mzn)

# Set include directories for the C wrapper
target_include_directories(minizinc_c_wrapper PRIVATE
    ${PROJECT_SOURCE_DIR}/include
    ${PROJECT_BINARY_DIR}/include
)

# Set RPATH for the C wrapper to find libmzn.so at runtime
# This is crucial for dynamic linking on Linux/Android
target_link_options(minizinc_c_wrapper PRIVATE
    -Wl,-rpath,\$ORIGIN/../lib # Relative to the shared library itself
    -Wl,-rpath,${CMAKE_INSTALL_PREFIX}/lib # Absolute path for installation
)

# Install the C wrapper library
install(TARGETS minizinc_c_wrapper
    DESTINATION ${CMAKE_INSTALL_LIBDIR}
)
```

## 3. Debugging Strategy: Print Statement Bisection

The goal is to pinpoint the exact line in `minizinc_c_wrapper.cpp` (or within the MiniZinc library calls) that causes the `SIGSEGV`.

**Steps:**
1.  **Instrument `minizinc_c_wrapper.cpp`:**
    *   Add `std::cerr << "DEBUG: <line_identifier>" << std::endl;` and `std::cerr.flush();` statements at strategic points within the `minizinc_env_new()`, `minizinc_parse_model()`, and `minizinc_parse_data_from_string()` functions.
    *   Start with broad sections (e.g., before and after `flattener->flatten()`, before and after `getEnv()`, before and after `env.model()`).
    *   Example:
        ```cpp
        // ...
        std::cerr << "DEBUG: Before flatten()" << std::endl; std::cerr.flush();
        flattener->flatten(model_s, filename_s);
        std::cerr << "DEBUG: After flatten()" << std::endl; std::cerr.flush();
        // ...
        ```
2.  **Recompile and Re-link:**
    ```bash
    cd build && cmake --build .
    ```
3.  **Run Rust FFI Tests:**
    ```bash
    cargo test --package minizinc_ffi
    ```
4.  **Analyze Output:** Observe the `stderr` output. The last debug message printed before the `SIGSEGV` indicates the problematic line or section.
5.  **Bisect:** Based on the crash location, add more granular debug statements within the problematic section, repeating steps 2-4 until the exact line is identified.

This systematic approach will help us narrow down the cause of the `SIGSEGV`.
