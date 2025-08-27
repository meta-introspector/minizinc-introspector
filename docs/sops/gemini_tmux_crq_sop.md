# Standard Operating Procedure: Launching Gemini in Tmux and Assigning CRQs

## 1. Overview
This SOP outlines the process for launching a dedicated Gemini CLI instance within a new tmux session and assigning tasks to it via Change Request (CRQ) files. It also covers advanced tmux control directly from the `tmux_controller`. This enables parallel execution of tasks, leverages specialized Gemini instances for specific roles, and provides fine-grained control over your tmux environment.

## 2. Prerequisites
- Rust toolchain installed.
- `tmux_controller` crate built in release mode.
- `gemini-cli` executable available at `~/storage/github/gemini-cli/target/release/gemini-cli`.
- A CRQ Markdown file (`.md`) detailing the task to be assigned.

## 3. Procedure

### 3.1. Build `tmux_controller` (if not already built or updated)
Ensure the `tmux_controller` crate is built to incorporate the latest features, including CRQ handling and modularized tmux commands.

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

### 3.5. Split Current Tmux Window Vertically
Splits the current tmux window into two vertical panes.

```bash
./target/release/tmux_controller split-vertical
```

### 3.6. Split Current Tmux Window Horizontally
Splits the current tmux window into two horizontal panes.

```bash
./target/release/tmux_controller split-horizontal
```

### 3.7. Select and Display a Tmux Session
Switches your current tmux client to the specified session, effectively bringing it to the foreground.

```bash
./target/release/tmux_controller select-session --session-name <session_name>
```

**Example:**
```bash
./target/release/tmux_controller select-session --session-name gemini-pro-helper
```

### 3.8. Split Window and Show Session
This command combines splitting the current window (horizontally by default) and then attaching to the specified session in the newly created pane. This is useful for quickly setting up a view to monitor your helper.

```bash
./target/release/tmux_controller show-session --session-name <session_name>
```

**Example:**
```bash
./target/release/tmux_controller show-session --session-name gemini-session-helper
```

## 4. Verification
- Confirm that the tmux session has been created using `tmux list-sessions`.
- Observe the output of the `tmux_controller` commands to ensure successful execution and instruction delivery.
- (Manual) Attach to the tmux session (`tmux attach -t <session_name>`) to directly observe the Gemini CLI's activity and responses.

## 5. Troubleshooting
- If `tmux_controller` fails to build, check for syntax errors in the Rust code and ensure all dependencies are met.
- If commands are not being sent to the tmux session, verify the session name and ensure the `gemini-cli` is running within it.
- Ensure the CRQ file path is correct and the file is readable by the `tmux_controller`.

## Commit History

- [Commit d08222dc45dc2f3501a0c3d72651b6776c462f89: feat: Implement robust Gemini tmux control and CRQ assignment](docs/commits/d08222dc45dc2f3501a0c3d72651b6776c462f89_feat_Implement_robust_Gemini_tmux_control_and_CRQ_assignment.md)
- [Commit 0cbb28d534ed2f7b056adaaeff81cbd0e82d87f6: feat: Enhance tmux_controller with comprehensive session management; introduce operational workflow CRQ](docs/commits/0cbb28d534ed2f7b056adaaeff81cbd0e82d87f6_feat_Enhance_tmux_controller_with_comprehensive_session_management_introduce_operational_workflow_CRQ.md)