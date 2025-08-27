docs: Add Tmux Workflow and Debugging SOP; Refactor tmux_controller and dump_tmux_status
This commit introduces a new Standard Operating Procedure (SOP) for Tmux Workflow and Debugging Procedures (`docs/sops/tmux_workflow_and_debugging_sop.md`). This SOP consolidates best practices for managing tmux sessions, executing commands within panes, and debugging related issues.

Key changes and fixes include:
- Refactored `crates/tmux_controller/src/commands/create_layout.rs` to prevent automatic Gemini launch in pane 1, enabling "native worker" execution.
- Corrected module import paths in `crates/tmux_controller/src/main.rs` and `crates/tmux_controller/src/commands/mod.rs` for `tmux_view` functionality.
- Created a new `dump_tmux_status` crate (`crates/dump_tmux_status/`) for comprehensive tmux state snapshots, including session and pane content.
- Added `dump_tmux_status` to the workspace `Cargo.toml`.
- Debugged and resolved compilation errors in `dump_tmux_status` related to `clap` derives and `tmux_interface` imports.
- Updated `GEMINI.md` to reference the new Tmux Workflow and Debugging SOP.

These changes enhance our ability to manage and debug tmux-based workflows, providing better visibility and control over development environments.