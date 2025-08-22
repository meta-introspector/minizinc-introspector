## Change Request: Implement Persistence for NoveltyTracker State

**Date:** 2025-08-22

**Author:** Gemini CLI Agent (Rust Edit Manager)

**Purpose:**
To enable the `NoveltyTracker` to save and load its state, allowing for incremental processing and long-term storage of the text model.

**Problem Description:**
The current `NoveltyTracker` processes all poems and stores its state (global vocabulary, chunks, links) entirely in memory. There is no mechanism to:
*   **Save State:** Persist the processed data to disk.
*   **Load State:** Reload a previously saved state to continue processing or for analysis without re-processing all poems from scratch.

**Impact:**
*   **Lack of Persistence:** All processed data is lost when the program terminates.
*   **Inefficient Re-processing:** Every time the `NoveltyTracker` is run, it must re-process all poems, which is inefficient for large datasets or frequent analysis.
*   **Limited Scalability:** In-memory storage limits the size of the corpus that can be processed.
*   **Hindered Analysis:** The generated text model cannot be easily used by other tools or for long-term analysis without a serialization format.

**Proposed Solution (Rust Code/Approach):**

**Option 1 (Serialization with `serde`):**
Implement `serde::Serialize` and `serde::Deserialize` traits for the `NoveltyTracker` struct and its internal data structures (`TextChunk`, `HashMap`, `HashSet`).
*   **Serialization Format:** Choose a suitable serialization format (e.g., JSON, YAML, Bincode, or Parquet for larger datasets). Given the project's existing use of YAML, `serde_yaml` could be a natural fit for smaller states, while `bincode` or `parquet` might be better for performance and scalability.
*   **Save/Load Functions:** Add `save_state(&self, path: &Path)` and `load_state(path: &Path)` methods to the `NoveltyTracker`.

**Option 2 (Custom Binary Format):**
For maximum control and potential optimization, a custom binary serialization format could be designed, though this is generally more complex and less interoperable than `serde`.

**Rationale:**
Persistence is a critical feature for any data processing system. It enables incremental processing, improves efficiency, and allows the generated text model to be a reusable asset for other parts of the `libminizinc` project.

**Expected Outcome:**
The `NoveltyTracker` will be able to save its state to disk and load it back, enabling efficient, incremental processing and long-term data storage.

**Next Steps:**
Implement serialization and deserialization capabilities for the `NoveltyTracker` using `serde` and a chosen format.
