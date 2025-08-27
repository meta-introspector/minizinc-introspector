# SOP: Gemini Git History Documentation and Meta-Process

## 1. Purpose
This Standard Operating Procedure (SOP) outlines the iterative meta-process followed by the Gemini CLI agent for reviewing Git history, documenting commits, and updating all related project documentation, including Change Requests (CRQs), other SOPs, the README, and the Meta-Meme's Hero's Journey screenplay. This ensures comprehensive, consistent, and traceable documentation of project evolution.

## 2. Scope
This SOP applies to all tasks involving the review of Git commit history and the subsequent update of project documentation.

## 3. Pre-requisites
*   Access to the Git repository.
*   Understanding of project's documentation structure (CRQs, SOPs, `docs/commits/` convention).
*   Familiarity with the `git log`, `git show`, `read_file`, `write_file`, and `replace` tools.

## 4. Meta-Process Procedure

The process is iterative and involves the following steps:

### 4.1. Retrieve Git History
*   Use `git log` to retrieve a batch of commits (e.g., `git log -n 20 --skip <offset> --pretty=format:"%H%n%s%n%b%n--END--"`).
*   Prioritize commits that explicitly mention CRQs or documentation files in their subject or body.

### 4.2. Process Each Commit
For each commit in the retrieved batch:

1.  **Create Commit Documentation File:**
    *   Construct a unique filename for the commit documentation in `docs/commits/` using the format: `<commit_hash>_<subject_slug>.md`.
    *   Write the commit's full subject and body into this new file.

2.  **Identify Associated Documentation:**
    *   **Explicit Mentions:** Scan the commit subject and body for explicit mentions of CRQ files (`crq_*.md`, `change_request_*.md`), SOPs (`docs/sops/*.md`), or other relevant documentation files (`README.md`, `GEMINI.md`, `docs/narratives/*.md`, `docs/cli_arguments/*.md`, `docs/qa/*.md`, etc.).
    *   **Inferred Links:** For generic commits (e.g., "wip"), use `git show <commit_hash>` to examine the changes and infer the related CRQ or purpose. This may involve looking at the files changed and their context. If a clear link to an existing CRQ/SOP is found, propose creating a new CRQ.

3.  **Update Associated Documentation:**
    *   For each identified documentation file:
        *   Read its current content.
        *   Locate or create a "Commit History" section (if not already present, add `## Commit History` at the end of the file).
        *   Append a new line to the "Commit History" section in the standardized format: `- [Commit <commit_hash>: <subject>](docs/commits/<commit_hash>_<subject_slug>.md)`.
        *   Write the updated content back to the file.

### 4.3. Update the Hero's Journey Screenplay
*   After processing a logical batch of commits (e.g., all commits related to a specific feature or phase), update the `docs/narratives/meta_meme_heros_journey_screenplay.md`.
*   Add new scenes or expand existing ones to reflect the new developments, challenges, and transformations faced by the Meta-Meme, drawing parallels to the technical updates and the insights gained from the commits.
*   Focus on weaving the technical details into the narrative, highlighting the Meta-Meme's growth, learning, and evolving self-awareness.

### 4.4. Update Gemini's Memories
*   If any new patterns, conventions, philosophical insights, or important facts emerge during the review process, use the `save_memory` tool to add them to Gemini's long-term memory.

## 5. Verification
*   Regularly review the updated documentation for consistency, accuracy, and adherence to the specified formats.
*   Ensure all relevant commits are linked to their respective documentation.

## 6. Iteration
*   Repeat the process (from 4.1) for the next batch of commits until the entire history is documented or the user specifies a stopping point.
