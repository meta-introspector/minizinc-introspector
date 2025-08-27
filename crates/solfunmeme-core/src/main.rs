//! This is the main entry point for the `solfunmeme-core` executable.
//! It initializes the asynchronous runtime and delegates the primary application
//! logic to the `launchpad_app` module.

mod orchestrator;
mod narrator;
mod dum_wrappers;
mod gemini_cli_options;
mod stages;

// The actual logic for launchpad is now in this module
mod launchpad_app;

use gemini_utils::gemini_eprintln;

/// The asynchronous main function for the `solfunmeme-core` application.
///
/// This function serves as the binary's entry point. It sets up the Tokio
/// runtime and then calls `launchpad_app::run_launchpad` to execute the
/// core logic of the application.
///
/// # Returns
///
/// A `Result` indicating success (`Ok(())`) or an error (`Err(String)`) if
/// the `run_launchpad` function encounters an issue.
#[tokio::main]
async fn main() -> Result<(), String> {
    gemini_eprintln!("Starting solfunmeme-core...");
    launchpad_app::run_launchpad().await
}

#[cfg(test)]
mod tests {
    // Note: The `main` function is typically not unit-tested directly as its
    // primary role is to serve as the application's entry point and delegate
    // to other functions. The core logic resides in `launchpad_app::run_launchpad`,
    // which is where testing efforts are focused.
    // This placeholder test ensures the `main` function compiles.
    #[tokio::test]
    async fn test_main_compiles() {
        assert!(true);
    }
}
