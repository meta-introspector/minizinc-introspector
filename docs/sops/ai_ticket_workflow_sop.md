## Standard Operating Procedure: AI Ticket System Workflow

**Title:** AI Ticket System Workflow: Integrating CRQs, MiniZinc, IAM, and Contextual Backpacks

**Description:**
This Standard Operating Procedure (SOP) defines the workflow for managing development tasks within the `libminizinc` project through an AI-driven ticket system. It establishes a direct link between development sessions, formal Change Requests (CRQs), MiniZinc-solved tasks, Identity and Access Management (IAM) rules, and contextual "backpacks" for AI agents. This workflow ensures traceability, formal verification, and optimized execution of development tasks, fostering a collaborative environment for both human and AI contributors.

**Purpose:**
*   To formalize the process by which AI agents (e.g., Gemini) engage with development tasks.
*   To ensure that every development session is linked to a defined CRQ, providing clear scope and traceability.
*   To integrate MiniZinc for formal definition and potential solving/verification of task components.
*   To enforce IAM rules for secure and controlled access to project resources during task execution.
*   To provide AI agents with comprehensive, contextual "backpacks" for efficient task execution.
*   To promote a collaborative and verifiable development environment.

**Scope:**
This SOP applies to all development tasks undertaken by AI agents within the `libminizinc` project, from initial task assignment to final verification and integration.

**Definitions:**
*   **AI Ticket:** A formal representation of a development task assigned to an AI agent.
*   **CRQ (Change Request):** A formal document outlining a proposed change, its scope, impact, and implementation plan. Each AI Ticket is associated with one or more CRQs.
*   **MiniZinc Solved Task:** A component of an AI Ticket formally defined and/or verified using a MiniZinc model, ensuring logical consistency and optimal solutions where applicable.
*   **IAM Rules:** Identity and Access Management policies governing the permissions of AI agents during task execution, enforced by the IAM Solver Engine.
*   **Contextual Backpack:** A portable, self-contained collection of all necessary information for an AI agent to execute a task, including relevant code, documentation, MiniZinc models, previous interactions, and configuration.

**Workflow:**

**Phase 1: Task Assignment and CRQ Association**

1.  **Task Identification:** A development task is identified (e.g., bug fix, feature implementation, refactoring).
2.  **AI Ticket Creation:** An AI Ticket is created for the task, detailing its objective and initial requirements.
3.  **CRQ Drafting:** A formal CRQ is drafted for the task, outlining the proposed change, its scope, benefits, impact, and implementation plan. This CRQ is reviewed and approved by relevant stakeholders.
4.  **CRQ Association:** The AI Ticket is formally associated with the approved CRQ(s).

**Phase 2: MiniZinc Modeling and IAM Policy Definition**

1.  **Task Decomposition:** The AI agent (or human collaborator) decomposes the task into smaller, verifiable components.
2.  **MiniZinc Modeling (if applicable):** For components requiring formal definition, optimization, or verification, a MiniZinc model is developed. This model formally defines the problem and its constraints.
3.  **IAM Policy Definition:** IAM policies are defined to govern the AI agent's access to resources (e.g., specific memory fields, files, external services) and actions during the execution of the task. These policies are integrated with the IAM Solver Engine.

**Phase 3: Contextual Backpack Assembly**

1.  **Context Gathering:** The AI agent gathers all necessary context for the task, including:
    *   Relevant code snippets and project files.
    *   Associated documentation (SOPs, architectural designs, previous CRQs).
    *   MiniZinc models related to the task.
    *   Logs and outputs from previous relevant sessions.
    *   Configuration files and environment variables.
2.  **Backpack Creation:** The gathered context is organized into a portable, self-contained "contextual backpack," ensuring the AI agent has all information readily available.

**Phase 4: Session Execution and Monitoring**

1.  **Session Launch:** The AI agent initiates a development session (e.g., via `launchpad` and `zos-stage-session-manager`), linking it to the AI Ticket and associated CRQ.
2.  **IAM Enforcement:** During the session, all agent actions are continuously checked against the defined IAM rules by the IAM Solver Engine. Unauthorized actions are prevented.
3.  **Task Execution:** The AI agent executes the task, leveraging the contextual backpack and interacting with the system (e.g., modifying code, running tests, interacting with Emacs or other Gemini instances).
4.  **Monitoring and Interruption:** The `zos-stage-process-monitor` TUI and `tmux` integration allow for real-time monitoring of processes. Human collaborators can interrupt or observe the session as needed.
5.  **MiniZinc Solving (Runtime):** If the task involves runtime decision-making or optimization, the MiniZinc model is invoked to solve specific sub-problems.

**Phase 5: Verification, Documentation, and Release**

1.  **Verification:** Upon task completion, the AI agent performs self-verification (e.g., running tests, linting, type checks). Human collaborators also review the changes.
2.  **Documentation Update:** All relevant documentation (code comments, architectural designs, SOPs, indexes) is updated to reflect the completed task.
3.  **CRQ Closure:** The associated CRQ(s) are formally closed, with a summary of the implementation and verification results.
4.  **Knowledge Integration:** Learnings from the completed task are integrated into the system's knowledge base, enriching future AI agent capabilities.

**Roles and Responsibilities:**
*   **AI Agent (Gemini):** Task execution, context gathering, self-verification, documentation updates.
*   **Human Collaborator:** Task assignment, CRQ review and approval, oversight, intervention, final verification.
*   **IAM Solver Engine:** Runtime enforcement of access policies.
*   **MiniZinc Models:** Formal definition and verification of task components.

**Metrics for Success:**
*   Reduced development cycle time for AI-assisted tasks.
*   Increased adherence to project standards and quality metrics.
*   Improved security posture through IAM enforcement.
*   Enhanced traceability of changes through CRQ linkage.
*   Increased collaboration efficiency between human and AI agents.

**Review and Improvement:**
This SOP will be reviewed periodically and updated as the project's capabilities and requirements evolve. Feedback from both human and AI collaborators is encouraged for continuous improvement.
