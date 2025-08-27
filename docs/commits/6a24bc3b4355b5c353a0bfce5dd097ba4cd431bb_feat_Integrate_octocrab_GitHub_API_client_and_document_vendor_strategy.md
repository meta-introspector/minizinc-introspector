feat: Integrate octocrab GitHub API client and document vendor strategy

This foundational commit integrates the `octocrab` Rust client for GitHub API interactions, enabling programmatic access to GitHub functionalities within the project.

Key changes include:
- Addition of the `vendor/octocrab` submodule and its integration into `Cargo.lock` and `Cargo.toml`.
- Updates to `zos-stage-github-repo-inspector` to leverage `octocrab` for GitHub API calls.
- Creation of `vendor/README.md`, which clarifies the purpose of the `vendor` directory for external references and submodules, emphasizing the project's "Rust-only" philosophy and future plans for reimplementation or compatibility.

This commit strengthens the project's capabilities for interacting with GitHub and provides clear guidelines for managing external dependencies.