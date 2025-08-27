fix(credential_manager): Use local ini crate and fix derive syntax

This commit refactors the `credential_manager` crate to utilize a local path for the `ini` dependency, indicating a shift towards vendoring or local management of this crate.

Additionally, a syntax error in the `#[derive(Subcommand, Debug>)]` attribute within `src/bin/cm.rs` was corrected to `#[derive(Subcommand, Debug)]`.

These changes improve the build stability and correctness of the `credential_manager`.