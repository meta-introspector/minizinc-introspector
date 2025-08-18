# SOP: Generating `constants.rs` with MiniZinc Proof

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the process for generating and maintaining the `crates/zos-bootstrap/src/constants.rs` file. This process leverages MiniZinc to "prove" which string constants are actively used within the codebase, ensuring that the `constants.rs` file contains only necessary declarations and helps eliminate unused constant warnings.

## 2. Scope
This SOP applies to the `crates/zos-bootstrap/src/constants.rs` file and any Rust source code within the `libminizinc` project that defines or uses string constants.

## 3. Procedure

### 3.1. Generate Proposed `constants.rs` Content

Execute the `generate-constants-file` command to extract all string literals from the Rust codebase and process them through a MiniZinc model. The output will be a proposed `constants.rs` file printed to your console.

```bash
cargo run --package zos-bootstrap -- generate-constants-file --output-file crates/zos-bootstrap/src/constants.rs
```

**Note:** Due to current environmental limitations, the tool cannot directly overwrite `crates/zos-bootstrap/src/constants.rs`. The generated content will be printed to `stdout`.

### 3.2. Manually Update `constants.rs`

Carefully copy the entire content displayed in your console (between the `--- Generated Constants ---` markers) and paste it into the `crates/zos-bootstrap/src/constants.rs` file, replacing its existing content.

### 3.3. Verify Constant Usage and Compilation

After manually updating the `constants.rs` file, run the following commands to verify that only used constants are present and that the project still compiles without new warnings or errors.

#### 3.3.1. Check for Unused Constants

This command will re-run the constant usage proof and should ideally report no unused constants from `constants.rs`.

```bash
cargo run --package zos-bootstrap -- extract-constants --prove-constants-usage
```

#### 3.3.2. Compile the Project

This command will compile the entire project and should report no new warnings or errors related to constants.

```bash
cargo check
```

## 4. Tools
*   `zos-bootstrap` (Rust crate within `libminizinc` project)
*   MiniZinc (used internally by `zos-bootstrap`)
*   Rust `cargo` command

## 5. Expected Outcome
*   The `crates/zos-bootstrap/src/constants.rs` file contains only string constants that are actively used within the codebase.
*   The `cargo check` command reports no warnings related to unused constants in `constants.rs`.
*   The `extract-constants --prove-constants-usage` command reports no unused constants from `constants.rs`.
*   Improved code maintainability and reduced technical debt related to constant management.
