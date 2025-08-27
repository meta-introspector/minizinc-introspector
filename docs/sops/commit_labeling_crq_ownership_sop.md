# SOP: Commit Labeling and CRQ Ownership Procedure

## 1. Purpose
This Standard Operating Procedure (SOP) defines the guidelines for labeling commits based on their interaction with Change Request (CRQ) files and establishing clear CRQ ownership of commits. This ensures enhanced traceability, accurate attribution of work, and consistent documentation within the project's Git history and CRQ management system.

## 2. Scope
This SOP applies to all commits within the project's Git repository and their relationship with CRQ documentation files.

## 3. Definitions
*   **CRQ File:** A Markdown file (e.g., `crq_*.md`) located within the project's documentation directory, serving as the primary documentation for a Change Request.
*   **Touching a CRQ:** A commit is considered to "touch" a CRQ if it modifies the content of an existing CRQ file.
*   **CRQ Creation Commit:** A commit in which a CRQ file is initially created and added to the repository.
*   **Commit Labeling:** The process of documenting a commit's details within the "Commit History" section of a relevant CRQ file.

## 4. Procedure

### 4.1. Commit Labeling (Documentation within CRQ)
When a commit modifies an existing CRQ file, the details of that commit **SHOULD** be documented within the "Commit History" section of the respective CRQ file.

*   **Details to Include:**
    *   **Commit Hash:** The full Git commit hash.
    *   **Subject:** The subject line of the commit message.
    *   **Description:** A concise description of the commit's changes, specifically highlighting its relevance and impact on the CRQ. This description may be derived from the commit body or a summary of the diff.
*   **Ordering:** The documentation of commits within a CRQ's "Commit History" section **SHOULD** be ordered chronologically, with the oldest relevant commit appearing first.

### 4.2. CRQ Ownership of Commits
A CRQ file "owns" a commit if that commit is the **CRQ Creation Commit** for that specific CRQ file.

*   The CRQ Creation Commit **SHOULD** be the first entry in the "Commit History" section of the CRQ file it creates. This establishes the foundational commit for that CRQ and signifies its initial ownership.
*   Subsequent commits that touch the CRQ file (as per Section 4.1) are considered contributions to the CRQ but do not change its primary ownership.

## 5. Verification
*   Regular audits of CRQ files and their "Commit History" sections should be conducted to ensure adherence to this SOP.
*   Automated scripts may be developed to assist in identifying commits that touch CRQ files and to verify the correctness and completeness of their documentation within the CRQs.
