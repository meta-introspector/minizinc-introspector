# CRQ-20250824-002_MiniZinc_Function_Syntax_Error

## 1. Problem Description

This CRQ documents a persistent and critical syntax parsing error encountered during the refactoring of MiniZinc models, specifically when defining functions with `float` types. The error prevents the successful compilation and execution of MiniZinc models containing such function definitions, blocking further progress on modularizing the `solfunmeme_vial_engine.mzn` model.

**Error Message:**
```
Error: syntax error, unexpected float, expecting identifier
```

**Context:**
This error occurs on lines defining functions, specifically at the point where the return type or argument types are declared. For example, in the following minimal test case:

```minizinc
% Minimal test for function syntax

function float: add_one(x: float) = x + 1.0;

solve satisfy;
output ["Result: ", show(add_one(5.0)), "\n"];
```

The error is reported on line 3: `function float: add_one(x: float) = x + 1.0;` at `float:`.

## 2. Investigation Steps & Observations

Multiple attempts have been made to resolve this syntax error, including:

*   **Consulting Project Examples:** Reviewed `git_history_model.mzn` and `gemini_self_model.mzn`, which contain working function definitions. The syntax observed in these files (`function <return_type>: <name>(<arg_name>: <arg_type>) = <expression>;`) was applied, but the error persisted.

*   **Consulting Standard Library Examples:** Searched `share/minizinc/std/` for function definitions. Found examples like `function string: show2d(...) = ...` in `stdlib/stdlib_string.mzn`, which uses the same syntax. Running `stdlib_string.mzn` directly confirmed its parsability (though it showed redefinition errors due to implicit includes, not syntax errors).

*   **Creating Minimal Test Cases:** Systematically reduced the complexity of the MiniZinc code to isolate the error. Even the simplest function definition with `float` types consistently failed.

*   **User Guidance:** Received direct guidance from the user on the correct syntax for function parameters (`<type>: <name>`). Applied this correction, leading to the following working example:
    ```minizinc
    function var float: add_one_float(var float: x) = x + 1.0;
    ```
    This specific syntax *did* parse and execute successfully in a standalone test. However, when attempting to apply the non-`var` version (`function float: clamp01(x: float) = ...`) to the `clamp01` function (which operates on fixed floats, not decision variables), the original error reappeared.

*   **Contradictory Behavior:** The most perplexing observation is the contradictory behavior: `function var float: ...` works, but `function float: ...` (without `var`) does not, despite `float` being a valid type and `git_history_model.mzn` using non-`var` types in function signatures (e.g., `string`, `array[int] of Commit`). This suggests a specific issue with `float` types when not declared as `var` in function signatures, or a very subtle interaction with the MiniZinc parser.

## 3. Failing Code Snippets

**Example 1: `minizinc_functions/clamp01.mzn` (fails)**
```minizinc
% clamp01.mzn
% Clamps a real number to the [0,1] interval
function float: clamp01(x: float) = max(0.0, min(1.0, x));
```

**Example 2: `minizinc_functions/clamp01.mzn` (attempt with `int` for debugging, also fails)**
```minizinc
% clamp01.mzn
% Clamps a real number to the [0,1] interval
function int: clamp01_int(x: int) = max(0, min(1, x));
```

## 4. Hypotheses & Next Steps for QA

**Hypotheses:**
*   **MiniZinc Version/Installation Issue:** There might be an incompatibility or bug in the specific MiniZinc version installed in this environment that affects parsing of non-`var` `float` types in function signatures.
*   **Subtle Syntax Rule for `float`:** There could be an extremely subtle and undocumented syntax rule for declaring functions that return or take `float` parameters when they are not `var` types.
*   **Parser Bug:** The MiniZinc parser itself might have a bug that misinterprets this specific syntax pattern.

**Assistance Required from QA Team:**
1.  **Verification:** Please attempt to compile and run the failing `clamp01.mzn` (Example 1) in your MiniZinc environment. Confirm if the error is reproducible.
2.  **Working Example:** If the error is reproducible, please provide a *minimal, working MiniZinc file* that defines a function which takes a `float` as an argument and returns a `float`, *without* using `var` for either the argument or the return type. This would serve as a definitive syntax reference.
3.  **Version Information:** Provide the exact MiniZinc version you are using (`minizinc --version`).

This information will be crucial in resolving this blocking issue and allowing us to proceed with the modularization of the MiniZinc codebase.

## Commit History

- [Commit b38d3101bad8de6c5acf56c65e52210c39fffc5f: docs: Create CRQ for MiniZinc Function Syntax Error](docs/commits/b38d3101bad8de6c5acf56c65e52210c39fffc5f_docs_Create_CRQ_for_MiniZinc_Function_Syntax_Error.md)
