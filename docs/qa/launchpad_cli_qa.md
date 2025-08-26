# Launchpad CLI Arguments QA

This document outlines the Quality Assurance (QA) test cases for the command-line arguments implemented in `crates/launchpad/src/launchpad_main.rs`. These tests aim to verify the correct functionality of `--gemini-instances`, `--record-session`, and `--background-detached` arguments, as well as their default behaviors.

## Test Cases

### 1. `--gemini-instances` Argument Verification

**Objective:** To ensure that `launchpad` correctly launches the specified number of Gemini CLI instances.

*   **Test Case 1.1: Default Behavior**
    *   **Action:** Run `launchpad` without specifying `--gemini-instances`.
    *   **Expected Result:** Exactly one Gemini CLI instance is launched.
*   **Test Case 1.2: Multiple Instances**
    *   **Action:** Run `launchpad --gemini-instances 3`.
    *   **Expected Result:** Exactly three Gemini CLI instances are launched.
*   **Test Case 1.3: Large Number of Instances (Optional, for performance testing)**
    *   **Action:** Run `launchpad --gemini-instances 10` (or a higher number, depending on system resources).
    *   **Expected Result:** Ten Gemini CLI instances are launched, and the system remains stable.

### 2. `--record-session` Argument Verification

**Objective:** To verify that `launchpad` correctly initiates an `asciinema` recording when specified.

*   **Test Case 2.1: Session Recording Enabled**
    *   **Action:** Run `launchpad --record-session`.
    *   **Expected Result:** An `asciinema` recording process is initiated, and a recording file is created (e.g., in the current directory or a designated recording directory). The recording should capture the session's output.
*   **Test Case 2.2: Session Recording Disabled (Default)**
    *   **Action:** Run `launchpad` without specifying `--record-session`.
    *   **Expected Result:** No `asciinema` recording process is initiated.

### 3. `--background-detached` Argument Verification

**Objective:** To confirm that `launchpad` can launch Gemini CLI instances in a detached background process.

*   **Test Case 3.1: Detached Background Process**
    *   **Action:** Run `launchpad --background-detached`.
    *   **Expected Result:** The `launchpad` command exits immediately, but the Gemini CLI instance(s) continue to run in the background. Verify by checking running processes (e.g., `ps aux | grep gemini`).
*   **Test Case 3.2: Foreground Process (Default)**
    *   **Action:** Run `launchpad` without specifying `--background-detached`.
    *   **Expected Result:** The `launchpad` command remains active until the launched Gemini CLI instance(s) complete or are terminated manually.

### 4. Interaction with `miniact` Simulation

**Objective:** To ensure that the default values for the new arguments are correctly applied during `miniact` simulation.

*   **Test Case 4.1: `miniact` Default Behavior**
    *   **Action:** Run `launchpad` in a `miniact` simulation mode (assuming such a mode exists and can be triggered).
    *   **Expected Result:** The `miniact` simulation should behave as if `--gemini-instances 1`, `--record-session false`, and `--background-detached false` were implicitly set.

## General Verification Steps

*   **Error Handling:** Test with invalid argument values (e.g., `--gemini-instances 0`, non-boolean values for flags) to ensure appropriate error messages are displayed.
*   **Resource Usage:** Monitor CPU and memory usage when launching multiple instances to identify potential performance bottlenecks.
*   **Log Verification:** Check `launchpad`'s logs for any relevant messages or errors related to the new arguments.
