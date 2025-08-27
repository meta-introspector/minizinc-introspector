# Tmux Controller CLI QA

This document outlines the Quality Assurance (QA) test cases for the `tmux_controller` CLI enhancements.

## Test Cases

### 1. Test `split-horizontal` with `--session-name`

*   **Objective:** Verify that `split-horizontal` correctly splits the specified tmux session.
*   **Steps:**
    1.  Create a new tmux session (e.g., `tmux new -s test_session`).
    2.  Run `cargo run -p tmux_controller -- split-horizontal --session-name test_session`
    3.  Verify that `test_session` is split horizontally.

### 2. Test `split-vertical` with `--session-name`

*   **Objective:** Verify that `split-vertical` correctly splits the specified tmux session.
*   **Steps:**
    1.  Create a new tmux session (e.g., `tmux new -s test_session`).
    2.  Run `cargo run -p tmux_controller -- split-vertical --session-name test_session`
    3.  Verify that `test_session` is split vertically.

## Commit History

- [Commit 23104bac1cf99fa82e998471ac1f929724700122: feat: Enhance launchpad and tmux_controller CLI with new arguments and documentation](docs/commits/23104bac1cf99fa82e998471ac1f929724700122_feat_Enhance_launchpad_and_tmux_controller_CLI_with_new_arguments_and_documentation.md)