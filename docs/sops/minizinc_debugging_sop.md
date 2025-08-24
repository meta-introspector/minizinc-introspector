# SOP: Debugging MiniZinc Syntax Errors

## 1. Purpose
This Standard Operating Procedure (SOP) provides a systematic approach to debugging syntax errors encountered during the compilation of MiniZinc models. It aims to guide developers through common error types and effective troubleshooting techniques.

## 2. Scope
This SOP applies to all MiniZinc models (`.mzn` files) within the project that fail to compile due to syntax errors.

## 3. Procedure

### 3.1. Initial Error Analysis

1.  **Examine the Compiler Output:** The `minizinc` compiler provides error messages with file paths, line numbers, and a description of the error. Pay close attention to:
    *   **Error Type:** Is it a `syntax error`, `type error`, `undefined identifier`, or `include error`?
    *   **Location:** The file path and line/column numbers pinpoint the exact location of the error.
    *   **Description:** The accompanying message provides clues about what the compiler expected versus what it found.

2.  **Common Error Patterns:**

    *   **`syntax error, unexpected identifier, expecting '('` (for `record` declarations):**
        *   **Cause:** Incorrect syntax for defining a record type.
        *   **Fix:** Use `type RecordName = record(field_name: field_type, ...);`. Ensure fields are declared as `name: type`.
        *   **Example:** `type Point = record(x: int, y: int);`

    *   **`syntax error, unexpected identifier, expecting ++ or ':'` (for array or variable declarations):**
        *   **Cause:** Missing a colon `:` after the variable name in a declaration.
        *   **Fix:** Add a colon between the type and the variable name.
        *   **Example:** `array[int] of string: my_array = [...];`

    *   **`syntax error, unexpected string, expecting identifier` (for function parameters or record fields):**
        *   **Cause:** This error message can be misleading. It often occurs when the syntax for function parameters or record fields is `type: name` but the compiler expects `name: type` or vice-versa, or if there's a cascading error from a previous line.
        *   **Fix (Function Parameters):** Ensure parameters are `type: name` (e.g., `function int: my_func(int: x) = ...;`).
        *   **Fix (Record Fields):** Ensure fields are `name: type` (e.g., `type MyRecord = record(field1: int, field2: string);`).

    *   **`type error: type error in operator application for '+'` (for array concatenation):**
        *   **Cause:** Using `+` to concatenate arrays.
        *   **Fix:** Use the `++` operator for array concatenation.

    *   **`type error: undefined identifier 'variable_name'`:**
        *   **Cause:** A variable is used but not declared, or its declaration is in a separate data file (`.dzn`) that was not provided during compilation.
        *   **Fix:** Declare the variable in the `.mzn` file, or ensure the corresponding `.dzn` file is passed to the `minizinc` command.

    *   **`include error: Cannot open file 'filename.dzn'`:**
        *   **Cause:** The specified `.dzn` file in an `include` statement is missing or the path is incorrect.
        *   **Fix:** Verify the file exists and the path is correct.

### 3.2. Step-by-Step Debugging

1.  **Start from the First Error:** MiniZinc compiler errors often cascade. Fixing the first reported error can resolve many subsequent errors.
2.  **Isolate the Problem:** If the error is complex, try to create a minimal MiniZinc model that reproduces only that specific error. This helps in isolating the problematic syntax.
3.  **Consult MiniZinc Documentation:** Refer to the official MiniZinc documentation for the exact syntax of language constructs. Pay attention to the version of MiniZinc you are using, as syntax can evolve.
4.  **Use `minizinc --help`:** For command-line related issues, consult the help message of your specific `minizinc` executable.
5.  **Incremental Changes:** Make small, incremental changes and recompile frequently to pinpoint the exact cause of the error.

## 4. Related Documents
*   SOP: MiniZinc Model Syntax Validation
*   Official MiniZinc Documentation (online)