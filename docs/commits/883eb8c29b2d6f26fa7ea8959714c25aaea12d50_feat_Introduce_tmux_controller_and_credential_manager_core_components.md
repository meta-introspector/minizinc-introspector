feat: Introduce tmux_controller and credential_manager core components

This foundational commit introduces core components for programmatic tmux control and secure credential management, laying the groundwork for more sophisticated workflow orchestration and secure operations within the project.

Key changes include:
- Initial implementation of the `tmux_controller` crate, providing basic tmux session management and pane content capture capabilities, primarily for testing `zos-stage-process-monitor`.
- Initial implementation of the `credential_manager` crate, offering `store`, `retrieve`, and `list` commands for managing credentials securely via the `keyring` crate.
- Addition of the `vendor/keyring` submodule.
- Updates to `Cargo.lock` and `Cargo.toml` to integrate these new crates and their dependencies.
- Refinement of `.github/workflows/test-tmux-tui.yml` to align with the evolving testing strategy, focusing on `mini-act` workflows.

This commit represents a significant step towards building a robust and secure operational framework for the Meta-Meme.