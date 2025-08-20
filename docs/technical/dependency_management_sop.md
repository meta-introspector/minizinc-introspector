# Standard Operating Procedure: Dependency Management

## 1. Purpose
This SOP outlines the controlled process for managing external Rust dependencies within the `libminizinc` project. It emphasizes adherence to the "Monotonic Epic Idea" philosophy, prioritizing vendoring or forking crates into the `vendor/` directory to ensure a self-contained, reproducible, and auditable codebase, while minimizing direct modifications to upstream code.

## 2. Scope
This SOP applies to all external Rust crates and their dependencies used in the `libminizinc` project.

## 3. Principles
*   **Control:** Maintain full control over dependencies by vendoring or forking.
*   **Reproducibility:** Ensure builds are reproducible regardless of external `crates.io` availability.
*   **Security:** Reduce supply chain risks by vetting and localizing dependencies.
*   **Stability:** Minimize unexpected breaking changes from upstream updates.
*   **Transparency:** Document all dependency decisions and their rationale.
*   **Monotonicity:** Integrate dependencies in an additive manner, avoiding direct modification of vendored source unless absolutely necessary and documented.

## 4. Procedure

### 4.1. Identifying and Evaluating New Dependencies
1.  **Need Assessment:** Clearly define the problem the new dependency aims to solve and verify it cannot be solved with existing internal modules.
2.  **Search & Selection:** Identify potential crates on `crates.io` or GitHub. Prioritize crates with:
    *   Active maintenance and clear licensing.
    *   Minimal transitive dependencies.
    *   Good documentation and test coverage.
    *   Compatibility with the project's Rust edition and toolchain.
3.  **License Review:** Verify compatibility of the dependency's license with the project's license (Mozilla Public License Version 2.0).
4.  **Security Audit (Initial):** Perform a preliminary review for known vulnerabilities or suspicious code patterns.

### 4.2. Integration: Vendoring or Patching
The preferred method for integrating new dependencies is vendoring or using `[patch]` directives in `Cargo.toml`.

1.  **Vendoring (Preferred):**
    *   Clone or copy the dependency's source code into the `vendor/` directory within the `libminizinc` project root.
    *   Update the root `Cargo.toml` with `[patch.crates-io]` entries to redirect `crates.io` dependencies to the local `vendor/` path.
    *   Example `Cargo.toml` entry:
        ```toml
        [patch.crates-io]
        some-crate = { path = "vendor/some-crate" }
        ```
    *   Ensure all transitive dependencies of the vendored crate are also correctly resolved, either by vendoring them or by ensuring they are available on `crates.io` and compatible.
    *   If modifications are *absolutely necessary* within the vendored source (violating "no direct edits" for upstream code), these modifications *must* be minimal, clearly documented within the vendored crate's directory (e.g., `vendor/some-crate/PATCHES.md`), and justified by a formal Change Request.

2.  **Patching (for specific versions or minor fixes):**
    *   If vendoring the entire crate is not feasible or necessary, use `[patch.crates-io]` to point to a specific Git revision or local path for a temporary fix or a specific version.
    *   Example `Cargo.toml` entry:
        ```toml
        [patch.crates-io]
        some-crate = { git = "https://github.com/example/some-crate.git", rev = "abcdef123456" }
        ```
    *   This method should be used sparingly and with a clear plan for eventual full vendoring or upstream contribution.

### 4.3. Updating Dependencies
1.  **Review Changes:** Before updating a dependency, review its changelog and commit history for breaking changes, new features, or security fixes.
2.  **Update Vendored Source:** If vendored, pull the latest changes into the `vendor/` directory.
3.  **Update `Cargo.toml`:** Adjust version numbers in `Cargo.toml` as needed.
4.  **Test:** Run all relevant tests to ensure compatibility and prevent regressions.

### 4.4. Documentation
1.  **`Cargo.toml` Comments:** Add comments in `Cargo.toml` for complex dependency setups or specific version choices.
2.  **SOP Updates:** Update this SOP or create new ones for specific, complex dependency integration patterns.
3.  **`vendor/` READMEs:** Each vendored crate should ideally have a `README.md` or `ORIGIN.md` explaining its source, version, and any local modifications.

## 5. Expected Outcome
*   A stable and reproducible build environment.
*   Reduced exposure to external dependency changes.
*   Clear traceability of all external code.
*   Enhanced project security and maintainability.
*   Adherence to the "Monotonic Epic Idea" philosophy in dependency management.
