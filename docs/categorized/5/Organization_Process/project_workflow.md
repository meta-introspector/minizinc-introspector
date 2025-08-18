# Project Workflow SOP

This document outlines the standard operating procedures (SOPs) for working on the `libminizinc` project, particularly concerning its integration with Gecode and the development of MiniZinc models.

## 1. Handling Gecode Build Issues

### 1.1 `cmake_minimum_required` Error
If `cmake ..` in `vendor/gecode/build` fails with a `cmake_minimum_required` compatibility error, re-run CMake with the `-DCMAKE_POLICY_VERSION_MINIMUM=3.5` flag:
```bash
cd /data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/build && cmake .. -DCMAKE_POLICY_VERSION_MINIMUM=3.5
```

### 1.2 C++ Compilation Errors (`weights.hpp`)
If Gecode compilation fails with errors related to `gecode/set/int/weights.hpp` (e.g., "no viable overloaded '='"), it indicates a `const` correctness issue with the `init` method. This method attempts to assign to `const SharedArray` members after construction, which is illegal.

**Resolution:** Remove both the declaration and definition of the `init` method from `gecode/set/int/weights.hpp`. The `OverweightValues` constructor already handles proper initialization.

### 1.3 Committing Gecode Submodule Changes
Changes to `vendor/gecode` (a Git submodule) must be committed within the submodule itself before being reflected in the parent repository.
1.  `cd /data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode`
2.  `git add gecode/set/int/weights.hpp` (or other modified files)
3.  `git commit -m "Fix: [Brief description of fix]"`
4.  `cd -` (to return to parent directory)
5.  `git add vendor/gecode` (to stage the submodule update in the parent repo)

## 2. Configuring `libminizinc` with Gecode

### 2.1 Gecode Discovery (`find_package`)
After building Gecode, `libminizinc`'s CMake might not automatically find it. Explicitly set `Gecode_ROOT` during `libminizinc`'s CMake configuration:
```bash
cd /data/data/com.termux/files/home/storage/github/libminizinc/build && cmake .. -DGecode_ROOT=/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/build
```

### 2.2 Header Inclusion (`config.hpp` not found)
If `libminizinc` compilation fails with `fatal error: 'gecode/support/config.hpp' file not found`, it means the Gecode build directory is not in `libminizinc`'s include paths. Add the following line to `libminizinc/CMakeLists.txt` (e.g., after existing `include_directories` calls):
```cmake
include_directories(${PROJECT_SOURCE_DIR}/vendor/gecode/build)
```
Then, re-run `cmake ..` and `make` in `libminizinc/build`.

## 3. MiniZinc Solver Discovery (`.msc` files)

`minizinc` discovers solvers via `.msc` (MiniZinc Solver Configuration) files. If a solver is not found (e.g., `Error: configuration error: no solver with tag org.gecode.gecode found`):
1.  **Identify config directories:** Run `minizinc --config-dirs` to see where `minizinc` looks for solver configurations (e.g., `userSolverConfigDir`).
2.  **Create `.msc` file:** Create a JSON file (e.g., `gecode.msc`) with the solver's `id`, `version`, `name`, and `executable` path (e.g., `/data/data/com.termux/files/home/storage/github/libminizinc/vendor/gecode/build/bin/fzn-gecode`).
3.  **Place `.msc` file:** Move the created `.msc` file to one of the discovered configuration directories (e.g., `/data/data/com.termux/files/home/.minizinc/solvers/`).

## 4. Debugging MiniZinc Model Type Errors

### 4.1 `type error: multiple assignment to the same variable`
This often occurs when a parameter has a default value in the `.mzn` model and is also assigned a value in the `.dzn` data file. To resolve, remove the assignment from the `.dzn` file if the default in `.mzn` is sufficient, or remove the default from `.mzn` if the `.dzn` file is the primary source of the value.

### 4.2 `type error: initialisation value for ... has invalid type-inst` (`float` vs `var float`)
This indicates a mismatch between a declared type (`float`) and an assigned value (`var float`). Decision variables (`var float`) cannot be assigned to fixed `float` parameters. Change the declared type to `var float` if it's meant to be a decision variable, or ensure the value being assigned is a fixed `float`.

### 4.3 `type error: return type of function does not match body`
Similar to 4.2, if a function is declared to return `float` but its body computes a `var float` (e.g., from operations on decision variables), change the function's return type to `var float`.

### 4.4 `syntax error, unexpected <` (Bit Shift Operator)
MiniZinc does not support bit shift operators (`<<`, `>>`) in parameter declarations. Manually calculate the value and use the literal integer.

## 5. Integer Discretization Approach

To enable solving with CP solvers like Gecode, floating-point values are discretized into integers. This involves:
1.  **`p` as `var int`:** Coordinates `p[i,k]` are now `var int` in a scaled range (e.g., `-SCALE_MAX..SCALE_MAX`).
2.  **Scaling Factor:** Introduce `SCALE_BITS` and `SCALE_MAX` parameters to define the integer range and scaling.
3.  **Integer Arithmetic:** All mathematical operations (dot product, powers, sums) must be performed using integer arithmetic. This requires careful scaling of all coefficients and constants (e.g., `kappa` values, `theta_vec`, `1.0`) to integers.
4.  **Unit Norm Constraint:** The constraint `sum(k in 1..d) (pow(p[i,k], 2.0)) = 1.0` becomes `sum(k in 1..d) (p[i,k] * p[i,k]) = SCALE_MAX * SCALE_MAX`.
5.  **Objective Function Adaptation:** All terms in the objective function must be converted to use scaled integer arithmetic. This is an ongoing task and requires careful consideration of precision and potential overflow.

## 6. General Project Practices

*   **Git Submodules:** Always remember to `cd` into submodules to commit changes within them, then `git add <submodule_path>` in the parent repository to update the submodule reference.
*   **Commit Messages:** Use `temp_commit_message.txt` for detailed commit messages, summarizing the *why* and *how* of changes, especially for complex fixes.
*   **Iterative Testing:** After each significant change, re-run `minizinc` to quickly identify and address new errors.

## 7. Modular MiniZinc Model Development

To manage complexity and facilitate debugging, MiniZinc models should be developed modularly and problems solved incrementally.

1.  **Start with a Base Case:** Begin with the simplest possible formulation that captures the core concept (e.g., a single variable, minimal constraints). Verify its correctness.
2.  **Incremental Complexity:** Gradually add more variables, dimensions, constraints, and objective terms. Test each addition to ensure it behaves as expected.
3.  **Refactor into Modules:** For larger models, consider breaking them into separate `.mzn` files using `include` directives. This allows for better organization and reusability of components (e.g., common functions, constraint patterns).

This SOP will be continuously updated as new procedures are established or existing ones are refined.