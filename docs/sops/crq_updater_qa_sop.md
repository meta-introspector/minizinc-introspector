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

**Commit:** `ea49de080e7e34d2e3171164a8b8acaceee84af0`
**Subject:** `refactor(crq_updater): Split functions into files and update CRQs`
**Description:**
Refactored the `crq_updater` crate to adhere to the "one declaration per file" convention.
- Moved functions (`find_crq_files`, `process_crq_file`, `extract_existing_history`, `get_commit_diff_summary`, `find_commit_from_oid`) into separate files within `src/functions/`.
- Updated `src/main.rs` to use the new module structure.
- Fixed compilation errors related to `git2` API changes (`parent_opt` to `parent`, `find_object` arguments, `diff.print` arguments).
- Updated `walkdir` dependency to `2.5.0`.
- Resolved `StripPrefixError` by canonicalizing `crq_path`.
- Successfully ran `crq_updater` to update `crq_launchpad_workflow_enhancements.md` and `docs/sops/crq_updater_qa_sop.md` with relevant commit history.

**Commit:** `ac86ef2eda7bd9de1a4ed252273b284b8d682d16`
**Subject:** `docs: Add Tmux Workflow and Debugging SOP; Refactor tmux_controller and dump_tmux_status`
**Description:**
This commit introduces a new Standard Operating Procedure (SOP) for Tmux Workflow and Debugging Procedures (`docs/sops/tmux_workflow_and_debugging_sop.md`). This SOP consolidates best practices for managing tmux sessions, executing commands within panes, and debugging related issues.

Key changes and fixes include:
- Refactored `crates/tmux_controller/src/commands/create_layout.rs` to prevent automatic Gemini launch in pane 1, enabling "native worker" execution.
- Corrected module import paths in `crates/tmux_controller/src/main.rs` and `crates/tmux_controller/src/commands/mod.rs` for `tmux_view` functionality.
- Created a new `dump_tmux_status` crate (`crates/dump_tmux_status/`) for comprehensive tmux state snapshots, including session and pane content.
- Added `dump_tmux_status` to the workspace `Cargo.toml`.
- Debugged and resolved compilation errors in `dump_tmux_status` related to `clap` derives and `tmux_interface` imports.
- Updated `GEMINI.md` to reference the new Tmux Workflow and Debugging SOP.

These changes enhance our ability to manage and debug tmux-based workflows, providing better visibility and control over development environments.