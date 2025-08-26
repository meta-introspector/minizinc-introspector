## CRQ: Enhance `launchpad` for Comprehensive Gemini CLI Management and Integrated Development Workflow

**Title:** Enhance `launchpad` for comprehensive Gemini CLI management and integrated development workflow.

**Description:**
This CRQ aims to extend the `launchpad` tool's capabilities to provide a robust and integrated environment for managing the Gemini CLI and facilitating a streamlined development workflow. Building upon the initial vendorization of `dum` and the basic `gemini` execution, this phase will focus on:

1.  **Complete Gemini CLI Management:**
    *   **Installation Verification:** Ensure the `install-gemini` stage reliably installs the Gemini CLI.
    *   **Option Parsing:** Implement robust parsing of command-line arguments into `GeminiCliOptions` within the `run-gemini` stage, allowing full control over Gemini CLI execution.

2.  **Integrated Development Workflow Stages:**
    *   **`cargo` Integration:** Introduce a `cargo` stage in `launchpad_main.rs` to execute `cargo` commands (e.g., `cargo build`, `cargo test`) via `orchestrator::run_cargo_command`.
    *   **`tmux` Integration:**
        *   Implement a `tmux` stage in `launchpad_main.rs` to execute `tmux` commands via `orchestrator::run_tmux_command`.
        *   Develop functionality within `launchpad` to launch and manage `tmux` sessions for various operations (e.g., running tests in a detached session, monitoring processes).
    *   **Sandboxed Execution (`pchroot`/`act-run`):** Integrate `pchroot` and `act-run` via a `sandbox` stage in `launchpad_main.rs` using `orchestrator::run_sandboxed_command`, enabling secure and isolated command execution.

3.  **Enhanced Livestreaming and Narration:**
    *   Refine the `narrator` module to provide more detailed and context-aware narration of `launchpad` operations.
    *   Explore and implement a "formatting stage" or mechanism to capture and livestream the output of executed commands, providing real-time feedback to the user.

**Branch Name:** `feature/launchpad-workflow-enhancements`

**Affected Components:**
*   `crates/launchpad/` (especially `src/launchpad_main.rs`, `src/orchestrator.rs`, `src/narrator.rs`, `src/gemini_cli_options.rs`, `src/dum_wrappers/gemini_cli_runner.rs`)
*   `zos-bootstrap-main/` (potential integration points for `launchpad` as a sub-tool)

**Verification:**
*   Unit tests for all new and modified functions.
*   Integration tests to verify end-to-end workflows (e.g., installing Gemini CLI, running a sandboxed command, launching a tmux session).
*   Manual testing of all new `launchpad` stages.

**Dependencies:**
*   Completion of current `launchpad` work (documentation and initial `dum` integration).
