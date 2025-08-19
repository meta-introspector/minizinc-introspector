# Code Analysis and Similarity Tool Development Plan

This document outlines the strategic plan for enhancing the code analysis and similarity tool within the `libminizinc` project.

## Current Status (as of latest commit)

*   **Feature Extraction:** The `analyze-duplicates` command (part of `zos-bootstrap-main`) now extracts comprehensive features from Rust files, including keywords and AST structural counts (functions, structs, enums, traits, modules).
*   **Similarity Scoring:** A basic similarity scoring mechanism has been implemented, using Jaccard similarity for keywords and normalized Euclidean distance for AST counts.
*   **Tool Identification:** 10 potential "code search tools" or components within the `libminizinc` project have been identified.
*   **Infrastructure:** `temp_zos_test` has been successfully renamed to `zos-bootstrap-main`, clarifying its role as the main executable. `LD_LIBRARY_PATH` linking issues have been resolved.

## Strategic Plan

### 1. Consolidate and Clean Up

*   **Objective:** Improve code quality and address existing warnings.
*   **Tasks:**
    *   Address the `unused field` warning for `file_path` in `CodeFeatureExtractor` (either utilize it or remove it).
    *   Revisit and implement a robust solution for keyword extraction from module names (`visit_item_mod`) to replace the current temporary workaround.
    *   Address any other minor warnings (e.g., unused imports) across the relevant crates.

### 2. Implement Caching and Persistence

*   **Objective:** Optimize performance by storing and reusing extracted code features.
*   **Tasks:**
    *   Define a suitable data structure for serializing `CodeFeatureExtractor` instances (or a simplified representation of their extracted features).
    *   Implement functions to save these serialized features to disk (e.g., JSON, custom binary format).
    *   Implement functions to load previously saved features from disk, allowing for incremental analysis and faster subsequent runs.

### 3. Refine Similarity Scoring

*   **Objective:** Enhance the accuracy and relevance of similarity scores.
*   **Tasks:**
    *   Evaluate the effectiveness of the current Jaccard and Euclidean distance metrics.
    *   Explore and implement more advanced similarity metrics (e.g., cosine similarity for keyword vectors, tree edit distance for ASTs, or graph-based similarities).
    *   Consider implementing weighted scoring for different feature types based on their perceived importance.

### 4. Integrate Targeted Tool Identification

*   **Objective:** Enable the tool to specifically identify and analyze the project's existing code search tools.
*   **Tasks:**
    *   Create a mechanism within the `analyze-duplicates` command to recognize the 10 identified "code search tools" (e.g., by their file paths or package names).
    *   Modify the analysis to specifically compare the features of these identified tools against each other and potentially against other parts of the codebase.
    *   Generate a report highlighting the similarities and differences among these tools, providing insights into their functional overlap or unique characteristics.

This plan will guide the iterative development of a more sophisticated and insightful code analysis and similarity tool.