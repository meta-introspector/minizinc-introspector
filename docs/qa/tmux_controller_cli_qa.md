# Tmux Controller CLI Arguments QA

This document outlines the Quality Assurance (QA) test cases for the `tmux_controller` commands, specifically focusing on the `split-horizontal` command and its new `--session-name` argument.

## Test Cases

### 1. `split-horizontal` Command Verification

**Objective:** To ensure that the `split-horizontal` command correctly splits `tmux` windows, both in the current session and in specified sessions.

*   **Test Case 1.1: Default Behavior (Split Current Window)**
    *   **Action:**
        1.  Ensure a `tmux` session is active.
        2.  Run `tmux_controller split-horizontal`.
    *   **Expected Result:** The current `tmux` window is split horizontally. A new pane is created below the original pane.

*   **Test Case 1.2: Targeted Session Split (Existing Session)**
    *   **Action:**
        1.  Create a new `tmux` session (e.g., `tmux new -s test_session`).
        2.  Switch to a different `tmux` session or detach from `test_session`.
        3.  Run `tmux_controller split-horizontal --session-name test_session`.
    *   **Expected Result:** A window within the `test_session` is split horizontally. Verify by attaching to `test_session` (`tmux attach -t test_session`).

*   **Test Case 1.3: Non-Existent Session Name**
    *   **Action:** Run `tmux_controller split-horizontal --session-name non_existent_session`.
    *   **Expected Result:** The command should return an error indicating that the specified session does not exist. The `tmux` environment should remain unchanged.

### 2. `split-vertical` Command Verification

**Objective:** To ensure that the `split-vertical` command correctly splits `tmux` windows, both in the current session and in specified sessions.

*   **Test Case 2.1: Default Behavior (Split Current Window)**
    *   **Action:**
        1.  Ensure a `tmux` session is active.
        2.  Run `tmux_controller split-vertical`.
    *   **Expected Result:** The current `tmux` window is split vertically. A new pane is created to the right of the original pane.

*   **Test Case 2.2: Targeted Session Split (Existing Session)**
    *   **Action:**
        1.  Create a new `tmux` session (e.g., `tmux new -s test_session`).
        2.  Switch to a different `tmux` session or detach from `test_session`.
        3.  Run `tmux_controller split-vertical --session-name test_session`.
    *   **Expected Result:** A window within the `test_session` is split vertically. Verify by attaching to `test_session` (`tmux attach -t test_session`).

*   **Test Case 2.3: Non-Existent Session Name**
    *   **Action:** Run `tmux_controller split-vertical --session-name non_existent_session`.
    *   **Expected Result:** The command should return an error indicating that the specified session does not exist. The `tmux` environment should remain unchanged.

## General Verification Steps

*   **Error Handling:** Test with invalid argument values (e.g., non-string values for session name if applicable) to ensure appropriate error messages are displayed.
*   **Log Verification:** Check `tmux_controller`'s logs for any relevant messages or errors related to the `split-horizontal` and `split-vertical` commands.
