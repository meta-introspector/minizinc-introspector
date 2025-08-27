# Change Request: Implement `--task` Argument for `tmux_controller create-layout`

## Objective
To enhance the `tmux_controller create-layout` command by allowing users to specify a task to be automatically executed in a designated `tmux` pane upon layout creation, streamlining workflow for common operations like running `crq_updater`.

## Scope
*   Modify `crates/tmux_controller/src/commands/create_layout.rs` to accept an optional `--task <TASK_NAME>` argument.
*   Define a mapping or logic to associate `TASK_NAME` (e.g., `crq-updater`) with the corresponding shell command (e.g., `cargo run --package crq_updater`).
*   Execute the specified task's command in Pane 1 (the middle pane) of the newly created `tmux` layout.

## Impact
*   **Positive:** Simplifies and automates the setup of development environments for specific tasks. Reduces manual steps for users. Improves user experience by integrating common workflows directly into layout creation.
*   **Negative:** Introduces new complexity to `tmux_controller`. Requires careful handling of task execution and error reporting. Potential for increased startup time if the task is resource-intensive.

## Verification Steps
1.  Run `cargo run --package launchpad -- tmux-controller-cmd create-layout --task crq-updater`.
2.  Verify that a new `tmux` session is created with the standard three-pane layout.
3.  Verify that the `crq_updater` command (or its output) is visible and running in the middle pane (pane 1).
4.  Test with an invalid task name to ensure graceful error handling and informative messages.
5.  Verify that the `create-layout` command without the `--task` argument still functions as expected, creating an empty middle pane.

## Proposed Implementation (High-Level)
1.  **Argument Parsing:** Update the `create_layout` command's argument parsing (likely using `clap`) to include an optional `task: Option<String>` field.
2.  **Task Mapping:** Inside the `handle_create_layout_command` function, after the `tmux` panes have been created and resized, add a conditional block. If a `task` argument is provided:
    *   Implement a `match` statement or a `HashMap` to map the `task` string (e.g., "crq-updater") to its corresponding shell command (e.g., "cargo run --package crq_updater").
    *   For unknown tasks, log an error and do not attempt to execute a command.
3.  **Command Execution:** Use `tmux_interface::TmuxCommand` to `send-keys` the mapped command to pane 1 (the middle pane), followed by `C-m` (simulating the Enter key press).
4.  **Error Handling:** Ensure robust error handling for cases where the `tmux` command fails or the task mapping is invalid.
5.  **Future Considerations (Out of Scope for this CRQ):** A more advanced implementation could allow users to define custom tasks and their associated commands via a configuration file, but this is beyond the scope of this initial CRQ.
