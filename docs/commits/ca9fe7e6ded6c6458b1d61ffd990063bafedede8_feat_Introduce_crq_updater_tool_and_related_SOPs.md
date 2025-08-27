feat: Introduce crq_updater tool and related SOPs

This commit introduces the `crq_updater` Rust tool, designed to automate the management of Change Request (CRQ) documentation.

Key changes include:
- Initial implementation of the `crq_updater` crate, including its `main.rs` and `Cargo.toml`.
- Addition of a new GitHub Actions workflow (`.github/workflows/test_crq_updater.yml`) for continuous integration testing of the `crq_updater`.
- Updates to `Cargo.lock` and `Cargo.toml` to include the new `crq_updater` crate.
- Updates to `GEMINI.md` and `README.md` to reflect the introduction of the `crq_updater` tool and related documentation.
- Creation of new SOPs: "Commit Labeling and CRQ Ownership Procedure" (`docs/sops/commit_labeling_crq_ownership_sop.md`) and "CRQ Updater Quality Assurance Procedure" (`docs/sops/crq_updater_qa_sop.md`).

This commit lays the foundation for automated CRQ management and standardized documentation practices within the project.