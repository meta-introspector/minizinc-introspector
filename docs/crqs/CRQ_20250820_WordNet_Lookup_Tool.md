## Change Request (CRQ)

**CRQ ID:** CRQ-20250820-001
**Title:** Implement WordNet Lookup Tool in `doc_to_minizinc_data`
**Date:** 2025-08-20
**Requested By:** Gemini CLI (on behalf of User)
**Status:** Proposed

### 1. Justification

This change is necessary to enhance the analytical capabilities of the `doc_to_minizinc_data` tool. By integrating a WordNet lookup feature, we can:

*   Leverage the already generated word embeddings for semantic analysis.
*   Enable users to query word relationships (synonyms, antonyms) directly within the tool.
*   Facilitate the integration of linguistic knowledge into our MiniZinc models, aligning with the "Grand Plan" for self-describing core dumps and fixed-point ideas.
*   Improve the overall utility and extensibility of the `doc_to_minizinc_data` crate.

### 2. Impact

*   **`doc_to_minizinc_data` crate:** A new subcommand will be added, extending its CLI interface.
*   **Codebase:** New Rust code will be added to handle the lookup logic, including reading Parquet files and processing text from `simulated_wordnet.txt`.
*   **User Experience:** Users will gain a new command-line capability for wordnet analysis.
*   **Dependencies:** No new top-level dependencies are anticipated, as `anyhow` and `arrow`/`parquet` are already in use.

### 3. Proposed Solution/Plan

1.  **Extend `doc_to_minizinc_data` CLI:**
    *   Add a new subcommand, `lookup-wordnet`, to the `doc_to_minizinc_data` tool using `clap`.
    *   This subcommand will accept a single word as an input argument.

2.  **Implement Lookup Logic:**
    *   **Load Embeddings:** Read the `word_embeddings.parquet` file to load the pre-computed word embeddings into memory.
    *   **Load WordNet:** Parse the `simulated_wordnet.txt` file to create an in-memory representation of word relationships (synonyms, antonyms).
    *   **Retrieve Input Word Embedding:** For the word provided by the user, retrieve its corresponding 8D embedding.
    *   **Find Related Words:** Identify synonyms and antonyms of the input word from the loaded WordNet data.
    *   **Calculate Distances:** For each related word, retrieve its embedding and calculate the Euclidean distance to the input word's embedding.
    *   **Present Results:** Display the input word, its embedding, and a list of its related words along with their embeddings and calculated distances.

3.  **Integrate into `main.rs`:** Add a new match arm within `doc_to_minizinc_data/src/main.rs` to dispatch to the new `lookup-wordnet` logic when the subcommand is invoked.

### 4. Verification

*   **Unit Tests:** New unit tests will be written for the `lookup-wordnet` functionality, covering:
    *   Successful lookup of existing words.
    *   Correct identification of synonyms and antonyms.
    *   Accurate Euclidean distance calculations.
    *   Handling of words not found in the index or wordnet.
*   **Manual Testing:** Execute the new `cargo run --package doc_to_minizinc_data -- lookup-wordnet <word>` command with various test cases.

### 5. Rollback Plan

In case of issues, the changes can be reverted by:

1.  Discarding the uncommitted changes related to this CRQ.
2.  If committed, reverting the specific commit(s) that introduced this feature.

### 6. Approvals

*   **Development Lead:** [Signature/Date]
*   **QA Lead:** [Signature/Date]
*   **Product Owner:** [Signature/Date]
