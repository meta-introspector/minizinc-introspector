feat: Implement --task argument for tmux_controller create-layout
This commit implements the `--task` argument for the `tmux_controller`'s
`create-layout` command, as outlined in `change_request_tmux_task_execution.md`.

Key changes include:
- Modified `crates/tmux_controller/src/main.rs` to accept `CreateLayoutArgs`
  for the `CreateLayout` command.
- Defined `CreateLayoutArgs` in `crates/tmux_controller/src/commands/create_layout.rs`
  to include an optional `task: Option<String>` field.
- Added logic in `handle_create_layout_command` to map the `task` name
  (currently supporting `crq-updater`) to its corresponding shell command
  and execute it in pane 1 of the newly created tmux layout.
- Corrected the `output_formatter::print_error` call to `output_formatter::print_info`
  due to the absence of a `print_error` function.

This enhancement streamlines the workflow by allowing automated task
execution upon tmux layout creation.