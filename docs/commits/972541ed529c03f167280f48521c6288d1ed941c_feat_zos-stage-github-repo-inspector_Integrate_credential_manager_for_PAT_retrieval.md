feat(zos-stage-github-repo-inspector): Integrate credential_manager for PAT retrieval

This commit integrates the `credential_manager` crate into `zos-stage-github-repo-inspector` to securely retrieve GitHub Personal Access Tokens (PATs).

Key changes include:
- Removal of the `--token` command-line argument, enhancing security by preventing PAT exposure in shell history.
- Utilization of `credential_manager::retrieve_credential` to fetch the PAT from the credential store.
- Updates to `Cargo.toml` to include `credential_manager` as a dependency.

This improves the security posture of the `zos-stage-github-repo-inspector` by centralizing PAT management.