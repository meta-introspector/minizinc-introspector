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

**Commit:** `[PLACEHOLDER_HASH]`
**Subject:** `refactor: launchpad stage system and tmux_controller layout creation`
**Description:**
This commit refactors the `launchpad` application to utilize a modular `Stage` trait and a dynamic stage registry, replacing the previous hardcoded `match` statement for stage dispatch. It introduces `TmuxStage` for direct `tmux` command execution and `TmuxControllerCmdStage` for orchestrating `tmux_controller` subcommands via `launchpad`. Furthermore, a new `create-layout` command has been added to `tmux_controller`, enabling the automated creation of a predefined `tmux` pane layout. This layout consists of three panes: a top status pane (2 lines high), a middle Gemini pane (50% of remaining height), and a bottom work/data pane (3 lines high). This demonstrates enhanced `tmux` orchestration capabilities through `launchpad` and `tmux_controller`, including splitting, sending commands, and creating custom layouts.

**Commit:** `[PLACEHOLDER_HASH]`
**Subject:** `refactor: launchpad stage system and tmux_controller layout creation`
**Description:**
This commit refactors the `launchpad` application to utilize a modular `Stage` trait and a dynamic stage registry, replacing the previous hardcoded `match` statement for stage dispatch. It introduces `TmuxStage` for direct `tmux` command execution and `TmuxControllerCmdStage` for orchestrating `tmux_controller` subcommands via `launchpad`. Furthermore, a new `create-layout` command has been added to `tmux_controller`, enabling the automated creation of a predefined `tmux` pane layout (vertical split, 70/30 proportion, with aggressive pane killing for a clean slate). This demonstrates enhanced `tmux` orchestration capabilities through `launchpad` and `tmux_controller`, including splitting, sending commands, and creating custom layouts.

**Commit:** `23104bac1cf99fa82e998471ac1f929724700122`
**Subject:** `feat: Enhance launchpad and tmux_controller CLI with new arguments and documentation`
**Description:**
This commit introduces significant enhancements to the `launchpad` and `tmux_controller` CLI tools, improving their flexibility and control over Gemini CLI instances and tmux sessions.

Key changes include:

- **`launchpad` CLI Enhancements:**
    - Added `--gemini-instances` to specify the number of Gemini CLI instances to launch.
    - Added `--record-session` to enable asciinema recording of sessions.
    - Added `--background-detached` to run Gemini instances in detached background processes.
    - These arguments provide finer control over Gemini CLI execution and session management.

- **`tmux_controller` CLI Enhancements:**
    - The `split-horizontal` and `split-vertical` commands now accept a `--session-name` argument.
    - This allows users to specify the target tmux session for split operations, providing more precise control over window management.

- **Documentation Updates:**
    - Created `docs/cli_arguments/launchpad_cli_arguments.md` to detail all `launchpad` CLI arguments.
    - Created `docs/cli_arguments/tmux_controller_cli_arguments.md` to document `tmux_controller` commands and their arguments.
    - Updated `crq_launchpad_workflow_enhancements.md` and `change_request_launch_gemini_tmux.md` to reflect the implemented features.

- **Quality Assurance (QA) Documentation:**
    - Created `docs/qa/launchpad_cli_qa.md` with test cases for `launchpad`'s new arguments.
    - Created `docs/qa/tmux_controller_cli_qa.md` with test cases for `tmux_controller`'s enhanced split commands.

These changes collectively contribute to a more robust and user-friendly environment for managing Gemini CLI and tmux-based workflows.

