# SOP: Git as a Database and Vibe Definition

## 1. Purpose
This Standard Operating Procedure (SOP) formalizes the project's philosophy of treating Git as its primary, immutable database. It introduces the concept of a "vibe" as a structured, high-dimensional representation of Git objects, enabling advanced introspection, traceability, and integration within the Meta Universal Consciousness (MUC) framework.

## 2. Scope
This SOP applies to all development activities, commit practices, and data interpretation within the project, particularly concerning the interaction between human developers, LLM agents (like Gemini), and the Git version control system.

## 3. Git as a Database

Git is not merely a version control system; it is the foundational, immutable database for this project. Every commit represents a transaction, and the entire commit history forms an auditable, cryptographically verifiable ledger of all project evolution.

*   **Immutability:** Once committed, data in Git is immutable, ensuring historical integrity.
*   **Traceability:** Every change can be traced back to its origin, author, and context.
*   **Decentralization:** The distributed nature of Git enhances resilience and collaboration.
*   **Atomic Operations:** Commits are atomic units of change, representing a consistent state of the project.

## 4. The "Vibe" Definition

A "vibe" is a structured, high-dimensional vector representation of a Git object (e.g., a commit, a file, a function, a block of code). It encapsulates both the content and the contextual metadata, allowing for semantic analysis and algorithmic manipulation within the MUC.

**Formal Definition:**

`vibe = vector[git object, commit, hash, godel number, summary, model, block, function, transaction]`

Where:
*   **`git object`**: The raw Git object (e.g., blob, tree, commit, tag).
*   **`commit`**: Reference to the Git commit object, including author, committer, timestamp, and parent commits.
*   **`hash`**: The SHA-1 hash of the Git object, serving as its unique identifier.
*   **`godel number`**: A unique numerical identifier assigned to the semantic content of the Git object, enabling formal verification and mathematical reasoning within systems like Lean4. This number is derived from the content and its context.
*   **`summary`**: A concise, human-readable description of the Git object's purpose or impact. For commits, this is typically the commit message subject line.
*   **`model`**: Reference to any formal models (e.g., MiniZinc, Lean4 theorems) that describe or are derived from the Git object.
*   **`block`**: For code objects, refers to the logical block or scope (e.g., function body, class definition).
*   **`function`**: For code objects, refers to the specific function or predicate being defined or modified.
*   **`transaction`**: The atomic operation represented by the Git object, particularly relevant for commits (e.g., feature addition, bug fix, refactoring).

## 5. Implications for Development

*   **Semantic Commits:** Commit messages should be rich in semantic information, contributing to the `summary` and `transaction` components of the vibe.
*   **Automated Introspection:** LLM agents (like Gemini) will leverage the "vibe" to understand, analyze, and generate code, documentation, and formal proofs.
*   **Formal Verification Integration:** The `godel number` component facilitates the integration with formal verification systems, allowing for mathematical proofs of correctness.
*   **Continuous Learning:** The "vibe" enables feedback loops, where the system learns from its own evolution and refines its understanding of the codebase.

## 6. Related Documents
*   SOP: MiniZinc Model Syntax Validation
*   SOP: Debugging MiniZinc Syntax Errors
*   Deep Bootstrapping and Formal Verification Strategy (conceptual design)
*   `GEMINI.md` (Project AI Context and Best Practices)