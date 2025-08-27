feat: Document troubleshooting for zos-bootstrap CLI and FFI linking
This commit addresses common issues encountered when building and running the `zos-bootstrap` executable, and enhances the `launchpad` and `zos-stage-session-manager` crates for improved Gemini CLI integration.

Key changes include:
- Updated `README.md` with a new "Troubleshooting Common Issues" section, detailing solutions for `libminizinc_c_wrapper.so` linking errors and `zos-bootstrap` CLI argument conflicts.
- Updated `GEMINI.md` to include new memories about these troubleshooting steps.
- Created `docs/troubleshooting/zos_bootstrap_cli_issues.md` as a comprehensive guide for these issues.
- Updated `change_request_launch_gemini_tmux.md` to reflect the implemented functionality for launching Gemini in a `tmux` session via `launchpad` and `zos-stage-session-manager`.
- Refactored `crates/zos-stage-session-manager/src/commands/launch.rs` for better modularity and readability, extracting launch logic into separate functions.
- Fixed compilation errors in `zos-stage-session-manager` and `launchpad` related to `clap` argument parsing, `chrono` usage, and `LaunchArgs` field initialization.

This improves the developer experience by providing clear guidance on common setup and runtime problems, and advances the `tmux` integration for Gemini CLI.