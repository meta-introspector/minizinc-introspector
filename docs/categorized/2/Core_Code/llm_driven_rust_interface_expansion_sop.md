# SOP: LLM-Driven Rust Interface Expansion

This Standard Operating Procedure (SOP) outlines the strategy for expanding the Rust interface of `libminizinc` through the intelligent assistance of profile-driven Large Language Model (LLM) agents. This process aims to deepen the integration between Rust and MiniZinc, accelerate code oxidation, and enhance the system's self-awareness.

## 1. Introduction

The project's vision includes a significant role for LLM agents in evolving the codebase, particularly in the context of the Rust Foreign Function Interface (FFI) and the oxidation of C++ code to Rust. This SOP details how LLM agents, guided by specific profiles, will drive this expansion.

## 2. LLM Agent Profiles for Rust Interface Expansion

LLM agents will be assigned specialized profiles to guide their behavior and focus during the Rust interface expansion. These profiles leverage the agent's inherent capabilities while directing them towards specific technical objectives.

*   **"Rust FFI Expert" Profile**:
    *   **Focus**: Generating and refining Rust FFI bindings, ensuring correct type mappings, memory management, and robust error propagation across the C++/Rust boundary.
    *   **Tasks**: Proposing new FFI function declarations, generating safe Rust wrappers, identifying ABI compatibility issues, and suggesting solutions for cross-language data exchange.
    *   **Metrics**: FFI binding correctness (compilation, runtime errors), memory safety, error handling completeness.

*   **"Rust Code Oxidizer" Profile**:
    *   **Focus**: Rewriting existing `libminizinc` C++ code into idiomatic Rust, leveraging coverage data and semantic understanding to prioritize and execute oxidation tasks.
    *   **Tasks**: Analyzing C++ code for Rust conversion opportunities, generating equivalent Rust implementations, ensuring performance parity, and integrating new Rust modules into the existing build system.
    *   **Metrics**: C++ line coverage reduction (as Rust replaces C++), Rust code quality (linting, style adherence), performance benchmarks.

*   **"Rust Test Generator" Profile**:
    *   **Focus**: Generating new Rust tests (both unit and integration) that specifically target uncovered C++ code paths, as identified by the MiniZinc coverage model.
    *   **Tasks**: Analyzing coverage reports and semantic gaps, proposing MiniZinc models and data files for test cases, generating Rust FFI calls to exercise these models, and integrating new tests into the CI pipeline.
    *   **Metrics**: C++ line coverage increase, bug discovery rate, test suite execution time.

## 3. Enhancing Rust FFI for LLM-Driven Operations

To enable LLM agents to effectively drive the Rust interface expansion, the Rust FFI itself needs to be enhanced to expose more MiniZinc functionality and facilitate structured data exchange.

*   **Expose More MiniZinc Functionality**:
    *   **Objective**: Identify and expose additional MiniZinc C++ functions via the FFI that are crucial for LLM agents to perform more complex tasks. This includes functions for direct manipulation of MiniZinc Abstract Syntax Trees (ASTs), advanced solver control, detailed model introspection, and access to internal data structures.
    *   **Process**: LLMs (guided by the "Rust FFI Expert" profile) can analyze MiniZinc C++ headers and documentation to propose new FFI functions. Human review and implementation would follow.
    *   **Reference**: [SOP: Extending the Rust FFI for MiniZinc](sops/rust_ffi_extension_sop.md)

*   **Structured Data Exchange**:
    *   **Objective**: Refine the FFI to facilitate more structured and efficient data exchange between Rust and MiniZinc. This is critical for LLMs to consume and produce complex data types.
    *   **Mechanism**: Define clear Rust structs that mirror MiniZinc C++ data structures (e.g., AST nodes, solver solutions, profiling data). Implement serialization/deserialization mechanisms across the FFI boundary.
    *   **Benefit**: Reduces the need for manual parsing of raw data, allowing LLMs to work with higher-level abstractions.

*   **Robust Error Propagation**:
    *   **Objective**: Ensure granular and informative error propagation across the FFI boundary.
    *   **Mechanism**: Implement custom error types in Rust that map directly to MiniZinc C++ error codes or exceptions. Ensure that FFI calls return `Result` types in Rust, providing clear success/failure indications and detailed error messages.
    *   **Benefit**: Allows LLMs to receive precise feedback on FFI-related issues, enabling more effective self-correction and refinement.

## 4. Implementation and Integration (High-Level)

The implementation of LLM-driven Rust code generation and refinement will follow an iterative process, integrated with the project's OODA loop and continuous integration pipeline.

*   **LLM-Assisted FFI Binding Generation**: LLMs will be prompted to generate Rust FFI bindings based on C++ function signatures and documentation.
*   **LLM-Driven Oxidation Workflow**: LLMs analyze C++ code and coverage, generate Rust equivalents, and verify them through CI.
*   **LLM-Driven Rust Test Generation**: LLMs analyze coverage gaps (from MiniZinc coverage model), generate Rust tests, and integrate them into the test suite.

This SOP will be continuously refined as the project progresses and LLM capabilities evolve.