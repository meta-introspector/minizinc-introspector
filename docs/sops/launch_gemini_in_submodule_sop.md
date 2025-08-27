# SOP: Launching Gemini CLI within a Submodule Project

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the steps for launching a dedicated instance of the Gemini CLI within a Git submodule project. This enables focused development, analysis, and interaction with the submodule's codebase, independent of the parent repository's primary Gemini instance.

## 2. Scope
This SOP applies to all submodule projects within the main repository where a separate Gemini CLI instance is required for specific tasks, debugging, or isolated development.

## 3. Prerequisites
*   The submodule must be correctly added and initialized within the parent repository.
    *   `git submodule add <repository_url> <path_to_submodule>`
    *   `git submodule update --init --recursive`
*   The Gemini CLI executable must be accessible (e.g., in PATH, or specified by absolute path).

## 4. Procedure: Launching Gemini in a Submodule

### 4.1. Navigate to the Submodule Directory
Before launching Gemini, change your current working directory to the root of the submodule project.

```bash
cd <path_to_submodule>
# Example: cd vendor/solfunmeme-dioxus
```

### 4.2. Launch the Gemini CLI
Execute the Gemini CLI from within the submodule's directory. You may need to specify the project root if Gemini requires it to correctly identify its context.

```bash
# Option 1: If Gemini CLI is in your system's PATH
gemini-cli --project-root .

# Option 2: If you need to specify the absolute path to the Gemini CLI executable
/path/to/gemini-cli --project-root .

# Option 3: Launching via launchpad (if launchpad is configured to do so)
# This would typically involve a launchpad configuration specific to the submodule.
# Example (conceptual, requires launchpad configuration):
# cargo run --package launchpad -- --target-repo-path . --gemini-cli-path /path/to/gemini-cli
```

**Note:** If you intend to launch Gemini in a `tmux` session for persistent operation or background execution, you would integrate the above command with `tmux_controller`'s `send-command` or `create-layout --task` functionality, targeting the submodule's directory.

## 5. Considerations
*   **Resource Management:** Running multiple Gemini instances (one for the parent, one for the submodule) will consume more system resources.
*   **Context Switching:** Be mindful of which Gemini instance you are interacting with to avoid applying changes to the wrong repository.
*   **Independent Operation:** Changes made by the submodule's Gemini instance will be committed and managed within the submodule's Git repository, independent of the parent repository's commits.
*   **Version Control:** Ensure the submodule's Git repository is properly managed (e.g., pulling updates, pushing changes) to maintain consistency.
