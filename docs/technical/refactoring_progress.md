# Refactoring Progress: `doc_to_minizinc_data` and MiniZinc Model

**Date:** August 18, 2025

## Overview
This document summarizes the ongoing refactoring efforts for the `doc_to_minizinc_data` Rust program and the associated MiniZinc model (`word_embedding_inference.mzn`). The primary goal is to improve modularity, maintainability, and address issues related to data generation and MiniZinc model execution.

## Changes Made to `doc_to_minizinc_data`
The `doc_to_minizinc_data` crate, responsible for processing documentation and generating MiniZinc data files (`.dzn`), has undergone significant restructuring:

*   **`src/main.rs` split:** The monolithic `main.rs` file has been broken down into smaller, more focused modules:
    *   `src/cli.rs`: Now contains the `Args` struct and `clap` setup for command-line argument parsing.
    *   `src/word_processing.rs`: Houses utility functions like `format_pair` and `format_triple`, along with the core word extraction and cleaning logic.
    *   `src/wordnet_processing.rs`: Contains the `generate_wordnet_constraints` function, responsible for reading the simulated WordNet and preparing logical relationships. Its return type was changed to `Vec<(String, String, f64)>` to allow for post-processing.
    *   `src/data_generation.rs`: Intended to contain the main logic for processing documents, generating word embeddings chunks, and co-occurrence data. This module also incorporates the crucial global WordNet filtering logic, ensuring that only words present in the corpus are considered for relations.
*   **Module Integration:** `src/main.rs` and `src/lib.rs` have been updated to correctly import and declare these new modules, respectively.

## Changes Made to MiniZinc Model
The `word_embedding_inference.mzn` MiniZinc model has been modularized to enhance readability and maintainability:

*   **`minizinc_data/data_declarations.mzn`:** A new file created to centralize all data declarations (e.g., `num_words`, `word_map`, `embeddings`, `num_relations`, `relation_pairs`, `desired_distances`, `num_bigrams`, `bigram_pairs`, `bigram_counts`, `num_trigrams`, `trigram_triples`, `trigram_counts`). This avoids redundant declarations in the main model.
*   **`minizinc_models/functions.mzn`:** Contains reusable function definitions, such as `euclidean_distance` and `get_embedding`.
*   **`minizinc_models/constraints.mzn`:** Holds the core constraints and the loss function definition (`actual_distances`, `loss`, and co-occurrence constraints).
*   **`minizinc_models/solve_output.mzn`:** Encapsulates the `solve` statement and any `output` statements.
*   **`word_embedding_inference.mzn`:** Now primarily acts as an orchestrator, including these modular files to construct the complete model.

## Current Challenges

*   **`src/data_generation.rs` Persistence Issue:** The most critical current challenge is an intermittent issue where the content written to `src/data_generation.rs` does not persist correctly, leading to an empty file and subsequent build failures. This is currently under investigation by the user.
*   **MiniZinc Model Execution:** Once the `doc_to_minizinc_data` program is stable and correctly generates filtered data, we anticipate the MiniZinc model to run without `array slice out of bounds` errors. The `UNSATISFIABLE` results are expected at this stage, as the embeddings are randomly initialized and not yet optimized.
