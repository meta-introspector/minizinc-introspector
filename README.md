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

The core MiniZinc embedding model can be executed using the `run_embedding_model_v6.sh` script. This script utilizes version vectors to specify the exact composition of model and data modules, and automatically generates a "proof tape" for each run, ensuring precise, traceable, and composable experimentation.

```bash
./scripts/run_embedding_model_v6.sh <main_model_version> <core_params_version> <kappa_params_version> <other_params_version> <relations_version> <vector_params_version>
```

**Example:**
```bash
./scripts/run_embedding_model_v6.sh v6 v1 v1 v1 v1 v1
```
For more details, refer to `docs/sops/run_model_sop_v3.md`.

### New to the Project? Start Here!

If you're new to this project, we highly recommend starting with our "N00b's Guide" for a simplified introduction to running the models and understanding the basics:

*   [Getting Started: A N00b's Guide to libminizinc](docs/n00b_guide.md)

## Recent Model Analysis and Debugging

This section documents recent findings and debugging efforts related to the MiniZinc embedding models.

### Analysis of v6 Model Unsatisfiability

When running the `v6` main model with `v1` data parameters (`./scripts/run_embedding_model_v6.sh v6 v1 v1 v1 v1 v1`), the model consistently resulted in `=====UNSATISFIABLE=====`. This indicates that, with the given constraints and parameters, no solution could be found.

The diagnostic process involved:
*   **Initial Debugging of Script Execution:** Enhancing `run_embedding_model_v6.sh` to provide immediate `head`/`tail` log output and display the full MiniZinc command for easier debugging.
*   **Resolving Include Path Issues:** Identifying and correcting an `include` path error in `embedding_sphere_final_v5.mzn` (which was temporarily modified during debugging) where `embedding_params_composed_v2.mzn` was being sought but the correct file was `embedding_params_core_composed_v3.mzn`. This was resolved by directly updating the `include` statement in `embedding_sphere_final_v5.mzn`.
*   **Addressing Undefined Identifiers:** A chain of "undefined identifier" errors (`kappa_global`, `epsilon`, `num_bindings`, `alpha_coeff`) were encountered. These were resolved by ensuring that `embedding_sphere_final_v5.mzn` (and by extension, `embedding_sphere_final_v6.mzn` as they share similar structures) explicitly included the necessary parameter composition files (`embedding_params_kappa_composed.mzn`, `embedding_params_other_composed.mzn`, `embedding_params_relations_v3.mzn`, `embedding_params_vector_composed_v3.mzn`) before their corresponding `.dzn` data files were processed.
*   **Pinpointing the Unsatisfiable Constraint:** The primary cause of the `=====UNSATISFIABLE===== `result for the `v6` model was identified as the unit norm constraint in `embedding_constraints.mzn`:
    ```minizinc
    constraint forall(i in 1..n) (
      sum(k in 1..d) (p_actual_values[i,k] * p_actual_values[i,k]) = PARTITION_SCALE * PARTITION_SCALE
    );
    ```
    This constraint, requiring the sum of squares of `p_actual_values` to exactly equal `PARTITION_SCALE * PARTITION_SCALE` (which is `1,000,000` with `PARTITION_SCALE = 1000`), proved too restrictive for the discrete integer values `p_actual_values` could take.
*   **Achieving Satisfiability:** By temporarily commenting out this specific constraint in `embedding_constraints.mzn`, the `v6` model became satisfiable, demonstrating that this constraint was indeed the bottleneck.

### Next Steps for Constraint Resolution

To achieve a satisfiable solution while maintaining the intent of the unit norm constraint, several approaches can be explored:
*   **Constraint Relaxation:** Modify the constraint to allow for a tolerance (e.g., `sum(...) <= PARTITION_SCALE * PARTITION_SCALE + tolerance`) or change the equality to an inequality (`<=` or `>=`).
*   **Parameter Adjustment:** Re-evaluate the `PARTITION_SCALE` value or the domain of `p_actual_values` to find a combination that makes the constraint achievable with discrete values.
*   **Alternative Modeling:** Explore alternative ways to model the unit norm concept within the discrete integer domain, potentially using different mathematical formulations or approximations.

### Debugging Script Enhancements

The `run_embedding_model_v6.sh` script has been enhanced to aid in debugging. It now:
*   Prints the current working directory.
*   Displays the full MiniZinc command being executed.
*   Shows the first 20 lines (`head -n 20`) of both `stdout.log` and `stderr.log` immediately after the MiniZinc run, providing quick access to diagnostic information.

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