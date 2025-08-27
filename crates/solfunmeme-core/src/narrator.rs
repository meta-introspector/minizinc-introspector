//! This module provides functions for narrating the progress and outcomes
//! of operations within the `launchpad` application. It uses `gemini_utils::gemini_eprintln!`
//! for output, which can be redirected for logging or livestreaming purposes.

use gemini_utils::gemini_eprintln;

/// Narrates a specific step in the process.
///
/// This function prints a formatted message to `stderr` indicating the start
/// or progress of a particular step.
///
/// # Arguments
///
/// * `step_description` - A string slice describing the step being narrated.
pub fn narrate_step(step_description: &str) {
    gemini_eprintln!("\n--- Narrating: :step_description: ---\n", step_description = step_description);
}

/// Narrates a successful outcome.
///
/// This function prints a formatted success message to `stderr`.
///
/// # Arguments
///
/// * `message` - A string slice containing the success message.
pub fn narrate_success(message: &str) {
    gemini_eprintln!("\n--- SUCCESS: :message: ---\n", message = message);
}

/// Narrates an error outcome.
///
/// This function prints a formatted error message to `stderr`.
///
/// # Arguments
///
/// * `message` - A string slice containing the error message.
pub fn narrate_error(message: &str) {
    gemini_eprintln!("\n--- ERROR: :message: ---\n", message = message);
}

/// Livestreams output to `stderr`.
///
/// This function is intended to simulate livestreaming output. In a production
/// environment, this might send data to a real-time streaming service.
/// For now, it simply prints the output to `stderr`.
///
/// # Arguments
///
/// * `output` - A string slice containing the output to be livestreamed.
pub fn livestream_output(output: &str) {
    // In a real scenario, this would send output to a livestreaming service
    // For now, we'll just print it to stderr
    gemini_eprintln!(":output:", output = output);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests primarily ensure that the functions compile and do not panic.
    // Capturing `eprintln!` output for assertion in unit tests is generally more complex
    // and often requires external crates or platform-specific hacks.

    #[test]
    fn test_narrate_step() {
        narrate_step("Testing step narration");
        // No assertion possible without capturing stderr
        assert!(true);
    }

    #[test]
    fn test_narrate_success() {
        narrate_success("Testing success narration");
        // No assertion possible without capturing stderr
        assert!(true);
    }

    #[test]
    fn test_narrate_error() {
        narrate_error("Testing error narration");
        // No assertion possible without capturing stderr
        assert!(true);
    }

    #[test]
    fn test_livestream_output() {
        livestream_output("Testing livestream output");
        // No assertion possible without capturing stderr
        assert!(true);
    }
}
