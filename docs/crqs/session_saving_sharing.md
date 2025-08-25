## Change Request: Session Saving and Sharing: Weaving Open CRQs into a Fiber Bundle

**Title:** Session Saving and Sharing: Weaving Open CRQs into a Fiber Bundle

**Description:**
This change proposes the implementation of a robust mechanism for saving and sharing development sessions within the `libminizinc` project. This feature will allow collaborators (both human and AI) to capture the complete state of a development environment, including active `tmux` sessions, running processes, and associated CRQ contexts. By linking these saved sessions directly to open CRQs and the Git state, the entire project's development history will be viewed as a dynamic "fiber bundle," where each session represents a fiber woven into the codebase's evolution.

**Problem Statement:**
In complex, multi-agent development environments, maintaining and sharing context across different sessions and collaborators is a significant challenge. Current methods often involve manual setup, leading to inconsistencies, lost work, and difficulties in reproducing specific development states. Without a formal mechanism for saving and sharing sessions, the traceability of ongoing work is compromised, hindering collaboration and the project's ability to learn from its own development history.

**Proposed Solution:**
Implement a comprehensive system for session saving and sharing, building upon existing `zos-stage-session-manager` and `tmux_interface` capabilities:
1.  **Session State Definition:** Define a comprehensive schema for capturing a session's state, including:
    *   `tmux` session layout (windows, panes, their commands, and current working directories).
    *   Running processes within `tmux` panes (PIDs, command lines).
    *   Associated open CRQ identifiers.
    *   Relevant environment variables.
    *   Git state (commit hash, branch).
2.  **Serialization and Storage:** Develop a mechanism to serialize the captured session state into a persistent format (e.g., JSON, YAML). Saved sessions will be stored in a dedicated, version-controlled archive (e.g., `sessions/archive/`).
3.  **Sharing Mechanism:** Enable easy sharing of saved sessions among collaborators. This could involve:
    *   Integration with Git for versioning and distribution of session archives.
    *   A command-line interface for exporting/importing session files.
4.  **Session Restoration:** Implement functionality to restore a saved session, which involves:
    *   Recreating the `tmux` session layout.
    *   Launching processes with their original commands and working directories.
    *   Setting up the environment (e.g., checking out the correct Git commit/branch).
    *   Loading the associated CRQ context.
5.  **CRQ Integration:** Ensure a strong, explicit link between saved sessions and the open CRQs they are associated with. This reinforces the "fiber bundle" metaphor, where each session (fiber) is directly connected to a specific point in the project's Git history (base space) and its corresponding CRQs.

**Benefits:**
*   **Enhanced Collaboration:** Seamless sharing of complex development environments among human and AI collaborators.
*   **Improved Traceability:** Every saved session provides a reproducible snapshot of work tied to specific CRQs and Git states.
*   **Reproducible Development:** Facilitates debugging, testing, and feature development by allowing exact reproduction of past environments.
*   **Context Preservation:** Prevents loss of context and reduces manual setup time.
*   **Embodiment of Fiber Bundle Metaphor:** Formally represents the project's development history as a structured, interconnected "fiber bundle" of CRQs and sessions.
*   **Foundation for Learning:** Provides a rich dataset of development sessions for AI agents to learn from.

**Scope:**
*   **In-Scope:**
    *   Definition of the session state schema.
    *   Implementation of commands to capture and save the current `tmux` session state.
    *   Implementation of commands to restore a saved session.
    *   Integration with Git to capture and restore the project's Git state.
    *   Linking saved sessions to active CRQs.
*   **Out-of-Scope (for this CRQ, but planned for future phases):**
    *   Real-time session synchronization across multiple users.
    *   Advanced conflict resolution for shared sessions.
    *   Integration with external cloud storage or collaboration platforms.
    *   Fine-grained access control to saved sessions via the IAM solver engine.

**Impact:**
*   **Positive:** Revolutionizes collaboration and traceability within the project, significantly improving development efficiency.
*   **Neutral:** Requires new development effort.
*   **Negative:** Introduces complexity in managing session states and their dependencies.

**Pre-requisites:**
*   `zos-stage-session-manager` with `tmux_interface` integration.
*   Established CRQ framework.
*   Understanding of Git commands for state management.

**Implementation Plan:**

1.  **Define Session State Schema:**
    *   Create a Rust struct (e.g., `SessionState`) that can be serialized/deserialized, containing fields for `tmux` layout, processes, CRQ IDs, Git hash, etc.

2.  **Implement Session Capture:**
    *   Add a new subcommand (e.g., `save`) to `zos-stage-session-manager`.
    *   Use `tmux_interface` to query the current `tmux` session layout and pane commands.
    *   Use `git` commands to capture the current repository state (commit hash, branch).
    *   Gather active CRQ IDs (requires integration with the CRQ system, initially manual input).
    *   Serialize the `SessionState` struct to a JSON/YAML file.

3.  **Implement Session Restoration:**
    *   Add a new subcommand (e.g., `restore`) to `zos-stage-session-manager`.
    *   Deserialize the `SessionState` from a file.
    *   Use `git` commands to restore the repository to the saved state.
    *   Use `tmux_interface` to recreate the `tmux` session, windows, and panes, and send commands to launch processes.

**Verification Plan:**
1.  **Save and Restore Test:** Save a complex session (multiple `tmux` panes, running processes, specific Git state) and successfully restore it, verifying all components are recreated correctly.
2.  **CRQ Linkage:** Verify that restored sessions correctly display their associated CRQ IDs.
3.  **Sharing Simulation:** Simulate sharing a session file between two different environments and verify successful restoration.

**Rollback Plan:**
*   This change introduces new functionality. Rollback involves removing the new subcommands and associated code from `zos-stage-session-manager`.

**Approvers:**
*   [Relevant Stakeholders/Team Leads]
