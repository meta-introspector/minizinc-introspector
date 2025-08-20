Very nice! Welcome, new friend, to the glorious ZOS Codebase Querying! You are now learning to make great analysis of code, very powerful! Like strong Kazakhstan horse, your queries will be fast and true!

# Standard Operating Procedure (SOP): ZOS Codebase Querying

## 1. Purpose

This SOP provides step-by-step instructions for new users ("noobs") on how to effectively use the ZOS codebase analysis and query tools. These tools help you analyze Rust codebases, understand their structure, find similar code, and perform targeted queries.

## 2. What the ZOS Codebase Tools Do

The ZOS codebase tools are a suite of powerful Rust programs designed to:

*   **Analyze Rust Projects**: They scan your Rust projects (identified by `Cargo.toml` files) to extract valuable information from `.rs` files.
*   **Generate Code Metrics**: For each Rust file, they calculate word counts and create a "bag of words" (a frequency map of all words).
*   **Smart Caching**: They intelligently cache analysis results per project, allowing for fast resumption and only re-processing changed files.
*   **Find Similarities**: They can identify crates that are structurally or semantically similar to each other based on their code content.
*   **Build a Term Index (Reverse Index)**: They create an index of keywords and the crates/files they appear in, enabling fast keyword searches.
*   **Identify Stopwords**: They can help you find common words that appear in almost all files, which might be useful to filter out for better search results.
*   **Perform Fast, Targeted Queries**: The `zos-fast-query` tool allows for quick analysis of shared terms and identification of related projects/directories.

## 3. Prerequisites

Before you start, ensure you have the following installed on your system:

*   **Rust and Cargo**: The Rust programming language and its package manager (Cargo) are essential. You can install them by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   **Git**: For managing code versions and fetching the `libminizinc` project.

## 4. Installation and Setup

1.  **Clone the Repository**: If you haven't already, clone the `libminizinc` repository (which contains the ZOS codebase tools):
    ```bash
    git clone <repository_url> # Replace <repository_url> with the actual URL
    cd libminizinc
    ```
2.  **Build the Tools**: Compile the ZOS codebase tools using Cargo. This might take a few minutes the first time.
    ```bash
    cargo build --workspace
    ```
    If the build is successful, you're ready to go!

## 5. Modes of Operation

The ZOS codebase tools operate in different modes, controlled by command-line arguments. You can run them from the `libminizinc` root directory.

### 5.1. `file_content_analyzer` Modes

The `file_content_analyzer` (located in `crates/rust_file_finder`) operates in different modes, controlled by the `--mode` command-line argument. You can run it using `cargo run --package file_content_analyzer -- --mode <MODE> [OPTIONS]`. Replace `<MODE>` with one of the following:

### 5.1.1. `full_analysis` (Default Mode)

This is the primary mode for analyzing your codebase and building the necessary caches.

*   **Purpose**: Scans your Rust projects, extracts code metrics, and builds caches for future operations.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode full_analysis
    ```
    *   **Note**: This process can take a significant amount of time, especially on the first run or after major code changes. It uses 16 threads for parallel processing to speed things up.
    *   **Resumability**: If interrupted, you can restart it, and it will resume from where it left off, only re-processing changed or new files.
    *   **Output**: Progress messages will be printed to your terminal (stderr). You can redirect this to a log file for review (e.g., `2> full_analysis.log`).

### 5.1.2. `read_cargo_toml` Mode

*   **Purpose**: Quickly view the content of all `Cargo.toml` files in your scanned directories.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode read_cargo_toml
    ```
*   **Output**: Prints the content of each `Cargo.toml` file to your terminal.

### 5.1.3. `crate_similarity` Mode

