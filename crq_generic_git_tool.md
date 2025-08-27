# Change Request: Generic Git and CRQ Management Tool

## 1. Purpose
This Change Request (CRQ) proposes the development of a generic tool to automate and standardize various aspects of Git repository management, CRQ (Change Request) status tracking, and system auditing. The primary goal is to enhance development workflow efficiency, ensure consistency in Git practices, improve traceability of changes, and provide comprehensive auditing capabilities across code, processes, and system interactions.

## 2. Scope
The tool will encompass functionalities related to:
*   Git branch management
*   CRQ status tracking and automated merging
*   Relating all code changes to specific CRQs
*   Standardized naming conventions for Git branches
*   Auditing of process changes
*   Auditing of file system (FS) changes
*   Integration with network diagnostics (tcpdump, strace, mitmproxy, routing) for comprehensive system auditing.
*   **Flexible and parameterized execution of arbitrary commands within managed environments (e.g., tmux panes).**
*   **Parameterized and composable launch of specific tools like Gemini CLI.**

## 3. Key Features / Requirements

### 3.1. Git Branch and CRQ Management
*   **Automated Branch Management:** Tools for creating, managing, and deleting branches based on predefined rules.
*   **CRQ Status Integration:** Link Git branches and commits directly to CRQ statuses, enabling automated updates of CRQ states based on Git operations (e.g., merging a branch updates CRQ to "Closed").
*   **Automated Merging:** Implement logic for automated merging of branches based on CRQ status and predefined criteria.
*   **CRQ Relationship Mapping:** Ensure all code changes (commits) are explicitly linked to one or more CRQs for full traceability.

### 3.2. Naming Conventions
*   **Branch Naming Enforcement:** Enforce standardized naming conventions for Git branches (e.g., `feature/CRQ-XYZ-short-description`, `bugfix/CRQ-ABC-fix-summary`).

### 3.3. Auditing and Traceability
*   **Process Change Auditing:** Monitor and log changes to development processes and workflows.
*   **File System Change Auditing:** Track and log modifications to the file system, especially within the project repository.
*   **Comprehensive System Auditing:** Integrate with system-level tools for deeper auditing:
    *   **TCP Dump Integration:** Capture and analyze network traffic related to development activities.
    *   **Strace Integration:** Trace system calls and signals of processes to understand their behavior.
    *   **MITM Proxy Integration:** Intercept and inspect network communications for security and debugging.
    *   **Routing Information:** Monitor and log network routing changes.

### 3.4. Flexible Execution and Parameterization
*   **Arbitrary Command Execution:** Ability to execute any specified shell command within a managed environment (e.g., a dedicated tmux pane), allowing for observation and interaction.
*   **Parameterized Tool Launch:** Provide comprehensive parameterization for launching specific tools, such as the Gemini CLI. This includes:
    *   Specifying the target project.
    *   Associating with a specific CRQ.
    *   Defining the Git branch context.
    *   Configuring storage locations for recordings and logs.
    *   Managing access permissions and credentials.
    *   Integration with `miniact` for workflow orchestration.
*   **Composability:** Design the tool with a focus on modularity and composability, allowing different functionalities and parameters to be combined flexibly.
*   **Gemini as Orchestrator:** The tool should support Gemini CLI acting as an orchestrator for other tools (e.g., `gitcrq`/`crq_updater`), enabling complex workflows and leveraging Gemini's capabilities for decision-making and dynamic execution.

## 4. Potential Technologies / Approach
*   **Core Language:** Rust (for performance, safety, and system-level access).
*   **Git Integration:** `git2` crate for interacting with Git repositories.
*   **CRQ Management:** Integration with existing CRQ tracking systems or a new internal CRQ data model.
*   **Auditing:** Utilize Rust's capabilities for system-level programming, potentially interacting with `auditd` or similar Linux auditing frameworks, and wrapping command-line tools like `tcpdump`, `strace`.
*   **Networking:** Rust's networking libraries for `mitmproxy`-like functionality.
*   **Tmux Integration:** Leverage `tmux_controller` and `tmux_interface` for session and pane management, including sending commands to specific panes.
*   **Inter-Agent Communication:** Leverage the proposed Virtual P2P Communication and Command System (`crq_p2p_comms_system.md`) for robust and flexible inter-agent communication and command delivery.
*   **Dynamic Stage Loading:** Leverage the proposed Launchpad Dynamic Stage Loading from File (`crq_launchpad_dynamic_stages.md`) for flexible and data-driven workflow orchestration.

## 5. Relationship to Existing CRQs/SOPs
This tool will significantly impact and potentially supersede parts of existing manual procedures related to Git workflow, CRQ updates, and auditing. It will aim to integrate with and enhance the "Commit Labeling and CRQ Ownership Procedure" SOP and the `crq_updater` tool, evolving them into a more automated and robust system.

## 6. Next Steps
*   Detailed design and architecture of the proposed tool.
*   Feasibility study for integrating various auditing components.
*   Prioritization of features.








## Commit History

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