# SOP: Gemini Tool Usage Guide

## 1. Purpose
This Standard Operating Procedure (SOP) provides a high-level guide for the Gemini CLI agent on how to effectively utilize the various tools developed within this project. It aims to streamline common development workflows and leverage the project's custom tooling for enhanced efficiency and consistency.

## 2. Scope
This SOP applies to all interactions where the Gemini CLI agent needs to perform tasks using the project's custom Rust tools.

## 3. General Principles for Tool Usage
*   **Prioritize Custom Tools:** Whenever a custom tool exists for a task, prioritize its use over generic shell commands or external utilities.
*   **Consult Specific SOPs/CRQs:** For detailed usage instructions, refer to the specific SOPs and CRQs related to each tool. This guide provides an overview; the detailed documentation contains the "how-to."
*   **Error Handling:** Always anticipate and handle potential errors from tool execution. Report unexpected behavior or failures to the user.
*   **Context Awareness:** Understand the context of the task and select the most appropriate tool or combination of tools.

## 4. Tool Catalog and Usage Guidelines

### 4.1. `launchpad`
*   **Purpose:** Orchestrates Gemini CLI execution and development workflows.
*   **Key Capabilities:** Launching Gemini instances, managing tmux sessions, running `cargo` commands, executing sandboxed commands.
*   **Relevant Documentation:**
    *   `crq_launchpad_workflow_enhancements.md`
    *   `docs/cli_arguments/launchpad_cli_arguments.md`
    *   `docs/qa/launchpad_cli_qa.md`
    *   `change_request_operational_workflow.md`

### 4.2. `tmux_controller`
*   **Purpose:** Provides programmatic control over tmux sessions and panes.
*   **Key Capabilities:** Creating, listing, killing sessions, splitting windows, sending commands to panes, capturing session output.
*   **Relevant Documentation:**
    *   `docs/cheatsheets/tmux_noob_cheatsheet.md`
    *   `docs/sops/gemini_tmux_crq_sop.md`
    *   `docs/cli_arguments/tmux_controller_cli_arguments.md`
    *   `docs/qa/tmux_controller_cli_qa.md`

### 4.3. `credential_manager`
*   **Purpose:** Securely stores and retrieves credentials (e.g., GitHub PATs, API keys).
*   **Key Capabilities:** Storing, retrieving, and importing credentials from various sources (AWS, GitHub CLI, Gemini CLI).
*   **Relevant Documentation:**
    *   `change_request_credential_import.md`
    *   `change_request_github_oauth.md`

### 4.4. `crq_updater`
*   **Purpose:** Automates the management of Change Request (CRQ) documentation.
*   **Key Capabilities:** Identifying relevant commits, extracting commit information, updating CRQ files with commit histories, ensuring chronological order and consistency.
*   **Relevant Documentation:**
    *   `docs/sops/crq_updater_qa_sop.md`
    *   `docs/sops/commit_labeling_crq_ownership_sop.md`
    *   `change_request_operational_workflow.md` (for automated commit classification)

### 4.5. `asciicast_processor`
*   **Purpose:** Processes asciinema recordings for analysis and Rust code generation.
*   **Key Capabilities:** Cleaning output data, filtering by regex, implementing consistency checks between raw and processed output.
*   **Relevant Documentation:**
    *   `docs/next_task_asciicast_processor_fix.md`
    *   `docs/recent_build_fixes_and_gemini_utils_refinement.md`

### 4.6. `vibe_analyzer`
*   **Purpose:** Code analysis and searching.
*   **Key Capabilities:** (To be further defined as the tool develops).
*   **Relevant Documentation:** (To be created as the tool develops).

## 5. Workflow Examples (High-Level)

*   **Launching a Gemini Session for a CRQ:**
    1.  Identify the CRQ to work on.
    2.  Use `launchpad` to create a new tmux session and launch a Gemini instance, passing the CRQ ID.
    3.  Use `tmux_controller` to manage panes and monitor output.
*   **Documenting a "wip" Commit:**
    1.  Identify the "wip" commit.
    2.  Use `git show` to examine changes.
    3.  Infer the related CRQ or project area.
    4.  Update the commit documentation file and link it to the relevant CRQ/SOP.
    5.  (Future) Use automated classification tools to assist in this process.

## 6. Continuous Improvement
This SOP is a living document. As new tools are developed or existing workflows are refined, this guide will be updated to reflect the latest best practices for Gemini's tool usage.
