# SOP: MiniZinc Model Performance Reconstruction

## 1. Purpose

This Standard Operating Procedure (SOP) outlines a systematic approach to deconstruct and analyze the performance of complex MiniZinc models. The goal is to identify performance bottlenecks, understand the computational cost of different model components, and gain insights into solver behavior, thereby facilitating optimization and informed model design.

## 2. Scope

This SOP applies to the analysis of any MiniZinc model within the project, particularly those exhibiting unexpected runtime characteristics or serving as critical components in larger systems (e.g., the `sieve_embedding.mzn` model).

## 3. Core Principle: Incremental Deconstruction and Reconstruction

Understanding performance involves breaking down a complex problem into manageable parts, analyzing each part in isolation, and then incrementally reintroducing complexity while observing its impact on runtime.

## 4. Procedure

### 4.1. Step 1: Establish a Baseline

Before optimizing, it's crucial to know the current performance.

*   **Action:** Run the target MiniZinc model with a representative dataset using the `minizinc_test_runner_rs`.
*   **Tool:** `minizinc_test_runner_rs` (e.g., `bash scripts/run_v7_debug_tests.sh` for sieve model).
*   **Output:** Note the total execution time and the `flatTime` from the MiniZinc solver's statistics (available in `full_output.log` within the proof tape).

### 4.2. Step 2: Model Simplification (Minimal Viable Model)

Create the simplest possible version of the model that still captures its core essence or a specific problematic component.

*   **Action:** Create a new `.mzn` file (e.g., `my_model_simplified.mzn`) that includes only the essential parameters and a minimal set of constraints. For example, for the `sieve_embedding.mzn` model, this might involve removing all embedding constraints and just focusing on the sieve itself.
*   **Principle:** Adhere to the "Monotonic Epic Idea" (add-only, never edit). Create a *new* simplified model, do not modify the original.
*   **Tool:** Text editor.

### 4.3. Step 3: Incremental Reintroduction of Complexity

Gradually add back components (parameters, variables, constraints) to the simplified model, measuring performance at each step.

*   **Action:**
    1.  Start with the simplified model.
    2.  Add one logical block of constraints or variables at a time.
    3.  For each addition, run the model using `minizinc_test_runner_rs` and record the execution time.
    4.  Observe how each addition impacts the `flatTime` and overall execution time.
*   **Tool:** `minizinc_test_runner_rs`, text editor.
*   **Tip:** Focus on components that introduce new variables, large arrays, or complex global constraints.

### 4.4. Step 4: Data Analysis and Visualization

Analyze the collected performance data to identify trends and bottlenecks.

*   **Action:** Use the `minizinc_report_generator_rs` to process the collected performance data.
*   **Tool:** `minizinc_report_generator_rs` (e.g., `path/to/minizinc_report_generator_rs_executable`).
*   **Output:** A Markdown report (e.g., `docs/sieve_performance_report.md`) containing tables and potentially plots (if visualization is implemented).

### 4.5. Step 5: Constraint Analysis

Identify specific constraints that are computationally expensive.

*   **Action:**
    1.  Examine the `full_output.log` files from the `proof_tapes/` for detailed MiniZinc solver statistics.
    2.  Look for warnings or errors related to specific constraints.
    3.  Consider temporarily commenting out suspicious constraints in a *new version* of the model to see their impact on performance.
*   **Tool:** Text editor, `cat` or `less` for `full_output.log`.

### 4.6. Step 6: Solver Interaction Analysis

Understand how the chosen MiniZinc solver is interacting with the model.

*   **Action:** Experiment with different solver options (e.g., search strategies, propagation levels) if available for your solver.
*   **Tool:** MiniZinc command-line options (refer to `minizinc --help`).

## 5. Tools and Resources

*   **`minizinc_test_runner_rs`**: Rust-based test runner for automated execution and timing.
*   **`minizinc_data_generator_rs`**: Rust-based DZN data generator.
*   **MiniZinc Solver**: The core constraint programming solver.
*   **`proof_tapes/`**: Directory containing detailed logs and input files for each run.
*   **`minizinc_report_generator_rs`**: Rust-based report generator for data analysis and visualization.

## 6. Related SOPs

*   [Monotonic Epic Idea - Add-Only, Never Edit Development](docs/sops/monotonic_epic_idea_sop.md)
*   [MiniZinc Model Performance Analysis and Debugging Report](docs/performance_analysis_report.md)

---
