# Tmux Noob Cheat Sheet

Tmux (Terminal Multiplexer) allows you to run multiple terminal sessions within a single window, detach from them, and reattach later. It's incredibly useful for managing long-running processes and organizing your workflow.

## Basic Concepts
- **Session:** A collection of windows and panes. You can detach from a session and reattach later.
- **Window:** Like a tab in a web browser, but for your terminal. Each window can have multiple panes.
- **Pane:** A division within a window, allowing you to see multiple terminal outputs at once.
- **Prefix Key:** The key combination you press before most tmux commands. By default, it's `Ctrl-b`.

## Essential Commands

| Action                       | Command (after `Ctrl-b`) | Description                                                              |
| :--------------------------- | :----------------------- | :----------------------------------------------------------------------- |
| **Start New Session**        | `tmux new -s <name>`     | Starts a new named session.                                              |
| **List Sessions**            | `tmux ls`                | Shows all active tmux sessions.                                          |
| **Attach to Session**        | `tmux attach -t <name>`  | Reconnects to a named session.                                           |
| **Detach from Session**      | `d`                      | Detaches from the current session, leaving it running in the background. |
| **Kill Session**             | `tmux kill-session -t <name>` | Terminates a named session.                                              |
| **Split Pane Vertically**    | `%`                      | Divides the current pane into two, side-by-side.                         |
| **Split Pane Horizontally**  | `"`                      | Divides the current pane into two, one above the other.                  |
| **Navigate Panes**           | `<arrow key>`            | Moves your cursor to the pane in the direction of the arrow.              |
| **Swap Panes**               | `o`                      | Cycles through panes in the current window.                              |
| **Toggle Pane Layout**       | `Space`                  | Cycles through predefined pane layouts.                                  |
| **Zoom Pane**                | `z`                      | Toggles zoom for the current pane (makes it full screen).                |
| **Scroll History**           | `[`                      | Enters copy mode. Use `PgUp`/`PgDown` or arrow keys to scroll. Press `q` to exit. |
| **Rename Current Session**   | `$`                      | Prompts to rename the current session.                                   |
| **Rename Current Window**    | `,`                      | Prompts to rename the current window.                                    |
| **Create New Window**        | `c`                      | Creates a new window in the current session.                             |
| **Next Window**              | `n`                      | Switches to the next window.                                             |
| **Previous Window**          | `p`                      | Switches to the previous window.                                         |
| **Kill Current Pane**        | `x`                      | Closes the current pane.                                                 |
| **Synchronize Panes**        | `:set synchronize-panes` | Toggles typing in all panes simultaneously (useful for identical commands). |

## Remember the Prefix Key!
Almost all commands start with `Ctrl-b`. Practice it often!

Enjoy your newfound terminal superpowers with Tmux! 
