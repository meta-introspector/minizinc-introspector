# Plan for Fuzzing MiniZinc Language Features to Increase C++ Code Coverage ("Oxidation")

## Introduction

This plan details a strategy to further increase the C++ code coverage of the `libminizinc` library by its Rust FFI tests, specifically by expanding the testing of MiniZinc language features. This approach, metaphorically termed "fuzzing the zinc," aims to exercise a broader range of the MiniZinc language itself, thereby uncovering more code paths in the underlying C++ implementation.

This initiative directly supports Phase 4 ("Improve & Operate") of the overarching "Plan for Systemic Increase of Rust-MiniZinc FFI C++ Code Coverage ('Oxidation')" (`docs/sops/plan_increase_oxidation.md`). It leverages principles from Six Sigma (for data-driven process improvement), GMP (for quality and controlled processes), and ISO 9000 (for continuous improvement and process approach).

## Methodology

### 1. Define Fuzzing Strategy (Six Sigma: Define)

**Objective:** To clearly define what "fuzzing the zinc" entails in this context and set specific targets for language feature coverage.

**Activities:**
*   **Identify MiniZinc Language Features:** Systematically enumerate key MiniZinc language constructs, data types, constraints, functions, and control flow elements. Examples include:
    *   **Data Types:** `int`, `float`, `bool`, `string`, `set of int`, `set of float`, `array of ...` (1D, 2D, multi-dimensional).
    *   **Declarations:** `var`, `par`, `enum`, `predicate`, `function`, `test`.
    *   **Constraints:** Arithmetic (`+`, `-`, `*`, `/`, `%`), relational (`=`, `!=`, `<`, `<=`, `>`, `>=`), logical (`and`, `or`, `not`, `xor`, `iff`), global constraints (e.g., `all_different`, `cumulative`, `circuit`).
    *   **Control Flow:** `if-then-else`, `forall`, `sum`, `product`, `min`, `max`.
    *   **Solve Items:** `solve satisfy`, `solve minimize`, `solve maximize`.
    *   **Output Items:** Various `output` statement formats.
    *   **Annotations:** Custom annotations and their usage.
*   **Prioritize Features:** Prioritize which language features to fuzz first based on:
    *   Their complexity and potential for unexpected behavior.
    *   Their current low coverage as identified in the `html_llvm_coverage_report` from previous analysis.
    *   Their criticality to typical MiniZinc model usage via the Rust FFI.
*   **Define Fuzzing Granularity:** Determine the level at which fuzzing inputs will be generated:
    *   Individual language constructs (e.g., a single complex constraint).
    *   Small, self-contained MiniZinc models demonstrating specific features.
    *   Combinations of multiple language features within a single model.
*   **Target Coverage Areas:** Explicitly link fuzzing efforts to specific uncovered regions in the C++ code (e.g., parser functions, flattener logic, solver interface code) that the identified MiniZinc features are expected to exercise.

### 2. Generate Fuzzing Inputs (Six Sigma: Measure/Analyze, GMP: Controlled Process)

**Objective:** To create a diverse, high-quality set of MiniZinc models (`.mzn`) and corresponding data files (`.dzn`) that effectively exercise the identified language features.

**Activities:**
*   **Automated MiniZinc Model Generation:**
    *   **Grammar-based Fuzzing:** If a formal grammar for MiniZinc is available (e.g., in ANTLR, BNF, or similar format), develop or adapt a fuzzer that generates syntactically valid but semantically diverse MiniZinc models. This ensures adherence to language rules while exploring varied structures.
    *   **Template-based Fuzzing:** Create a library of MiniZinc model templates with placeholders for different data types, variable declarations, constraints, and solve goals. Develop a Rust program to programmatically fill these templates with a wide range of values and combinations.
    *   **Mutation-based Fuzzing:** Take existing MiniZinc models (e.g., from `minizinc_models/` or previous fuzzing runs) and apply intelligent mutations. Mutations could include changing literal values, altering operators, adding/removing constraints, modifying array dimensions, or changing variable types.
