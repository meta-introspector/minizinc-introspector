# Generating the MiniZinc Data File (.dzn)

## Objective
To demonstrate how the collected `AstNumericalVector`s are formatted and written into a MiniZinc data file (`.dzn`), which serves as input for the MiniZinc model.

## Process Overview
As observed in `crates/zos-bootstrap/src/commands/ast_to_minizinc.rs`, the `ast_to_minizinc` command iterates through all extracted `AstNumericalVector`s and serializes their `numerical_vector` field into a MiniZinc data file. It also includes the `target_index` for the element to be optimized.

## Example Numerical Vectors (from Step 1)

| AST Element Type | Vocabulary String     | Assigned Prime (Example) | Numerical Vector |
| :--------------- | :-------------------- | :----------------------- | :--------------- |
| Crate            | `crate_my_crate`      | 11                       | 11               |
| Function         | `fn_add`              | 13                       | 13               |
| FunctionArg      | `fn_arg_a`            | 17                       | 17               |
| FunctionArgType  | `fn_arg_type_i32`     | 19                       | 19               |
| FunctionArg      | `fn_arg_b`            | 23                       | 23               |
| FunctionArgType  | `fn_arg_type_i32`     | 19                       | 19               |
| Identifier       | `ident_a`             | 29                       | 29               |
| Identifier       | `ident_b`             | 31                       | 31               |

## Expected `ast_data.dzn` Content
Given the example numerical vectors above, and assuming `target_index = 1` (targeting the `crate_my_crate` element), the generated `ast_data.dzn` file would look like this:

```minizinc
ast_elements_numerical = [
11,
13,
17,
19,
23,
19,
29,
31
];

target_index = 1;
```

## Explanation
- `ast_elements_numerical`: This array contains the numerical representations of each AST element extracted from the Rust code, in the order they were processed.
- `target_index`: This integer variable specifies which element in the `ast_elements_numerical` array the MiniZinc model should focus its optimization efforts on. It is 1-indexed, consistent with MiniZinc array indexing.

This `.dzn` file provides the concrete data that the MiniZinc model will operate on.
