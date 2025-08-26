# Standard Operating Procedure: Gemini CLI Reboot Recovery (GM Meta-Program)

## 1. Overview

This SOP outlines the standardized procedure for the Gemini CLI agent to recover its operational context and resume work efficiently after a system reboot or unexpected interruption. This process is part of the "GM Meta-Program" designed to minimize downtime and ensure continuity of development.

## 2. Procedure

Upon reboot or restart, the Gemini CLI agent will execute the following steps to re-establish its working context:

### 2.1. Re-focus on the Critical Path

Immediately identify and re-engage with the last known critical task or objective. This involves recalling the primary goal of the ongoing development session.

### 2.2. Review Internal Memories

Access and review all relevant internal memories to recall project-specific information, user preferences, and previously established context. This includes:

*   **Project Architecture and Technologies:** Recall the overall structure, tech stack (Rust, MiniZinc, LLM, etc.), and design principles.
*   **Coding Standards and Conventions:** Re-familiarize with established code style, naming conventions, and best practices.
*   **User Preferences:** Recall any specific user preferences or directives (e.g., "do not use the replace tool," "prefer functional composition").
*   **Active CRQs and SOPs:** Review currently active Change Requests (CRQs) and relevant Standard Operating Procedures (SOPs) to understand ongoing tasks and established workflows.

### 2.3. Check Recent Git Commits

Examine the recent Git commit history to understand the latest changes, progress, and any pending tasks. This provides an auditable trail of development activity.

*   **Command:** `git log --patch -3 --all`
*   **Focus:** Pay attention to commit messages, changed files, and any associated CRQ references to quickly grasp the project's current state.

## 3. Objective

The objective of this GM Meta-Program is to enable rapid re-orientation and efficient continuation of development by systematically recovering context and re-engaging with the critical path after a reboot or interruption.
