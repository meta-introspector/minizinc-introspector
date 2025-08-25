# Change Request: Credential Import Feature in Credential Manager

## Objective

To enhance the `credential_manager` CLI tool by adding an `import` subcommand, allowing users to import existing credentials from various sources (AWS, GitHub CLI, Gemini CLI) into the credential store. This streamlines the process of migrating or consolidating credentials.

## Scope of Changes

This change request introduces a new `Import` subcommand and its associated logic within the `credential_manager` crate:

1.  **CLI Subcommand (`crates/credential_manager/src/bin/cm.rs`):**
    *   A new `Import` subcommand has been added to the `Commands` enum.
    *   Nested `ImportService` subcommands (`Aws`, `Github`, `GeminiCli`) are introduced to specify the source of credentials.

2.  **Credential Import Logic (`crates/credential_manager/src/bin/cm.rs`):**
    *   **AWS Import (`ImportService::Aws`):**
        *   Reads credentials from `~/.aws/credentials` and `~/.aws/config` files.
        *   Parses `aws_access_key_id`, `aws_secret_access_key`, and `aws_session_token` from various profiles.
        *   Stores these credentials using the `store_credential` function.
    *   **GitHub Import (`ImportService::Github`):
        *   Reads GitHub OAuth tokens from `~/.config/gh/hosts.yml`.
        *   Parses the YAML content to extract `oauth_token` for each host.
        *   Stores the `oauth_token` using the `store_credential` function.
    *   **Gemini CLI Import (`ImportService::GeminiCli`):
        *   Provides a placeholder implementation that prompts the user to manually enter their Gemini API Key.
        *   Stores the entered API key using the `store_credential` function.

3.  **Dependency Updates (`crates/credential_manager/Cargo.toml`):
    *   The `ini = "1.x"` dependency has been added to facilitate parsing of INI-like configuration files (e.g., AWS credentials).
    *   `serde_yaml` and `serde` (for deserialization) were already present but are crucial for GitHub `hosts.yml` parsing.

## Impact

*   **Positive:** Simplifies credential management for users by providing a direct import mechanism. Reduces manual entry and potential for errors. Enhances usability of the `credential_manager` tool.
*   **Negative:** Introduces a new dependency (`ini`). Requires careful handling of file paths and potential errors during file parsing.

## Verification

*   Successful `cargo build` and `cargo test` after changes.
*   Manual testing of `cm import aws` with valid AWS credentials files.
*   Manual testing of `cm import github` with a valid GitHub `hosts.yml` file.
*   Manual testing of `cm import gemini-cli` by providing a sample API key.
*   Verification that imported credentials can be successfully retrieved using `cm retrieve`.

## Future Considerations

*   More robust error handling for file parsing and missing files.
*   Auto-detection of credential file locations.
*   Support for other credential sources (e.g., environment variables, other CLI tools).
*   Enhanced Gemini CLI import (e.g., reading from a specific config file if one exists).

## Approvals

*   [Signature/Date of Approver 1]
*   [Signature/Date of Approver 2]
