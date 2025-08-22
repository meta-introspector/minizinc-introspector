---
title: Automated Regex Template Generation from Unmatched Lines
status: Proposed
date: 2025-08-22
authors:
  - Gemini CLI
---

# Automated Regex Template Generation from Unmatched Lines

## 1. Problem Statement

Currently, the `poem_yaml_fixer` tool identifies unmatched lines in Markdown files, and a generalized regex can be generated from these lines. However, there is no automated mechanism to integrate these newly generated regexes back into the system as reusable templates. This manual process hinders the continuous improvement and self-correction capabilities of the `poem_yaml_fixer`, requiring human intervention to update the regex patterns.

## 2. Proposed Solution

Implement a system within the `poem_yaml_fixer` to automatically generate, name, and propose new regex modules based on unique unmatched lines. This will enable the tool to learn and adapt to new patterns encountered in Markdown files, reducing manual intervention and enhancing its self-correction capabilities.

### 2.1. Key Features

*   **Unique Unmatched Line Identification**: The system will identify and collect unique unmatched lines from the `regex_match_details.txt` report.
*   **Inline Regex Generation**: For each unique unmatched line, `grex` will be used to generate a generalized regular expression. This process will occur inline during the `poem_yaml_fixer`'s execution or as a post-processing step.
*   **Automated Template Creation**: The generated regexes will be used to create new regex templates.
*   **Intelligent Naming Convention**: Each new regex template will be named based on a truncated version of the text it matches, ensuring human readability and relevance.
*   **New Regex Module Proposal**: The system will propose the creation of new Rust modules (e.g., `crates/poem_yaml_fixer/src/regex_templates/new_template_name.rs`) containing these new regex templates. This proposal will be presented in a format that allows the user to easily review and commit the changes.
*   **Value Matching**: The proposed regex modules will not only match the new patterns but also extract relevant values from the matched lines, similar to existing regex templates.

## 3. Acceptance Criteria

*   The `poem_yaml_fixer` successfully identifies unique unmatched lines.
*   For each unique unmatched line, a generalized regex is automatically generated.
*   New regex templates are created based on these generated regexes.
*   The new templates are named intelligently based on truncated matched text.
*   The system proposes new Rust modules containing these templates, ready for user review and commit.
*   The proposed regexes correctly match the intended patterns and extract relevant values.

## 4. Impact

*   **Increased Automation**: Reduces manual effort in updating regex patterns.
*   **Enhanced Self-Correction**: The `poem_yaml_fixer` can adapt to new patterns autonomously.
*   **Improved Efficiency**: Faster iteration and deployment of new regexes.
*   **Scalability**: The system can handle a growing variety of Markdown file structures.

## 5. Technical Details (High-Level)

*   **Data Flow**: The `regex_match_details.txt` will be parsed to extract unique unmatched lines.
*   **Regex Generation**: The `grex` crate will be utilized for generating regexes.
*   **File Generation**: New `.rs` files will be generated within a designated directory (e.g., `crates/poem_yaml_fixer/src/regex_templates/`) for each new regex template.
*   **Module Integration**: A mechanism will be needed to dynamically load or suggest integration of these new regex modules into the `poem_yaml_fixer`'s regex matching pipeline. This might involve updating `mod.rs` files or a configuration file.
*   **Naming Logic**: A function will be developed to truncate and sanitize the unmatched lines to create valid and descriptive Rust module/function names.
*   **Proposal Mechanism**: The system will output a clear summary of the proposed new regex modules, potentially including the content of the new files, for user review.
