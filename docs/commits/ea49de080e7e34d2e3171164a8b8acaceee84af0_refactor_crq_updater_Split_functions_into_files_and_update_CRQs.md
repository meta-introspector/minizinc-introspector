refactor(crq_updater): Split functions into files and update CRQs
Refactored the `crq_updater` crate to adhere to the "one declaration per file" convention.
- Moved functions (`find_crq_files`, `process_crq_file`, `extract_existing_history`, `get_commit_diff_summary`, `find_commit_from_oid`) into separate files within `src/functions/`.
- Updated `src/main.rs` to use the new module structure.
- Fixed compilation errors related to `git2` API changes (`parent_opt` to `parent`, `find_object` arguments, `diff.print` arguments).
- Updated `walkdir` dependency to `2.5.0`.
- Resolved `StripPrefixError` by canonicalizing `crq_path`.
- Successfully ran `crq_updater` to update `crq_launchpad_workflow_enhancements.md` and `docs/sops/crq_updater_qa_sop.md` with relevant commit history.