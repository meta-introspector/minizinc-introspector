## Change Request (CRQ): Automated Gemini Task Execution via MiniAct and DUM Runtime in Tmux

**Title:** Automated Gemini Task Execution via MiniAct and DUM Runtime in Tmux

**Description:**
Develop a system to automate the launch and management of the Gemini agent for specific tasks. This involves launching Gemini within a new, isolated tmux session, orchestrated by MiniAct, with the primary objective for Gemini to work on a specified Change Request (CRQ). Furthermore, Gemini will be tasked with constructing and utilizing its own DUM (Dynamic Universal Machine) Node.js runtime tailored for the CRQ's requirements.

**Justification/Motivation:**
*   **Automated Workflow:** Enable hands-off execution of complex Gemini tasks, improving efficiency and reproducibility.
*   **Isolated Environments:** Provide dedicated and isolated environments (tmux sessions, custom DUM runtimes) for each Gemini task, preventing conflicts and ensuring clean execution.
*   **Self-Management & Adaptability:** Empower Gemini to configure its own execution environment (DUM runtime) based on task needs, demonstrating advanced self-management capabilities.
*   **Scalability:** Lay the groundwork for running multiple Gemini instances concurrently on different tasks.
*   **Enhanced Debugging:** Isolated sessions facilitate easier monitoring and debugging of Gemini's operations.

**Scope:**
*   **In-Scope:**
    *   Integration with MiniAct to launch `cargo run launchpad` command.
    *   Automatic creation of a new tmux session and split-screen layout for Gemini's operation.
    *   Passing a specific CRQ ID to the launched Gemini instance.
    *   Gemini's ability to interpret the `--via dum` flag and initiate the construction of a custom Node.js DUM runtime.
    *   Gemini's ability to define and execute a custom task within the DUM runtime relevant to the given CRQ.
*   **Out-of-Scope (Initial Phase):**
    *   Full-fledged DUM runtime development (focus on Gemini's ability to *construct* and *use* it).
    *   Complex inter-session communication beyond initial launch parameters.
    *   Advanced error recovery and self-healing mechanisms for failed sessions.

**High-Level Plan/Approach:**
1.  **MiniAct Integration:**
    *   Modify MiniAct to accept and parse the `launchpad` command with its arguments (`--model`, `--crq`, `--mode`, `--inside`, `--via`).
    *   Implement MiniAct logic to create a new tmux session and execute the `cargo run launchpad` command within it.
2.  **Gemini Launchpad Module:**
    *   Develop a new Rust module (`launchpad`) within the project that acts as the entry point for this automated execution.
    *   This module will parse the command-line arguments, including the `--crq` ID and `--via dum` flag.
3.  **CRQ Processing Logic (Gemini):**
    *   Enhance Gemini's core logic to understand and process the provided CRQ ID, retrieving the CRQ details.
    *   Based on the CRQ, Gemini will determine the custom task for the DUM runtime.
4.  **DUM Runtime Construction (Gemini):**
    *   Implement Gemini's capability to generate Node.js scripts and configuration files necessary to set up a minimal DUM runtime.
    *   Gemini will then execute these scripts to launch its custom DUM environment.
5.  **Task Execution within DUM (Gemini):**
    *   Gemini will interact with the DUM runtime to execute the custom task defined for the CRQ.

**Dependencies/Prerequisites:**
*   Existing MiniAct framework.
*   Tmux installed and configured.
*   Node.js and npm/yarn installed for DUM runtime construction.
*   CRQ management system (to retrieve CRQ details by ID).

**Success Criteria:**
*   Successful launch of Gemini in a new tmux session via MiniAct.
*   Gemini correctly identifies and loads the specified CRQ.
*   Gemini successfully constructs and launches its custom DUM Node.js runtime.
*   Gemini executes the custom task within the DUM runtime without errors.
*   The tmux session remains active and accessible for monitoring.