*   **Data File Generation (`.dzn`):** For each generated MiniZinc model, create corresponding `.dzn` files with diverse input data. This should include:
    *   **Edge Cases:** Empty sets, maximum/minimum integer/float values, zero, negative numbers, large arrays.
    *   **Boundary Values:** Values at the limits of variable domains.
    *   **Invalid Inputs:** (Optional, for robustness testing) Inputs that are syntactically valid but semantically incorrect to test error handling paths.
*   **Input Diversity & Volume:** Ensure the generation process produces a high volume of unique and diverse inputs to maximize the exploration of C++ code paths.

### 3. Integrate Fuzzing with Rust FFI Tests (ITIL: Service Transition, GMP: Validation)

**Objective:** To seamlessly incorporate the generated fuzzing inputs into the existing Rust FFI testing framework, ensuring their execution contributes to coverage data.

**Activities:**
*   **Rust Test Harness Development:** Develop a robust Rust test harness that:
    *   Iterates through the generated MiniZinc models and `.dzn` files.
    *   Uses the Rust FFI to load, parse, flatten, solve, and retrieve results from these models.
    *   Handles potential errors, exceptions, or crashes gracefully, logging them for later analysis.
    *   Can be configured to run a subset of fuzzing inputs or the entire generated set.
*   **Automated Execution:** Integrate the fuzzing test harness into the existing `cargo test` workflow. This ensures that fuzzing runs automatically as part of the standard test suite.
*   **Coverage Data Collection:** Verify that running these fuzzing tests, through the Rust FFI, continues to generate accurate LLVM profile data for the C++ backend. This confirms that the fuzzing efforts are directly contributing to the "oxidation" metric.

### 4. Analyze Fuzzing Results & Iterate (Six Sigma: Improve, ITIL: CSI)

**Objective:** To evaluate the effectiveness of fuzzing in increasing coverage, identify new bugs, and refine the fuzzing strategy.

**Activities:**
*   **Coverage Report Analysis:** After each fuzzing run, utilize the `generate_llvm_html_report.sh` and `generate_llvm_text_summary.sh` scripts to generate and analyze the C++ code coverage reports. Observe the increase in overall coverage and identify newly covered regions.
*   **Crash/Error Triage:** Systematically investigate any crashes, assertion failures, or unexpected errors identified during fuzzing. These often point to critical bugs, unhandled edge cases, or vulnerabilities in the C++ code.
*   **Test Case Minimization:** For any identified crashes or interesting behaviors, use tools (e.g., `creduce` or manual effort) to minimize the MiniZinc model and data that trigger the issue. This creates focused, reproducible regression test cases.
*   **Feedback Loop:** Use the insights gained from coverage analysis and bug identification to refine the fuzzing input generation strategy. Focus future fuzzing efforts on areas that still exhibit low coverage or are prone to issues.
*   **Documentation:** Document new test cases, identified bugs (with minimal reproducible examples), and any improvements made to the fuzzing process or tools.

### 5. Control & Sustain (Six Sigma: Control, ISO 9000: Continuous Improvement)

**Objective:** To ensure the fuzzing process is sustainable, integrated into the development workflow, and contributes to long-term coverage goals.

**Activities:**
*   **CI/CD Integration:** Integrate the fuzzing tests into the Continuous Integration/Continuous Delivery (CI/CD) pipeline. Configure the pipeline to run fuzzing tests automatically on every code change or on a scheduled basis.
*   **Performance Monitoring:** Continuously monitor the execution time and resource consumption of fuzzing tests to ensure they remain efficient and do not unduly impact development cycles.
*   **Coverage Thresholds:** Consider implementing coverage thresholds in the CI/CD pipeline to prevent regressions in C++ code coverage.
*   **Regular Review:** Periodically review the overall fuzzing strategy, its effectiveness, and its contribution to the project's quality goals. Adapt the strategy as the codebase evolves or new MiniZinc language features are introduced.
*   **Knowledge Sharing:** Share best practices and lessons learned from fuzzing efforts across the development team.
