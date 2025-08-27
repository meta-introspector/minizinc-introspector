feat: Integrate gemini_utils into mini-act; initiate launchpad workflow enhancements

This foundational commit integrates `gemini_utils` and `kantspel_macros` into the `mini-act` project, enabling advanced logging and context management capabilities.

Concurrently, it initiates a major roadmap for `launchpad` by creating `crq_launchpad_workflow_enhancements.md`, which outlines a comprehensive plan for enhancing Gemini CLI management and integrated development workflows.

Extensive refactoring and new feature development within `launchpad` are also included to support the outlined workflow enhancements, such as:
- Refactoring `launchpad_main.rs` to use `clap` and integrate `gemini_cli_runner`.
- Creation of `gemini_cli_runner.rs` for Gemini CLI execution via `dum_lib`.
- Documentation and test stubs for `dum_wrappers` functions.
- Creation of `gemini_cli_options.rs` for Gemini CLI argument definitions.
- Comprehensive documentation of `narrator` functions.
- Significant refactoring of `orchestrator.rs` to include new command execution functions (`run_dum_task`, `run_cargo_command`, `run_tmux_command`, `run_sandboxed_command`).
- Introduction of the `stages` module for modular workflow execution.

This commit marks a pivotal moment in the project's evolution, laying the groundwork for sophisticated self-management and orchestration tools.