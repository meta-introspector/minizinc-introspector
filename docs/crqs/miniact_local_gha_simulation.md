## Change Request (CRQ): MiniAct Capability: Local Simulation of GitHub Action Workflow Execution

**Title:** MiniAct Capability: Local Simulation of GitHub Action Workflow Execution

**Description:**
Develop a new capability within MiniAct and the `launchpad` tool to simulate the execution of a specified GitHub Action workflow locally. This will allow developers to test and iterate on GitHub Action workflows and the underlying commands they execute without requiring a full GitHub Actions runner environment or pushing changes to a remote repository. The command will be structured as `launchpad --miniact --task <workflow_file.yml>`.

**Justification/Motivation:**
*   **Accelerated Development:** Enable rapid local testing and debugging of GitHub Action workflows and their associated scripts, significantly reducing development cycles.
*   **Offline Development:** Allow developers to work on CI/CD pipelines and automated tasks without constant internet connectivity or reliance on GitHub's infrastructure.
*   **Resource Optimization:** Reduce consumption of GitHub Actions minutes and resources by shifting testing to local environments.
*   **Enhanced Control:** Provide developers with direct control over the execution environment during workflow simulation.

**Scope:**
*   **In-Scope:**
    *   MiniAct's ability to parse the `launchpad --miniact --task <workflow_file.yml>` command.
    *   MiniAct's ability to read and interpret the specified GitHub Action workflow file (`.yml`).
    *   Extraction of the `run` commands from the workflow's `steps` section.
    *   Local execution of these extracted `run` commands within the MiniAct environment.
    *   Basic handling of `inputs` defined in `workflow_dispatch` (e.g., passing them as environment variables or command-line arguments to the executed scripts).
*   **Out-of-Scope (Initial Phase):**
    *   Full emulation of GitHub Actions runner environment (e.g., `uses` actions, complex matrix strategies, service containers).
    *   Integration with GitHub API for status updates or checks.
    *   Handling of secrets or complex environment variable injection beyond simple inputs.
    *   Support for all GitHub Actions features (focus on `run` steps).

**High-Level Plan/Approach:**
1.  **MiniAct Command Parsing:**
    *   Extend `crates/mini-act/src/main.rs` to add a new subcommand or argument to `clap` for `launchpad --miniact --task <workflow_file.yml>`.
    *   Leverage `crates/mini-act/src/gemini_context_args.rs` as a reference for defining new command-line arguments.
2.  **Workflow File Parsing:**
    *   Utilize the existing `serde_yaml` dependency (already in `crates/mini-act/Cargo.toml`) to parse the `.yml` workflow file.
    *   Adapt the `crates/mini-act/src/workflow.rs` structs (`Workflow`, `Job`, `Step`) to fully capture the necessary GitHub Actions workflow syntax, especially `on:workflow_dispatch:inputs`.
3.  **Step Extraction and Execution:**
    *   Enhance `crates/mini-act/src/runner.rs` to iterate through the parsed workflow's jobs and steps.
    *   For each `run` step, execute the command using `std::process::Command`, similar to how `runner.rs` currently handles `run` commands.
4.  **Input Handling:**
    *   Develop a mechanism to read `inputs` from the `workflow_dispatch` section of the parsed workflow.
    *   Map these inputs to environment variables or command-line arguments that will be available to the executed `run` steps.
5.  **Environment Simulation (Basic):**
    *   Set up a minimal environment for the executed commands, including basic GitHub Actions runner variables (e.g., `GITHUB_WORKSPACE`).

**Dependencies/Prerequisites:**
*   Existing MiniAct framework (`crates/mini-act`).
*   Rust `serde_yaml` crate (already a dependency).
*   Familiarity with GitHub Actions workflow syntax and execution model.

**Success Criteria:**
*   MiniAct successfully parses and executes a simple GitHub Action workflow file locally.
*   The output of the local execution matches the expected output of the `run` steps in the workflow.
*   Inputs provided to the `workflow_dispatch` are correctly passed to the executed commands.