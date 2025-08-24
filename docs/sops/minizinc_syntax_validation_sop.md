# SOP: MiniZinc Model Syntax Validation

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the process for validating the syntax of MiniZinc models within the project using the automated `test_recent_mzn_files.sh` script. This ensures that all MiniZinc models adhere to the language's syntax rules and compile without errors, contributing to overall code quality and system stability.

## 2. Scope
This SOP applies to all `.mzn` files within the project's repository, particularly those that have been recently modified or newly introduced.

## 3. Procedure

### 3.1. Prerequisites
*   A local clone of the project repository.
*   The `minizinc` executable (from `LIBMINIZINC_BUILD_DIR`) must be built and accessible.
*   The `scripts/test_recent_mzn_files.sh` script must be present and executable.

### 3.2. Execution Steps

1.  **Navigate to the project root directory:**
    ```bash
    cd /data/data/com.termux/files/home/storage/github/libminizinc
    ```

2.  **Run the MiniZinc syntax validation script:**
    Execute the script located in the `scripts/` directory.
    ```bash
    ./scripts/test_recent_mzn_files.sh
    ```

### 3.3. Interpretation of Results

*   **Successful Compilation:** If a model compiles successfully, the script will output `[OK] Compiled successfully.` for that file.
*   **Failed Compilation:** If a model fails to compile, the script will output `[ERROR] Compilation failed!` along with the MiniZinc compiler's error messages.

### 3.4. Troubleshooting

*   **"Unrecognized option or bad format" error:** This indicates an incorrect flag passed to the `minizinc` executable. Verify the `minizinc` version and its supported flags using `minizinc --help`. Update `test_recent_mzn_files.sh` accordingly.
*   **"requires that a solver is selected explicitly" error:** Ensure `--solver Gecode` (or another appropriate solver) is included in the `minizinc` command within the script.
*   **Syntax Errors:** If the MiniZinc compiler reports syntax errors, refer to the "SOP: Debugging MiniZinc Syntax Errors" for guidance.
*   **File Not Found:** Ensure the file path in `RECENT_MZN_FILES` array in `test_recent_mzn_files.sh` is correct and the file exists.

## 4. Related Documents
*   SOP: Debugging MiniZinc Syntax Errors
*   `scripts/test_recent_mzn_files.sh`