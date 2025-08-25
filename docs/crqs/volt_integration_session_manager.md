## Change Request: `Volt` Integration into `zos-stage-session-manager`

**Title:** Integrate `Volt` for Node.js Execution Management into `zos-stage-session-manager`

**Description:**
This change proposes integrating `Volt`, a Rust-based Node.js package manager and execution tool, into the `zos-stage-session-manager`. This integration will enable the `session-manager` to directly manage Node.js project execution, including package installation and script running, providing a Rust-controlled and consistent environment for Node.js workflows.

**Problem Statement:**
As the `libminizinc` project expands to encompass diverse ecosystems, including Node.js, there is a need for a robust, Rust-native solution to manage Node.js dependencies and script execution. Relying solely on traditional Node.js tools like `npm` or `npx` introduces external dependencies and potential inconsistencies with the project's Rust-centric approach. Integrating `Volt` will align Node.js management with our Rust-first strategy and facilitate future fine-grained control and security measures.

**Proposed Solution:**
Extend the `zos-stage-session-manager` to include capabilities for managing Node.js projects using `Volt`:
1.  **`Volt` Command Execution:** Implement functionality to execute `Volt` commands (e.g., `volt install`, `volt run <script>`) from within `zos-stage-session-manager`.
2.  **Command-Line Arguments:** Introduce new command-line arguments to `zos-stage-session-manager` to specify Node.js project paths, `Volt` commands to run, and potentially specific scripts or arguments for those scripts.
3.  **Session Management:** Integrate `Volt` commands within `tmux` sessions managed by `zos-stage-session-manager`, allowing for structured execution and monitoring of Node.js processes.

**Benefits:**
*   **Rust-Native Node.js Management:** Aligns Node.js dependency and execution management with the project's Rust-first philosophy.
*   **Consistency and Control:** Provides a consistent and programmatically controllable interface for Node.js workflows.
*   **Enhanced Security (Future):** Lays the groundwork for applying IAM policies and fine-grained control to Node.js processes managed by `Volt`.
*   **Improved Performance:** Leverages `Volt`'s speed and efficiency for Node.js operations.
*   **Modularity:** Extends existing `zos-stage-session-manager` capabilities rather than creating a new, separate stage for basic Node.js management.

**Scope:**
*   **In-Scope:**
    *   Modification of `crates/zos-stage-session-manager/src/main.rs` to add `Volt`-related command-line arguments.
    *   Implementation of logic to execute `volt install` and `volt run <script>` commands within `tmux` panes.
    *   Error handling for `Volt` command execution.
*   **Out-of-Scope (for this CRQ, but planned for future phases):**
    *   Comprehensive `Volt` feature integration (e.g., `volt publish`, `volt test`).
    *   Advanced `Volt` project setup or configuration management.
    *   Deep integration with the IAM solver engine for `Volt`-managed processes (this will be a subsequent CRQ).
    *   Automatic detection or management of Node.js versions (relying on `Volt`'s internal mechanisms or external tools like `fnm`).

**Impact:**
*   **Positive:** Streamlines Node.js development workflows within the project and enhances control.
*   **Neutral:** No direct impact on existing non-Node.js functionalities.
*   **Negative:** Introduces a dependency on `Volt` and requires `Volt` to be installed on the system where `zos-stage-session-manager` is run.

**Pre-requisites:**
*   `Volt` must be installed and accessible in the system's PATH.
*   Familiarity with `Volt`'s command-line interface.

**Implementation Plan:**

1.  **Modify `LaunchArgs` in `zos-stage-session-manager/src/main.rs`:**
    *   Add optional arguments for `volt_command` (e.g., `install`, `run`), `volt_project_path`, and `volt_script_name`.

2.  **Implement `Volt` Execution Logic:**
    *   Within the `Launch` command's logic, check for the presence of `volt_command` arguments.
    *   If `volt_command` is specified, construct the appropriate `volt` command string.
    *   Launch `volt` commands within a new `tmux` pane, similar to how Gemini instances are launched.
    *   Ensure proper error handling for `Volt` command failures.

**Verification Plan:**
1.  **Build and Run:** Successfully build and run `zos-stage-session-manager` with `Volt` integration.
2.  **`Volt` Install Test:** Launch `zos-stage-session-manager` to execute `volt install` in a sample Node.js project and verify successful package installation.
3.  **`Volt` Run Test:** Launch `zos-stage-session-manager` to execute `volt run <script>` in a sample Node.js project and verify successful script execution.
4.  **Error Handling:** Test scenarios where `Volt` commands fail (e.g., invalid project path, missing script) and verify appropriate error reporting.

**Rollback Plan:**
*   This change modifies an existing component. Rollback involves reverting the changes made to `crates/zos-stage-session-manager/src/main.rs` and its `Cargo.toml` (if any new dependencies were added for `Volt` beyond what's already there for `tmux_interface`).

**Approvers:**
*   [Relevant Stakeholders/Team Leads]
