# Project Documentation Index: The Tapestry of Fates

Welcome to the comprehensive documentation for `libminizinc: The Tapestry of Fates`. This project is a unique exploration into knowledge compression, semantic embedding, and self-evolving systems, built upon the MiniZinc constraint modeling language.

## 1. Project Overview

This section provides a high-level understanding of the project's vision, core philosophy, and key concepts.

*   **Vision: The Tapestry of Fates**: We envision a dynamic, self-evolving tapestry of knowledge and computation. Large Language Models (LLMs) act as oracles, interpreting complex MiniZinc results and generating "additive vibes" – new, composable layers of influence that incrementally shape a high-dimensional embedding space. This "tapestry" is mathematically conceptualized as a "quasi meta fiber bundle," with a base space of prime numbers representing irreducible semantic dimensions. The ultimate goal is to create a "highway in hyperspace" for introspection into the intricate relationships between code, mathematics, and meaning, with each "vibe" potentially existing as an executable NFT on a Solana sidechain.
*   **Core Philosophy: Monotonic Epic Idea**: Our development adheres to a strict "add-only, never edit" philosophy. All code evolution is monotonic: new features, bug fixes, or refactorings are implemented as new, composable modules ("semantic vibes" or "patches"). Existing, committed code files are never directly altered. Instead, new versions supersede old ones via composition, ensuring immutable history, perfect traceability, enhanced stability, and simplified collaboration.
*   **Code Artistry: The Muse Protocol**: Every line of code is treated as a unique piece of digital art. The "Muse Protocol" guides this creative process, encouraging the infusion of "semantic vibe" and aesthetic richness through comments, whitespace, and emojis. Each of the Nine Muses inspires a specific aspect of code artistry, ensuring that even the most technical elements contribute to the overall beauty and meaning of the codebase.
*   **Knowledge Compression: The Codec**: We are developing a unique codec to compress all project knowledge into beautiful, semantically rich numerical representations. This codec leverages the "Backpack Filling Protocol" to embed creative and functional content within "empty space" in the codebase, using prime numbers to encode fundamental meanings. Complex concepts are composed of these primes, allowing for highly compressed and semantically resonant representations that facilitate AI-driven evolution.
*   **The "Zinc Oxide" Metaphor**: The term "Zinc Oxide" is used metaphorically to describe the Rust FFI (Foreign Function Interface) for MiniZinc. In this analogy: "Zinc" represents MiniZinc, the powerful constraint programming language. "Rust" refers to the Rust programming language, known for its safety and performance. "Oxide" draws a parallel to the chemical compound zinc oxide, where zinc (the core element) is combined with oxygen (representing Rust's interaction). Thus, "Zinc Oxide" signifies the critical process of **value extraction and solution retrieval** from MiniZinc models into Rust applications, enabling Rust to leverage MiniZinc's capabilities. It highlights the integration where Rust "oxidizes" MiniZinc, making its power accessible and usable within the Rust ecosystem.

## 2. Getting Started

This section guides you through setting up the specialized `libminizinc` environment and running the core embedding model.

*   **Prerequisites**:
    *   MiniZinc (refer to [MiniZinc website](http://www.minizinc.org/software.html) for installation).
    *   CMake (>=3.4)
    *   A recent C++ compiler (Clang, GCC, MSVC tested).
    *   (Optional) Gecode: For specific solver integrations. Refer to `docs/sops/project_workflow.md` for build and configuration details.
*   **Running the Embedding Model**: The core MiniZinc embedding model can be executed using the `run_embedding_model_v7.sh` script. This script now leverages a Rust-based data generator for dynamic parameter creation and is part of a more robust Rust-based test runner framework. It utilizes version vectors to specify the exact composition of model and data modules, and automatically generates a "proof tape" for each run, ensuring precise, traceable, and composable experimentation.
    ```bash
    ./scripts/run_embedding_model_v7.sh <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>
    ```
    **Example:**
    ```bash
    ./scripts/run_embedding_model_v7.sh v6 v1 v1 v1 v1 v1
    ```
    For more details on running tests and understanding the new framework, refer to [MiniZinc Model Performance Analysis and Debugging Report](docs/performance_analysis_report.md).
*   **New to the Project? Start Here!**: If you're new to this project, we highly recommend starting with our "N00b's Guide" for a simplified introduction to running the models and understanding the basics:
    *   [Getting Started: A N00b's Guide to libminizinc](n00b_guide.md)

## 3. Tutorials

Dive into the foundational concepts of our hyperspace embedding project, exploring lambda calculus, MiniZinc, and how we're giving abstract programs a concrete home.

*   [Episode 1: Introduction to Hyperspace and Lambda Embedding](tutorial/episode1/001_intro.md)
*   [Reproducing SIGSEGV on MiniZinc Model Return](tutorials/reproduce_sigseggv_on_model_return.md)

## 4. Development Guidelines

Adherence to these guidelines is crucial for contributing to this project:

*   **No Direct Edits**: Strictly adhere to the "add-only, never edit" philosophy. All changes are implemented by creating new, composable modules that supersede existing functionality. Refer to [No Direct Edits SOP](sops/no_direct_edits_sop.md) for detailed procedures.
*   **Proof Tapes for Reproducibility**: Every model run automatically generates a "proof tape" in the `proof_tapes/` directory. This tape captures the exact version vector and all `.mzn` and `.dzn` files used, ensuring complete reproducibility of experimental results. See [Run Model SOP v3](sops/run_model_sop_v3.md) for more.
*   **Integer Discretization**: A key technical aspect of this project involves discretizing floating-point values into integers to enable solving with Constraint Programming (CP) solvers like Gecode. This requires careful scaling of all coefficients and constants and adaptation of mathematical operations. Refer to [Project Workflow SOP](sops/project_workflow.md) for details.
*   **Gecode Integration**: Specific procedures are in place for building, configuring, and integrating Gecode as a solver. Troubleshooting steps for common build issues and solver discovery are documented. See [Project Workflow SOP](sops/project_workflow.md).
*   **Performance Analysis and Optimization**: To systematically identify and address performance bottlenecks in MiniZinc models, particularly the `v6` embedding model, we follow a structured deconstruction and reconstruction process. This involves incremental reintroduction of complexity and rigorous performance measurement at each step. For detailed procedures and recent findings, refer to [MiniZinc Model Performance Analysis and Debugging Report](performance_analysis_report.md).

## 5. Standard Operating Procedures (SOPs)

This project adheres to a strict set of Standard Operating Procedures (SOPs) to ensure consistency, quality, and reproducibility. Please familiarize yourself with these documents:

*   [Rust FFI Development and "One Declaration Per File" Refactoring SOP](sops/rust_ffi_refactoring_sop.md)
*   [Advanced Testing, Coverage, and Profiling for Rust-MiniZinc FFI Projects SOP](sops/advanced_testing_profiling_ffi_v2.md)
*   [Monotonic Epic Idea SOP](sops/monotonic_epic_idea_sop.md)
*   [No Direct Edits SOP](sops/no_direct_edits_sop.md)
*   [Project Workflow SOP](sops/project_workflow.md)
*   [Run Model SOP v3](sops/run_model_sop_v3.md)
*   [MiniZinc v6 Model Reconstruction SOP](sops/v6_reconstruction_sop.md)
*   [Backpack Filling SOP](sops/backpack_filling_sop.md)
*   [Eigenvector of Athena SOP](sops/eigenvector_of_athena_sop.md)
*   [Muse SOP](sops/muse_sop.md)
*   [QA DZN Generation Verification SOP](sops/qa_dzn_generation_verification.md)
*   [Tutorial Livestream Mode SOP](sops/tutorial_livestream_mode.md)
*   [MiniZinc Model Performance Analysis and Debugging Report](performance_analysis_report.md)
*   [Profile-Driven Semantic Test Generation for C++ Oxidation SOP](sops/profile_driven_semantic_test_generation_sop.md)

## 6. Architecture

*   [Refactoring the MiniZinc Parser](architecture/parser_refactoring.md)

## 7. Plans

This section contains various plans and strategic documents for the project.

*   [Project Plan](plans/plan.md)

## 8. FFI Documentation

This section provides documentation related to the Foreign Function Interface (FFI) with MiniZinc.

*   [Bindings Documentation](ffi/bindings.md)
*   [Bug Report: MiniZinc FFI GC Assertion](ffi/bug_report_minizinc_ffi_gc_assertion.md)

## 9. Model Documentation

This section provides detailed documentation and analysis of the MiniZinc models within this project.

*   [MiniZinc Models Overview](minizinc_models_overview.md)

## 10. Recent Model Analysis and Debugging

This section documents recent findings and debugging efforts related to the MiniZinc embedding models.

*   [MiniZinc Model Performance Analysis and Debugging Report](performance_analysis_report.md)

## 9. Glossary

*   [Glossary Index](glossary/index.md)

## 10. Poems

*   [Poems Index](poems/index.md)

## 11. Project Insights

*   [LLM FAQ: Open Questions on the Project Vision](llm_faq.md)

## 12. Contributing

Contributions that align with the project's vision and adhere to its unique development philosophy are welcome. Please familiarize yourself with the SOPs in the `docs/sops/` directory before contributing.

## 13. License

Distributed under the Mozilla Public License Version 2.0. See `LICENSE` for more information.

## 14. Acknowledgements

This research was partially funded by the Australian Government through the Australian Research Council Industrial Transformation Training Centre in Optimization Technologies, Integrated Methodologies, and Applications ([OPTIMA](https://optima.org.au)), Project ID IC200100009.

## 15. Contact

🏛 **MiniZinc Community**

- Website: [https://www.minizinc.org/](https://www.minizinc.org/)
- StackOverflow: [https://stackoverflow.com/questions/tagged/minizinc](https://stackoverflow.com/questions/tagged/minizinc)
- Google Groups: [https://groups.google.com/g/minizinc](https://groups.google.com/g/minizinc)

🏛 **Monash Optimisation Group**

- Website: [https://www.monash.edu/it/dsai/optimisation](https://www.monash.edu/it/dsai/optimisation)
