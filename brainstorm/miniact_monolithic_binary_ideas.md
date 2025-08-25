# Brainstorm: "Miniact" and Monolithic Binary

## Problem Statement
The current development and deployment process involves managing multiple external executables and libraries (e.g., MiniZinc, Gecode, various C++ libraries) and ensuring correct environment variable settings (like `LD_LIBRARY_PATH`). This can lead to complex setup procedures, environment-specific issues, and challenges in ensuring consistent and reproducible builds and tests across different environments (local, CI/CD, various operating systems like Android).

## Core Idea: "Miniact" and Monolithic Binary
The goal is to create a highly self-contained and portable solution by packaging essential tools and libraries into a single, monolithic executable. This "Miniact" would simplify deployment, enhance reproducibility, and streamline the development workflow. The inspiration is similar to BusyBox, where many common Unix utilities are combined into a single executable.

## Key Components/Features

### 1. Chroot Environment
*   **Purpose:** To provide an isolated and controlled execution environment, ensuring that the "Miniact" operates consistently regardless of the host system's configuration. This prevents conflicts with system-wide libraries or tools.
*   **Mechanism:** The "Miniact" would be capable of setting up a minimal chroot environment on the fly, containing only the necessary files and directories for its operation.

### 2. Static Linking/Bundling
*   **Rust Components:** Prioritize static linking for all Rust crates to eliminate external Rust library dependencies.
*   **C/C++ Components (MiniZinc, Gecode, etc.):**
    *   Investigate options for static compilation of MiniZinc and Gecode. This might involve modifying their build systems (CMake files) to produce static libraries.
    *   If static linking is not feasible or desirable for all components, explore bundling shared libraries (`.so` files) directly within the "Miniact" package. These bundled libraries would then be made accessible to the "Miniact" via `LD_LIBRARY_PATH` or similar mechanisms within the chroot.

### 3. Orchestration
*   **Centralized Control:** A primary entry point (the "Miniact" executable itself) would orchestrate all operations.
*   **Workflow Management:** This orchestrator would handle:
    *   Setting up the chroot environment.
    *   Placing bundled binaries and libraries within the chroot.
    *   Configuring environment variables (e.g., `LD_LIBRARY_PATH`) within the chroot.
    *   Executing specific commands (e.g., `cargo build`, `cargo test`, MiniZinc model runs, documentation updates) by calling the appropriate internal functions or bundled executables.

### 4. Stripping/Optimization
*   **Size Reduction:** Implement techniques to reduce the size of the final monolithic binary. This includes:
    *   Stripping debug symbols.
    *   Using link-time optimization (LTO).
    *   Potentially using tools like `upx` for further compression (though this might impact performance).
    *   Careful selection of features and dependencies to include.

## Benefits
*   **Simplified Deployment:** Distribute a single file instead of managing multiple packages and dependencies.
*   **Improved Reproducibility:** Consistent behavior across different environments due to isolated chroot and bundled dependencies.
*   **Reduced Dependency Management:** Less reliance on system-wide installations and package managers.
*   **Enhanced Portability:** Easier to move and run the entire development/testing environment.

## Challenges
*   **Complexity of Implementation:** Building a robust monolithic binary and chroot environment is a significant engineering effort, especially for cross-platform compatibility.
*   **Maintenance Overhead:** Keeping the bundled components updated and ensuring compatibility can be challenging.
*   **Initial Build Times:** The process of compiling and bundling everything into a single binary might be time-consuming.
*   **Licensing:** Careful consideration of licenses for all included third-party components is crucial to ensure compliance.

## Related Ideas/Context

### `libmzn.so` Linking Issue
The recent resolution of the `libmzn.so` linking issue in `minizinc_ffi` tests highlights the need for robust dependency management. The `LD_LIBRARY_PATH` solution, while effective, is a manual step that the "Miniact" could automate and encapsulate. The "Miniact" would ensure that `libmzn.so` (and other necessary shared libraries) are always found by the FFI tests, either by static linking or by correctly setting the `LD_LIBRARY_PATH` within its controlled environment.

### GitHub Actions Integration
The "Miniact" could be integrated into GitHub Actions workflows. Instead of individual steps for building MiniZinc, setting paths, and running tests, a single "Miniact" step could orchestrate the entire process, making the CI/CD pipeline cleaner and more reliable.

### BusyBox as an Inspiration
BusyBox serves as a strong inspiration for the "Miniact" concept. Its success in providing a compact, monolithic executable for embedded systems demonstrates the feasibility and benefits of packaging multiple utilities into one. The "Miniact" would apply this principle to the `libminizinc` development and testing ecosystem.
