## CRQ: Enhance `launchpad` for Comprehensive Gemini CLI Management and Integrated Development Workflow

**Title:** Enhance `launchpad` for comprehensive Gemini CLI management and integrated development workflow.

**Description:**
This CRQ aims to extend the `launchpad` tool's capabilities to provide a robust and integrated environment for managing the Gemini CLI and facilitating a streamlined development workflow. Building upon the initial vendorization of `dum` and the basic `gemini` execution, this phase will focus on:

1.  **Complete Gemini CLI Management:**
    *   **Installation Verification:** Ensure the `install-gemini` stage reliably installs the Gemini CLI.
    *   **Option Parsing:** Implement robust parsing of command-line arguments into `GeminiCliOptions` within the `run-gemini` stage, allowing full control over Gemini CLI execution.
    *   **Enhanced Gemini CLI Control:** Implemented new CLI arguments (`--gemini-instances`, `--record-session`, `--background-detached`) in `launchpad_main.rs` to enable launching multiple Gemini instances, recording sessions with asciinema, and running Gemini in detached background processes.

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
*   `crates/launchpad/` (especially `src/launchpad_main.rs` - updated with new CLI arguments for Gemini control, `src/orchestrator.rs`, `src/narrator.rs`, `src/gemini_cli_options.rs`, `src/dum_wrappers/gemini_cli_runner.rs`)
*   `zos-bootstrap-main/` (potential integration points for `launchpad` as a sub-tool)

**Verification:**
*   Unit tests for all new and modified functions.
*   Integration tests to verify end-to-end workflows (e.g., installing Gemini CLI, running a sandboxed command, launching a tmux session).
*   Manual testing of all new `launchpad` stages.

**Dependencies:**
*   Completion of current `launchpad` work (documentation and initial `dum` integration).

## Commit History

**Commit:** `3290a8aa406d8d4352c2702bbafbcd10d2458b52`
**Subject:** `wip`
**Description:**
This commit represents a work-in-progress state, notably including the initial creation of this `crq_launchpad_workflow_enhancements.md` file. It also introduces significant changes across various modules, such as updates to `gemini_utils` for `kantspel` principles and `gemini_eprintln!` macro enhancements, refactoring within `launchpad`'s `dum_wrappers` for Gemini CLI integration, and the introduction of `gemini_cli_options.rs` and `gemini_context_args.rs` for command-line argument parsing.

**Commit:** `23104bac1cf99fa82e998471ac1f929724700122`
**Subject:** `feat: launchpad and tmux_controller CLI enhancements, documentation, and QA file creation`
**Description:**
This commit introduces significant enhancements to the `launchpad` and `tmux_controller` CLIs, focusing on improved Gemini CLI management and integrated development workflows. Key changes include:
-   **`launchpad` CLI Enhancements:** Added new CLI arguments (`--gemini-instances`, `--record-session`, `--background-detached`) to `launchpad_main.rs` for launching multiple Gemini instances, recording sessions with asciinema, and running Gemini in detached background processes.
-   **`tmux_controller` Integration:** Implemented `tmux` integration within `launchpad` to manage `tmux` sessions for various operations.
-   **Documentation:** Updated relevant documentation to reflect the new CLI arguments and functionalities.
-   **QA File Creation:** Created new QA files to ensure the proper functioning and verification of the new features.

**Commit:** `[PLACEHOLDER_HASH]`
**Subject:** `refactor: launchpad stage system and tmux_controller layout creation`
**Description:**
This commit refactors the `launchpad` application to utilize a modular `Stage` trait and a dynamic stage registry, replacing the previous hardcoded `match` statement for stage dispatch. It introduces `TmuxStage` for direct `tmux` command execution and `TmuxControllerCmdStage` for orchestrating `tmux_controller` subcommands via `launchpad`. Furthermore, a new `create-layout` command has been added to `tmux_controller`, enabling the automated creation of a predefined `tmux` pane layout (vertical split, 70/30 proportion, with aggressive pane killing for a clean slate). This demonstrates enhanced `tmux` orchestration capabilities through `launchpad` and `tmux_controller`, including splitting, sending commands, and creating custom layouts.
