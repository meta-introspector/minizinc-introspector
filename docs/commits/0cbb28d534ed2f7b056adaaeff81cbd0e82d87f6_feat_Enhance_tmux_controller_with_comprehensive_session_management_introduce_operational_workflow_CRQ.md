feat: Enhance tmux_controller with comprehensive session management; introduce operational workflow CRQ

This foundational commit significantly enhances the `tmux_controller` crate, providing comprehensive programmatic control over tmux sessions and panes.

Key changes include:
- Introduction of new `tmux_controller` commands for creating, listing, killing, selecting, showing, sending commands to, splitting, and capturing output from tmux sessions.
- Integration of `uuid` dependency for unique session/pane identification.
- Creation of `change_request_operational_workflow.md`, outlining a robust and reproducible development workflow for Gemini CLI interactions, leveraging `asciinema` and `act run`.
- Updates to `docs/cheatsheets/tmux_noob_cheatsheet.md` and `docs/sops/gemini_tmux_crq_sop.md` to document the new `tmux_controller` commands and their usage.

This commit marks a significant step towards automated and auditable Gemini CLI development workflows.