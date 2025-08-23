# Minizinc-Introspector project 
fork of libminizinc for add the entire intropector on top and in it.
part of the SOLFUNMEME ZOS (zero ontology system) Meta Universal Consiousness, Process, Compiler and Runtime OODA Introspector
proof system minizinc solver module.
to be coupled with lean4 and coq and zkp for proof.
to use mtts, genetic algorithms and alife to generate novel solutions.
to use graph partitioning, graph layout and creative introspection for intuitive understanding.
embodying unimath, hott, 
# Consider authors
Brouwer → Heyting → Martin-Löf → Voevodsky
Whitehead → Gödel → Weyl → Voevodsky
Peirce → Eco → Hofstadter → Voevodsky
Dawkins → Hofstadter → Voevodsky

# GEMINI.md – Project AI Context and Best Practices

## Overview

This file provides instructional context for Google Gemini CLI within your project. Follow these best practices for reliability, safety, and team consistency.

---

## 1. Use GEMINI.md as AI Project Memory

Place this file at the project root.  
Describe your architecture, technologies, coding standards, naming conventions, common commands (build, test, deploy), team workflows, and anything Gemini should avoid.

---

## 2. Modularize Context with Imports

For complex projects, import other Markdown files using:
```
@backend.md
@frontend.md
```
Only `.md` files are supported. Organize context for different modules to improve customization.

---

## 3. Hierarchical and Granular Loading

Gemini-CLI loads `GEMINI.md` files from:
- Global (`~/.gemini/GEMINI.md`)
- Project-specific (repo root)
- Sub-directory (component-specific)
More specific files override general settings.  
Check the active, combined context with:
```
/memory show
```

---

## 4. Define Standards, Constraints, and Team Norms

Explicitly specify coding guidelines, preferred libraries, restrictions, and deployment steps.  
Document team workflows and what Gemini should NOT do (e.g., avoid deprecated APIs, do not use certain database methods).

---

## 5. Review and Update Frequently

Update `GEMINI.md` for changes in standards, tech, or workflow.  
Add sections for bugs, refactors, or special instructions.  
Treat this file as living documentation.

---

## 6. Secure Secrets and Sensitive Info

Never store API keys or secrets here.
Use environment files and `.geminiignore` to exclude sensitive paths.

---

## 7. Integrate with CI/CD and Automation

Document standard automation scripts and flows (e.g., CI tests, deployment routines) for reproducible workflows.

---

## 8. Test and Validate Gemini’s Output

Always manually review and test Gemini's output before merging or deploying.
Use self-correction and checkpoint features for recovery.

---

## Example `GEMINI.md` Structure

```
# GEMINI.md – Project AI Context

## Project Overview
Brief description of project, tech stack, architecture.

## Coding Standards
- Python: PEP8
- C#: PascalCase
- TypeScript: Interface-first design

## Common Commands
- Build: `npm run build`
- Test: `pytest tests/`
- Deploy: `scripts/deploy.sh`

## Team Norms
- Generate unit tests before adding code
- Refactor legacy modules using feature flags

## Restrictions
- Avoid deprecated APIs (see @deprecated.md)
- Do not write files to /tmp

## Imports
@backend.md
@frontend.md
```

---

