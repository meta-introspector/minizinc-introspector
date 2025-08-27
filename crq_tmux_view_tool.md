# Change Request: Tmux View Tool for Session State Capture and Review

## 1. Purpose
This Change Request (CRQ) proposes the development of a "Tmux View" tool designed to capture and present the complete state of active `tmux` sessions. The primary goal is to provide developers with a comprehensive overview of their `tmux` environment, facilitating debugging, understanding complex multi-pane workflows, and enabling historical analysis of session states.

## 2. Scope
This CRQ focuses on:
*   Capturing the textual content of all active `tmux` panes.
*   Capturing relevant `tmux` session, window, and pane metadata (e.g., names, IDs, dimensions, layout).
*   Presenting this captured information in a structured and easily reviewable format.

## 3. Key Features / Requirements

### 3.1. Comprehensive Session State Capture
*   **Pane Content:** Capture the full textual output and current display of all panes across all active `tmux` sessions.
*   **Metadata Capture:** Include `tmux` session names, window names, pane IDs, dimensions (width, height), and layout information.
*   **Timestamping:** Each capture should be timestamped for historical analysis.

### 3.2. Structured Data Representation
*   **Format:** Store captured session states in a structured, machine-readable format (e.g., JSON or YAML) to enable programmatic analysis.
*   **Schema:** Define a clear schema for the captured data, including fields for session, window, and pane details, along with their respective content.

### 3.3. Review and Analysis Capabilities
*   **Display:** A simple command-line interface to display the captured session state.
*   **Search:** Ability to search within the captured pane content.
*   **Summarization (Future):** Potential integration with LLMs to summarize complex session states or identify anomalies.
*   **Diffing (Future):** Ability to compare two captured session states to highlight changes.

### 3.4. Integration and Naming
*   **Tool Name:** The tool will be named "tmux view" or a similar intuitive name.
*   **Integration:** Designed to integrate with existing `tmux_controller` capabilities for capturing output.

## 4. Potential Technologies / Approach
*   **Core Language:** Rust.
*   **`tmux_controller`:** Leverage `tmux_controller capture-session-output` as the primary mechanism for data collection.
*   **Serialization/Deserialization:** `serde` with `serde_json` or `serde_yaml` for data storage.
*   **Filesystem:** Standard Rust `std::fs` for managing captured state files.

## 5. Relationship to Existing CRQs/SOPs
This CRQ builds upon:
*   "Tmux Watch Workflow for Real-time Monitoring and Logging" (`crq_tmux_watch_workflow.md`): Provides the foundation for streaming and logging individual command outputs, which can be aggregated by "tmux view".
*   "Virtual P2P Communication and Command System" (`crq_p2p_comms_system.md`): Could potentially be used for sharing captured session states between agents.
*   "Generic Git and CRQ Management Tool" (`crq_generic_git_tool.md`): "tmux view" could be a diagnostic or monitoring component of this larger tool.

## 6. Next Steps
*   Define the precise schema for the captured `tmux` session state.
*   Implement the capture logic using `tmux_controller`.
*   Develop basic display and search functionalities.
