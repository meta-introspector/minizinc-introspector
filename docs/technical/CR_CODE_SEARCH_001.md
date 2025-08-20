## Change Request: CR_CODE_SEARCH_001

**Date:** 2025-08-20

**Requester:** Gemini CLI

**Description:** Implement a "self-similar code search" capability within the `libminizinc` project.

**Justification:** To leverage existing term recognition and file discovery tools (`zos-fast-query`, `rust_file_finder`) for advanced code analysis, enabling the identification of semantically similar code sections based on textual patterns (e.g., a poem). This enhances code understanding, refactoring efforts, and knowledge management, aligning with ITIL's Service Operation and Continual Service Improvement principles.

**Proposed Solution:**
1.  **Identify Search Terms from Query:** Utilize `zos-fast-query::TermRecognizer` to extract key terms from a user-provided textual query (e.g., a poem).
2.  **Discover Target Files:** Employ `rust_file_finder` to locate all relevant Rust source files (`.rs`) within the codebase.
3.  **Analyze File Content:** For each identified Rust file, read its content and use `zos-fast-query::TermRecognizer` to extract its terms.
4.  **Calculate Similarity:** Compare the extracted terms from the query with the terms from each file using a defined similarity metric (e.g., Jaccard index, common term count).
5.  **Report Results:** Present a ranked list of files based on their similarity to the query.

**Impact:** Low. Utilizes existing, stable components. No direct modification to core project logic. Primarily a new analytical capability. Adheres to GMP (Good Manufacturing Practices) by not altering existing, validated code paths.

**Risk:** Low. Minimal risk of system disruption. Potential for high resource consumption on very large codebases, which will be monitored. Six Sigma principles will be applied to optimize performance if resource consumption becomes an issue.

**Rollback Plan:** Not applicable, as no persistent changes are made to the codebase beyond the execution of the search. The operation is idempotent and read-only in terms of persistent state.

**Approvers:** (Conceptual: User/Project Lead)

**Status:** Proposed
