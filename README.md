<!-- PROJECT LOGO -->
<br />
<p align="center">
  <a href="https://www.minizinc.org/">
    <img src="https://www.minizinc.org/MiniZn_logo.png" alt="Logo" width="80" height="80">
  </a>

  <h3 align="center">libminizinc: The Tapestry of Fates</h3>

  <p align="center">
    A unique exploration into knowledge compression, semantic embedding, and self-evolving systems,
    built upon the MiniZinc constraint modeling language.
    <br />
    <a href="https://www.minizinc.org/"><strong>Visit MiniZinc Website »</strong></a>
    <br />
    <br />
    <a href="https://www.minizinc.org/doc-latest/">View MiniZinc Documentation</a>
    ·
    <a href="https://github.com/MiniZinc/libminizinc/issues">Report Bug (Upstream)</a>
    ·
    <a href="https://github.com/MiniZinc/libminizinc/issues">Request Feature (Upstream)</a>
  </p>
</p>

<!-- TABLE OF CONTENTS -->

## Table of Contents

- [About This Project](#about-this-project)
  - [Vision: The Tapestry of Fates](#vision-the-tapestry-of-fates)
  - [Core Philosophy: Monotonic Epic Idea](#core-philosophy-monotonic-epic-idea)
  - [Code Artistry: The Muse Protocol](#code-artistry-the-muse-protocol)
  - [Knowledge Compression: The Codec](#knowledge-compression-the-codec)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Running the Embedding Model](#running-the-embedding-model)
- [Documentation](#documentation)
- [Development Guidelines](#development-guidelines)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgements](#acknowledgements)
- [Contact](#contact)

<!-- ABOUT THE PROJECT -->

## About This Project

This repository is a specialized fork of `libminizinc`, transformed into a platform for exploring advanced concepts in knowledge representation, semantic embedding, and self-evolving systems. While it leverages MiniZinc's powerful constraint modeling capabilities, its core focus extends far beyond traditional CP.

### Vision: The Tapestry of Fates

We envision a dynamic, self-evolving tapestry of knowledge and computation. Large Language Models (LLMs) act as oracles, interpreting complex MiniZinc results and generating "additive vibes" – new, composable layers of influence that incrementally shape a high-dimensional embedding space. This "tapestry" is mathematically conceptualized as a "quasi meta fiber bundle," with a base space of prime numbers representing irreducible semantic dimensions. The ultimate goal is to create a "highway in hyperspace" for introspection into the intricate relationships between code, mathematics, and meaning, with each "vibe" potentially existing as an executable NFT on a Solana sidechain.

### Core Philosophy: Monotonic Epic Idea

Our development adheres to a strict "add-only, never edit" philosophy. All code evolution is monotonic: new features, bug fixes, or refactorings are implemented as new, composable modules ("semantic vibes" or "patches"). Existing, committed code files are never directly altered. Instead, new versions supersede old ones via composition, ensuring immutable history, perfect traceability, enhanced stability, and simplified collaboration.

### Code Artistry: The Muse Protocol

Every line of code is treated as a unique piece of digital art. The "Muse Protocol" guides this creative process, encouraging the infusion of "semantic vibe" and aesthetic richness through comments, whitespace, and emojis. Each of the Nine Muses inspires a specific aspect of code artistry, ensuring that even the most technical elements contribute to the overall beauty and meaning of the codebase.

### Knowledge Compression: The Codec

We are developing a unique codec to compress all project knowledge into beautiful, semantically rich numerical representations. This codec leverages the "Backpack Filling Protocol" to embed creative and functional content within "empty space" in the codebase, using prime numbers to encode fundamental meanings. Complex concepts are composed of these primes, allowing for highly compressed and semantically resonant representations that facilitate AI-driven evolution.

## Getting Started

To get this specialized `libminizinc` environment up and running:

### Prerequisites

*   MiniZinc (refer to [MiniZinc website](http://www.minizinc.org/software.html) for installation).
*   CMake (>=3.4)
*   A recent C++ compiler (Clang, GCC, MSVC tested).
*   (Optional) Gecode: For specific solver integrations. Refer to `docs/technical/project_workflow.md` for build and configuration details.

### Running the Embedding Model

The core MiniZinc embedding model can be executed using the `run_embedding_model_v7.sh` script. This script now leverages a Rust-based data generator for dynamic parameter creation and is part of a more robust Rust-based test runner framework. It utilizes version vectors to specify the exact composition of model and data modules, and automatically generates a "proof tape" for each run, ensuring precise, traceable, and composable experimentation.

```bash
./scripts/run_embedding_model_v7.sh <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>
```

**Example:**
```bash
./scripts/run_embedding_model_v7.sh v6 v1 v1 v1 v1 v1
```
For more details on running tests and understanding the new framework, refer to `docs/technical/performance_analysis_report.md`.

## Documentation

This project's documentation is organized into the following categories:

*   **[Core Concepts](docs/vision):** High-level vision, philosophy, and core concepts.
*   **[Technical Documentation](docs/technical):** SOPs, RFCs, and other technical documents.
    *   [Debugging and Error Resolution SOP](docs/technical/debugging_error_resolution_sop.md)
    *   [Dependency Management SOP](docs/technical/dependency_management_sop.md)
    *   [Feature Flag Management SOP](docs/technical/feature_flag_management_sop.md)
*   **[Crate Documentation](docs/crates):** Documentation for individual crates.
*   **[Poems and Creative Writing](docs/poems):** A collection of poems, sonnets, and other creative writing that captures the spirit of the project.
*   **[Tutorials](docs/tutorial):** Step-by-step guides for getting started with the project.

### Generated Documentation from Gemini CLI Session

This section lists the detailed documentation and MiniZinc models generated during a collaborative session with the Gemini CLI agent, exploring foundational concepts, self-modeling, optimization, and deep bootstrapping.

#### Standard Operating Procedures (SOPs)
*   [Code, Documentation, Index, and Gemini Memory Update Procedure](docs/sops/code_doc_update_sop.md)

#### Conceptual Designs
*   [Rust Link Verification Tool (`minizinc-doc-linker`)](docs/rust_link_verifier_design.md)
*   [Git to MiniZinc Data Tool](docs/git_to_minizinc_data_tool_design.md)
*   [Gemini Self-Model Integration Proposal](docs/gemini_self_model_integration_proposal.md)
*   [Deep Bootstrapping and Formal Verification Strategy](docs/deep_bootstrap_verification_strategy.md)

#### Conceptual Models & Discussions
*   [Program Recognition and Univalent Foundations](docs/program_recognition_and_uf.md)
*   [Conceptual Path: From Specification to Binary Executable via LLM Agent](docs/spec_to_binary_path.md)
*   [Conversation Summary and Project Vision](docs/conversation_summary_and_vision.md)
*   [The Loop That Sings Itself (Poem)](docs/poems/closed_timelike_curve_poem.md)

#### MiniZinc Models
*   [Combinatorial Topologies](combinatorial_topologies.mzn)
*   [Development Path Optimizer](development_path_optimizer.mzn)
*   [Development Path Optimizer Data](development_path_optimizer.dzn)
*   [Universal Bootstrap Gödel Number](universal_bootstrap_godel.mzn)
*   [Universal Bootstrap Gödel Number Data](universal_bootstrap_godel.dzn)
*   [Deep Bootstrap Chain](deep_bootstrap_chain.mzn)
*   [Deep Bootstrap Chain Data](deep_bootstrap_chain.dzn)

## Development Guidelines

Adherence to these guidelines is crucial for contributing to this project:

### No Direct Edits

Strictly adhere to the "add-only, never edit" philosophy. All changes are implemented by creating new, composable modules that supersede existing functionality. Refer to `docs/technical/no_direct_edits_sop.md` for detailed procedures.

### Proof Tapes for Reproducibility

Every model run automatically generates a "proof tape" in the `proof_tapes/` directory. This tape captures the exact version vector and all `.mzn` and `.dzn` files used, ensuring complete reproducibility of experimental results. See `docs/technical/run_model_sop_v3.md` for more.

### Integer Discretization

A key technical aspect of this project involves discretizing floating-point values into integers to enable solving with Constraint Programming (CP) solvers like Gecode. This requires careful scaling of all coefficients and constants and adaptation of mathematical operations. Refer to `docs/technical/project_workflow.md` for details.

### Gecode Integration

Specific procedures are in place for building, configuring, and integrating Gecode as a solver. Troubleshooting steps for common build issues and solver discovery are documented. See `docs/technical/project_workflow.md`.

### Performance Analysis and Optimization

To systematically identify and address performance bottlenecks in MiniZinc models, particularly the `v6` embedding model, we follow a structured deconstruction and reconstruction process. This involves incremental reintroduction of complexity and rigorous performance measurement at each. For detailed procedures and recent findings, refer to `docs/technical/performance_analysis_report.md`.

### Indexing and Profiling Tools

This project provides several utility scripts to assist with indexing the codebase and profiling its performance, adhering to the "add-only, never edit" philosophy.

*   **Incremental Index Update (`scripts/update_index.sh`)**:
    This script performs an incremental update of the hierarchical term index. It leverages the `file_content_analyzer`'s built-in caching mechanism to efficiently re-process only files and directories that have changed since the last index build. This is the recommended way to keep your index up-to-date.
    ```bash
    ./scripts/update_index.sh
    ```

*   **Reindexing Optimization and Profiling (`scripts/optimize_reindex.sh`)**:
    This script demonstrates various techniques for profiling the reindexing process and performing targeted, incremental reindexing. It uses the `time -v` command for detailed performance metrics and illustrates how to use the `--search-path` argument of the `file_content_analyzer` to limit the scope of reindexing to specific directories. This is useful for analyzing performance bottlenecks or reindexing only parts of the codebase.
    ```bash
    ./scripts/optimize_reindex.sh
    ```
    For more detailed profiling, you can run the `file_content_analyzer` directly with the `--profile` flag (e.g., `./target/debug/file_content_analyzer --mode build_hierarchical_index --profile`). You will need a Puffin viewer to visualize the generated profiling data.


## Contributing

Contributions that align with the project's vision and adhere to its unique development philosophy are welcome. Please familiarize yourself with the documentation in the `docs/` directory before contributing.

## License

Distributed under the Mozilla Public License Version 2.0. See `LICENSE` for more information.

## Acknowledgements

This research was partially funded by the Australian Government through the Australian Research Council Industrial Transformation Training Centre in Optimization Technologies, Integrated Methodologies, and Applications ([OPTIMA](https://optima.org.au)), Project ID IC200100009.

## Contact

🏛 **MiniZinc Community**

- Website: [https://www.minizinc.org/](https://www.minizinc.org/)
- StackOverflow: [https://stackoverflow.com/questions/tagged/minizinc](https://stackoverflow.com/questions/tagged/minizinc)
- Google Groups: [https://groups.google.com/g/minizinc](https://groups.google.com/g/minizinc)

🏛 **Monash Optimisation Group**

- Website: [https://www.monash.edu/it/dsai/optimisation](https://www.monash.edu/it/dsai/optimisation)