### Change Request: Launch Gemini in Tmux Session

**Task:** Launch a new instance of the Gemini CLI in a dedicated tmux session, utilizing the 'pro' model, to serve as a trusted helper operating in parallel.

**Coordination:** This task will be coordinated with another instance of the Gemini CLI.

**Implementation Details:**

This functionality is implemented within the `launchpad` and `zos-stage-session-manager` crates. The `launchpad` orchestrates the call to `zos-stage-session-manager`, which then handles the `tmux` session creation and Gemini CLI execution.

- The `LaunchpadArgs` struct in `crates/launchpad/src/launchpad_main.rs` and `LaunchArgs` in `crates/zos-stage-session-manager/src/commands/launch.rs` have been extended to include `gemini_cli_path` and other Gemini CLI related arguments. Specifically, `--gemini-instances`, `--record-session`, and `--background-detached` have been added to `launchpad` to provide finer control over Gemini CLI instances within tmux sessions.
- The `zos-stage-session-manager` now contains logic to create a `tmux` session and launch the Gemini CLI within it, dynamically constructing the Gemini command based on the provided arguments.
- The `split-horizontal` command in `crates/tmux_controller/src/commands/split_horizontal.rs` has been enhanced with a `--session-name` argument, allowing users to specify the target tmux session for the split operation.
- The `split-vertical` command in `crates/tmux_controller/src/commands/split_vertical.rs` has also been enhanced with a `--session-name` argument, providing similar control for vertical splits.

**Usage:**

To launch a Gemini CLI instance in a `tmux` session, use the `launchpad` executable with the following arguments:

```bash
cargo run -p launchpad -- run-gemini --mode tmux --inside gemini --model pro --gemini-cli-path /data/data/com.termux/files/home/storage/github/gemini-cli
```

**Explanation of Arguments:**

*   `run-gemini`: The stage identifier for running the Gemini CLI.
*   `--mode tmux`: Specifies that Gemini should be launched within a `tmux` session.
*   `--inside gemini`: Indicates that the `tmux` session should contain a Gemini CLI instance.
*   `--model pro`: Activates the 'pro' model for the launched Gemini instance.
*   `--gemini-cli-path /data/data/com.termux/files/home/storage/github/gemini-cli`: Specifies the absolute path to the Gemini CLI executable. **(Important: Adjust this path if your Gemini CLI installation is in a different location.)**

**Purpose:** To enable parallel processing and enhanced assistance by leveraging multiple Gemini instances for complex tasks.

## Commit History

- [Commit d08222dc45dc2f3501a0c3d72651b6776c462f89: feat: Implement robust Gemini tmux control and CRQ assignment](docs/commits/d08222dc45dc2f3501a0c3d72651b6776c462f89_feat_Implement_robust_Gemini_tmux_control_and_CRQ_assignment.md)
- [Commit 23104bac1cf99fa82e998471ac1f929724700122: feat: Enhance launchpad and tmux_controller CLI with new arguments and documentation](docs/commits/23104bac1cf99fa82e998471ac1f929724700122_feat_Enhance_launchpad_and_tmux_controller_CLI_with_new_arguments_and_documentation.md)
- [Commit 19bbe4f5ee5368d5c239e894df678af8b5541c49: feat: Document troubleshooting for zos-bootstrap CLI and FFI linking](docs/commits/19bbe4f5ee5368d5c239e894df678af8b5541c49_feat_Document_troubleshooting_for_zos-bootstrap_CLI_and_FFI_linking.md)
- [Commit 20b720532c6ce3b5d66355b54e696bea554974c7: feat(launchpad): Integrate Gemini CLI launch and argument parsing](docs/commits/20b720532c6ce3b5d66355b54e696bea554974c7_feat_launchpad_Integrate_Gemini_CLI_launch_and_argument_parsing.md)
- [Commit 3243108068fc3f6864733613ea948c2acb353c30: refactor(ci): Refine GHA workflow for Gemini launch and submodule strategy](docs/commits/3243108068fc3f6864733613ea948c2acb353c30_refactor_ci_Refine_GHA_workflow_for_Gemini_launch_and_submodule_strategy.md)