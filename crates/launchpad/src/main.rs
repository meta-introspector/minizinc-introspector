//! This is the main entry point for the `launchpad` executable.
//! It initializes the asynchronous runtime and delegates the primary application
//! logic to the `launchpad_main` module.


mod orchestrator;
mod narrator;
mod dum_wrappers;
mod launchpad_main;
mod gemini_cli_options;

/// The asynchronous main function for the `launchpad` application.
///
/// This function serves as the binary's entry point. It sets up the Tokio
/// runtime and then calls `launchpad_main::run_launchpad` to execute the
/// core logic of the application.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`) if
/// the `run_launchpad` function encounters an issue.
#[tokio::main]
async fn main() -> Result<(), String> {
    launchpad_main::run_launchpad().await
}

#[cfg(test)]
mod tests {
    // Note: The `main` function is typically not unit-tested directly as its
    // primary role is to serve as the application's entry point and delegate
    // to other functions. The core logic resides in `launchpad_main::run_launchpad`,
    // which is where testing efforts are focused.
    // This placeholder test ensures the `main` function compiles.
    #[tokio::test]
    async fn test_main_compiles() {
        assert!(true);
    }
}
