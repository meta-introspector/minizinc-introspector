## Vendor Directory: External References for MiniZinc Integration

This directory contains external projects vendored as Git submodules. These projects (`minizin-js`, `minizinc-jll`, etc.) are included **purely as references** for the development of our Rust-based MiniZinc model and API integration.

**Purpose:**
*   To provide concrete examples of how MiniZinc is integrated with other language ecosystems (JavaScript, Julia).
*   To serve as a reference for API design and compatibility testing during the development of our native Rust MiniZinc bindings and model.

**Strategy:**
*   The presence of non-Rust code within these submodules does **not** imply a deviation from the project's core **Rust-only** philosophy.
*   The functionality demonstrated by these reference projects will eventually be reimplemented in Rust, or their APIs will be fully compatible with our Rust-based system.
*   Once the native Rust integration is robust and verified, these submodules will be re-evaluated, potentially updated to their latest versions, or their direct usage phased out in favor of our Rust implementations.

This approach ensures that our Rust-centric development benefits from existing integration patterns while maintaining a clean, consistent, and formally verifiable Rust codebase.
