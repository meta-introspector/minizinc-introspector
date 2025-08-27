## Task Document: Monolithic Application Integration

**Objective:**
To consolidate the core functionalities of `crq_updater`, `zos-bootstrap`, `vibe_analyzer`, `launchpad`, `tmux_controller`, `gemini_utils`, and `mini-act` into a single, unified Rust application. This integration aims to streamline development, simplify deployment, enhance inter-component communication, and lay further groundwork for formal verification and self-optimization.

**Scope:**

1.  **Identify Core Logic:** Extract the essential, non-redundant logic from each specified crate.
2.  **Unified CLI:** Design a single, comprehensive command-line interface using `clap` that exposes the functionalities of all integrated components as subcommands or arguments.
3.  **Internal Module Structure:** Organize the extracted code into a well-defined internal module structure within the new monolithic application, adhering to the "one declaration per file" principle and other project coding standards.
4.  **Dependency Management:** Consolidate and manage all external and internal dependencies within the single `Cargo.toml` of the monolithic application. Resolve any version conflicts or feature incompatibilities.
5.  **Inter-Component Communication:** Refactor existing inter-crate communication (e.g., `tmux_controller` calling `gemini_cli_manager`) to direct internal function calls or shared data structures within the monolithic application.
6.  **Logging:** Ensure all logging across the integrated components consistently uses `gemini_utils::gemini_eprintln!`.
7.  **Testing:** Develop a comprehensive test suite for the monolithic application, covering integrated functionalities and ensuring no regressions.
8.  **Documentation Update:** Update relevant documentation (e.g., `GEMINI.md`, `README.md`, SOPs, CRQs) to reflect the new monolithic architecture.

**High-Level Plan:**

1.  **Create New Workspace Member:** Initialize a new Rust crate (e.g., `crates/meta-orchestrator` or `crates/solfunmeme-core`) that will house the monolithic application.
2.  **Migrate `gemini_utils`:** As a foundational utility, `gemini_utils` (or its core logic) will be integrated early to ensure consistent logging.
3.  **Integrate `zos-bootstrap`:** Migrate `zos-bootstrap`'s core functionalities, especially file system operations and MiniZinc-related data processing.
4.  **Integrate `crq_updater`:** Port the logic for Git interaction, regex parsing, and Markdown file updates.
5.  **Integrate `vibe_analyzer`:** Incorporate code analysis and search capabilities.
6.  **Integrate `tmux_controller`:** Migrate `tmux` control functionalities. This might require careful handling of external `tmux` commands.
7.  **Integrate `launchpad`:** Port the high-level orchestration logic, adapting it to call internal modules instead of external binaries.
8.  **Integrate `mini-act`:** Incorporate the GitHub Actions workflow simulation capabilities. This will be a significant integration point.
9.  **Refactor and Consolidate:** Iteratively refactor the integrated code, remove redundancies, and ensure adherence to project standards.
10. **Build and Test:** Continuously build and run tests to catch integration issues early.

    

### Implementation Notes: `gemini_utils::gemini_eprintln!` Syntax

During the integration of `launchpad`'s `narrator` module, a critical learning point emerged regarding the correct syntax for `gemini_utils::gemini_eprintln!`. Unlike standard Rust formatting macros that use `{}` for placeholders, `gemini_eprintln!` adheres to the project's `kantspel` principles and requires the `::variable-name::` syntax for dynamic content insertion.

**Incorrect Usage (leading to "Too many positional arguments provided" or "Named argument ... is not used" errors):**
```rust
gemini_eprintln!("Hello, {name}!", name = my_variable);
gemini_eprintln!("Value: {}", my_variable);
```

**Correct Usage (using `::variable-name::` syntax):**
```rust
gemini_eprintln!("Hello, ::name::!", name = my_variable);
gemini_eprintln!("Value: ::my_variable::", my_variable = my_variable);
```

This ensures compliance with `kantspel` and proper interpretation by the macro. This lesson is crucial for consistent logging and self-documentation across the monolithic application.

**Expected Benefits:**

*   **Simplified Deployment:** A single binary for easier distribution and execution.
*   **Improved Performance:** Reduced overhead from inter-process communication.
*   **Enhanced Maintainability:** Centralized codebase for easier management and refactoring.
*   **Stronger Type Safety:** Direct function calls within a single application can leverage Rust's type system more effectively.
*   **Foundation for Formal Verification:** A more cohesive codebase simplifies the path towards formal verification of the entire system.
