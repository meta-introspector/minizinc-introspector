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
