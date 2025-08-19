# `zos-bootstrap-main` Overview

The `zos-bootstrap-main` crate serves as the primary command-line interface (CLI) entry point for a comprehensive bootstrap and development toolchain. Built using the `clap` crate, it provides a unified interface for various operations related to building, testing, running, and analyzing components within the project.

## Functionality

- **Dynamic `LD_LIBRARY_PATH` Configuration:** Automatically determines the project root and sets the `LD_LIBRARY_PATH` environment variable to include the `build` directory. This is crucial for ensuring that dynamically linked libraries (such as `libminizinc`) are correctly found at runtime.
- **Command Dispatching:** Parses command-line arguments and dispatches control to specific handler functions for each subcommand. This modular design allows for easy extension with new functionalities.
- **Supported Commands:** The CLI supports a wide range of commands, including (but not limited to):
    - `build`: Handles project compilation.
    - `test`: Executes project tests.
    - `run`: Runs compiled executables.
    - `debug`: Provides debugging functionalities.
    - `clean`: Cleans build artifacts.
    - `extract-constants`: Extracts constants from source code.
    - `generate-params`: Generates MiniZinc parameters.
    - `generate-constants-file`: Generates a constants file.
    - `analyze-constants`: Analyzes extracted constants (placeholder).
    - `ast-to-minizinc`: Converts Abstract Syntax Trees (ASTs) to MiniZinc models.
    - `code-search`: Performs code searches.
    - `bootstrap`: Initiates a bootstrapping process (placeholder).
    - `self-optimize`: Executes self-optimization routines.
    - `test-ast-to-minizinc`: Tests AST to MiniZinc conversion.
    - `analyze-duplicates`: Analyzes code for duplicates.

## Purpose

This CLI application is central to the project's development workflow, providing:
- A streamlined way to interact with various build and analysis tools.
- Automation for common development tasks.
- A foundation for continuous integration and self-improvement processes.
- A mechanism for integrating different components of the `zos_bootstrap` ecosystem.