*   **Purpose**: Find crates that are most similar to a specified target crate based on their code content.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode crate_similarity --target-crate <TARGET_CRATE_NAME> --most-similar <NUMBER_OF_RESULTS>
    ```
    *   Replace `<TARGET_CRATE_NAME>` with the name of the crate you want to compare against (e.g., `file_content_analyzer`).
    *   Replace `<NUMBER_OF_RESULTS>` with how many top similar crates you want to see (e.g., `10`).
    *   **Example**: To find the top 5 crates most similar to `my_awesome_crate`:
        ```bash
        cargo run --package file_content_analyzer -- --mode crate_similarity --target-crate my_awesome_crate --most-similar 5
        ```
*   **Prerequisite**: You must run `full_analysis` at least once before using this mode to ensure the cache is built.
*   **Output**: Prints a list of similar crates and their similarity percentages.

### 5.1.4. `migrate_cache` Mode

*   **Purpose**: Migrates old, monolithic cache files (`file_analysis_cache.json`) to the new, distributed per-project cache files (`.file_analysis_summary.json`). This mode is typically run once if you have an older cache.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode migrate_cache
    ```
*   **Output**: Provides messages about the migration process.

### 5.1.5. `search_keywords` Mode

*   **Purpose**: Search for specific keywords across your analyzed Rust codebase and find which crates contain them.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode search_keywords --keywords <KEYWORD1> <KEYWORD2> ...
    ```
    *   Replace `<KEYWORD1> <KEYWORD2> ...` with the words you want to search for. Separate multiple keywords with spaces.
    *   **Example**: To find crates containing "parser" and "ast":
        ```bash
        cargo run --package file_content_analyzer -- --mode search_keywords --keywords parser ast
        ```
*   **Prerequisite**: You must run `full_analysis` at least once before using this mode to ensure the term index is built.
*   **Output**: Lists crates that contain the specified keywords and how many of the keywords they match.

### 5.1.6. `generate_stopword_report` Mode

*   **Purpose**: Helps identify potential stopwords by listing words that appear in a very high percentage of your Rust files.
*   **Usage**:    ```bash
    cargo run --package file_content_analyzer -- --mode generate_stopword_report
    ```
*   **Prerequisite**: You must run `full_analysis` at least once before using this mode.
*   **Output**: Prints a report of words and the percentage of files they appear in, sorted by percentage (highest first). Words appearing in 99% or more files are highlighted as strong candidates for stopwords.

### 5.1.7. `build_hierarchical_index` Mode

*   **Purpose**: Builds a comprehensive, hierarchical term index (reverse index) that maps every unique word found in your Rust codebase to the specific files where it appears. This index is crucial for efficient keyword searching and understanding the distribution of terms across your project.
*   **Usage**:    ```bash
    cargo run --package file_content_analyzer -- --mode build_hierarchical_index
    ```
*   **Prerequisite**: You must run `full_analysis` at least once before using this mode, as it relies on the per-project analysis summaries (`.file_analysis_summary.json`) generated by that mode.
*   **Output**: Prints progress messages to stderr and saves the complete hierarchical term index to `hierarchical_term_index.json` in the root of your search directory (e.g., `/data/data/com.termux/files/home/storage/github/`).

### 5.1.8. `find_exact_shared_files_terms` Mode

*   **Purpose**: Identifies groups of terms that appear in the exact same set of files. This is particularly useful for finding common code patterns, highly correlated concepts, or potential duplicate code across different parts of your projects.
*   **Usage**:    ```bash
    cargo run --package file_content_analyzer -- --mode find_exact_shared_files_terms [--search-path <PATH>]
    ```
    *   `--search-path <PATH>`: (Optional) Limits the search to files within a specific subdirectory. Replace `<PATH>` with the absolute path to the directory you want to search within.
*   **Prerequisite**: You must run `build_hierarchical_index` at least once before using this mode to ensure the hierarchical term index is built.
*   **Output**: Groups terms by the exact set of files they appear in. Each group lists the common files and the terms found within them. This helps in identifying common code across projects.

### 5.2. `zos-fast-query` Tool

The `zos-fast-query` tool (located in `crates/zos-fast-query`) is designed for fast, targeted analysis of shared terms, especially for identifying related projects and directories.

*   **Purpose**: Analyzes a JSON file containing shared term groups (e.g., `temp_shared_terms.json` generated by `find_exact_shared_files_terms` mode) to identify files from other projects/directories that share terms with a specified filter term or the input file itself.
*   **Usage**:
    ```bash
    cargo run --package zos-fast-query -- -i <INPUT_FILE_PATH> [-f <FILTER_TERM>]
    ```
    *   `<INPUT_FILE_PATH>`: The absolute or relative path to the JSON file containing shared term groups (e.g., `./temp_shared_terms.json`).
    *   `<FILTER_TERM>`: (Optional) A string term to filter by. Only groups containing files whose paths include this term (or the input file path itself) will be considered relevant. From these relevant groups, files from *other* projects/directories will be identified.
*   **Prerequisite**: You must have a JSON file containing shared term groups, typically generated by the `file_content_analyzer`'s `find_exact_shared_files_terms` mode.
*   **Output**: Prints a summary of unique files found in other projects/directories related to the filter term or input file, along with a list of those related projects/directories and their file counts.

## 6. Recommended Workflow

1.  **Initial Setup & Full Analysis**: Run `full_analysis` first to build your initial cache.
    ```bash
    ./run_similarity_analysis.sh # This script automates full_analysis and then crate_similarity
    ```
    Or, if you just want the full analysis:
    ```bash
    cargo run --package file_content_analyzer -- --mode full_analysis 2> full_analysis.log
    ```
2.  **Build Hierarchical Index**: After running `full_analysis`, build the hierarchical term index. This creates the necessary data for efficient keyword searching.
    ```bash
    cargo run --package file_content_analyzer -- --mode build_hierarchical_index
    ```
3.  **Find Shared Terms**: Generate the `temp_shared_terms.json` file using the `find_exact_shared_files_terms` mode.
    ```bash
    cargo run --package file_content_analyzer -- --mode find_exact_shared_files_terms > temp_shared_terms.json
    ```
4.  **Analyze Shared Terms with `zos-fast-query`**: Use the `zos-fast-query` tool to analyze the `temp_shared_terms.json` file and identify related projects.
    ```bash
    cargo run --package zos-fast-query -- -i ./temp_shared_terms.json -f rust_file_finder
    ```
    Replace `rust_file_finder` with any term you want to use for filtering.
5.  **Migrate Cache (if applicable)**: If you have an old `file_analysis_cache.json` from previous versions, run `migrate_cache` once.
    ```bash
    cargo run --package file_content_analyzer -- --mode migrate_cache
    ```
6.  **Explore Similarities**: Use `crate_similarity` to find similar crates.
    ```bash
    cargo run --package file_content_analyzer -- --mode crate_similarity --target-crate file_content_analyzer --most-similar 10
    ```
7.  **Search for Keywords**: Use `search_keywords` to find crates by specific terms.
    ```bash
    cargo run --package file_content_analyzer -- --mode search_keywords --keywords your_keyword_here
    ```
8.  **Refine Stopwords**: Use `generate_stopword_report` to identify and potentially add more stopwords to the `STOPWORDS` constant in `main.rs` for better analysis results.
    ```bash
    cargo run --package file_content_analyzer -- --mode generate_stopword_report
    ```

## 7. Interpreting Results

*   **Similarity Percentages**: Higher percentages mean more similar code content based on the bag-of-words model.
*   **Keyword Search Results**: Crates are listed with the count of matching keywords. A higher count means more of your search terms were found in that crate.
*   **Stopword Report**: Words with very high percentages (e.g., 99%+) are strong candidates for being added to the `STOPWORDS` list in `main.rs`. Remember to rebuild the tool after modifying `STOPWORDS`.
*   **`zos-fast-query` Results**: The tool will output the total count of unique files from other projects/directories that are related to your filter term or input file, along with a sorted list of those related projects/directories and the number of files found within them. This helps you understand the interconnectedness of your codebase.

## 8. Troubleshooting

*   **"Target crate not found"**: Ensure you have run `full_analysis` successfully and that the crate name is spelled correctly.
*   **Build Errors**: If `cargo build` fails, check the error messages carefully. Common issues include missing dependencies (ensure `Cargo.toml` is correct) or syntax errors (if you modified the code).
*   **Slow Performance**: The `full_analysis` can be slow on large codebases. It's multi-threaded, but still needs to read and process many files. Subsequent runs should be faster due to caching.

---

**Remember to rebuild the tools (`cargo build --workspace`) after any changes to their source code.**