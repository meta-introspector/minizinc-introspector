# SOP: Tmux Workflow and Debugging Procedures

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the standardized procedures for managing `tmux` sessions, executing commands within `tmux` panes, debugging `launchpad` and `crq_updater` related issues, and utilizing `tmux_view` for session state analysis. It consolidates recent learnings and best practices for efficient and auditable development workflows within the `libminizinc` project.

## 2. Scope
This SOP applies to all development activities involving:
*   `tmux` session and pane management.
*   Execution of `launchpad` dynamic stages.
*   Debugging Rust applications, particularly `crq_updater`.
*   Inter-agent communication and monitoring via `tmux` pane output.

## 3. Tmux Session Management and Command Execution

### 3.1. Creating Standard Tmux Layouts
To create a predefined `tmux` layout (e.g., for work/data, Gemini, and status panes) without automatically launching Gemini in specific panes (for "native worker" execution):
```bash
car go run --package launchpad -- tmux-controller-cmd create-layout
```
*   **Audit:** Visually confirm the `tmux` layout in your terminal.

### 3.2. Sending Commands to Specific Tmux Panes
To execute a command within a specific `tmux` pane (e.g., session 5, window 0, pane 1):
```bash
car go run --package launchpad -- tmux-controller-cmd send-command --command "send-keys -t 5:0.1 'your_command_here' C-m"
```
*   **`your_command_here`**: Replace with the actual command to execute (e.g., `launchpad /path/to/dynamic_stage.yaml --watch`).
*   **`5:0.1`**: Replace with the target `tmux` pane identifier (`<session_id>:<window_index>.<pane_index>`).
*   **`C-m`**: Simulates pressing Enter.
*   **Audit:** Visually confirm the command execution and output in the target `tmux` pane.

### 3.3. Debugging Tmux Pane Targeting
If a command is not appearing in the expected `tmux` pane, verify pane targeting using a simple `echo` command:
```bash
car go run --package launchpad -- tmux-controller-cmd send-command --command "send-keys -t 5:0.1 'echo \"hello world\"' C-m"
```
*   **Audit:** Visually confirm "hello world" appears in the target pane. If it does, the pane targeting is correct, and the issue lies with the command being sent.

## 4. `launchpad` Dynamic Stage Execution

### 4.1. Running Dynamic Stages with `--watch`
To execute a `launchpad` dynamic stage (defined in a YAML file) with real-time output streaming to the `tmux` pane and logging to a file:
```bash
launchpad /path/to/dynamic_stage.yaml --watch
```
*   Ensure the `dynamic_stage.yaml` includes a `log_file` parameter for persistent logging.
*   **Audit:** Observe the output streaming in the `tmux` pane and verify the content of the specified `log_file`.

## 5. `crq_updater` Debugging and Fixes

### 5.1. Debugging Hanging Issues
If `crq_updater` (or any other Rust application) appears to hang when executed via `launchpad` or `tmux`, run it directly in your current terminal to observe its real-time output and identify the hang point or error messages:
```bash
car go run --package crq_updater -- --dry-run # or without --dry-run for actual run
```
*   **Audit:** Analyze the console output for errors, warnings, or unexpected behavior.

### 5.2. Resolving Duplicate History Entries
*   **Problem:** `crq_updater` was adding duplicate commit history entries to CRQ files.
*   **Fix:** Modified `crates/crq_updater/src/functions/extract_existing_history.rs` to correctly identify and skip duplicate entries based on commit hash.

### 5.3. Resolving New File History Insertion
*   **Problem:** `crq_updater` was not correctly adding commit history to newly created CRQ files, or inserting it at an incorrect location.
*   **Fix:** Modified `crates/crq_updater/src/functions/process_crq_file.rs` to simplify the `final_content` construction logic, ensuring the history section is correctly appended to the `pre_history_content` if it doesn't already exist.

## 6. Inter-Agent Communication and `tmux_view`

### 6.1. Capturing and Analyzing Tmux Pane Output
To capture the current state of all `tmux` panes and save their content to files for analysis:
```bash
car go run --package tmux_controller -- tmux-view
```
*   This command will list the paths to the captured files (e.g., `sessions/<session_id>/<pane_id>/<timestamp>_capture.txt`).
*   **Audit:** Read the content of the relevant captured file (e.g., `read_file /path/to/capture.txt`) and search for specific messages or patterns (e.g., "FOR GEMINI").

### 6.2. Extracting Messages from Pane Output
After capturing `tmux` pane output, read the relevant capture file and search for the desired message. For example, to find "FOR GEMINI" in a captured file:
```bash
# Assuming you have read the file content into your context
# Search for "FOR GEMINI" in the content.
```
*   **Audit:** Confirm the presence and content of the message.

## 7. Rust Development Workflow Best Practices

### 7.1. Creating New Crates/Modules
*   **Principle:** Avoid temporary files. All new functionalities should be implemented as modules within existing crates or as new, dedicated crates.
*   **Procedure:**
    1.  Create a new directory for the crate (e.g., `crates/my_new_crate`).
    2.  Create `src/main.rs` (for binaries) or `src/lib.rs` (for libraries) within the new crate directory.
    3.  Create a `Cargo.toml` file within the new crate directory.
    4.  Add the new crate to the `[workspace.members]` array in the project's root `Cargo.toml`.

### 7.2. Debugging Rust Compilation Errors
*   **`unresolved import`:**
    *   Verify the module path. Ensure `use` statements correctly reflect the module's location (e.g., `use crate::commands::my_module;` if `my_module.rs` is in `src/commands/`).
    *   Ensure the module is declared as `pub mod my_module;` in its parent `mod.rs` file.
    *   Check `Cargo.toml` for correct dependencies and feature flags.
*   **`conflicting implementations`:**
    *   Review `#[derive(...)]` macros. For `clap`, `clap::Parser` often implies `clap::Args`, so deriving both can cause conflicts. Remove redundant derives.
*   **`no function or associated item named 'parse'`:**
    *   Ensure the necessary trait is in scope (e.g., `use clap::Parser;` for `parse()` method).

## 8. Reboot Procedure (KitKat Meta-Program)
After completing a significant set of changes or reaching a stable checkpoint, follow the KitKat Meta-Program:
1.  **Pause and Assess:** Review current work, identify completed tasks and remaining items.
2.  **Document Current State:** Update relevant CRQs, SOPs, and narrative documents.
3.  **Define New Strategic Plan:** Outline the next set of objectives.
4.  **Commit Current Work:** Create a clear and concise commit message summarizing the changes.
5.  **Conceptual Reboot:** Mentally reset to focus on the new plan.

## 9. Commit History

- [Commit ac86ef2eda7bd9de1a4ed252273b284b8d682d16: docs: Add Tmux Workflow and Debugging SOP; Refactor tmux_controller and dump_tmux_status](docs/commits/ac86ef2eda7bd9de1a4ed252273b284b8d682d16_docs_Add_Tmux_Workflow_and_Debugging_SOP_Refactor_tmux_controller_and_dump_tmux_status.md)