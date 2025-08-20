## MiniZinc Dry Run Plan: Conceptual Overview

This plan outlines the execution strategy for the `word_embedding_inference.mzn` model and a minimal test case to demonstrate the identified MiniZinc parsing bug.

### 1. MiniZinc Commands (What would be run)

**A. Full Incremental Solving (Conceptual Loop):**

The primary execution involves an iterative process, likely orchestrated by a shell script or Rust program, that processes each data chunk. For each chunk `X`, the command would conceptually be:

```bash
/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc \
    /data/data/com.termux/files/home/storage/github/libminizinc/word_embedding_inference.mzn \
    /data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/huggingface/word_embeddings_chunk_X.dzn \
    /data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/data_declarations.mzn \
    -s # (or other solver flags for optimization/output, e.g., --solver Gecode)
```

*   `/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc`: The absolute path to the MiniZinc executable.
*   `word_embedding_inference.mzn`: The main MiniZinc model that defines the embedding problem, constraints, and objective function.
*   `word_embeddings_chunk_X.dzn`: A data file containing the words and their initial 8D embeddings for the current chunk. These are now generated in `minizinc_data/huggingface/`.
*   `data_declarations.mzn`: A data file containing the logical relationships (e.g., desired distances between word pairs) that the model will use for optimization.

**B. Minimal Test Case to Demonstrate Bug:**

To specifically demonstrate the "undefined identifier" parsing bug, we would run `test_num_words.mzn` with a single generated data chunk. This test model simply tries to access a variable (`num_words`) defined within the `.dzn` file.

```bash
/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc \
    /data/data/com.termux/files/home/storage/github/libminizinc/test_num_words.mzn \
    /data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/huggingface/word_embeddings_chunk_0.dzn
```

### 2. Size and Complexity Estimates

*   **Total Words:** The `doc_to_minizinc_data` tool processes all relevant source and documentation files, resulting in approximately **276,400 unique words** (this is an upper bound, as the actual count depends on the content).
*   **Total Chunks:** These words are divided into **2764 chunks**, with each chunk containing up to 100 words.
*   **Embeddings Dimension:** Each word is represented by an **8-dimensional floating-point embedding**.
*   **Relations:** The number of logical relationships (from `simulated_wordnet.txt`) will be relatively small for the minimal test case (e.g., 1-2 relations). For a full run, this could scale up.
*   **Model Complexity (`word_embedding_inference.mzn`):** The model's complexity per run is kept manageable by processing data in chunks. It involves:
    *   Declaring arrays for words, embeddings, relations, and desired distances.
    *   Calculating Euclidean distances between word pairs.
    *   Defining a loss function (Mean Squared Error) to minimize.
    *   The core solving task is to find optimal embedding values that minimize this loss.

### 3. Estimated Runtime

*   **Per Chunk Runtime:** For a single chunk (up to 100 words, 8D embeddings, plus relations), the estimated runtime is in the order of **seconds to tens of seconds**. This is a rough estimate and can vary significantly based on:
    *   The specific MiniZinc solver used (e.g., Gecode, Chuffed, CPLEX).
    *   The complexity of the constraints and objective function within `word_embedding_inference.mzn`.
    *   The underlying hardware performance.
*   **Total Full Run Runtime (Conceptual):** If the MiniZinc parsing bug were resolved and all 2764 chunks were processed sequentially, the total estimated runtime would be substantial:
    *   Assuming an average of **5 seconds per chunk**: `2764 chunks * 5 seconds/chunk = 13,820 seconds` (approximately **3.8 hours**).
    *   Assuming an average of **10 seconds per chunk**: `2764 chunks * 10 seconds/chunk = 27,640 seconds` (approximately **7.6 hours**).

    This highlights the necessity of the incremental solving approach and the potential future need for parallel processing or more optimized MiniZinc models/solvers.

### 4. How the Bug is Demonstrated

When executing the minimal test case command:

```bash
/data/data/com.termux/files/home/storage/github/libminizinc/build/minizinc \
    /data/data/com.termux/files/home/storage/github/libminizinc/test_num_words.mzn \
    /data/data/com.termux/files/home/storage/github/libminizinc/minizinc_data/huggingface/word_embeddings_chunk_0.dzn
```

**Expected Outcome (if the bug is present):**

MiniZinc will produce an error message similar to:

```
Error: type error: undefined identifier 'num_words'
```

This error is the manifestation of the bug. It indicates that MiniZinc fails to correctly parse and recognize the `num_words` variable (and other variables) defined within `word_embeddings_chunk_0.dzn`, even though the `.dzn` file itself is syntactically correct and contains the definition. This suggests an issue with MiniZinc's data file processing or its interaction with the environment.
