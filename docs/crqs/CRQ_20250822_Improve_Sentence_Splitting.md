## Change Request: Improve Sentence Splitting Robustness

**Date:** 2025-08-22

**Author:** Gemini CLI Agent (Rust Edit Manager)

**Purpose:**
To enhance the accuracy and robustness of sentence splitting within the `NoveltyTracker` to better handle complex linguistic structures.

**Problem Description:**
The current `sentence_regex` (`r"([.!?]\s*)"`) used in `NoveltyTracker::process_all_poems` is a very basic sentence splitter. It relies solely on common punctuation marks (`.`, `!`, `?`) followed by whitespace. This approach is prone to errors and will incorrectly split sentences in several common scenarios, such as:
*   Abbreviations (e.g., "Dr. Smith", "U.S.A.")
*   Ellipses (e.g., "The story continued...")
*   Sentences that do not end with standard punctuation (e.g., headings, list items, code snippets within markdown).
*   Direct quotes that contain internal punctuation.

**Impact:**
Inaccurate sentence splitting leads to:
*   Incorrect `TextChunk` boundaries.
*   Misrepresentation of word novelty and linking, as words might be attributed to the wrong chunk or novelties missed due to incorrect chunking.
*   Reduced quality of the overall text model and subsequent analyses.

**Proposed Solution (Rust Code/Approach):**
Replace the simple regex-based sentence splitter with a more sophisticated, rule-based or machine-learning-based approach.

**Option 1 (Rule-based Enhancement):**
Enhance the current regex to include more complex rules for common abbreviations and edge cases. This would involve a more intricate regex pattern and potentially a post-processing step to merge incorrectly split segments.

**Option 2 (NLP Library Integration - Recommended for Phase 2):**
For truly robust sentence splitting, integrate a dedicated NLP library that provides pre-trained sentence tokenizers. While this is a Phase 2 consideration for full NLP, a robust sentence splitter is foundational. If a suitable Rust NLP library is identified (e.g., `rust-stemmers`, `whatlang`, or a vendored solution), its sentence tokenization capabilities should be prioritized.

**Rationale:**
Accurate sentence splitting is fundamental to the `NoveltyTracker`'s ability to correctly identify `TextChunk` units and subsequently track word novelty and links. Investing in a more robust splitter early will prevent cascading errors in later stages of text analysis.

**Expected Outcome:**
More accurate `TextChunk` generation, leading to a higher quality and more reliable text model.

**Next Steps:**
Investigate and implement a more robust sentence splitting mechanism within `NoveltyTracker::process_all_poems`. Prioritize a solution that balances accuracy with performance, considering the project's constraints on external dependencies.
