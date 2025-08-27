# SOP: Collaborative Interactive Git Rebase

## 1. Purpose

This SOP outlines a structured approach for multiple team members (or automated agents) to collaborate on a single interactive Git rebase operation. It aims to improve efficiency by parallelizing the rebase process while maintaining a clean and consistent commit history.

## 2. Prerequisites

*   All team members have access to the shared Git repository.
*   A designated "Rebase Coordinator" is responsible for initial setup and final integration.
*   Clear communication channels are established (e.g., chat, issue tracker).
*   Familiarity with Git interactive rebase commands (`reword`, `edit`, `squash`, `fixup`, `exec`, `continue`, `abort`).

## 3. Roles and Responsibilities

*   **Rebase Coordinator:**
    *   Initiates the interactive rebase.
    *   Divides the `git-rebase-todo` list into manageable parts.
    *   Assigns parts to individual contributors.
    *   Provides clear instructions and context for each part.
    *   Integrates the rebased parts back into the main branch.
    *   Handles any conflicts arising during integration.
    *   Communicates overall progress and roadblocks.
*   **Contributors:**
    *   Take ownership of their assigned rebase part.
    *   Execute the rebase commands for their commits.
    *   Resolve any conflicts within their assigned commits.
    *   Communicate progress, questions, and completion status to the Coordinator.

## 4. Procedure

### 4.1. Initiation by Rebase Coordinator

1.  **Ensure Clean State:** The Coordinator ensures the target branch (e.g., `feature/community-docs`) is clean and up-to-date with its upstream.
    ```bash
    git checkout feature/community-docs
    git pull --rebase origin/feature/community-docs # Or appropriate upstream
    ```
2.  **Start Interactive Rebase:** Initiate the interactive rebase, specifying the range of commits to be rebased.
    ```bash
    git rebase -i <base_commit_hash>
    ```
    *   **Note:** Git will open an editor with the `git-rebase-todo` file. **Do NOT proceed yet.**
3.  **Save `git-rebase-todo`:** Copy the content of the `.git/rebase-merge/git-rebase-todo` file to a shared location or a new file in the project root (e.g., `rebase_todo_full.txt`).
    ```bash
    cp .git/rebase-merge/git-rebase-todo rebase_todo_full.txt
    ```
4.  **Abort Initial Rebase:** Abort the currently running interactive rebase. This is crucial to allow individual contributors to work independently.
    ```bash
    git rebase --abort
    ```
5.  **Divide and Distribute:**
    *   Divide the `rebase_todo_full.txt` file into `N` parts (e.g., 7 parts as requested). Each part should contain a contiguous block of commits.
    *   Create `N` new files (e.g., `rebase_part_1.txt`, `rebase_part_2.txt`, ..., `rebase_part_7.txt`).
    *   Assign each part to a Contributor.
    *   For each part, the Coordinator should provide:
        *   The `rebase_part_X.txt` file.
        *   The `base_commit_hash` (the commit *before* the first commit in their `rebase_part_X.txt`).
        *   Instructions on the desired action for each commit (e.g., `reword`, `fixup`).
        *   Specific guidance for "wip", "wup", "wow" commits (e.g., "analyze changes and provide a descriptive one-liner").

### 4.2. Execution by Contributors

1.  **Create a Working Branch:** Each Contributor creates a new branch based on the `base_commit_hash` provided by the Coordinator.
    ```bash
    git checkout -b my-rebase-part-<X> <base_commit_hash>
    ```
2.  **Apply Commits from Part:** Manually apply the commits from their `rebase_part_X.txt` using `git cherry-pick` or by creating new commits with the desired messages.
    *   **Option A (Cherry-pick and Amend - Recommended for `reword`):**
        ```bash
        # For each commit in rebase_part_X.txt:
        git cherry-pick <commit_hash>
        git commit --amend -m "New descriptive message"
        ```
    *   **Option B (Interactive Rebase on a Subset - More Complex):**
        *   This involves creating a temporary branch and performing an interactive rebase on just their assigned commits. This is more advanced and requires careful handling to avoid conflicts with other parts.
        *   `git rebase -i <commit_before_first_in_part> <last_commit_in_part>`
3.  **Resolve Conflicts:** If conflicts arise during `cherry-pick` or rebase, resolve them immediately and `git add` the resolved files.
4.  **Verify:** Ensure the changes in their part are correct and the commit messages are as desired.
5.  **Push to Personal Remote (Optional but Recommended):** Push their rebased branch to a personal remote for backup and visibility.
    ```bash
    git push origin my-rebase-part-<X>
    ```
6.  **Notify Coordinator:** Inform the Coordinator that their part is complete.

### 4.3. Integration by Rebase Coordinator

1.  **Update Main Branch:** The Coordinator ensures their `feature/community-docs` branch is up-to-date with the latest upstream.
    ```bash
    git checkout feature/community-docs
    git pull --rebase origin/feature/community-docs
    ```
2.  **Integrate Parts Sequentially:** For each completed Contributor's branch:
    ```bash
    git merge --no-ff my-rebase-part-<X>
    ```
    *   **Resolve Conflicts:** If merge conflicts occur, resolve them and commit.
3.  **Final Review:** Perform a final review of the integrated `feature/community-docs` branch.
4.  **Push to Upstream:** Push the fully rebased and integrated branch to the main upstream.
    ```bash
    git push origin feature/community-docs
    ```
    *   **Note:** If the goal is to merge into `main` (as per user's previous instruction), the Coordinator would then merge `feature/community-docs` into `main` and push `main`.

## 5. Communication Guidelines

*   **Clear Assignments:** Coordinator provides unambiguous instructions for each part.
*   **Regular Updates:** Contributors provide regular updates on their progress.
*   **Immediate Reporting:** Any issues or unexpected behavior are reported immediately to the Coordinator.
*   **Version Control:** All intermediate branches and commits are version-controlled.
