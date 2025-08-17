# Rust Busy Box: Integration and Transition Plan (Conceptual)

## Date: August 16, 2025

## 1. Introduction
This document outlines the conceptual plan for integrating the new Rust Busy Box (`ragit-bootstrap`) into the `libminizinc` project and the strategy for transitioning away from the existing shell scripts. This process will adhere to the "Monotonic Epic Idea" by superseding, rather than directly modifying, existing components.

## 2. Integration Strategy

### 2.1. Project Structure Integration
The `ragit-bootstrap` executable will reside in a new top-level crate, `crates/ragit-bootstrap`, within the project's workspace.

```
libminizinc/
├── crates/
│   └── ragit-bootstrap/
│       ├── src/
│       └── Cargo.toml
├── scripts/                  # Existing shell scripts (will be superseded)
├── tools/                    # Existing Rust FFI and data generators
├── build/
├── ...
```

### 2.2. Build System Integration
The `ragit-bootstrap` tool itself will be built as part of the main project's `Cargo` workspace.

*   **`Cargo.toml` (root):** Add `crates/ragit-bootstrap` to the `[workspace.members]` section.
*   **Initial Build:** `cargo build --release -p ragit-bootstrap` will compile the busy box.

## 3. Transition Strategy: Superseding Shell Scripts

The transition will be gradual, following the "Monotonic Epic Idea" (add-only, never edit).

### 3.1. Phase 1: Parallel Existence and Documentation
*   **Action:** Implement the Rust Busy Box functionalities in parallel with the existing shell scripts.
*   **Documentation:** Update relevant SOPs and READMEs to introduce the new `ragit-bootstrap` commands as the preferred method, while noting the existence of the superseded shell scripts for historical context.
*   **Example:** For building Gecode, the documentation would state: "Use `ragit-bootstrap build gecode` (preferred) or `scripts/build_gecode.sh` (legacy)."

### 3.2. Phase 2: Orchestration Layer Shift
*   **Action:** Modify higher-level orchestration scripts (e.g., `build_and_test.sh`, `run_all_minizinc_tests.sh`) to call the new `ragit-bootstrap` commands instead of directly executing the individual shell scripts.
*   **Example:** `build_and_test.sh` would be modified to call `ragit-bootstrap build ffi` and `ragit-bootstrap test rust-ffi` instead of `cmake --build build/` and `reproduce_crash.sh`.
*   **Benefit:** This allows for a controlled transition, where the top-level scripts act as a shim, gradually deprecating the underlying shell scripts.

### 3.3. Phase 3: Direct Usage and Deprecation
*   **Action:** Encourage direct usage of `ragit-bootstrap` commands by developers.
*   **Deprecation:** Mark the superseded shell scripts as deprecated (e.g., by moving them to a `scripts/legacy/` directory or adding a clear deprecation notice at the top of the script). They will remain in the repository for historical traceability, but new development will exclusively use `ragit-bootstrap`.

## 4. Implementing "make bootstrap zos"

The `make bootstrap zos` command will serve as the ultimate top-level orchestrator, leveraging the new Rust Busy Box.

### 4.1. `Makefile` Integration
*   **Action:** Create or modify the main `Makefile` (e.g., at the project root) to include a `bootstrap-zos` target.
*   **Target Definition:** This target will primarily call the `ragit-bootstrap` executable with a sequence of commands to perform the full bootstrap process.

```makefile
# Conceptual Makefile snippet
.PHONY: bootstrap-zos

bootstrap-zos:
	@echo "--- Starting ZOS Bootstrap Process ---"
	# Build all core components
	./target/release/ragit-bootstrap build all
	# Run all tests
	./target/release/ragit-bootstrap test all
	# (Optional) Run initial embedding model
	./target/release/ragit-bootstrap run embedding-v7 v6 v1 v1 v1 v1
	@echo "--- ZOS Bootstrap Process Complete ---"
```

### 4.2. Benefits of `make bootstrap zos`
*   **Single Entry Point:** Provides a unified, high-level command for developers to bootstrap the entire system.
*   **Robustness:** Leverages the improved error handling and reliability of the Rust Busy Box.
*   **Self-Orchestration:** The `Makefile` acts as a declarative definition of the bootstrap process, which is then executed by the intelligent `ragit-bootstrap` tool.
*   **Alignment with "Big Idea":** This command represents the system's ability to orchestrate its own foundational setup, a step towards computational self-awareness.

## 5. Adherence to Monotonic Epic Idea
*   The Rust Busy Box is added as a new crate.
*   Shell scripts are superseded, not deleted.
*   The `Makefile` is modified to call the new tool, but the old scripts remain.
*   This ensures a traceable, additive evolution of the project's operational core.

This integration and transition plan aims for a smooth, controlled shift to a Rust-native bootstrap system, enhancing the project's robustness and aligning with its core philosophies.
