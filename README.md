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
- [Tutorials](#tutorials)
  - [Episode 1: Introduction to Hyperspace and Lambda Embedding](#episode-1-introduction-to-hyperspace-and-lambda-embedding)
- [Development Guidelines](#development-guidelines)
  - [No Direct Edits](#no-direct-edits)
  - [Proof Tapes for Reproducibility](#proof-tapes-for-reproducibility)
  - [Integer Discretization](#integer-discretization)
  - [Gecode Integration](#gecode-integration)
  - [Performance Analysis and Optimization](#performance-analysis-and-optimization)
- [Recent Model Analysis and Debugging](#recent-model-analysis-and-debugging)
- [Model Documentation](#model-documentation)
- [Standard Operating Procedures (SOPs)](#standard-operating-procedures-sops)
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
*   (Optional) Gecode: For specific solver integrations. Refer to `docs/sops/project_workflow.md` for build and configuration details.

### Running the Embedding Model

The core MiniZinc embedding model can be executed using the `run_embedding_model_v7.sh` script. This script now leverages a Rust-based data generator for dynamic parameter creation and is part of a more robust Rust-based test runner framework. It utilizes version vectors to specify the exact composition of model and data modules, and automatically generates a "proof tape" for each run, ensuring precise, traceable, and composable experimentation.

```bash
./scripts/run_embedding_model_v7.sh <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>
```

**Example:**
```bash
./scripts/run_embedding_model_v7.sh v6 v1 v1 v1 v1 v1
```
For more details on running tests and understanding the new framework, refer to [MiniZinc Model Performance Analysis and Debugging Report](docs/performance_analysis_report.md).

### New to the Project? Start Here!

If you're new to this project, we highly recommend starting with our "N00b's Guide" for a simplified introduction to running the models and understanding the basics:

*   [Getting Started: A N00b's Guide to libminizinc](docs/n00b_guide.md)

## Recent Model Analysis and Debugging

This section documents recent findings and debugging efforts related to the MiniZinc embedding models.

*   [MiniZinc Model Performance Analysis and Debugging Report](docs/performance_analysis_report.md)

## Model Documentation

This section provides detailed documentation and analysis of the MiniZinc models within this project.

*   [MiniZinc Models Overview](docs/minizinc_models_overview.md)

## Tutorials

### Episode 1: Introduction to Hyperspace and Lambda Embedding

Dive into the foundational concepts of our hyperspace embedding project, exploring lambda calculus, MiniZinc, and how we're giving abstract programs a concrete home.

*   [Read Episode 1: Introduction to Hyperspace and Lambda Embedding](docs/tutorial/episode1/001_intro.md)

## Development Guidelines

Adherence to these guidelines is crucial for contributing to this project:

### No Direct Edits

Strictly adhere to the "add-only, never edit" philosophy. All changes are implemented by creating new, composable modules that supersede existing functionality. Refer to `docs/sops/no_direct_edits_sop.md` for detailed procedures.

### Proof Tapes for Reproducibility

Every model run automatically generates a "proof tape" in the `proof_tapes/` directory. This tape captures the exact version vector and all `.mzn` and `.dzn` files used, ensuring complete reproducibility of experimental results. See `docs/sops/run_model_sop_v3.md` for more.

### Integer Discretization

A key technical aspect of this project involves discretizing floating-point values into integers to enable solving with Constraint Programming (CP) solvers like Gecode. This requires careful scaling of all coefficients and constants and adaptation of mathematical operations. Refer to `docs/sops/project_workflow.md` for details.

### Gecode Integration

Specific procedures are in place for building, configuring, and integrating Gecode as a solver. Troubleshooting steps for common build issues and solver discovery are documented. See `docs/sops/project_workflow.md`.

### Performance Analysis and Optimization

To systematically identify and address performance bottlenecks in MiniZinc models, particularly the `v6` embedding model, we follow a structured deconstruction and reconstruction process. This involves incremental reintroduction of complexity and rigorous performance measurement at each step. For detailed procedures and recent findings, refer to [MiniZinc Model Performance Analysis and Debugging Report](docs/performance_analysis_report.md).

## Standard Operating Procedures (SOPs)

This project adheres to a strict set of Standard Operating Procedures (SOPs) to ensure consistency, quality, and reproducibility. Please familiarize yourself with these documents:

*   [Monotonic Epic Idea SOP](docs/sops/monotonic_epic_idea_sop.md)
*   [No Direct Edits SOP](docs/sops/no_direct_edits_sop.md)
*   [Project Workflow SOP](docs/sops/project_workflow.md)
*   [Run Model SOP v3](docs/sops/run_model_sop_v3.md)
*   [MiniZinc v6 Model Reconstruction SOP](docs/sops/v6_reconstruction_sop.md)
*   [Backpack Filling SOP](docs/sops/backpack_filling_sop.md)
*   [Eigenvector of Athena SOP](docs/sops/eigenvector_of_athena_sop.md)
*   [Muse SOP](docs/sops/muse_sop.md)
*   [QA DZN Generation Verification SOP](docs/sops/qa_dzn_generation_verification.md)
*   [Tutorial Livestream Mode SOP](docs/sops/tutorial_livestream_mode.md)
*   [MiniZinc Model Performance Analysis and Debugging Report](docs/performance_analysis_report.md)

## Contributing

Contributions that align with the project's vision and adhere to its unique development philosophy are welcome. Please familiarize yourself with the SOPs in the `docs/sops/` directory before contributing.

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