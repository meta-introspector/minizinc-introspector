feat(credential_manager): Implement credential import and GitHub OAuth (disabled)

This commit implements significant credential import and authentication features within the `credential_manager` tool's `cm` binary.

Key changes include:
- Addition of `ImportService` for importing credentials from AWS configuration, GitHub CLI's `hosts.yml`, and direct input for Gemini CLI API Keys.
- Initial implementation of GitHub OAuth authentication logic, though it is currently disabled for further refactoring.
- Integration of various new dependencies (`hyper`, `tokio`, `reqwest`, `oauth2`, `url`, `serde`, `serde_yaml`) to support these functionalities.
- Updates to `Cargo.toml` and `Cargo.lock` to reflect the new dependencies.

This commit significantly expands the `credential_manager`'s capabilities for secure credential management.