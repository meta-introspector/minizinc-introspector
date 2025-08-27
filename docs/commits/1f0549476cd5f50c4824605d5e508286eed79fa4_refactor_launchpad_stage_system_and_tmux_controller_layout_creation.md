refactor: launchpad stage system and tmux_controller layout creation

This commit refactors the `launchpad` application to utilize a modular `Stage` trait and a dynamic stage registry, replacing the previous hardcoded `match` statement for stage dispatch.

Key changes include:
- Introduced `TmuxStage` for direct `tmux` command execution and `TmuxControllerCmdStage` for orchestrating `tmux_controller` subcommands via `launchpad`.
- Added a new `create-layout` command to `tmux_controller`, enabling the automated creation of a predefined `tmux` pane layout (vertical split, 70/30 proportion, with aggressive pane killing for a clean slate).
- Documented the refactoring process and the new `create-layout` functionality in `docs/narratives/launchpad_refactoring_narrative.md`.
- Updated `crq_launchpad_workflow_enhancements.md` to reflect these changes.

This refactoring improves modularity, extensibility, and maintainability of the `launchpad` application, aligning with the project's principles of structured and composable development.