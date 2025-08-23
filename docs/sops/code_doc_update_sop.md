# SOP: Code, Documentation, Index, and Gemini Memory Update Procedure

## 1. Purpose
To establish a standardized, auditable, and quality-controlled procedure for updating project code, associated documentation, internal indexes, and Gemini CLI's operational memories. This SOP ensures consistency, traceability, and adherence to quality management principles (GMP, ISO 9000, ITIL, Six Sigma) and architectural best practices (C4, UML).

## 2. Scope
This SOP applies to all modifications of source code, project documentation (including Markdown, RST, etc.), any generated indexes (e.g., search indexes, cross-reference indexes), and updates to the Gemini CLI's persistent memory.

## 3. Definitions
*   **Code:** Source files (e.g., Rust, MiniZinc, C++).
*   **Documentation:** User manuals, API docs, design documents, SOPs, READMEs.
*   **Index:** Any structured data facilitating search or navigation (e.g., `GEMINI.md`, search indexes, cross-reference maps).
*   **Gemini Memory:** Persistent facts and configurations stored by the Gemini CLI agent.
*   **Link:** A reference from code to documentation, or documentation to code (e.g., file paths, function names, section headers).

## 4. Responsibilities
*   **Gemini CLI Agent:** Execute steps, maintain memory, generate reports.
*   **Human Operator:** Review, approve, provide input, and oversee the process.

## 5. Procedure

### 5.1. Phase 1: Planning and Impact Analysis (ITIL Service Design, Six Sigma Define/Measure)

*   **5.1.1. Change Request Review:**
    *   Review the proposed code/documentation change.
    *   Identify all affected code modules, documentation files, and potential index impacts.
    *   *Guideline (ISO 9000):* Ensure clear understanding of customer (user) requirements.
*   **5.1.2. Link Dependency Mapping (C4 Model, UML):**
    *   For code changes, identify all documentation sections that reference the modified code.
    *   For documentation changes, identify all code sections that reference the modified documentation.
    *   Utilize existing tools (e.g., the proposed Rust link checker) or manual inspection to map dependencies.
    *   *Guideline (UML):* Consider generating or updating component/sequence diagrams if the change significantly alters system interactions.
*   **5.1.3. Risk Assessment:**
    *   Assess potential risks (e.g., broken links, outdated information, build failures).
    *   Develop mitigation strategies.
    *   *Guideline (Six Sigma):* Identify critical-to-quality (CTQ) aspects of the change.

### 5.2. Phase 2: Implementation (GMP, ISO 9000 Production, Six Sigma Improve)

*   **5.2.1. Version Control:**
    *   Create a dedicated Git branch for the changes.
    *   *Guideline (ITIL Change Management):* All changes must be tracked.
*   **5.2.2. Code Modification:**
    *   Implement code changes adhering to project coding standards (e.g., Rust idiomatic practices, one declaration per file).
    *   Ensure new code is well-commented where necessary (why, not what).
    *   *Guideline (GMP):* Follow established procedures; ensure changes are made in a controlled environment.
*   **5.2.3. Documentation Update:**
    *   Update all affected documentation files to reflect code changes.
    *   Ensure consistency in terminology, formatting, and style.
    *   Create new documentation as required.
    *   *Guideline (ISO 9000):* Maintain accurate and up-to-date documentation.
*   **5.2.4. Link Maintenance:**
    *   **Crucial:** For every code change, update corresponding documentation links. For every documentation change, update corresponding code references.
    *   Use relative paths where appropriate, and ensure absolute paths are correct.
    *   *Guideline (Lean4, MiniZinc, Coq):* Ensure formal correctness and consistency of references.
*   **5.2.5. Index Update:**
    *   If the change impacts any project-specific indexes (e.g., `GEMINI.md` for project context, search indexes), update them accordingly.
    *   This may involve running specific index generation scripts.

### 5.3. Phase 3: Verification and Validation (Six Sigma Control, ISO 9000 Inspection)

*   **5.3.1. Build and Test:**
    *   Perform a full project build (`cargo build`).
    *   Run all relevant unit, integration, and system tests (`cargo test`).
    *   *Guideline (GMP):* Verify that the product meets specifications.
*   **5.3.2. Link Verification (Rust Tool):**
    *   Execute the Rust link verification tool across the entire codebase and documentation.
    *   Address any reported broken or inconsistent links.
    *   Generate a link verification report.
*   **5.3.3. Documentation Review:**
    *   Conduct a peer review or automated linting of updated documentation for clarity, accuracy, and completeness.
    *   *Guideline (ITIL Service Operation):* Ensure documentation supports operational processes.
*   **5.3.4. Gemini Memory Update:**
    *   If the change introduces new project facts, standards, or operational procedures relevant to Gemini's long-term memory, use the `save_memory` tool to record them.
    *   *Example:* `save_memory(fact="New SOP for code and doc updates has been implemented.")`
    *   *Guideline (ITIL Knowledge Management):* Capture and maintain organizational knowledge.

### 5.4. Phase 4: Release and Post-Implementation Review (ITIL Service Transition/Operation)

*   **5.4.1. Merge and Deploy:**
    *   Merge the feature branch into the main development branch after all checks pass.
    *   Deploy the updated code and documentation.
*   **5.4.2. Post-Implementation Review:**
    *   Periodically review the effectiveness of the change and the SOP itself.
    *   Gather feedback for continuous improvement.
    *   *Guideline (Six Sigma Control):* Monitor performance to ensure sustained improvement.

## 6. Records
*   Git commit history (including detailed commit messages).
*   Link verification reports.
*   Test reports.
*   Gemini CLI memory logs.

## 7. References
*   Project Coding Standards
*   Project Documentation Guidelines
*   GMP Guidelines
*   ISO 9000 Standards
*   ITIL Framework Documentation
*   Six Sigma Methodology
*   C4 Model Documentation
*   UML Specifications
