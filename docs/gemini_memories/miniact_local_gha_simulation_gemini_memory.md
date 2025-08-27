# Gemini Memory: Implementing MiniAct Local GitHub Action Workflow Simulation

This document serves as a specialized memory for the Gemini agent when working on the "MiniAct Capability: Local Simulation of GitHub Action Workflow Execution" CRQ. It provides focused guidance and context for the implementation task.

## 1. Project Context: MiniAct Overview

MiniAct is a Rust-based tool designed to orchestrate and manage workflows, particularly those related to Gemini agent tasks. It currently handles:
*   Parsing command-line arguments using `clap`.
*   Reading and executing basic workflow definitions (though not full GHA syntax yet).
*   Executing shell commands.
*   Interacting with Git.

**Key Files in MiniAct:**
*   `crates/mini-act/src/main.rs`: Main entry point, command parsing.
*   `crates/mini-act/src/runner.rs`: Logic for running jobs and steps, executing commands.
*   `crates/mini-act/src/workflow.rs`: Data structures for parsing workflow YAML.
*   `crates/mini-act/src/gemini_context_args.rs`: Defines command-line arguments for Gemini-related tasks.
*   `crates/mini-act/Cargo.toml`: Project dependencies, including `serde_yaml`.

## 2. Task Focus: Local GHA Simulation

The primary goal is to enable MiniAct to read a GitHub Action workflow `.yml` file and execute its `run` steps locally, mimicking a GitHub Actions runner environment.

## 3. Implementation Guidance

### 3.1. Command-Line Interface (CLI) Extension

*   **Modify `crates/mini-act/src/main.rs`**:
    *   Add a new `clap` subcommand or argument structure to handle `launchpad --miniact --task <workflow_file.yml>`.
    *   Consider adding options for passing `workflow_dispatch` inputs (e.g., `--input <key>=<value>`).

### 3.2. GitHub Action Workflow Parsing

*   **Modify `crates/mini-act/src/workflow.rs`**:
    *   Extend the `Workflow`, `Job`, and `Step` structs to accurately deserialize GitHub Actions workflow YAML. Pay close attention to:
        *   `on:workflow_dispatch:inputs` structure.
        *   `jobs.<job_id>.steps.run` commands.
        *   `jobs.<job_id>.steps.uses` (initially out-of-scope, but consider how to gracefully ignore/warn).
        *   `env` blocks at workflow, job, and step levels.
*   **Utilize `serde_yaml`**: The crate is already a dependency. Use it for deserialization.

### 3.3. Step Execution Logic

*   **Modify `crates/mini-act/src/runner.rs`**:
    *   Adapt `run_workflow` and `run_job` to process the newly parsed GHA workflow structure.
    *   For `run` steps, use `std::process::Command` to execute the commands.
    *   Ensure environment variables (including those from `env` blocks and `workflow_dispatch` inputs) are correctly set for each executed command.
    *   Implement basic simulation of GitHub Actions environment variables (e.g., `GITHUB_WORKSPACE`).

### 3.4. Input Handling

*   **Map `workflow_dispatch` inputs**: Convert inputs provided via the CLI (e.g., `--input crq_id=123`) into environment variables accessible by the `run` steps.

## 4. Testing Strategy

*   **Unit Tests**: Write comprehensive unit tests for:
    *   CLI argument parsing.
    *   YAML deserialization of various GHA workflow structures (especially `workflow_dispatch` and `run` steps).
    *   Environment variable substitution logic.
*   **Integration Tests**: Create simple `.yml` workflow files in a test directory and write integration tests that:
    *   Call MiniAct with the new command.
    *   Verify that the `run` commands within the `.yml` are executed correctly.
    *   Check that inputs are passed through as expected.

## 5. Debugging Tips

*   Use `eprintln!` liberally within `crates/mini-act` for debugging output.
*   Leverage `cargo test -- --nocapture` to see `eprintln!` output during tests.
*   Remember the `kantspel` principles for clear logging.

## 6. Success Criteria Reminder

*   MiniAct successfully parses and executes a simple GitHub Action workflow file locally.
*   The output of the local execution matches the expected output of the `run` steps in the workflow.
*   Inputs provided to the `workflow_dispatch` are correctly passed to the executed commands.
