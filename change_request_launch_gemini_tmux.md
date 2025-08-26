### Change Request: Launch Gemini in Tmux Session

**Task:** Launch a new instance of the Gemini CLI in a dedicated tmux session, utilizing the 'pro' model, to serve as a trusted helper operating in parallel.

**Coordination:** This task will be coordinated with another instance of the Gemini CLI.

**Implementation Details:**
- Utilize a Rust command to send necessary keys to a new tmux session.
- The Gemini CLI will be launched from the `storage/github/gemini-cli` directory.
- Ensure the 'pro' model is activated for the launched Gemini instance.

**Purpose:** To enable parallel processing and enhanced assistance by leveraging multiple Gemini instances for complex tasks.