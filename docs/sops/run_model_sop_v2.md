# SOP: Running the MiniZinc Embedding Model with Version Vectors and Proof Tapes (Version 2)

## 1. Purpose

This Standard Operating Procedure (SOP) describes how to execute the MiniZinc embedding model using the `run_embedding_model_v6.sh` script. This version leverages version vectors to specify the exact composition of model and data modules, and automatically generates a "proof tape" for each run, ensuring precise, traceable, and composable experimentation. This aligns with the "Monotonic Epic Idea" and the "DNA of the meta-meme" concept.

## 2. Prerequisites

*   The `libminizinc` project is cloned and set up.
*   All necessary MiniZinc model (`.mzn`) and data (`.dzn`) files, including their versioned variants, are present in the `MINIZINC_MODELS_DIR` and `MINIZINC_DATA_DIR` as defined in the `.env` file.
*   The `run_embedding_model_v6.sh` script is executable.

## 3. Procedure

To run the MiniZinc embedding model, execute the `run_embedding_model_v6.sh` script from the project root, providing a version vector as command-line arguments.

### 3.1 Version Vector Structure

The version vector consists of six parameters, each corresponding to a specific module or data file:

1.  `<main_model_version>`: The version of the main model file (e.g., `v6` for `embedding_sphere_final_v6.mzn`).
2.  `<core_params_version>`: The version of the core parameters data file (e.g., `v1` for `example_core_params_v1.dzn`).
3.  `<kappa_params_version>`: The version of the kappa parameters data file.
4.  `<other_params_version>`: The version of the other parameters data file.
5.  `<relations_version>`: The version of the relations data file.
6.  `<vector_params_version>`: The version of the vector parameters data file.

### 3.2 Execution Command

```bash
./scripts/run_embedding_model_v6.sh <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>
```

**Example:**

To run `embedding_sphere_final_v6.mzn` with the initial versions of all data files:

```bash
./scripts/run_embedding_model_v6.sh v6 v1 v1 v1 v1 v1
```

## 4. Proof Tape Generation

For each run, a "proof tape" is automatically generated in the `proof_tapes/` directory within the project root. Each proof tape is stored in a subdirectory named by a timestamp (e.g., `proof_tapes/20231027_143000/`).

A proof tape contains:

*   `version_vector.txt`: Records the exact version vector used for the run.
*   `stdout.log`: Captures the standard output of the MiniZinc model.
*   `stderr.log`: Captures the standard error of the MiniZinc model.
*   `error.log`: (Optional) Records any errors encountered during script execution.
*   Copies of all `.mzn` and `.dzn` files used for the run, ensuring complete reproducibility.

## 5. Output

The script will execute the MiniZinc model. The primary output will be directed to `stdout.log` and `stderr.log` within the generated proof tape directory. A summary message will be printed to the console.

## 6. Troubleshooting

*   **"Error: Model file not found." / "Error: Data file not found."**: Ensure that the specified versioned files exist in their respective directories (`MINIZINC_MODELS_DIR`, `MINIZINC_DATA_DIR`).
*   **"MiniZinc model run failed!"**: Check the `stderr.log` within the proof tape directory for MiniZinc-specific errors (e.g., type errors, syntax errors, unsatisfiability).
*   **"Usage: ..."**: Ensure all required version parameters are provided in the correct order.

## 7. Review and Improvement

This SOP is a living document and will be updated as the model evolves or new execution patterns emerge.
