feat(launchpad): Integrate Gemini CLI launch and argument parsing
This commit integrates the Gemini CLI launch functionality into the `launchpad` tool.

Key changes include:
- Refactored `crates/launchpad/src/launchpad_main.rs` to use `clap::Parser` for argument parsing, enabling a more robust command-line interface.
- Enhanced `crates/launchpad/src/gemini_cli_options.rs` to define and parse Gemini CLI specific arguments, including `ApprovalMode` and `TelemetryTarget` enums.
- Modified `crates/launchpad/src/orchestrator.rs` to delegate `run_gemini_cli` to `zos-stage-session-manager`, passing all relevant Gemini CLI arguments.
- Updated `.github/workflows/launch_gemini_broadcast_crq.yml` to reflect the new `launchpad` invocation.
- Added `crates/zos-stage-doh` as a new workspace member.
- Updated `crates/zos-stage-session-manager/src/main.rs` to mirror `LaunchpadArgs` for its own arguments.

This work establishes a foundation for orchestrated Gemini CLI execution and addresses parts of `change_request_launch_gemini_tmux.md` and `change_request_operational_workflow.md`.

CRQ: change_request_launch_gemini_tmux.md
CRQ: change_request_operational_workflow.md