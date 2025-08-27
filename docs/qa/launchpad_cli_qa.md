# Launchpad CLI QA

This document outlines the Quality Assurance (QA) test cases for the `launchpad` CLI arguments.

## Test Cases

### 1. Test `--gemini-instances`

*   **Objective:** Verify that `launchpad` correctly launches the specified number of Gemini CLI instances.
*   **Steps:**
    1.  Run `cargo run -p launchpad -- --gemini-instances 2`
    2.  Verify that two Gemini CLI instances are launched.

### 2. Test `--record-session`

*   **Objective:** Verify that `launchpad` enables asciinema recording of sessions.
*   **Steps:**
    1.  Run `cargo run -p launchpad -- --record-session`
    2.  Verify that asciinema recording is initiated.

### 3. Test `--background-detached`

*   **Objective:** Verify that `launchpad` runs Gemini instances in detached background processes.
*   **Steps:**
    1.  Run `cargo run -p launchpad -- --background-detached`
    2.  Verify that Gemini instances are running in detached background processes.

## Commit History

- [Commit 23104bac1cf99fa82e998471ac1f929724700122: feat: Enhance launchpad and tmux_controller CLI with new arguments and documentation](docs/commits/23104bac1cf99fa82e998471ac1f929724700122_feat_Enhance_launchpad_and_tmux_controller_CLI_with_new_arguments_and_documentation.md)