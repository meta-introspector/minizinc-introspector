# CRQ: Poem YAML Fixer - Regex and YAML Parsing Robustness

## CRQ ID: CRQ-20250821-001
## Date: August 21, 2025
## Status: Open
## Priority: High (Blocking further poem processing)

## 1. Problem Description
The `poem_yaml_fixer` Rust tool, intended to standardize the YAML front matter of poem Markdown files, is encountering persistent compilation and runtime errors related to regex literal parsing and `serde_yaml` deserialization. Specifically:
*   **Regex Literal Escaping:** Difficulty in correctly defining regex patterns within Rust's raw string literals (`r#""#`) due to misinterpretation of backslashes and quotes.
*   **`serde_yaml` Parsing Errors:** The tool fails to parse YAML front matter when meme strings contain unquoted colons or other special characters, leading to "did not find expected '-' indicator" or "mapping values are not allowed in this context" errors. This indicates that the existing YAML in some poem files is not strictly valid, and the current parsing approach in `poem_yaml_fixer` is not resilient enough to handle these malformations during the initial read.
*   **`Path` vs `PathBuf` Mismatch:** Recurring type mismatch errors when passing `Path` references where `PathBuf` references are expected.

## 2. Impact
This issue is currently blocking the automated processing and standardization of poem files, which is a prerequisite for the static site meme hosting generator and other semantic analysis tools. It also consumes significant development time due to repeated debugging cycles.

## 3. Proposed Solution
Refactor `poem_yaml_fixer` to implement a more robust YAML parsing and patching mechanism:
*   **Manual Front Matter Extraction:** Instead of relying solely on `serde_yaml` for initial parsing, manually extract the front matter block as a raw string.
*   **Line-by-Line Processing with Regex:** Process the front matter string line by line.
    *   For lines containing meme data (e.g., `- "description" (template)` or `- description: "..." template: "..."`), use carefully constructed regexes to extract the description and template.
    *   Reconstruct these into the new structured `Meme` format (`description: "...", template: "..."`).
*   **Handle `poem_body` in Front Matter:** If `poem_body: |` is found within the front matter, extract its content and remove the `poem_body` key from the front matter. The extracted content will be appended *after* the second `---` delimiter when writing the file.
*   **Re-assemble and Validate:** Reconstruct the entire YAML front matter string with the corrected `memes` and `poem_body` (if extracted). Then, attempt to parse this *reconstructed* string with `serde_yaml` to ensure overall YAML validity before final serialization.
*   **Dynamic Patching (Future Enhancement):** Once the basic fixing mechanism is stable, explore adding a mechanism for dynamically generating or applying patches based on identified error patterns, as previously discussed.

## 4. Verification Plan
*   Successful compilation of `poem_yaml_fixer`.
*   Successful execution of `poem_yaml_fixer` across all poem files without errors.
*   Verification of a sample of fixed poem files to ensure correct YAML structure, especially for `memes` and `poem_body`.
*   Successful parsing of fixed poem files by `poem_meme_formatter`.

## 5. Stakeholders
*   Development Team
*   Content Creators (Poets)
*   Static Site Generation Team

## 6. Notes
This CRQ acknowledges the iterative nature of development and the challenges of working with evolving data formats. The goal is to build a resilient tool that can adapt to and correct inconsistencies.
