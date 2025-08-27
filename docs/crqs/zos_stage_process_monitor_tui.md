## Change Request: `zos-stage-process-monitor` TUI for `tmux` Integration

**Title:** Implement `zos-stage-process-monitor` TUI for `tmux` Integration

**Description:**
This change proposes the creation of a new Rust binary, `zos-stage-process-monitor`, designed to provide a Terminal User Interface (TUI) for monitoring processes within active `tmux` sessions. This TUI will enable both AI agents (like Gemini) and human users to observe and potentially interact with running processes, enhancing system observability and facilitating collaborative control.

**Problem Statement:**
As the `libminizinc` project evolves towards a multi-agent, self-aware system, the ability to monitor and understand the state of running processes across various `tmux` sessions is crucial. Current methods of process inspection (e.g., direct `ps` commands) lack a unified, interactive interface suitable for both AI and human interaction, hindering fine-grained control and collaborative debugging.

**Proposed Solution:**
Develop `zos-stage-process-monitor` as a dedicated Rust subcrate that:
1.  Utilizes the `tmux_interface` crate to discover and interact with existing `tmux` sessions, windows, and panes.
2.  Executes standard process monitoring commands (e.g., `ps aux`) within selected `tmux` panes.
3.  Presents the gathered process information in an interactive TUI, allowing users (human or AI) to view, filter, and potentially select processes.
4.  Aims to reuse existing Rust TUI libraries for efficient development and adherence to established patterns.

**Benefits:**
*   **Enhanced System Observability:** Provides a centralized, interactive view of all processes running within `tmux`.
*   **AI-Human Collaboration:** Offers a common interface for both AI and human users to monitor and understand system state.
*   **Foundation for Control:** Lays the groundwork for future features allowing AI to interact with and manage specific processes.
*   **Modularity:** Separates process monitoring logic into a dedicated, reusable component.
*   **Adherence to Standards:** Follows modular design principles (C4 model) and leverages existing Rust ecosystem tools.

**Scope:**
*   **In-Scope:**
    *   Creation of the `crates/zos-stage-process-monitor` subcrate.
    *   Integration with `tmux_interface` to list `tmux` sessions, windows, and panes.
    *   Execution of `ps` commands within a specified `tmux` pane and capture of its output.
    *   Implementation of a basic TUI to display `ps` output.
    *   Identification and integration of suitable existing Rust TUI libraries (e.g., `ratatui`, `crossterm`).
*   **Out-of-Scope (for this CRQ, but planned for future phases):**
    *   Advanced process control (e.g., killing processes, sending signals).
    *   Complex filtering or sorting within the TUI beyond basic capabilities.
    *   Integration with the IAM solver engine for process-level access control (this will be a subsequent CRQ).
    *   Integration with `Volt` or other Node.js specific process monitoring.

**Impact:**
*   **Positive:** Significantly improves the ability to monitor and understand the system's runtime state.
*   **Neutral:** No direct impact on existing functionality.
*   **Negative:** Requires new development effort and introduces a new dependency.

**Pre-requisites:**
*   `tmux` must be installed and running on the system.
*   Familiarity with Rust TUI development.

**Implementation Plan:**

1.  **Create `crates/zos-stage-process-monitor` Subcrate (Completed):**
    *   The directory `crates/zos-stage-process-monitor` has been created.
    *   `crates/zos-stage-process-monitor/Cargo.toml` has been created with `tmux_interface`, `ratatui`, and `crossterm` dependencies.
    *   The subcrate has been added to the root `Cargo.toml` members.

2.  **Initialize TUI Framework (Completed - Skeleton):**
    *   The basic TUI structure has been set up in `crates/zos-stage-process-monitor/src/main.rs` using `ratatui` and `crossterm`.
    *   The `main.rs` currently lists `tmux` sessions and panes.

3.  **Integrate `tmux_interface` (In Progress):**
    *   The `tmux_interface` is initialized in `main.rs` and currently lists available `tmux` sessions and panes.
    *   Future work will involve allowing selection of a target pane for process monitoring.

4.  **Execute `ps` Command (To Do):**
    *   This step will involve using `tmux_interface::send_keys` to send the `ps aux` command to a selected `tmux` pane and capturing its output.

5.  **Display in TUI (To Do):**
    *   This step will involve parsing the `ps` output and displaying it in a readable format within the TUI, along with implementing basic TUI navigation.

**Verification Plan:**
1.  **Build and Run:** Successfully build and run `zos-stage-process-monitor` via `launchpad`.
2.  **`tmux` Session Detection:** Verify that the TUI correctly lists active `tmux` sessions.
3.  **Process Display:** Confirm that `ps` output is correctly captured and displayed within the TUI for selected panes.
4.  **Basic Interaction:** Test basic TUI navigation (if implemented).

**Rollback Plan:**
*   This change introduces a new, independent component. Rollback involves removing the `crates/zos-stage-process-monitor` directory and its entry from the root `Cargo.toml`.

**Approvers:**
*   [Relevant Stakeholders/Team Leads]
