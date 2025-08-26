### Change Request: Launchpad Tool for Gemini CLI Management

**Task:** Develop a Rust-based Launchpad tool to streamline the installation and execution of the Gemini CLI, incorporating `dum` for efficient task management.

**Objective:** To create a robust, reproducible, and easily manageable system for deploying and running Gemini CLI instances, facilitating development and testing workflows.

**Requirements:**
- The Launchpad tool must be written in Rust.
- It must be capable of installing the Gemini CLI (e.g., via `npm` or other distribution methods).
- It must be capable of running the Gemini CLI with specified arguments (e.g., `--model`).
- The tool should vendorize `dum` (`https://github.com/egoist/dum`) and utilize its capabilities for task orchestration and execution.
- The Launchpad tool should be designed for local testing first, followed by integration and testing within a tmux environment.

**Deliverables:**
- A new Rust crate (e.g., `crates/launchpad_tool`) containing the Launchpad functionality.
- Integration of `dum` as a vendored dependency.
- Functions for installing and running the Gemini CLI.
- Unit and integration tests for the Launchpad tool.
- Documentation for the Launchpad tool's usage and architecture.

**Testing Strategy:**
1.  **Local Testing:** Verify core functionalities (installation, execution) in a local development environment.
2.  **Tmux Integration Testing:** Test the Launchpad tool's ability to manage Gemini CLI instances within tmux sessions, ensuring compatibility and correct behavior.
