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
