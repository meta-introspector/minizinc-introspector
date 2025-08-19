# Standard Operating Procedure (SOP): Using the `file_content_analyzer`

## 1. Purpose

This SOP provides step-by-step instructions for new users ("noobs") on how to effectively use the `file_content_analyzer` tool. This tool helps you analyze Rust codebases, understand their structure, find similar code, and perform keyword searches.

## 2. What `file_content_analyzer` Does

The `file_content_analyzer` is a powerful Rust program designed to:

*   **Analyze Rust Projects**: It scans your Rust projects (identified by `Cargo.toml` files) to extract valuable information from `.rs` files.
*   **Generate Code Metrics**: For each Rust file, it calculates word counts and creates a "bag of words" (a frequency map of all words).
*   **Smart Caching**: It intelligently caches analysis results per project, allowing for fast resumption and only re-processing changed files.
*   **Find Similarities**: It can identify crates that are structurally or semantically similar to each other based on their code content.
*   **Build a Term Index (Reverse Index)**: It creates an index of keywords and the crates/files they appear in, enabling fast keyword searches.
*   **Identify Stopwords**: It can help you find common words that appear in almost all files, which might be useful to filter out for better search results.

## 3. Prerequisites

Before you start, ensure you have the following installed on your system:

*   **Rust and Cargo**: The Rust programming language and its package manager (Cargo) are essential. You can install them by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   **Git**: For managing code versions and fetching the `file_content_analyzer` project.

## 4. Installation and Setup

1.  **Clone the Repository**: If you haven't already, clone the `libminizinc` repository (which contains `file_content_analyzer`):
    ```bash
    git clone <repository_url> # Replace <repository_url> with the actual URL
    cd libminizinc
    ```
2.  **Navigate to the Tool's Directory**: The `file_content_analyzer` is located within the `crates/rust_file_finder` directory (even though its internal name is `file_content_analyzer`).
    ```bash
    cd crates/rust_file_finder
    ```
3.  **Build the Tool**: Compile the `file_content_analyzer` using Cargo. This might take a few minutes the first time.
    ```bash
    cargo build
    ```
    If the build is successful, you're ready to go!

## 5. Modes of Operation

The `file_content_analyzer` operates in different modes, controlled by the `--mode` command-line argument. You can run it from the `libminizinc` root directory using `cargo run --package file_content_analyzer -- --mode <MODE> [OPTIONS]`. Replace `<MODE>` with one of the following:

### 5.1. `full_analysis` (Default Mode)

This is the primary mode for analyzing your codebase and building the necessary caches.

*   **Purpose**: Scans your Rust projects, extracts code metrics, and builds caches for future operations.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode full_analysis
    ```
    *   **Note**: This process can take a significant amount of time, especially on the first run or after major code changes. It uses 16 threads for parallel processing to speed things up.
    *   **Resumability**: If interrupted, you can restart it, and it will resume from where it left off, only re-processing changed or new files.
    *   **Output**: Progress messages will be printed to your terminal (stderr). You can redirect this to a log file for review (e.g., `2> full_analysis.log`).

### 5.2. `read_cargo_toml` Mode

*   **Purpose**: Quickly view the content of all `Cargo.toml` files in your scanned directories.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode read_cargo_toml
    ```
*   **Output**: Prints the content of each `Cargo.toml` file to your terminal.

### 5.3. `crate_similarity` Mode

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

### 5.4. `migrate_cache` Mode

*   **Purpose**: Migrates old, monolithic cache files (`file_analysis_cache.json`) to the new, distributed per-project cache files (`.file_analysis_summary.json`). This mode is typically run once if you have an older cache.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode migrate_cache
    ```
*   **Output**: Provides messages about the migration process.

### 5.5. `search_keywords` Mode

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

### 5.6. `generate_stopword_report` Mode

*   **Purpose**: Helps identify potential stopwords by listing words that appear in a very high percentage of your Rust files.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode generate_stopword_report
    ```
*   **Prerequisite**: You must run `full_analysis` at least once before using this mode.
*   **Output**: Prints a report of words and the percentage of files they appear in, sorted by percentage (highest first). Words appearing in 99% or more files are highlighted as strong candidates for stopwords.

### 5.7. `build_hierarchical_index` Mode

*   **Purpose**: Builds a comprehensive, hierarchical term index (reverse index) that maps every unique word found in your Rust codebase to the specific files where it appears. This index is crucial for efficient keyword searching and understanding the distribution of terms across your project.
*   **Usage**:
    ```bash
    cargo run --package file_content_analyzer -- --mode build_hierarchical_index
    ```
*   **Prerequisite**: You must run `full_analysis` at least once before using this mode, as it relies on the per-project analysis summaries (`.file_analysis_summary.json`) generated by that mode.
*   **Output**: Prints progress messages to stderr and saves the complete hierarchical term index to `hierarchical_term_index.json` in the root of your search directory (e.g., `/data/data/com.termux/files/home/storage/github/`).

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
2.  **Migrate Cache (if applicable)**: If you have an old `file_analysis_cache.json` from previous versions, run `migrate_cache` once.
    ```bash
    cargo run --package file_content_analyzer -- --mode migrate_cache
    ```
3.  **Explore Similarities**: Use `crate_similarity` to find similar crates.
    ```bash
    cargo run --package file_content_analyzer -- --mode crate_similarity --target-crate file_content_analyzer --most-similar 10
    ```
4.  **Search for Keywords**: Use `search_keywords` to find crates by specific terms.
    ```bash
    cargo run --package file_content_analyzer -- --mode search_keywords --keywords your_keyword_here
    ```
5.  **Refine Stopwords**: Use `generate_stopword_report` to identify and potentially add more stopwords to the `STOPWORDS` constant in `main.rs` for better analysis results.
    ```bash
    cargo run --package file_content_analyzer -- --mode generate_stopword_report
    ```

## 7. Interpreting Results

*   **Similarity Percentages**: Higher percentages mean more similar code content based on the bag-of-words model.
*   **Keyword Search Results**: Crates are listed with the count of matching keywords. A higher count means more of your search terms were found in that crate.
*   **Stopword Report**: Words with very high percentages (e.g., 99%+) are strong candidates for being added to the `STOPWORDS` list in `main.rs`. Remember to rebuild the tool after modifying `STOPWORDS`.

## 8. Troubleshooting

*   **"Target crate not found"**: Ensure you have run `full_analysis` successfully and that the crate name is spelled correctly.
*   **Build Errors**: If `cargo build` fails, check the error messages carefully. Common issues include missing dependencies (ensure `Cargo.toml` is correct) or syntax errors (if you modified the code).
*   **Slow Performance**: The `full_analysis` can be slow on large codebases. It's multi-threaded, but still needs to read and process many files. Subsequent runs should be faster due to caching.

---

**Remember to rebuild the `file_content_analyzer` (`cargo build`) after any changes to its source code (e.g., modifying `STOPWORDS` in `main.rs`).**