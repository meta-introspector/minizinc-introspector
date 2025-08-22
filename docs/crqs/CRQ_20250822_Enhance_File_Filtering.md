## Change Request: Enhance File Filtering in Poem Processing

**Date:** 2025-08-22

**Author:** Gemini CLI Agent (Rust Edit Manager)

**Purpose:**
To make the filtering of non-poem markdown files in `NoveltyTracker::process_all_poems` more robust and maintainable.

**Problem Description:**
The current filtering mechanism in `NoveltyTracker::process_all_poems` uses a hardcoded list of specific filenames to exclude (e.g., `CRQ_20250822_Fix_Agent_Sonnet.md`, `manager_lament_20250822.md`). This approach is brittle and not scalable:
*   **Maintenance Overhead:** Every new CRQ, log file, or other non-poem markdown file added to `docs/poems/` will require manual updates to this exclusion list.
*   **Error Prone:** Forgetting to add a new file to the list will lead to incorrect processing of non-poem content.
*   **Lack of Flexibility:** It doesn't allow for more general exclusion patterns (e.g., all files starting with "CRQ_").

**Impact:**
*   Processing of irrelevant files, leading to noise in the `global_vocabulary` and `TextChunk` data.
*   Increased processing time for irrelevant files.
*   Maintenance burden as new documentation files are added.

**Proposed Solution (Rust Code/Approach):**

**Option 1 (Pattern-based Exclusion):**
Implement a more flexible exclusion mechanism using regular expressions or glob patterns. This could involve:
*   Loading exclusion patterns from a configuration file (e.g., `regex_config.toml` or a new `poem_processing_config.toml`).
*   Using `Regex::is_match` or `glob` patterns to filter filenames.

**Option 2 (Dedicated Poem Directory/Tagging):**
*   **Dedicated Directory:** If feasible, organize poems into a dedicated subdirectory (e.g., `docs/poems/content/`) and process only that directory.
*   **Front Matter Tagging:** Introduce a specific field in the YAML front matter (e.g., `type: poem`) and filter files based on this tag after parsing the front matter. This would require a preliminary pass to read front matter.

**Rationale:**
A robust file filtering mechanism is crucial for ensuring that only relevant poem content is processed, maintaining the integrity and quality of the generated text model.

**Expected Outcome:**
Automated and accurate filtering of non-poem markdown files, reducing maintenance overhead and improving the quality of the processed data.

**Next Steps:**
Implement a more flexible file exclusion mechanism in `NoveltyTracker::process_all_poems`, preferably using pattern-based filtering or a configuration-driven approach.
