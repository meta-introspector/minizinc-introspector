# SOP: CRQ Updater Quality Assurance Procedure

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the steps for performing Quality Assurance (QA) on the `crq_updater` Rust program. The goal is to ensure the program correctly identifies relevant commits, accurately extracts and formats commit information, and updates CRQ files without unintended side effects, adhering to the "Commit Labeling and CRQ Ownership Procedure" SOP.

## 2. Scope
This SOP applies to all development and testing cycles of the `crq_updater` program.

## 3. Pre-requisites
*   A clean Git working directory (no uncommitted changes). Use `git status` to verify.
*   The `crq_updater` program has been successfully built (`cargo build --package crq_updater`).
*   Familiarity with the "Commit Labeling and CRQ Ownership Procedure" SOP.

## 4. QA Procedure

### 4.1. Dry Run Verification

1.  **Execute in Dry Run Mode:**
    ```bash
    cargo run --package crq_updater -- --dry-run
    ```
2.  **Review Dry Run Output:**
    *   Carefully examine the console output for each CRQ file. The output will show the *proposed* changes.
    *   **Verify Proposed Changes:**
        *   **Correctness of New Entries:** Ensure that any new commit entries (those not previously in the CRQ) are correctly formatted and contain accurate hash, subject, and description.
        *   **Chronological Order:** Confirm that all commit entries (both existing and proposed new ones) are in strict chronological order (oldest first).
        *   **Description Accuracy:** For commits with empty bodies, verify that the generated diff summary accurately reflects the changes to the CRQ file.
        *   **Creation Commit Handling:** For CRQ creation commits, ensure the description is correctly set to "This commit created the CRQ file. The file's initial content serves as its primary description."
        *   **No Duplicates:** Confirm that no duplicate commit entries are proposed.
        *   **No Unintended Modifications:** Ensure that parts of the CRQ file outside the "Commit History" section are not proposed for modification.

### 4.2. Actual Run and Post-Execution Verification

1.  **Execute in Actual Mode:**
    ```bash
    cargo run --package crq_updater
    ```
2.  **Review Git Status:**
    ```bash
    git status
    ```
    *   Verify that only the expected CRQ files are listed as modified.
    *   Confirm no other files have been unexpectedly changed.
3.  **Review Git Diff:**
    ```bash
    git diff
    ```
    *   Perform a detailed review of the `git diff` output for each modified CRQ file.
    *   **Confirm Actual Changes:** Ensure the changes applied to the files precisely match the proposed changes observed during the dry run.
    *   **Verify No Regression:** Confirm that existing, correctly formatted commit entries have not been altered or corrupted.
4.  **Idempotency Test:**
    *   Run the `crq_updater` in actual mode again immediately:
        ```bash
        cargo run --package crq_updater
        ```
    *   Then, check `git status` again:
        ```bash
        git status
        ```
    *   **Expected Result:** The `git status` should report "nothing to commit, working tree clean." This verifies that running the program multiple times produces the same result and does not introduce new changes.

## 5. Reporting
*   Any discrepancies or unexpected behavior observed during the QA process must be reported and addressed before the `crq_updater` is considered ready for use.
## Commit History

**Commit:** `ca9fe7e6ded6c6458b1d61ffd990063bafedede8`
**Subject:** `wip`
**Description:**
This commit created the CRQ file. The file's initial content serves as its primary description.