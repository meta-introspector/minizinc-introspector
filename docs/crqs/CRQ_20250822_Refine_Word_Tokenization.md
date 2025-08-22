## Change Request: Refine Word Tokenization and Filtering

**Date:** 2025-08-22

**Author:** Gemini CLI Agent (Rust Edit Manager)

**Purpose:**
To improve the precision of word tokenization and filtering within the `NoveltyTracker` for a cleaner and more semantically accurate vocabulary.

**Problem Description:**
The current `word_regex` (`r"\b\w+\b"`) and basic filtering (`word.len() > 1`) in `NoveltyTracker::process_sentence` are simplistic. This approach may lead to:
*   **Suboptimal Tokenization:** Hyphenated words (e.g., "well-being"), contractions (e.g., "don't"), or words with internal apostrophes (e.g., "o'clock") might be incorrectly split or treated as separate tokens if the desired behavior is to keep them as single units.
*   **Inclusion of Noise:** While `word.len() > 1` filters single characters, it doesn't exclude common "stopwords" (e.g., "is", "and", "of") which often carry little semantic weight for novelty tracking. It also doesn't handle numbers or symbols that might be part of a word (e.g., "C++", "Rust-1.0").

**Impact:**
*   **Inflated Vocabulary:** The vocabulary (`global_vocabulary`) may contain noise or redundant entries (e.g., "don" and "t" instead of "don't").
*   **Less Meaningful Novelty:** Novelty might be tracked for words that are common stopwords, diluting the focus on truly semantically significant new terms.
*   **Reduced Linking Quality:** Links based on noisy or poorly tokenized words may not accurately reflect semantic connections.

**Proposed Solution (Rust Code/Approach):**

**Option 1 (Enhanced Regex and Custom Filtering):**
*   **Refine `word_regex`:** Develop a more sophisticated regex pattern to handle hyphenation, contractions, and other specific cases as desired.
*   **Implement a Stopword List:** Create a `HashSet<String>` of common English stopwords and filter out words present in this set during tokenization. This list could be loaded from a configuration file or hardcoded.
*   **Handle Numbers/Symbols:** Define a clear policy for words containing numbers or special characters (e.g., remove them, keep them, or treat them as distinct tokens).

**Option 2 (NLP Library Integration - Recommended for Phase 2):**
For advanced tokenization (e.g., handling different languages, specific linguistic phenomena), a dedicated NLP library would be ideal. This would also facilitate lemmatization (reducing words to their base form, e.g., "running" -> "run"), which is crucial for "syntax-aware" processing.

**Rationale:**
A cleaner and more accurate vocabulary is essential for meaningful novelty tracking and effective chunk linking. By refining tokenization and filtering, we ensure that the `NoveltyTracker` focuses on semantically relevant terms.

**Expected Outcome:**
A more precise and semantically richer `global_vocabulary`, leading to more accurate novelty detection and chunk linking.

**Next Steps:**
Implement improved word tokenization and filtering within `NoveltyTracker::process_sentence`. Prioritize the inclusion of a configurable stopword list.
