# MiniZinc FFI Usage in Rust

This document outlines how to use the Rust Foreign Function Interface (FFI) to interact with the MiniZinc library, enabling parsing and inspection of MiniZinc models directly from Rust.

## 1. Project Setup

Ensure your Rust project is configured to link against the `minizinc_ffi` crate and the underlying C++ MiniZinc library. The `build.rs` file in your Rust crate should handle the necessary linking flags.

## 2. Building the Project

To build both the C++ FFI wrapper and the Rust project, navigate to the project root directory and run the `build_and_test.sh` script:

```bash
../../build_and_test.sh
```

This script will:
1. Build the C++ FFI wrapper using CMake.
2. Build the Rust `minizinc_ffi` crate and run its tests.

## 3. Basic Usage: Parsing and Inspecting a MiniZinc Model

The `minizinc_ffi` crate provides a `MiniZincEnvironment` struct to manage the MiniZinc context and `MiniZincModel` to represent a parsed MiniZinc model.

Here's an example of how to parse a simple MiniZinc model and access its basic properties:

```rust
use minizinc_ffi::MiniZincEnvironment;

fn main() {
    // 1. Create a MiniZinc environment
    let env = MiniZincEnvironment::new().expect("Failed to create MiniZinc environment");

    // 2. Define a MiniZinc model code and a filename
    let model_code = "int: x; solve satisfy;";
    let filename = "my_model.mzn";

    // 3. Parse the MiniZinc model
    let model = env.parse_model(model_code, filename).expect("Failed to parse MiniZinc model");

    // 4. Access basic model properties
    println!("Parsed Model Filename: {}", model.filename());
    println!("Parsed Model Filepath: {}", model.filepath());
    println!("Parsed Model Number of Items: {}", model.num_items());

    // The MiniZincModel is automatically freed when it goes out of scope
    // due to its Drop implementation.
}
```

## 4. Running Tests

The `minizinc_ffi` crate includes unit tests that demonstrate its functionality. You can run them using:

```bash
cargo test --package minizinc_ffi
```

These tests cover environment creation, model parsing, and basic data parsing.

## 5. Further Development

This FFI provides a foundation for deeper integration. Future work can include:
*   **Recursive dumping of model contents:** Traversing the `MiniZincItem` hierarchy to extract detailed information about variable declarations, constraints, and other model elements.
*   **Accessing specific item types:** Implementing methods on `MiniZincItem` to safely cast to specific item types (e.g., `VarDeclI`, `ConstraintI`) and access their properties.
*   **Solving models:** Integrating with MiniZinc's solving capabilities to find solutions to parsed models.
