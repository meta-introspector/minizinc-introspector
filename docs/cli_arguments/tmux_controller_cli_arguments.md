# Tmux Controller CLI Arguments

The `tmux_controller` crate provides a set of commands for programmatically interacting with `tmux` sessions and windows. This document details the command-line arguments available for each `tmux_controller` command.

## `split-horizontal` Command

This command splits the current `tmux` window horizontally. It can optionally target a specific session.

### Arguments

### `--session-name <NAME>`
*   **Type:** `Option<String>`
*   **Default:** `None`
*   **Purpose:** Specifies the name of the `tmux` session where the window should be split. If not provided, the current active session's window will be split.

## Example Usage

```bash
# Split the current window horizontally
tmux_controller split-horizontal

# Split a window in a specific session named 'my_session'
tmux_controller split-horizontal --session-name my_session
```

## `split-vertical` Command

This command splits the current `tmux` window vertically. It can optionally target a specific session.

### Arguments

### `--session-name <NAME>`
*   **Type:** `Option<String>`
*   **Default:** `None`
*   **Purpose:** Specifies the name of the `tmux` session where the window should be split. If not provided, the current active session's window will be split.

## Example Usage

```bash
# Split the current window vertically
tmux_controller split-vertical

# Split a window in a specific session named 'my_session'
tmux_controller split-vertical --session-name my_session
```