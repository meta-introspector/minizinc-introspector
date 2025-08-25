## Change Request: GitHub Repository Integration: Issues, Pull Requests, and Review Comments as Signals

**Title:** GitHub Repository Integration: Issues, Pull Requests, and Review Comments as Signals

**Description:**
This epic Change Request (CRQ) outlines the comprehensive integration of GitHub issues, pull requests (PRs), and their associated review comments into the `libminizinc` project's development workflow. These GitHub entities will be treated as critical "signals" that inform, trigger, and enrich our internal CRQ-driven development process, enhancing traceability, collaboration, and the contextual awareness of AI agents.

**Problem Statement:**
Currently, feedback, bug reports, feature requests, and code review discussions originating from GitHub issues and PRs exist largely outside our formalized CRQ-driven development cycle. This creates a disconnect between external communication and internal task management, leading to:
*   Manual and error-prone transfer of information.
*   Delayed integration of critical feedback.
*   Lack of automated traceability from external signals to internal development tasks.
*   Limited contextual awareness for AI agents regarding the origins and discussions surrounding a task.

**Proposed Solution:**
Implement a robust system for integrating GitHub issues, PRs, and review comments as actionable signals:
1.  **Automated Data Fetching:** Periodically fetch and synchronize data from specified GitHub repositories, including:
    *   **Issues:** Title, description, labels, assignees, comments, state (open/closed).
    *   **Pull Requests:** Title, description, source/target branches, status, review comments, associated commits.
2.  **Signal Processing and Interpretation:** Develop logic to interpret these fetched GitHub entities as "signals." Examples include:
    *   New issue creation (potential new task).
    *   Issue comment (new information, clarification).
    *   PR review comment (feedback, required change).
    *   PR merged/closed (task completion, new code integrated).
3.  **CRQ Linkage and Management:** Establish a formal, bidirectional linkage between GitHub issues/PRs and internal CRQs. This could involve:
    *   Automated creation of draft CRQs from new issues.
    *   Updating CRQ status based on linked PR state.
    *   Embedding GitHub comments directly into CRQ context.
    *   Allowing manual association of existing issues/PRs with CRQs.
4.  **Contextual Enrichment:** Use the fetched GitHub data to enrich the "contextual backpack" provided to AI agents working on associated CRQs, giving them a deeper understanding of the task's origin and discussion history.
5.  **Tooling Exploration:** Identify and leverage existing Rust crates for efficient and reliable interaction with the GitHub API.

**Benefits:**
*   **Enhanced Traceability:** Direct linkage from external feedback to internal development tasks and code changes.
*   **Automated Feedback Integration:** Reduces manual effort and delays in incorporating feedback.
*   **Richer Context for AI Agents:** Provides AI agents with comprehensive historical and discussion context for tasks.
*   **Improved Collaboration:** Streamlines communication between external contributors and internal development.
*   **Alignment with AI Ticket System:** Fully integrates external signals into the formalized AI-driven development workflow.
*   **Proactive Issue Management:** Enables AI agents to proactively monitor and respond to GitHub events.

**Scope:**
*   **In-Scope:**
    *   Development of a Rust crate (e.g., extending `github-repo-inspector`) for fetching GitHub issues, PRs, and their comments.
    *   Implementation of basic signal processing logic (e.g., identifying new issues, new comments).
    *   Mechanism for manual association of GitHub entities with CRQs (e.g., via CRQ metadata).
    *   Local storage of fetched GitHub data for caching and historical analysis.
*   **Out-of-Scope (for this CRQ, but planned for future phases):**
    *   Automated CRQ creation from all GitHub events (initial focus on manual/semi-automated linkage).
    *   Complex natural language processing (NLP) of comments for sentiment analysis or automated task prioritization.
    *   Bidirectional synchronization (e.g., updating GitHub issues from CRQ status).
    *   Integration with the IAM solver engine for fine-grained access control to GitHub data.
    *   Real-time webhook processing (initial focus on periodic polling).

**Impact:**
*   **Positive:** Significantly improves the project's ability to respond to external feedback and manage its development lifecycle.
*   **Neutral:** Requires new development effort and introduces a dependency on GitHub API.
*   **Negative:** Potential for rate limiting issues with GitHub API if not managed carefully.

**Pre-requisites:**
*   GitHub Personal Access Token (PAT) with appropriate permissions.
*   Established CRQ framework.
*   `github-repo-inspector` (renamed from `github-actions-inspector`) as a base.

**Implementation Plan:**

1.  **Extend `github-repo-inspector` (In Progress):**
    *   The `crates/zos-stage-github-repo-inspector` crate has been created and its `Cargo.toml` updated to use `octocrab`.
    *   `src/main.rs` has been refactored to use `octocrab` for fetching workflow runs, jobs, artifacts, and logs.
    *   **Current Status:** Facing persistent compilation errors related to `octocrab` API usage, specifically with method calls and type mismatches. This is actively being debugged.

2.  **Basic Signal Processing (To Do):**
    *   Add logic to identify new issues or new comments since the last fetch.

3.  **Local Data Storage (To Do):**
    *   Implement a simple local caching mechanism (e.g., JSON files) for fetched GitHub data to reduce API calls.

4.  **CRQ Linkage (Manual) (To Do):**
    *   Define a convention for manually linking GitHub entities to CRQs (e.g., adding a CRQ ID to an issue description or a specific label).

**Verification Plan:**
1.  **Build and Run:** Successfully build and run the extended `github-repo-inspector`.
2.  **Data Fetching:** Verify that issues, PRs, and comments are correctly fetched and displayed.
3.  **Signal Detection:** Test the ability to detect new issues or comments.
4.  **CRQ Linkage Test:** Manually link a few GitHub issues/PRs to CRQs and verify the association can be retrieved.

**Rollback Plan:**
*   This change modifies an existing component. Rollback involves reverting the changes made to `crates/github-repo-inspector/src/main.rs` and its `Cargo.toml` (if any new dependencies were added).

**Approvers:**
*   [Relevant Stakeholders/Team Leads]
