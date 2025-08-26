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

**Expected Outcome:** A fully recorded, reproducible, and sandboxed development session, enabling detailed analysis and debugging of Gemini's behavior and interactions.

**Deliverables:**
- Launchpad script/tool.
- Integration with `asciinema`.
- `act run` configuration for sandboxed environments.
- Documentation for the new operational workflow.
