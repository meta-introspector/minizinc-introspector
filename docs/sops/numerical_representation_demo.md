# Numerical Representation of Rust Code Snippet

## Objective
To demonstrate how a simple Rust code snippet is converted into a numerical representation using the project's `numerical_vector_generator` and `ast_to_numerical_vector_converter` components.

## Input Rust Code Snippet
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## Assumed `PRIME_MAP` Initial State
The `PRIME_MAP` in `numerical_vector_generator.rs` is initialized with:
- "security": 2
- "modularity": 3
- "authentication": 5
- "legacy": 7

New vocabulary items will be assigned the next available prime numbers sequentially.

## Expected Numerical Vectors (Simplified Trace)

Assuming a `crate_name` of "my_crate", here's a trace of how the AST elements would be converted into numerical vectors. The "Assigned Prime (Example)" column shows a hypothetical prime assigned based on sequential availability after the initial `PRIME_MAP` values.

| AST Element Type | Vocabulary String     | Assigned Prime (Example) | Numerical Vector (Product of Primes) |
| :--------------- | :-------------------- | :----------------------- | :----------------------------------- |
| Crate            | `crate_my_crate`      | 11                       | 11                                   |
| Function         | `fn_add`              | 13                       | 13                                   |
| FunctionArg      | `fn_arg_a`            | 17                       | 17                                   |
| FunctionArgType  | `fn_arg_type_i32`     | 19                       | 19                                   |
| FunctionArg      | `fn_arg_b`            | 23                       | 23                                   |
| FunctionArgType  | `fn_arg_type_i32`     | 19 (reused)              | 19                                   |
| Identifier       | `ident_a`             | 29                       | 29                                   |
| Identifier       | `ident_b`             | 31                       | 31                                   |

## Explanation
The `ast_to_numerical_vector_converter.rs` uses `syn` to traverse the Rust Abstract Syntax Tree (AST) of the input code. For each recognized AST element (like a function, argument, or identifier), it constructs a unique "vocabulary string" (e.g., "fn_add", "fn_arg_a").

These vocabulary strings are then passed to `numerical_vector_generator.rs`'s `get_prime_for_vocabulary` function. This function assigns a unique prime number to each new vocabulary item. If an item has been seen before, its previously assigned prime is reused.

Finally, `compose_numerical_vector` is used to create the `numerical_vector`. In this simplified example, each `AstElement` is represented by a single prime, so the numerical vector is simply that prime. In more complex scenarios (e.g., for composite AST nodes), it could be the product of multiple primes.

This process effectively transforms the structural and semantic information of the Rust code into a numerical format, which can then be used as input for MiniZinc models.
