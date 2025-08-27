# Change Request: Launchpad Dynamic Stage Loading from File

## 1. Purpose
This Change Request (CRQ) proposes enhancing the `launchpad` application to support dynamic loading and execution of operational stages defined in external files (e.g., YAML, TOML). The primary goal is to increase the flexibility and composability of `launchpad` workflows, enabling more complex and data-driven orchestration of tasks without requiring recompilation or hardcoding of new stages. This is a critical enabler for the Virtual P2P Communication and Command System.

## 2. Scope
This CRQ focuses on modifying `launchpad` to:
*   Read stage definitions and execution parameters from structured data files.
*   Interpret these definitions to construct and execute commands or sequences of operations.
*   Integrate seamlessly with the existing `launchpad` stage dispatch mechanism.

## 3. Key Features / Requirements

### 3.1. File-based Stage Definition
*   **Supported Formats:** Support stage definitions in YAML and/or TOML formats.
*   **Schema Definition:** Define a clear and extensible schema for stage definition files, including:
    *   `stage_name`: Identifier for the dynamic stage.
    *   `command`: The shell command or internal `launchpad`/`tmux_controller` command to execute.
    *   `args`: Arguments to pass to the command.
    *   `environment_variables`: Optional environment variables to set for the command.
    *   `working_directory`: Optional working directory for the command.
    *   `output_handling`: How to capture and process command output (e.g., log to file, capture to variable).
    *   `dependencies`: Other stages or conditions that must be met before execution.
    *   `metadata`: Arbitrary key-value pairs for additional context (e.g., CRQ ID, description).

### 3.2. Dynamic Stage Execution
*   **File Path as Stage Identifier:** Allow `launchpad` to accept a file path (e.g., `@path/to/stage.yaml`) as a `stage_identifier`.
*   **Parsing and Validation:** `launchpad` must parse the content of the specified file and validate it against the defined schema.
*   **Command Orchestration:** Dynamically construct and execute the specified commands, potentially leveraging `tmux_controller` for pane management and `orchestrator::run_command` for execution.

### 3.3. Integration with P2P Communication System
*   **Payload Interpretation:** The dynamic stage loading mechanism should be able to interpret payloads from the Virtual P2P Communication and Command System (e.g., `crq_p2p_comms_system.md`), where the payload might be a path to a dynamic stage definition file.

## 4. Potential Technologies / Approach
*   **Core Language:** Rust.
*   **Serialization/Deserialization:** `serde` with `serde_yaml` and/or `toml` crates for parsing stage definition files.
*   **Dynamic Dispatch:** Potentially use a trait object or enum-based approach to represent dynamically loaded stages.
*   **Filesystem Access:** Standard Rust `std::fs` for reading files.

## 5. Relationship to Existing CRQs/SOPs
This CRQ is a foundational component for the "Virtual P2P Communication and Command System" (`crq_p2p_comms_system.md`) and will significantly enhance the capabilities envisioned in the "Generic Git and CRQ Management Tool" (`crq_generic_git_tool.md`).
*   This CRQ will be a foundational component for the "Tmux Watch Workflow for Real-time Monitoring and Logging" (`crq_tmux_watch_workflow.md`), enabling dynamic execution within `tmux` environments.

## 6. Next Steps
*   Define the precise YAML/TOML schema for dynamic stage definition files.
*   Implement parsing and validation logic within `launchpad`.
*   Develop a proof-of-concept for a simple dynamic stage.








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