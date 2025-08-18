# Test Plan and Orchestration: Leveraging the Rust Busy Box

## Date: August 16, 2025

## 1. Introduction
This document outlines the updated test plan and orchestration strategy for the `libminizinc` project, transitioning from individual shell scripts to leveraging the new Rust Busy Box (`ragit-bootstrap`). The goal is to create a more robust, unified, and maintainable testing infrastructure.

## 2. Core Principles
*   **Unified Testing:** All tests will be orchestrated through `ragit-bootstrap` subcommands.
*   **Declarative Orchestration:** `Makefile` will serve as the primary declarative layer for defining test targets.
*   **Robustness:** Leverage Rust's error handling for clearer test failures and better debugging.
*   **Reproducibility:** Continue to emphasize proof tapes for all MiniZinc model runs.

## 3. Updated Makefile Structure (Conceptual)

The main `Makefile` (e.g., at the project root) will be updated to reflect the new `ragit-bootstrap` commands.

```makefile
# Conceptual Makefile for libminizinc project

.PHONY: all clean test build bootstrap-zos

# Default target
all: build test

# Build targets
build:
	@echo "--- Building all core components ---"
	./target/release/ragit-bootstrap build all

build-gecode:
	./target/release/ragit-bootstrap build gecode

build-libminizinc:
	./target/release/ragit-bootstrap build libminizinc

build-ffi:
	./target/release/ragit-bootstrap build ffi

build-rust-ffi:
	./target/release/ragit-bootstrap build rust-ffi

build-ffi-declarations:
	./target/release/ragit-bootstrap build ffi-declarations

build-solvers:
	./target/release/ragit-bootstrap build solvers

build-coverage:
	./target/release/ragit-bootstrap build coverage

# Test targets
test: test-c-abi test-rust-ffi test-minizinc-models

test-c-abi:
	@echo "--- Running C ABI tests ---"
	./target/release/ragit-bootstrap test c-abi

test-rust-ffi:
	@echo "--- Running Rust FFI tests ---"
	./target/release/ragit-bootstrap test rust-ffi

test-minizinc-models:
	@echo "--- Running MiniZinc model tests ---"
	./target/release/ragit-bootstrap test minizinc-models

test-dzn-gen:
	@echo "--- Testing DZN generation and verification ---"
	./target/release/ragit-bootstrap test dzn-gen 10 100 # Example parameters

test-dzn-gen-rust:
	@echo "--- Testing Rust DZN generator ---"
	./target/release/ragit-bootstrap test dzn-gen-rust 5 # Example parameter

test-coverage:
	@echo "--- Running coverage tests ---"
	./target/release/ragit-bootstrap test coverage

# Run targets (for specific model executions)
run-embedding-v6:
	@echo "--- Running Embedding Model v6 ---"
	./target/release/ragit-bootstrap run embedding-v6 v6 v1 v1 v1 v1 v1 # Example parameters

run-embedding-v7:
	@echo "--- Running Embedding Model v7 ---"
	./target/release/ragit-bootstrap run embedding-v7 v6 v1 v1 v1 v1 10 # Example parameters

run-minimal-mzn:
	@echo "--- Running Minimal MiniZinc Model ---"
	./target/release/ragit-bootstrap run minimal v6 v1 v1 v1 v1 v1 # Example parameters

run-test-driver:
	@echo "--- Running MiniZinc Test Driver ---"
	./target/release/ragit-bootstrap run driver 10 100 # Example parameters

# Debug targets
debug-reproduce-crash:
	@echo "--- Reproducing FFI Crash ---"
	./target/release/ragit-bootstrap debug reproduce-crash

debug-reproduce-ffi-bug:
	@echo "--- Reproducing Specific FFI Bug ---"
	./target/release/ragit-bootstrap debug reproduce-ffi-bug

debug-v7-tests:
	@echo "--- Running Focused v7 Debug Tests ---"
	./target/release/ragit-bootstrap debug v7-tests

# Clean targets
clean:
	@echo "--- Cleaning all build artifacts ---"
	./target/release/ragit-bootstrap clean all

clean-build:
	./target/release/ragit-bootstrap clean build

clean-coverage:
	./target/release/ragit-bootstrap clean coverage

clean-proof-tapes:
	./target/release/ragit-bootstrap clean proof-tapes

# Bootstrap ZOS command (ultimate orchestrator)
bootstrap-zos:
	@echo "--- Starting ZOS Bootstrap Process ---"
	./target/release/ragit-bootstrap build all
	./target/release/ragit-bootstrap test all
	# Optional: Run initial embedding model after successful build and test
	./target/release/ragit-bootstrap run embedding-v7 v6 v1 v1 v1 v1 10
	@echo "--- ZOS Bootstrap Process Complete ---"
```

## 4. Test Execution Flow
*   Developers will primarily interact with the `Makefile` targets.
*   Each `Makefile` target will invoke the appropriate `ragit-bootstrap` subcommand.
*   `ragit-bootstrap` will handle the underlying logic, including subprocess execution, error handling, and proof tape generation.

## 5. Benefits of the New Orchestration
*   **Centralized Control:** All build, test, and run operations are managed through a single Rust executable.
*   **Improved Reliability:** Rust's type safety and error handling replace fragile shell script logic.
*   **Enhanced Maintainability:** Easier to add new tests or modify existing workflows within a structured Rust codebase.
*   **Better Feedback:** `ragit-bootstrap` can provide more detailed progress and error messages.
*   **Alignment with "Big Idea":** This unified orchestration is a step towards the project's self-aware and self-evolving system, where the system can orchestrate its own development and testing.