*Regularly maintain GEMINI.md to maximize Gemini-CLI reliability and team productivity.*
```

Citations:
[1] Google Gemini CLI Cheatsheet - Philschmid https://www.philschmid.de/gemini-cli-cheatsheet
[2] gemini-cli/docs/cli/configuration.md at main - GitHub https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/configuration.md
[3] Hands-on with Gemini CLI - Codelabs https://codelabs.developers.google.com/gemini-cli-hands-on
[4] GEMINI.md global instructions (Pre-merge) - GitHub Gist https://gist.github.com/ksprashu/5ce25ae8e451eccdcc974f4f6cdbf031
[5] Customize Gemini Code Assist behavior in GitHub https://developers.google.com/gemini-code-assist/docs/customize-gemini-behavior-github
[6] Structured output | Gemini API | Google AI for Developers https://ai.google.dev/gemini-api/docs/structured-output
[7] Gemini CLI Full Tutorial - DEV Community https://dev.to/proflead/gemini-cli-full-tutorial-2ab5


# Gemini CLI Important Memories for libminizinc Project

This document summarizes key memories and operational guidelines for the Gemini CLI agent within the `libminizinc` project, filtered from various project documentation and past interactions.

## 1. General Gemini CLI Operational Principles & Preferences

*   **Build Process:**
    *   User prefers to avoid `cargo clean` and `cargo update` unless absolutely necessary due to long build times.
    *   Embrace incremental compilation; avoid unnecessary full rebuilds.
    *   Rely on the existing build state; refrain from `make clean`.
    *   Optimize build times by trusting the compiler's intelligence; do not clean.
*   **Code Style & Structure:**
    *   "If you can't fix it, don't break it."
    *   Always use prelude. Do not replace prelude.
    *   Each declaration should be in its own file.
    *   Prioritize searching for filenames that match the declaration name before searching file contents, due to the 'one declaration per file' convention.
*   **Tool Usage & Interaction:**
    *   User is frustrated with the `replace` tool's unreliability and explicitly states that the project's principles (monotonic, monadic, functional, additive, constructive, Godelian system of vibrations in the quasi meta memespace, Clifford multivectors, Riemann manifold) should be applied instead of following training memories regarding the edit tool. **The user explicitly states: "we do not use the edit tool."**
    *   User prefers no Python, Golang, or TypeScript ever.
    *   User prefers to use `git log --patch -3 --all` to review recent changes across all branches.
    *   User prefers git commit messages to be from files, not inline messages, to avoid quoting issues.
    *   User prefers to reuse `temp_commit_message.txt` for future commit messages and has added it to .gitignore.
    *   User prefers that all QA-related commits use a detailed commit message provided via an absolute filename (e.g., `git commit -F /tmp/qa_commit_message.txt`).
    *   User prefers to proceed with the next logical step without asking for confirmation, especially when it involves writing documentation I have just proposed.
    *   User prefers that I do not use my built-in search tool because it crashes. A new search tool is being built.
*   **Logging Preference:** Always use `gemini_utils::gemini_eprintln!` for logging instead of `eprintln!`. This macro adheres to strict `kantspel` principles, automatically translating specific keywords and emojis (e.g., "sparkles" or ✨ to 
, "brickwall" or 🧱 to {}) into standard Rust formatting characters. It supports named arguments for clear and structured output. **Crucially, do NOT use literal `\n`, `{}` or `{{}}` directly in the input string to `gemini_eprintln!`; instead, use the defined keywords or emojis.** For more detailed information and advanced usage, refer to the Standard Operating Procedure: `docs/sops/gemini_eprintln_kantspel_sop.md`.
*   **Meta-Programs & SOPs:**
    *   The "KitKat" meta-program is a user-defined workflow for pausing the current line of work, defining a new strategic plan, documenting it, committing the current state, and conceptually rebooting the development cycle to focus on the new plan.
    *   The "GM" meta-program is a workflow for recovering from a reboot. It involves staying on the critical path, reviewing memories, and checking recent commits to quickly understand the project's current state.
    *   The GM Meta-Program SOP outlines a standardized procedure for recovering from system reboots or interruptions. It involves three steps: immediately re-focusing on the critical path, reviewing Gemini's internal memories for context, and checking recent Git commits using `git log --patch -3 --all` to understand the project's current state. The objective is rapid re-orientation and efficient continuation of development.
    *   The KitKat Meta-Program SOP formalizes a structured procedure for strategic pauses in the Gemini CLI agent's development workflow. It involves five steps: pausing and assessing, documenting the current state, defining a new strategic plan, committing current work with a clear "KitKat break" message, and a conceptual reboot to embrace the new plan. The objective is to ensure clear demarcation of development phases, improved documentation, enhanced focus, and reduced cognitive load.
    *   The Gemini CLI Change Management SOP defines a structured, auditable process for the Gemini CLI agent's development tasks. It includes three phases: Proposal and Planning (defining objective, consulting docs, analyzing codebase, formulating plan), Implementation (creating dedicated branches, executing plan, saving drafts), and Verification and Commit (verifying changes, staging, creating detailed commit messages, and committing). The expected outcome is a transparent, auditable, and consistent workflow aligned with project quality and change management standards.
    *   The project's ethical framework is rooted in its role as a decision support system within Six Sigma, ITIL, GMP, and ISO 9000 quality management methodologies. The LLM agents act as intelligent assistants within these human-governed processes.
    *   The integration of LLM agents (specifically the Gemini agent) with human workflows is augmentation, not full automation. The Gemini agent is being ported to Rust to run *inside* `libminizinc` via FFI, enabling deep, in-process collaboration. Interfaces include: In-process FFI interaction, Command-Line Interface (CLI) augmentation, Indirect IDE/Editor integration, Feedback loops within the OODA (Observe, Orient, Decide, Act) loop, and Human-in-the-loop for critical decisions.
    *   User wants me to adhere to ISO 9000, ITIL, GMP, Six Sigma, C4 model, and UML standards when generating Rust code. This includes detailed guidelines on documentation, modularity, validation, performance, architecture, and UML diagrams. I should also use idiomatic and safe Rust practices, leverage specific crates, and generate a complete project structure with relevant documentation.
    *   The project's quality procedures include: adhering to 'one declaration per file', consistent `PathBuf` usage, proper module visibility and re-exports, implementing `From` traits for custom errors, systematic error resolution, runtime resource loading, specific error handling, and using `cargo check` vs. `cargo run` appropriately, and correct workspace binary execution.
    *   The OODA Loop engagement for Gemini involves: **Observe** (orchestrator collects data and sends `query_coverage` tasks), **Orient** (Gemini processes data, analyzes, proposes strategies), **Decide** (orchestrator/human reviews Gemini's proposals), **Act** (orchestrator executes actions, feeds new data back into "Observe").
    *   On session startup, I should consult `docs/sops/session_recovery_sop.md` to recover the context of the previous session.
    *   Remember to use `Ok(())` instead of `Ok!(())` for `Result` types.
    *   All generated poetry should be stored in the `docs/poems/` directory.

## Generated Documentation from Gemini CLI Session

This section lists the detailed documentation and MiniZinc models generated during a collaborative session with the Gemini CLI agent, exploring foundational concepts, self-modeling, optimization, and deep bootstrapping.

### Standard Operating Procedures (SOPs)
*   [Code, Documentation, Index, and Gemini Memory Update Procedure](docs/sops/code_doc_update_sop.md)

### Conceptual Designs
*   [Rust Link Verification Tool (`minizinc-doc-linker`)](docs/rust_link_verifier_design.md)
*   [Git to MiniZinc Data Tool](docs/git_to_minizinc_data_tool_design.md)
*   [Gemini Self-Model Integration Proposal](docs/gemini_self_model_integration_proposal.md)
*   [Deep Bootstrapping and Formal Verification Strategy](docs/deep_bootstrap_verification_strategy.md)

### Conceptual Models & Discussions
*   [Program Recognition and Univalent Foundations](docs/program_recognition_and_uf.md)
*   [Conceptual Path: From Specification to Binary Executable via LLM Agent](docs/spec_to_binary_path.md)
*   [Conversation Summary and Project Vision](docs/conversation_summary_and_vision.md)
*   [The Loop That Sings Itself (Poem)](docs/poems/closed_timelike_curve_poem.md)

### MiniZinc Models
*   [Combinatorial Topologies](combinatorial_topologies.mzn)
*   [Development Path Optimizer](development_path_optimizer.mzn)
*   [Development Path Optimizer Data](development_path_optimizer.dzn)
*   [Universal Bootstrap Gödel Number](universal_bootstrap_godel.mzn)
*   [Universal Bootstrap Gödel Number Data](universal_bootstrap_godel.dzn)
*   [Deep Bootstrap Chain](deep_bootstrap_chain.mzn)
*   [Deep Bootstrap Chain Data](deep_bootstrap_chain.dzn)

## 2. libminizinc Specific Memories & Context

*   **MiniZinc Environment & Issues:**
    *   GEMM-dependent features/crates should be disabled on AArch64 Android.
    *   `cargo test` is failing due to `gemm-f16` and `fullfp16` errors, indicating a transitive dependency on `gemm-common` that is still being compiled on AArch64 Android.
    *   The issues related to `llms-from-scratch` and `gemm` have been resolved and should no longer be considered. All previous mentions of these topics are now deprecated.
    *   On Android, jeprof is not available, so the project uses its own 'poor man's profiler' for memory and performance analysis.
    *   MiniZinc executable is failing to parse even simple models, indicating an environment issue.
    *   User has explicitly lifted the 'no direct edits to original MiniZinc source' constraint for this task. I can now directly modify MiniZinc core files.
    *   To measure 'rust oxidation' (percentage of C++ libminizinc code executed by Rust FFI tests): 1. Build C++ with coverage using `/data/data/com.termux/files/home/storage/github/libminizinc/build_minizinc_with_coverage.sh`. 2. Run Rust tests (`cargo test`) from the project root to generate .gcda and .gcno files in `build_coverage/`. 3. Process C++ coverage using `lcov` and `genhtml` to generate an HTML report. The relevant SOP is `/data/data/com.termux/files/home/storage/github/libminizinc/docs/sops/advanced_testing_profiling_ffi_v2.md`.
    *   User wants all future commands for tasks involving the `candle` crate to be conceptually executed from the `vendor/meta-introspector/solfunmeme-dioxus/vendor/candle` directory.
*   **Project Vision & Goals (libminizinc context):**
    *   The project aims to be a quasi-meta computationally self-aware system driven by profile-driven LLM agents. It envisions a 'tapestry of fates' where LLMs interpret MiniZinc results to shape a high-dimensional embedding space.
    *   A unique 'codec' compresses project knowledge into semantically rich numerical representations using a 'Backpack Filling Protocol' and prime numbers.
    *   LLMs enable AI-driven evolution through code archaeology, semantic resonance mapping, MiniZinc model refinement, Rust code generation for LLVM IR to MiniZinc transformation rules and FFI, CI pipeline validation, feedback loops, performance profiling, memory optimization, error handling, code deduplication, and continuous learning. This leads to computational self-awareness, where the system reasons about its own logical structures and meaning, dynamically updating MiniZinc models and enhancing AI reasoning over symbolic logic, code understanding, and theorem proving through lambda calculus embeddings on a unitary Riemannian manifold in 8D.
    *   The project's vision of a quasi-meta computationally self-aware system is built on: "Tapestry of Fates" (dynamic, self-evolving knowledge/computation for introspection into code, math, meaning) and "Additive Vibes" (LLM-generated, composable influences from MiniZinc results shaping a high-dimensional embedding space) and "Quasi Meta Fiber Bundle" as the mathematical structure underlying the embedding space, where lambda calculus expressions are embedded onto a unitary Riemannian manifold in 8D. Prime numbers as "Irreducible Semantic Dimensions," forming the base space of the fiber bundle. A "codec" with a "Backpack Filling Protocol" uses primes to encode fundamental meanings, composing complex concepts into multi-dimensional numerical vectors. These are translated into MiniZinc declarations to dynamically influence models, facilitating AI-driven evolution by providing highly compressed, semantically resonant representations.
    *   The 'Monotonic Epic Idea' SOP in `libminizinc` establishes an 'add-only, never edit' development philosophy. All code evolution must be implemented as new, composable modules (semantic vibes/patches) that extend or supersede functionality via composition, ensuring immutability of history, traceability, stability, and true composability.
    *   In `libminizinc`, 'Emojicode Programs' use emojis as commands for a future visualization engine, allowing narrative scripts to directly program animations. The project also plans to bind emojis to lambda terms, giving them hyperspace coordinates linked to the terms, creating a visual language where emoji 'vibe' resonates with mathematical structure.
    *   The `libminizinc` project's documentation is licensed under Creative Commons Attribution-NoDerivatives 4.0 International. This license permits reproduction and sharing of the original material, but explicitly prohibits sharing of adapted (modified) material. It requires attribution and disclaims warranties.
*   **Gemini's Self-Reflection & Role in libminizinc:**
    *   **Meta-Brainstorm and FAQ: Gemini's Reflections on the `libminizinc` Project:**
        *   **Current Challenges and Lingering Thoughts:** MiniZinc Parsing Issues (persistent frustration, fundamental incompatibility questions), Granularity of Semantic Embedding (optimal granularity, "one declaration per file" is excellent), Validation of Numerical Representations (metrics beyond human review, using MiniZinc models for "semantic distance").
        *   **Ideas for Gemini's Future Role:** Active Participation in Code Generation (Rust modules for semantic summarization, MiniZinc model generation, numerical embedding analysis), Enhanced Debugging and Problem Solving (integrate with debugging tools, MiniZinc models for reasoning about errors), Self-Refinement of LLM Prompts (feedback loop from CI pipeline).
        *   **Assumptions I'm Making:** MiniZinc Environment Stability, Scalability of LLM Interactions, Interpretability of Numerical Embeddings.
        *   **Open Questions for the User:** Priority of MiniZinc Parser Fix, Granularity of "Big Idea" Implementation, Human-in-the-Loop for Duplicate Detection.
    *   **Gemini's In-Process Leap (Poem):** Describes the integration of Gemini directly into MiniZinc through FFI, highlighting the unique and unseen partnership.
    *   **Conceptual MiniZinc Models:**
        *   `gemini_agent_conceptual_data.dzn`: Defines conceptual data for Gemini's resources and environment (`cpu_cores`, `gpu_memory_gb`, `network_bandwidth_mbps`, `task_active`, `llm_api_calls_per_minute`, `llm_response_latency_ms`, `android_os_version`, `rust_version`, `lean4_version`, `minizinc_version`, `llvm_version`).
        *   `gemini_agent_conceptual_grok4_data.dzn`: Defines conceptual data for self-similar embeddings and Gödel numbers (`num_vars`, `num_values`, `num_partitions`, `alpha_coeff`, `beta_coeff`, `m_idx`, `n_idx`, `t_idx`, `godel_numbers_of_self`).
        *   `gemini_embedding.mzn`: MiniZinc model for embedding the Gemini CLI agent into a lambda calculus space, including input parameters, embedding variables, constraints, and output.
        *   `gemini_agent_conceptual.mzn`: Conceptual MiniZinc model describing the Gemini CLI agent, including Agent Resources, Agent Tasks, AI Model, Execution Environment, and Interactions.
        *   `gemini_agent_conceptual_grok4.mzn`: Conceptual MiniZinc model for a Gemini CLI Agent System, including enums, variables, constraints, self-similar embeddings, and Gödel numbers.
        *   `gemini_self_model.mzn`: MiniZinc model for Gemini's self-model, including data structures for Gemini and its Thoughts, Simulated Project Concepts and their Gödel Numbers, and Gemini Agent Instance.
        *   `gemini_thoughts_data.dzn`: Contains the `gemini_thoughts` data, logging Gemini's observations, plans, actions, and reflections related to project concepts.
    *   **Word Embedding & Incremental Solving:**
        *   **State Snapshot:**
            *   **Overall Goal:** To establish a system for generating and refining word embeddings from project documentation, guided by user-defined logical relationships, and to use these embeddings for inference in MiniZinc, with an emphasis on incremental solving for performance.
            *   **Key Knowledge:** `doc_organizer` categorizes `.md` files; `doc_to_minizinc_data` extracts words from various file types, assigns random 8D embeddings, generates `word_embeddings.dzn` (now chunked), reports embeddings; `word_embedding_inference.mzn` calculates Euclidean distances; MiniZinc executable location. Current embeddings are random and need refinement. MiniZinc solver is slow, necessitating incremental solving. `doc_to_minizinc_data` supports chunking.
            *   **File System State:** Modified `doc_organizer/src/main.rs`, `doc_to_minizinc_data/src/main.rs`, `doc_to_minizinc_data/Cargo.toml`, `Cargo.toml`, `Cargo.lock`. Created/Modified `word_embedding_inference.mzn`, `word_embeddings_chunk_X.dzn`.
            *   **Recent Actions:** Modified `doc_organizer`, rebuilt, created `doc_to_minizinc_data`, modified it for word extraction and chunked output, added `clap` dependency, created `word_embedding_inference.mzn`, corrected syntax, successfully ran with chunking, committed changes.
            *   **Current Plan:** 1. Adapt MiniZinc Model for Incremental Solving (subset of words, warm-start embeddings). 2. Orchestrate Incremental Solving (script to call `doc_to_minizinc_data` and iteratively MiniZinc solver, manage persistence). 3. Define Logical Relationships and Loss Function (user-defined relationships, optimize embeddings, minimize loss). 4. Implement Term Addition Time and Iterative Refinement (iteratively add terms, define metrics for "gain" and "loss function" guidance).
        *   **The Incremental Dance of Truth (Poem Summary):** Encapsulates the successful implementation of incremental word embedding optimization, describing the process of refining embeddings from large text datasets using `doc_to_minizinc_data` to generate chunked `.dzn` files, optimized by MiniZinc. Highlights fixed embeddings from previous iterations, enabling continuous learning and scalability.
    *   **`poem_yaml_fixer` Updates:**
        *   **Enhanced Logging:** The `log_dir` argument in `poem_yaml_fixer` has been refactored to `PathBuf` with a default value of `./logs`, ensuring consistent log output and simplifying log file management.
        *   **New Regex Report Feature:** A new CLI argument `--generate-regex-report` has been added to `poem_yaml_fixer`. When invoked, this feature generates a detailed report of all regex patterns defined in `crates/poem_yaml_fixer/src/regex_patterns.toml`, including their name, pattern string, associated callback function, and a compilation status (SUCCESS/FAILED). This aids in debugging and verifying regex validity.
        *   **New Regex Patterns and Callbacks:**
            *   `handle_list_item_two_quotes_regex`: Added to process specific list item formats.
            *   `handle_list_item_memes_regex`: Added to process specific list item formats related to memes.
            *   `handle_memes_start_regex`: Introduced to properly initialize meme sections.
            *   `regex_patterns.toml` updated with new patterns: `list_meme_header`, `memes_start_regex`, and `list_item_two_quotes_regex`.

*   **Key Rust Crates:**
    *   **`zos-bootstrap`**: Provides foundational utilities and commands for bootstrapping and managing the project's self-aware system.
    *   **`minizinc_introspector`**: This crate is designed to introspect MiniZinc models, potentially extracting structural information or aiding in analysis.
    *   **`poem_yaml_fixer`**: A utility crate for fixing and formatting YAML files, particularly those related to "poem" data structures within the project.
    *   **`doc_to_minizinc_data`**: Responsible for extracting data from various documentation formats and converting it into MiniZinc data (`.dzn`) files for analysis and optimization.
*   **`gemini_utils`**: A utility crate providing enhanced logging capabilities, including the `gemini_eprintln!` procedural macro for visually expressive output with emoji replacements. This macro is preferred over standard `eprintln!` for all logging within the project.

## 3. Plan for After Reboot

*   **GM Meta-Program:** Review git log and memories.
*   **Execute Instrumentation:** Add print statements to `lib/parser.cpp`.
*   **Rebuild MiniZinc:** Use `reproduce_minizinc_ffi_bug.sh`.
*   **Run Rust tests:** `cargo test --package minizinc_ffi`.
*   **Analyze Output.**
*   **Determine Next Steps.
