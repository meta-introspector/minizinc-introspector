# Launchpad CLI Arguments

The `launchpad` tool provides a streamlined way to manage and interact with the Gemini CLI, facilitating development and testing workflows. This document details the command-line arguments available for configuring `launchpad`'s behavior.

## `LaunchpadArgs` Structure

The following arguments can be passed to the `launchpad` executable:

### `--target-repo-url <URL>`
*   **Type:** `Option<String>`
*   **Default:** `None`
*   **Purpose:** Specifies the URL of the target Git repository. This repository will be cloned and used as the working environment for the Gemini CLI.

### `--workflow-file-in-repo <PATH>`
*   **Type:** `Option<String>`
*   **Default:** `None`
*   **Purpose:** Specifies the path to a workflow file within the target repository. This file defines the sequence of operations or tasks that `launchpad` should execute.

### `--gemini-cli-path <PATH>`
*   **Type:** `Option<String>`
*   **Default:** `None`
*   **Purpose:** Specifies the absolute path to the Gemini CLI executable. If not provided, `launchpad` might attempt to locate it or assume it's in the system's PATH.

### `--gemini-instances <NUMBER>`
*   **Type:** `usize`
*   **Default:** `1`
*   **Purpose:** Specifies the number of Gemini CLI instances to launch. Useful for parallel execution or testing scenarios requiring multiple agents.

### `--record-session`
*   **Type:** `bool`
*   **Default:** `false`
*   **Purpose:** If set to `true`, the session will be recorded using `asciinema`. This is useful for auditing, demonstrations, or debugging.

### `--background-detached`
*   **Type:** `bool`
*   **Default:** `false`
*   **Purpose:** If set to `true`, the launched Gemini CLI instances will run in a detached background process. This allows `launchpad` to exit while the Gemini instances continue to operate.

### Catch-all Arguments (`-- <ARGS>...`)
*   **Type:** `Vec<String>`
*   **Purpose:** Any arguments provided after `--` will be passed directly to the staged binary (e.g., the Gemini CLI or another tool being orchestrated by `launchpad`). This allows for flexible control over the underlying tools.

## Example Usage

```bash
# Launch Gemini CLI with a specific repository and workflow file
launchpad --target-repo-url https://github.com/user/my-project.git --workflow-file-in-repo workflows/my_workflow.yaml

# Launch 3 Gemini instances and record the session
launchpad --gemini-instances 3 --record-session

# Launch Gemini in a detached background process and pass arguments to it
launchpad --background-detached -- --model gemini-pro --temperature 0.7
```