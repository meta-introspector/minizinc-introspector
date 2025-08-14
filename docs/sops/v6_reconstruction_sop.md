# SOP: MiniZinc v6 Model Reconstruction for Bottleneck Identification

## 1. Purpose
This Standard Operating Procedure (SOP) outlines a systematic approach to deconstruct and reconstruct the MiniZinc `v6` embedding model. The objective is to identify performance bottlenecks, particularly those causing slow termination or non-termination, by incrementally reintroducing complexity and measuring the impact of each step. This adheres to the "Monotonic Epic Idea" of implementing new functionality as composable modules rather than editing existing ones.

## 2. Scope
This SOP applies to the MiniZinc `v6` embedding model and its associated data files (`.dzn`) and scripts within the `libminizinc` project.

## 3. Procedure

### Phase 1: Baseline (Minimal Working Model)
**Objective:** Establish the simplest possible working version of the `v6` model that compiles and terminates quickly.

**Steps:**
1.  Identify the core components of `v6` (e.g., `embedding_helpers.mzn`, `embedding_output.mzn`, and a minimal `embedding_params_core_v2.mzn` with only essential parameters like `n`, `d`, `P`).
2.  Create a new, simplified `run_embedding_model_v6_baseline.sh` script that only includes these minimal components.
3.  Run the baseline model and record its termination time and resource usage. This will serve as the performance benchmark.

**Expected Outcome:** A functional, fast-terminating `v6` baseline model and a recorded performance benchmark.

### Phase 2: Incremental Complexity Introduction
**Objective:** Systematically reintroduce features, parameters, and constraints from the original `v6` model, one logical module at a time.

**Steps:**
1.  **Module Identification:** Break down the original `v6` model into distinct, self-contained modules (e.g., `embedding_objective_v3.mzn`, `embedding_params_kappa_composed.mzn`, `embedding_params_relations_v3.mzn`, `embedding_params_vector_composed_v3.mzn`, `embedding_params_other_composed.mzn`).
2.  **Stepwise Integration:** For each module:
    *   Integrate the module into the current working model (starting with the baseline).
    *   Create a new version of the `run_embedding_model_v6_stepX.sh` script that includes the newly added module.
    *   Ensure all necessary `.dzn` parameter files for the new module are included.
    *   Run the model and record its termination time and resource usage.
    *   Document any changes in behavior or performance.
3.  **Parameter Increments:** For complex parameter files (e.g., `embedding_params_vector_composed_v3.mzn`), consider introducing parameters in smaller groups or even individually if a module is suspected to be problematic.

**Expected Outcome:** A series of progressively more complex `v6` model versions, each with documented performance characteristics, leading to the identification of the module(s) or parameter(s) causing performance degradation.

### Phase 3: Performance Measurement and Bottleneck Identification
**Objective:** Analyze the performance data collected in Phase 2 to pinpoint the exact source of the performance bottleneck.

**Steps:**
1.  **Data Analysis:** Compare the termination times and resource usage across all incremental steps.
2.  **Anomaly Detection:** Identify the specific step(s) where a significant increase in termination time or resource consumption occurs, or where non-termination begins.
3.  **Root Cause Analysis:** Investigate the newly introduced module(s) or parameter(s) at the problematic step to understand why they are causing the performance issue. This may involve:
    *   Reviewing the MiniZinc code for complex constraints, large domains, or inefficient formulations.
    *   Analyzing the generated FlatZinc for unexpected size or structure.
    *   Using MiniZinc's profiling capabilities if available and applicable.

**Expected Outcome:** Clear identification of the performance bottleneck(s) within the `v6` model.

### Phase 4: Optimization and Refactoring
**Objective:** Address the identified bottlenecks through targeted optimization or refactoring, adhering to the "Monotonic Epic Idea."

**Steps:**
1.  **Targeted Optimization:** Focus efforts on the identified problematic module(s) or parameter(s).
2.  **Composable Solutions:** Instead of directly editing the problematic code, create new, optimized versions of the modules or parameters. These new versions should be designed to be composable and replace the inefficient parts without altering the original.
3.  **Iterative Testing:** Re-run the model with the optimized components and verify performance improvements.
4.  **Documentation:** Document the optimization strategies employed and their impact.

**Expected Outcome:** An optimized `v6` model with improved performance, and a clear record of the optimization process.

## 4. Tools
*   MiniZinc compiler and solver
*   `run_embedding_model_v6.sh` (and its incremental variants)
*   System monitoring tools (for CPU, memory usage)
*   Text editor for MiniZinc and shell scripts
*   `git` for version control and tracking changes

## 5. Expected Outcome
*   A detailed understanding of the performance characteristics of the `v6` MiniZinc model.
*   Identification of specific modules, parameters, or constraints responsible for performance bottlenecks.
*   Optimized versions of the `v6` model components, leading to faster termination and reduced resource consumption.
*   Comprehensive documentation of the deconstruction, reconstruction, and optimization process, serving as a guide for future model development.
