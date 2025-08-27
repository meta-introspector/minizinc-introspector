# Change Request: Tmux Watch Workflow for Real-time Monitoring and Logging

## 1. Purpose
This Change Request (CRQ) proposes the implementation of a "Tmux Watch Workflow" to enable real-time monitoring and persistent logging of command execution within a split Termux environment. This workflow aims to enhance collaborative debugging, provide immediate feedback on long-running processes, and ensure comprehensive capture of command outputs for later analysis.

## 2. Scope
This CRQ focuses on:
*   Executing arbitrary commands within designated `tmux` panes.
*   Introducing a `--watch` option to the `launchpad` application to facilitate simultaneous streaming of command output to the terminal and logging to a specified file.
*   Defining and utilizing `tmux` screen layouts optimized for command observation and interaction.
*   Leveraging the concept of a "native `tmux` worker pane" for dedicated command execution.

## 3. Key Features / Requirements

### 3.1. Tmux Pane Execution and Observation
*   **Targeted Pane Execution:** Ability to direct command execution to a specific `tmux` pane (e.g., pane 0 for work/data, pane 1 for Gemini interaction).
*   **Real-time Streaming:** Command output should be streamed directly to the `tmux` pane's display for immediate visual feedback.

### 3.2. `launchpad --watch` Option
*   **Simultaneous Output:** When the `--watch` option is enabled in `launchpad`, the output of the executed command (stdout and stderr) must be simultaneously:
    *   Streamed to the `tmux` pane's display (inheriting `Stdio::inherit()`).
    *   Written to a specified log file.
*   **Log File Configuration:** The log file path should be configurable, ideally via the dynamic stage definition.

### 3.3. Screen Layout Composition
*   **Predefined Layouts:** Utilize `tmux_controller`'s `create-layout` command to establish standardized `tmux` pane layouts (e.g., three-pane layout with dedicated work, Gemini, and status panes).
*   **Workflow Integration:** Integrate the execution of commands within these predefined layouts as part of a cohesive workflow.

### 3.4. Native Tmux Worker Pane
*   **Dedicated Execution Environment:** Establish the concept of a "native `tmux` worker pane" where `launchpad` instances can be run with the `--watch` option, acting as dedicated execution and monitoring agents.
*   **Self-referential Execution (Future):** Explore the possibility of `launchpad` instances sending commands to other `launchpad` instances running in worker panes, enabling complex, distributed workflows.

## 4. Potential Technologies / Approach
*   **Core Language:** Rust.
*   **`launchpad` Modifications:** Extend `LaunchpadArgs` with a `--watch` flag. Modify `run_launchpad` to handle `Stdio::inherit()` and `fs::File::create()` for simultaneous output.
*   **`tmux_controller`:** Utilize existing `tmux_controller` commands (`send-command`, `create-layout`) to orchestrate the `tmux` environment.
*   **Filesystem:** Standard Rust `std::fs` for log file management.
*   **Inter-Agent Communication:** This workflow will heavily rely on the Virtual P2P Communication and Command System (`crq_p2p_comms_system.md`) for passing dynamic stage definitions and potentially receiving status updates.

## 5. Relationship to Existing CRQs/SOPs
This CRQ builds upon:
*   "Launchpad Dynamic Stage Loading from File" (`crq_launchpad_dynamic_stages.md`): Provides the mechanism for `launchpad` to interpret command definitions from files.
*   "Virtual P2P Communication and Command System" (`crq_p2p_comms_system.md`): Provides the underlying communication channel for sending commands and receiving status.
*   "Generic Git and CRQ Management Tool" (`crq_generic_git_tool.md`): This workflow will be a key operational mode for the proposed generic tool.
*   This workflow will provide the raw data and execution context for the "Tmux View Tool for Session State Capture and Review" (`crq_tmux_view_tool.md`).

## 6. Next Steps
*   Implement the `--watch` option in `launchpad`.
*   Develop a proof-of-concept for sending a dynamic stage with `--watch` to a `tmux` pane.
*   Refine the `tmux` layout and pane targeting mechanisms.
