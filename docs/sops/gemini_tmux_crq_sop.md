# Standard Operating Procedure: Launching Gemini in Tmux and Assigning CRQs

## 1. Overview
This SOP outlines the process for launching a dedicated Gemini CLI instance within a new tmux session and assigning tasks to it via Change Request (CRQ) files. This enables parallel execution of tasks and leverages specialized Gemini instances for specific roles.

## 2. Prerequisites
- Rust toolchain installed.
- `tmux_controller` crate built in release mode.
- `gemini-cli` executable available at `~/storage/github/gemini-cli/target/release/gemini-cli`.
- A CRQ Markdown file (`.md`) detailing the task to be assigned.

## 3. Procedure

### 3.1. Build `tmux_controller` (if not already built or updated)
Ensure the `tmux_controller` crate is built to incorporate the latest features, including CRQ handling.

```bash
cargo build --release -p tmux_controller
```

### 3.2. Create a New Tmux Session
Create a new detached tmux session where the Gemini CLI instance will run. Choose a descriptive session name (e.g., `gemini-pro-helper`).

```bash
./target/release/tmux_controller create --session-name <session_name>
```

**Example:**
```bash
./target/release/tmux_controller create --session-name gemini-pro-helper
```

### 3.3. Launch Gemini CLI in the Tmux Session
Launch the Gemini CLI executable within the newly created tmux session. You can specify the model and project directory.

```bash
./target/release/tmux_controller gemini --session-name <session_name> --model <model_name> --project <project_name>
```

**Example:**
```bash
./target/release/tmux_controller gemini --session-name gemini-pro-helper --model pro --project gemini-cli
```

### 3.4. Assign a Task using a CRQ
Assign a task to the Gemini instance by providing the name of a CRQ Markdown file. The `tmux_controller` will read the CRQ content and send it as an instruction to the Gemini instance.

```bash
./target/release/tmux_controller gemini --session-name <session_name> --crq <crq_file_name>
```

**Example:**
```bash
./target/release/tmux_controller gemini --session-name gemini-pro-helper --crq change_request_oauth_rust_module.md
```

## 4. Verification
- Confirm that the tmux session has been created using `tmux list-sessions`.
- Observe the output of the `tmux_controller` commands to ensure successful execution and instruction delivery.
- (Manual) Attach to the tmux session (`tmux attach -t <session_name>`) to directly observe the Gemini CLI's activity and responses.

## 5. Troubleshooting
- If `tmux_controller` fails to build, check for syntax errors in the Rust code and ensure all dependencies are met.
- If commands are not being sent to the tmux session, verify the session name and ensure the `gemini-cli` is running within it.
- Ensure the CRQ file path is correct and the file is readable by the `tmux_controller`.
