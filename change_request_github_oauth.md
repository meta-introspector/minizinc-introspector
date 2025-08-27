# Change Request: GitHub OAuth2 Integration in Credential Manager

## Objective

To implement GitHub OAuth2 authentication within the `credential_manager` crate, enabling users to securely authenticate with GitHub and store Personal Access Tokens (PATs) for use by other tools in the `libminizinc` project. This aims to provide a `gh auth`-like experience for seamless integration with GitHub workflows.

## Scope of Changes

This change request encompasses significant modifications to the `credential_manager` crate, including:

1.  **Dependency Updates (`Cargo.lock` and `crates/credential_manager/Cargo.toml`):**
    *   Introduction of new core dependencies required for OAuth2 flow and asynchronous operations:
        *   `oauth2`: For handling the OAuth2 protocol.
        *   `reqwest`: An asynchronous HTTP client for making token exchange requests.
        *   `tokio`: An asynchronous runtime for managing concurrent operations, including the local HTTP server.
        *   `hyper`: A fast, safe HTTP library used to implement a local server for receiving OAuth2 redirects.
        *   `tokio-stream`: For handling streams of data, particularly from the local HTTP server.
        *   `url`: For robust URL parsing and manipulation.
        *   `http`: HTTP types for `hyper` and `reqwest` integration.
    *   The `oauth2` crate is vendored locally (`path = "../../vendor/oauth2"`), indicating a controlled dependency.

2.  **Credential Manager Logic (`crates/credential_manager/src/bin/cm.rs`):**
    *   **New `Auth` Subcommand:** A new `Auth` subcommand has been added to the `Cli` enum, allowing users to initiate authentication flows.
    *   **`AuthService::Github`:** A nested subcommand specifically for GitHub OAuth2 authentication.
    *   **Asynchronous `main` function:** The `main` function has been converted to an `async` function, annotated with `#[tokio::main]`, to support non-blocking I/O operations required for the local HTTP server and network requests.
    *   **OAuth2 Flow Implementation:**
        *   **Client Configuration:** Sets up the OAuth2 client with GitHub's authorization and token URLs, and a `localhost` redirect URI.
        *   **Authorization URL Generation:** Generates the URL for the user to authorize the application in their browser, requesting the `repo` scope for GitHub API access.
        *   **Local HTTP Server for Redirect:** Starts a `hyper` server on `http://localhost:8080/redirect` to capture the authorization code and state from GitHub's redirect.
        *   **CSRF Protection:** Implements CSRF (Cross-Site Request Forgery) state verification to enhance security.
        *   **Token Exchange:** Exchanges the received authorization code for an access token using `reqwest`.
        *   **PAT Storage:** Stores the obtained GitHub Personal Access Token (PAT) securely using the `credential_manager`'s `store_credential` function.
    *   **Placeholder Credentials:** The client ID and secret are currently placeholders (`"YOUR_GITHUB_CLIENT_ID"`, `"YOUR_GITHUB_CLIENT_SECRET"`), indicating further configuration is required.

3.  **Submodule Update (`vendor/rust-emojicons`):**
    *   The `rust-emojicons` submodule has been updated to a newer commit, reflecting potential improvements or bug fixes in emoji handling.

## Impact

*   **Positive:** Enables secure and streamlined GitHub authentication for `libminizinc` tools, reducing manual PAT management. Facilitates local testing of GitHub Actions workflows using `mini-act` by providing a robust authentication mechanism.
*   **Negative:** Introduces several new dependencies, potentially increasing build times and binary size. Requires users to configure GitHub OAuth application credentials.

## Verification

*   Successful `cargo build` and `cargo test` after changes.
*   Manual testing of `cm auth github` command to verify the OAuth2 flow, including browser redirection, local server capture, token exchange, and credential storage.
*   Verification that `mini-act` can utilize the stored GitHub PAT for local workflow execution.

## Future Considerations

*   Externalization of GitHub OAuth client ID and secret (e.g., via environment variables or configuration files).
*   Support for other OAuth providers if needed.
*   Error handling improvements and user feedback during the OAuth flow.

## Approvals

*   [Signature/Date of Approver 1]
*   [Signature/Date of Approver 2]

## Commit History

- [Commit b3bb2456e46ebf74b07006da2ff6eb8fafbd9d9d: docs(crq): Update GitHub Repo Integration Epic CRQ with Status](docs/commits/b3bb2456e46ebf74b07006da2ff6eb8fafbd9d9d_docs_crq_Update_GitHub_Repo_Integration_Epic_CRQ_with_Status.md)
- [Commit 84d41e99dc14c290d2255323172bbc6d1cbcb816: docs(crq): Draft GitHub Repository Integration Epic CRQ](docs/commits/84d41e99dc14c290d2255323172bbc6d1cbcb816_docs_crq_Draft_GitHub_Repository_Integration_Epic_CRQ.md)
- [Commit f59e4e46ba17bf2d240d7ffe57c9098428f3ff87: docs: Add CRQ for GitHub OAuth2 integration](docs/commits/f59e4e46ba17bf2d240d7ffe57c9098428f3ff87_docs_Add_CRQ_for_GitHub_OAuth2_integration.md)
- [Commit 51a5cf242d2b1ad7684dc743a144736707a8543c: feat(credential_manager): Implement credential import and GitHub OAuth (disabled)](docs/commits/51a5cf242d2b1ad7684dc743a144736707a8543c_feat_credential_manager_Implement_credential_import_and_GitHub_OAuth_disabled.md)
- [Commit 972541ed529c03f167280f48521c6288d1ed941c: feat(zos-stage-github-repo-inspector): Integrate credential_manager for PAT retrieval](docs/commits/972541ed529c03f167280f48521c6288d1ed941c_feat_zos-stage-github-repo-inspector_Integrate_credential_manager_for_PAT_retrieval.md)
- [Commit 6a24bc3b4355b5c353a0bfce5dd097ba4cd431bb: feat: Integrate octocrab GitHub API client and document vendor strategy](docs/commits/6a24bc3b4355b5c353a0bfce5dd097ba4cd431bb_feat_Integrate_octocrab_GitHub_API_client_and_document_vendor_strategy.md)