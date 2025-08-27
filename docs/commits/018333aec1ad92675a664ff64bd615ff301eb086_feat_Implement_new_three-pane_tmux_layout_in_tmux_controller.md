feat: Implement new three-pane tmux layout in tmux_controller
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