## Review of `reproduce_crash.sh`

*   **Purpose:** This script is specifically designed to reproduce a crash in the MiniZinc FFI. It sets the `LD_LIBRARY_PATH` and then runs the Rust FFI tests, logging their standard output and error to files.
*   **Key Commands and Dependencies:**
    *   `export LD_LIBRARY_PATH`: Crucial for ensuring the Rust executable and C wrapper can find the necessary shared libraries at runtime. The paths are hardcoded absolute paths.
    *   `cargo test --package minizinc_ffi`: Executes the Rust FFI tests.
    *   `> crash_reproduce_stdout.log 2> crash_reproduce_stderr.log`: Redirects stdout and stderr to log files.
    *   `if [ $? -eq 0 ]`: Checks the exit code of the `cargo test` command to determine if the tests passed (crash not reproduced) or failed (crash likely reproduced).
    *   `cat crash_reproduce_stderr.log`: Prints the stderr log to the console for immediate inspection.
*   **Relevance to FFI, MiniZinc, and the "Big Idea":**
    *   **FFI:** This script is directly related to FFI stability. Its sole purpose is to trigger and capture a crash within the FFI, which is essential for debugging and verifying fixes.
    *   **MiniZinc:** The crash occurs within the FFI's interaction with MiniZinc.
    *   **"Big Idea":**
        *   **Reliability:** The existence and use of such a script underscore the project's commitment to building a highly reliable FFI. A stable FFI is a non-negotiable prerequisite for the "big idea" to function, as it forms the communication backbone.
        *   **Debugging:** This is a dedicated debugging tool, part of the "Observe" phase of the OODA loop, providing concrete evidence of a problem.
        *   **Monotonic Epic Idea:** While the script itself is for reproducing a negative outcome (a crash), the overall goal of using it is to enable an additive fix that improves the system without directly altering stable components.

This script is a clear indicator of the project's rigorous approach to FFI stability and debugging.
