## QA Document: `zos-stage-process-monitor` TUI

### What is `zos-stage-process-monitor`?
`zos-stage-process-monitor` is a new tool (a "stage" in our system) built in Rust. Think of it as a special window into your computer's brain, specifically designed to show you what's happening inside your `tmux` sessions.

### What does it do?
It provides a **Terminal User Interface (TUI)**, which is like a graphical application that runs directly in your terminal. Its main job is to:
1.  **List your `tmux` sessions:** If you use `tmux` to organize your terminal windows, this tool will show you all your active `tmux` sessions.
2.  **Show `tmux` panes:** Within each session, `tmux` has windows, and windows have panes. This tool will list all those panes.
3.  **Monitor processes (coming soon!):** In future updates, it will be able to run commands like `ps aux` (which lists all running processes) inside any of your `tmux` panes and display the results in a clear, easy-to-read format.

### Why do we need it?
This tool is super important for a few reasons:
*   **See what's happening:** It helps both humans and our AI (Gemini) understand what programs are running and where.
*   **Teamwork with AI:** It's a step towards letting Gemini and humans work together more smoothly. Gemini can use this tool to monitor its own tasks or other programs, and you can see what Gemini is doing.
*   **Future Control:** It's building the foundation for Gemini to eventually control and manage these processes more intelligently.

### How do I run it?
Since `zos-stage-process-monitor` is a "stage" in our system, you run it using our main launcher tool, `launchpad`.

First, you'll need to build the project (if you haven't already) to make sure `launchpad` and `zos-stage-process-monitor` are ready. Then, you can run it from your terminal like this:

```bash
launchpad zos-stage-process-monitor
```

### How do I use it?
Once you run it, you'll see a screen pop up in your terminal. For now, it will list your `tmux` sessions and panes. You can exit the tool by pressing the `q` key.

### What's next for this tool?
We're actively working on adding the ability to:
*   Actually run `ps aux` and other commands in `tmux` panes.
*   Parse and display the process information in a user-friendly way.
*   Allow you to select specific processes or panes to focus on.

Stay tuned for more updates!
