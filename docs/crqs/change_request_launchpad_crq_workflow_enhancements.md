## Change Request (CRQ): Launchpad CRQ Workflow Enhancements

**Title:** Enhance Launchpad for CRQ-driven Workflow Simulation and Orchestration

**Description:**
This CRQ documents the ongoing enhancements to the `launchpad` tool, focusing on its evolution into a more robust orchestrator for Gemini CLI execution and the introduction of CRQ-driven workflow simulation. It consolidates recent architectural changes and outlines the next steps for implementing a dedicated stage for GitHub Action workflow simulation via MiniAct.

**Justification/Motivation:**
*   **Improved Orchestration:** Centralize Gemini CLI execution logic within `zos-stage-session-manager` for better modularity and control.
*   **Robust Argument Parsing:** Leverage `clap` for comprehensive and type-safe command-line argument handling in `launchpad`.
*   **CRQ-driven Automation:** Establish a framework for `launchpad` to interpret CRQs and orchestrate complex workflows, starting with GitHub Action simulation.
*   **Enhanced `tmux` Integration:** Ensure seamless launching and management of Gemini CLI within `tmux` sessions.

**Scope of Changes (Completed in this session):**

*   **`launchpad` Argument Parsing Refactoring:**
    *   `crates/launchpad/src/launchpad_main.rs`: Refactored to use `clap::Parser` for robust command-line argument parsing. Introduced `LaunchpadArgs` struct to capture all relevant arguments.
    *   `crates/launchpad/src/gemini_cli_options.rs`: Enhanced to define and parse Gemini CLI specific arguments, including `ApprovalMode` and `TelemetryTarget` enums, and implemented `from_args` for programmatic parsing.
*   **Architectural Shift for Gemini CLI Execution:**
    *   `crates/launchpad/src/orchestrator.rs`: Modified `run_gemini_cli` to delegate the execution of Gemini CLI to `zos-stage-session-manager`, passing all relevant `LaunchpadArgs`. This establishes `zos-stage-session-manager` as the central point for Gemini CLI orchestration.
    *   `crates/launchpad/src/dum_wrappers/gemini_cli_runner.rs`: Updated `run_gemini_cli` to be `async` and to accept `mode`, `inside`, and `via` arguments. (Note: The `tmux` integration logic originally planned for this module will now be implemented in `zos-stage-session-manager`).
*   **New ZOS Stage and Mirroring Arguments:**
    *   `Cargo.toml`: Added `crates/zos-stage-doh` as a new workspace member.
    *   `crates/zos-stage-session-manager/src/main.rs`: Updated to mirror `LaunchpadArgs` for its own command-line arguments, enabling it to receive and process Gemini CLI options delegated from `launchpad`.
*   **CI/CD Workflow Update:**
    *   `.github/workflows/launch_gemini_broadcast_crq.yml`: Updated to reflect the new `launchpad` invocation with `run-gemini` and `--via doh`.

**Next Steps (Future Work):**

1.  **Implement `simulate-gha-workflow` Stage in `launchpad`:**
    *   Add a new stage to `crates/launchpad/src/launchpad_main.rs` (e.g., `simulate-gha-workflow`).
    *   This stage will be responsible for:
        *   Reading a specified CRQ file (e.g., `docs/crqs/miniact_local_gha_simulation.md`).
        *   Extracting the GitHub Action workflow file path and any `inputs` from the CRQ.
        *   Cloning or fetching the target repository into a temporary sandbox directory.
        *   Invoking `zos-stage-session-manager` with arguments to launch `MiniAct` within `tmux`, passing the workflow file and inputs.
2.  **Implement `tmux` Integration in `zos-stage-session-manager`:**
    *   `zos-stage-session-manager` will be enhanced to handle:
        *   Creation and management of `tmux` sessions.
        *   Launching `MiniAct` within the `tmux` session.
        *   Passing the GitHub Action workflow file and inputs to `MiniAct`.
        *   Implementing session recording capabilities (e.g., `tmux pipe-pane`).
3.  **`MiniAct` Integration and Execution:**
    *   `MiniAct` will then parse and execute the GitHub Action workflow within the sandbox, as outlined in `docs/crqs/miniact_local_gha_simulation.md`.

**Dependencies/Prerequisites:**
*   Completion of the `tmux` integration within `zos-stage-session-manager`.
*   Further development of `MiniAct` as per `docs/crqs/miniact_local_gha_simulation.md`.

**Related CRQs:**
*   `change_request_launch_gemini_tmux.md`: This CRQ's objectives are now being primarily addressed within `zos-stage-session-manager`.
*   `change_request_operational_workflow.md`: This CRQ is being advanced by the improved orchestration capabilities.
*   `docs/crqs/miniact_local_gha_simulation.md`: This CRQ defines the target functionality for the `simulate-gha-workflow` stage.

**Success Criteria:**
*   `launchpad` can successfully initiate a CRQ-driven GitHub Action workflow simulation.
*   `zos-stage-session-manager` effectively orchestrates `tmux` sessions and launches `MiniAct` within them.
*   `MiniAct` correctly executes the simulated GitHub Action workflows.