**Commit:** `ca9fe7e6ded6c6458b1d61ffd990063bafedede8`
**Subject:** `wip`
**Description:**
Changes to this file in this commit:
```diff
Fdiff --git a/crq_launchpad_workflow_enhancements.md b/crq_launchpad_workflow_enhancements.md
index 1f0bfb1..1ab2b6a 100644
--- a/crq_launchpad_workflow_enhancements.md
+++ b/crq_launchpad_workflow_enhancements.md
H@@ -34,3 +34,19 @@ This CRQ aims to extend the `launchpad` tool's capabilities to provide a robust 
 
 **Dependencies:**
 *   Completion of current `launchpad` work (documentation and initial `dum` integration).
+
+## Commit History
+
+**Commit:** `3290a8aa406d8d4352c2702bbafbcd10d2458b52`
+**Subject:** `wip`
+**Description:**
+This commit represents a work-in-progress state, notably including the initial creation of this `crq_launchpad_workflow_enhancements.md` file. It also introduces significant changes across various modules, such as updates to `gemini_utils` for `kantspel` principles and `gemini_eprintln!` macro enhancements, refactoring within `launchpad`'s `dum_wrappers` for Gemini CLI integration, and the introduction of `gemini_cli_options.rs` and `gemini_context_args.rs` for command-line argument parsing.
+
+**Commit:** `23104bac1cf99fa82e998471ac1f929724700122`
+**Subject:** `feat: launchpad and tmux_controller CLI enhancements, documentation, and QA file creation`
+**Description:**
+This commit introduces significant enhancements to the `launchpad` and `tmux_controller` CLIs, focusing on improved Gemini CLI management and integrated development workflows. Key changes include:
+-   **`launchpad` CLI Enhancements:** Added new CLI arguments (`--gemini-instances`, `--record-session`, `--background-detached`) to `launchpad_main.rs` for launching multiple Gemini instances, recording sessions with asciinema, and running Gemini in detached background processes.
+-   **`tmux_controller` Integration:** Implemented `tmux` integration within `launchpad` to manage `tmux` sessions for various operations.
+-   **Documentation:** Updated relevant documentation to reflect the new CLI arguments and functionalities.
+-   **QA File Creation:** Created new QA files to ensure the proper functioning and verification of the new features.

```

**Commit:** `4c692c2827e9ca0bf9b54f56cc1d370002124060`
**Subject:** `refactor: launchpad stage system and tmux_controller layout creation`
**Description:**
This commit refactors the `launchpad` application to utilize a modular `Stage` trait and a dynamic stage registry, replacing the previous hardcoded `match` statement for stage dispatch.

Key changes include:
- Introduced `TmuxStage` for direct `tmux` command execution and `TmuxControllerCmdStage` for orchestrating `tmux_controller` subcommands via `launchpad`.
- Added a new `create-layout` command to `tmux_controller`, enabling the automated creation of a predefined `tmux` pane layout (vertical split, 70/30 proportion, with aggressive pane killing for a clean slate).
- Documented the refactoring process and the new `create-layout` functionality in `docs/narratives/launchpad_refactoring_narrative.md`.
- Updated `crq_launchpad_workflow_enhancements.md` to reflect these changes.

This refactoring improves modularity, extensibility, and maintainability of the `launchpad` application, aligning with the project's principles of structured and composable development.

**Commit:** `018333aec1ad92675a664ff64bd615ff301eb086`
**Subject:** `feat: Implement new three-pane tmux layout in tmux_controller`
**Description:**
This commit refactors the `create-layout` command in `tmux_controller` to establish a new, more functional three-pane tmux layout. The previous two-pane 70/30 split has been replaced with a layout optimized for development workflow.

The new layout consists of:
- Top pane (pane 0): Dedicated to Work/Data.
- Middle pane (pane 1): Dedicated to Gemini, with `gemini` command automatically launched.
- Bottom pane (pane 2): Dedicated to Status, with `cargo run --package launchpad_status` automatically launched.

Pane sizes are adjusted for optimal visibility: Gemini pane is 3 lines high, and Status pane is 2 lines high.

Additionally, this commit includes:
- Cleanup: Removed unused `create_layout` import in `crates/tmux_controller/src/main.rs`.
- Documentation: Updated `crq_launchpad_workflow_enhancements.md` and `docs/narratives/launchpad_refactoring_narrative.md` to reflect the new layout and its purpose.
- Dependency: `Cargo.lock` and `Cargo.toml` updated to include `launchpad_status`.

**Commit:** `ea49de080e7e34d2e3171164a8b8acaceee84af0`
**Subject:** `refactor(crq_updater): Split functions into files and update CRQs`
**Description:**
Refactored the `crq_updater` crate to adhere to the "one declaration per file" convention.
- Moved functions (`find_crq_files`, `process_crq_file`, `extract_existing_history`, `get_commit_diff_summary`, `find_commit_from_oid`) into separate files within `src/functions/`.
- Updated `src/main.rs` to use the new module structure.
- Fixed compilation errors related to `git2` API changes (`parent_opt` to `parent`, `find_object` arguments, `diff.print` arguments).
- Updated `walkdir` dependency to `2.5.0`.
- Resolved `StripPrefixError` by canonicalizing `crq_path`.
- Successfully ran `crq_updater` to update `crq_launchpad_workflow_enhancements.md` and `docs/sops/crq_updater_qa_sop.md` with relevant commit history.

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