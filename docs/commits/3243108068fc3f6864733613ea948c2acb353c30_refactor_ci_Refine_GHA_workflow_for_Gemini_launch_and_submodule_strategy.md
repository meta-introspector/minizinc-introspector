refactor(ci): Refine GHA workflow for Gemini launch and submodule strategy

This commit refines the GitHub Actions workflow for launching Gemini, shifting the orchestration responsibility from MiniAct directly to Launchpad.

Key changes include:
- Updated `.github/workflows/launch_gemini_broadcast_crq.yml` to directly invoke `launchpad` for Gemini CLI execution, simplifying the CI workflow.
- Refinements in `mini-act/src/runner/job_runner.rs` for environment variable substitution and command execution within MiniAct.
- Modified `mini-act/tests/workflows/test_workflow_gemini.yml` to simulate `gemini-cli` execution, focusing testing on the orchestration logic.
- Re-added the `gemini-cli` submodule, indicating a revised strategy for its management within the repository.

This work contributes to a more streamlined and robust CI/CD pipeline for Gemini CLI development.