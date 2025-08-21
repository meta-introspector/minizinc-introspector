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
Refactor `poem_yaml_fixer` to implement a more robust YAML parsing and patching mechanism. This tool will act as a pre-processor to ensure all poem Markdown files conform to a strictly valid YAML front matter structure before further processing by `poem_meme_formatter`.

*   **Manual Front Matter Extraction:** The tool will read the file content line by line and manually identify the `---` delimiters to extract the raw front matter string and the raw poem body string.
*   **Resilient Front Matter Parsing:** It will attempt to parse the extracted raw front matter string into a `serde_yaml::Value`. If this initial parse fails due to malformed YAML (e.g., unquoted strings with colons), the tool will attempt to apply specific patching heuristics.
    *   **Patching Heuristic for Meme Lines:** For lines identified as meme entries (e.g., starting with `-` and containing `"` and `(`), the tool will use regex to extract the description and template, and then reconstruct the line into a strictly valid YAML format (e.g., `- description: "..."
  template: "..."
`).
    *   **Patching Heuristic for `poem_body: |`:** If a line `poem_body: |` is found within the front matter, the tool will extract the subsequent indented lines as the poem body content. This `poem_body` key will then be removed from the front matter, and its content will be appended *after* the second `---` delimiter when the file is rewritten.
    *   **General Quoting Fallback:** For other unparseable lines, the tool will attempt to quote the entire line to make it a valid YAML scalar.
*   **Reconstruction and Validation:** After applying patching heuristics, the tool will reconstruct the entire YAML front matter string and attempt to parse it again with `serde_yaml` to ensure overall YAML validity.
*   **Final File Rewrite:** The tool will then write the fixed YAML front matter and the poem body (now correctly positioned after the second `---` delimiter) back to the file.

This approach ensures that `poem_yaml_fixer` can handle the existing malformations and produce clean, valid YAML for subsequent tools.

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