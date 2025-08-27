### Change Request: Operational Workflow for Gemini CLI Development

**Task:** Establish a robust and reproducible development workflow for Gemini CLI interactions, leveraging `asciinema` for session recording and `act run` for creating clean, sandboxed environments.

**Objective:** To enhance the consistency, reproducibility, and analyzability of Gemini CLI development sessions.

**Phases:**

#### Phase 1: Launchpad Integration
- **Goal:** Simultaneously initiate `asciinema` recording and launch the Gemini CLI.
- **Mechanism:** Develop or utilize a "launchpad" script or tool that orchestrates the concurrent startup of `asciinema` and the Gemini CLI.
- **Recording Scope:** Ensure `asciinema` captures the entire terminal session, including all Gemini's output, user inputs, and any other terminal activities.

#### Phase 2: Clean Context with Act Run
- **Goal:** Construct a clean and isolated development context for each session.
- **Mechanism:** Employ "act run" (or a similar containerization/virtualization tool) to create a controlled environment.
- **Context Components:**
    - **`git checkout`:** The environment will start with a specific, clean state of the codebase, ensuring consistency and reproducibility.
    - **Quasi-chroot:** A near-isolated filesystem environment will be established to minimize external dependencies and interference.
    - **Full Sandbox:** The entire execution will occur within a secure and reproducible sandbox, preventing unintended system modifications and ensuring consistent results across runs.

#### Phase 3: Capture Session Output for Analysis
- **Goal:** Programmatically capture the textual content of all panes within active tmux sessions for analysis and reporting.
- **Mechanism:** Extend `tmux_controller` with a `capture-session-output` command.
- **Approach:**
    - List all active tmux sessions and their panes.
    - For each pane, execute `tmux capture-pane -p` directly from the host system (via `tmux_controller`).
    - Store the captured content in a uniquely named file within a structured session store (`sessions/<session_name>/<pane_id>/<crq_prefix><session_name>_<timestamp>_capture.txt`) inside the project root.
    - The filename will incorporate an optional CRQ number, the session name, and a timestamp for traceability.
- **Challenges:** Ensuring `tmux` commands execute correctly and robustly handling file creation/deletion across the host/tmux boundary.

#### Phase 4: Automated Commit Classification

- **Goal:** Automatically classify "wip" commits to relevant CRQs based on the files changed.
- **Mechanism:** Develop a component (potentially integrated into `crq_updater` or a new tool) that analyzes the diff of "wip" commits.
- **Approach:**
    - For each "wip" commit, retrieve the list of changed files.
    - Map file paths to known CRQs or project areas. This could involve:
        - Predefined mappings (e.g., `crates/launchpad/` -> `crq_launchpad_workflow_enhancements.md`).
        - Heuristics based on keywords in file paths or content.
        - Leveraging the "prime categorization" system.
    - Update the commit documentation file with a more descriptive subject and body, reflecting the inferred CRQ.
    - Link the commit to the inferred CRQ.
- **Challenges:** Developing robust mapping heuristics, handling ambiguous cases, and ensuring accuracy.

**Expected Outcome:** A fully recorded, reproducible, and sandboxed development session, enabling detailed analysis and debugging of Gemini's behavior and interactions.

**Deliverables:**
- Launchpad script/tool.
- Integration with `asciinema`.
- `act run` configuration for sandboxed environments.
- `tmux_controller` `capture-session-output` command implementation.
- Documentation for the new operational workflow.

## Commit History

- [Commit 20b720532c6ce3b5d66355b54e696bea554974c7: feat(launchpad): Integrate Gemini CLI launch and argument parsing](docs/commits/20b720532c6ce3b5d66355b54e696bea554974c7_feat_launchpad_Integrate_Gemini_CLI_launch_and_argument_parsing.md)
- [Commit 3243108068fc3f6864733613ea948c2acb353c30: refactor(ci): Refine GHA workflow for Gemini launch and submodule strategy](docs/commits/3243108068fc3f6864733613ea948c2acb353c30_refactor_ci_Refine_GHA_workflow_for_Gemini_launch_and_submodule_strategy.md)
- [Commit 0cbb28d534ed2f7b056adaaeff81cbd0e82d87f6: feat: Enhance tmux_controller with comprehensive session management; introduce operational workflow CRQ](docs/commits/0cbb28d534ed2f7b056adaaeff81cbd0e82d87f6_feat_Enhance_tmux_controller_with_comprehensive_session_management_introduce_operational_workflow_CRQ.md)
- [Commit 883eb8c29b2d6f26fa7ea8959714c25aaea12d50: feat: Introduce tmux_controller and credential_manager core components](docs/commits/883eb8c29b2d6f26fa7ea8959714c25aaea12d50_feat_Introduce_tmux_controller_and_credential_manager_core_components.md)