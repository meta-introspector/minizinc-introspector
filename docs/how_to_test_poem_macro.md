# How to Test the `poem_macro` (for N00bs)

This document will guide you through the process of testing the `poem_macro` in this project. Don't worry if you're new to Rust or command-line tools – we'll go step-by-step!

## What is a Macro?

In Rust, a macro is like a super-powered function that writes code for you. Instead of running at runtime, macros run *before* your code is compiled. They take some input and generate more Rust code, which then gets compiled along with the rest of your project.

The `poem_macro` is an "attribute macro," which means you use it by putting `#[poem_function]` right above a function definition. It's supposed to do something special to that function.

## The Test Setup

We have a special "test crate" (think of it as a mini-project just for testing this macro) called `poem_macro_test_crate`. This crate is designed to use the `poem_macro` and then try to compile the result.

You can find this test crate here: `/data/data/com.termux/files/home/storage/github/libminizinc/crates/poem_macro_test_crate`

Inside that folder, the main file is `src/lib.rs`, which looks something like this:

```rust
// crates/poem_macro_test_crate/src/lib.rs
use poem_macros::poem_function;
use anyhow::Result;
use std::collections::HashMap;

#[poem_function]
fn my_poem_function(
    _line: &str,
    _captures: Vec<String>,
    _fixed_fm: &mut HashMap<String, String>,
) -> Result<()> {
    Ok(())
}
```

The important part here is the `#[poem_function]` line. This is where we're telling the Rust compiler to apply our macro to `my_poem_function`.

## How to Run the Test

To run the test, you'll use a command in your terminal. This command tells Rust's build system (`cargo`) to try and compile and test our `poem_macro_test_crate`.

1.  **Open your terminal:** If you're using Termux on Android, open the Termux app.
2.  **Navigate to the project root:** You should already be in `/data/data/com.termux/files/home/storage/github/libminizinc`. If not, you can get there by typing:
    ```bash
    cd /data/data/com.termux/files/home/storage/github/libminizinc
    ```
3.  **Run the test command:** Type the following command and press Enter:
    ```bash
    cargo run --package poem_macro_impl_test
    ```
    This command directly executes the `poem_macro_impl_test` binary. This test binary is designed to:
    *   Generate Rust code by applying the `poem_macro` to a dummy function.
    *   Create a temporary `cargo` project.
    *   Attempt to compile the generated code within this temporary project.
    *   Report the compilation outcome.

## How to Inspect the Output

After running the command, you will see the output directly in your terminal.

1.  **Understanding the Output:**
    Look for lines that start with `error:` or `warning:`. These are important messages from the Rust compiler.

    During the development of the `poem_macro`, you might have encountered errors similar to these:
    *   `error: expected one of :, ;, or =, found my_dummy_function` (Initial syntax error in static declaration)
    *   `error: missing type for static item` (Missing type annotation for the static)
    *   `error[E0412]: cannot find type PoemFnPtr in this scope` (Type alias not in scope of generated code)
    *   `error[E0425]: cannot find value fn_name_str in this scope` (Variable not in scope of generated code)
    *   `error[E0015]: cannot call non-const method ... in statics` (Attempting to call non-const functions in static initializers)

    The goal of fixing the macro is to run this command and see `Generated code compiled successfully!` without any `error:` lines!

## Manually Reproducing the Bug (Advanced Debugging)

Sometimes, the compiler errors from a macro can be confusing. To get a clearer picture of what's going wrong, we can manually "expand" the macro and try to compile its output directly. This helps us see the exact code the macro is generating. While the `poem_macro_impl_test` now automates much of this, understanding the manual process is valuable for deep debugging.

**Steps to Manually Reproduce the Bug:**

1.  **Identify the Macro's Input:**
    *   We used the `my_poem_function` from `crates/poem_macro_test_crate/src/lib.rs` as the input to our `poem_function` macro.

2.  **Understand the Macro's Generation Logic:**
    *   We looked at the `poem_function_impl` function in `crates/poem_macro_impl/src/lib.rs`. This function contains a `quote!` macro block that defines the structure of the code the macro generates.
    *   We identified placeholders like `#input_fn` (the original function), `#fn_name` (the name of the original function), and `#_helper_fn_name` (a generated helper function name).

3.  **Manually Expand the Macro:**
    *   We took the `quote!` block and manually substituted the placeholders with their actual values from `my_poem_function`.
    *   For example, `#input_fn` became the full `my_poem_function` definition, `#fn_name` became `my_dummy_function`, and `#_helper_fn_name` became `__get_fn_my_dummy_function`.

4.  **Create a File with the Expanded Code:**
    *   We created a new file: `crates/poem_macro_test_crate/src/generated_macro_output.rs`.
    *   We pasted the manually expanded code into this file.

5.  **Attempt to Compile the Expanded Code:**
    *   We modified `crates/poem_macro_test_crate/src/lib.rs` to include the `generated_macro_output.rs` module and added a simple test function that would attempt to compile it.
    *   We then ran `cargo build --package poem_macro_test_crate`.

**Observation:**

When we tried to compile the `generated_macro_output.rs` file, we observed the same compilation errors as when the macro was applied directly. This confirmed that the bug was indeed in the code generated by the macro, and not in how the macro was being applied. This manual expansion process allowed us to pinpoint the exact syntax errors within the generated code.

## The Fix and Verification

We identified that the primary issue was the `static` declaration within the macro's generated code, and the scope of types and variables.

To fix this, we performed the following steps in `crates/poem_macro_impl/src/lib.rs`:

1.  **Moved `PoemFnPtr` definition:** The `type PoemFnPtr = ...` definition was moved inside the `quote!` block so it is in scope within the generated code.
2.  **Used `stringify!` for function name:** Replaced `fn_name_str.to_string()` with `stringify!(#fn_name).to_string()` to correctly get the function name string within the generated code.
3.  **Corrected static variable naming:** Used `let static_name = quote::format_ident!("__REGISTER_FN_{}", fn_name);` outside the `expanded` block, and then `pub static #static_name:` inside the `expanded` block to correctly generate the static variable name.
4.  **Called helper function:** Ensured `__get_fn_my_dummy_function()` was called with parentheses to return the boxed closure, resolving the `mismatched types` error.
5.  **Used `LazyLock` for static initialization:** Wrapped the static initialization in `std::sync::LazyLock::new(|| { ... })` to allow for non-const function calls (`to_string()` and `__get_fn_my_dummy_function()`) within the static initializer, resolving the `E0015` errors.

After applying these fixes, running `cargo run --package poem_macro_impl_test` now shows:

```
Generated code compiled successfully!
```

This confirms that the `poem_macro` is now generating valid Rust code that compiles without errors. This is a significant milestone!
