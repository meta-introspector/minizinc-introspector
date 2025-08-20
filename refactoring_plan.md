## Current Plan

## Term Recognition System Refactoring 🧠🔍

This section details the ongoing refactoring efforts for the term recognition system, primarily within the `zos-fast-query` and `vocabulary_dfa_generator` crates.

**Objectives:**
*   Improve memory efficiency by moving from monolithic data structures to chunk-based processing.
*   Enhance maintainability through modular code generation and templating.
*   Ensure robust handling of Unicode filenames and paths.
*   Establish a clear, configurable build process.
*   Implement advanced analysis capabilities, such as subpattern identification.

**Progress & Challenges:**
*   **Modularization:** `zos-fast-query`'s build process has been refactored into dedicated modules (`term_loader`, `chunk_generator`, `index_writer`).
*   **Configuration:** Introduced `build_config.toml` for centralized path management.
*   **Filename Sanitization:** Implemented Unicode-aware filename sanitization (UXXXX format).
*   **Build Script Integration:** Faced significant challenges integrating `build_utils` functions between the build script and the main application, leading to temporary workarounds (e.g., embedding functions directly in `build.rs`). This highlights the need for a more robust shared utility crate or a clearer separation of concerns.
*   **DFA Generation:** Ensured `vocabulary_dfa_generator` correctly generates DFA modules, though `dfa_tester` still requires explicit execution of the generator.
*   **Subpattern Analysis:** Initial command (`subpattern_analysis`) has been added to `zos-fast-query`, with logic for identifying terms that are substrings of others.

**Next Steps:**
*   Resolve remaining build complexities related to shared utility functions between build scripts and main crates.
*   Refine the `subpattern_analysis` command and expand its capabilities for deeper term topology analysis.
*   Address minor warnings across the workspace to improve overall code quality.

## Current Plan

1.  **Adapt MiniZinc Model for Incremental Solving:**
    *   Modify `word_embedding_inference.mzn` to accept a subset of words and their relationships from a specific chunk's `.dzn` file.
    *   Explore ways to "fix" or "warm-start" embeddings for words that have already been optimized in previous chunks, potentially by passing them as fixed parameters to the solver.
2.  **Orchestrate Incremental Solving:**
    *   Develop a new script or extend an existing one to:
        *   Call `doc_to_minizinc_data` to generate word chunks.
        *   Iteratively call the MiniZinc solver for each chunk.
        *   Manage the persistence and loading of optimized embeddings between iterations.
3.  **Define Logical Relationships and Loss Function:**
    *   Define a mechanism for user-defined logical relationships between words (e.g., desired distances between word pairs).
    *   Modify the MiniZinc model to optimize embeddings based on these desired distances, minimizing a defined loss function. This will be integrated with the chunking.
4.  **Implement Term Addition Time and Iterative Refinement:**
    *   Implement a process for iteratively adding new terms and their relationships, documenting the time of addition.
    *   Define metrics for measuring the "gain" from refining embeddings and how the "loss function" will be used to guide the process. This will also be integrated with the chunking and incremental solving.
