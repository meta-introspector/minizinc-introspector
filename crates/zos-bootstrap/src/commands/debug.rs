use clap::{Args, Subcommand};
use crate::utils::error::{Result, ZosError};
use crate::utils::subprocess;
use crate::utils::paths;

#[derive(Args, Clone)]
pub struct DebugArgs {
    #[command(subcommand)]
    pub command: Option<DebugCommands>,
}

#[derive(Subcommand, Clone)]
pub enum DebugCommands {
    /// Attempts to reproduce a known FFI crash
    ReproduceCrash {},
    /// Attempts to reproduce a specific FFI bug
    ReproduceFfiBug {},
    /// Runs focused v7 debug tests
    V7Tests {},
}

pub fn handle_debug_command(args: DebugArgs) -> Result<()> {
    match args.command {
        Some(DebugCommands::ReproduceCrash {}) => {
            reproduce_crash()?;
        }
        Some(DebugCommands::ReproduceFfiBug {}) => {
            reproduce_ffi_bug()?;
        }
        Some(DebugCommands::V7Tests {}) => {
            run_v7_debug_tests()?;
        }
        None => {
            println!("No debug command provided. Use --help for more information.");
        }
    }
    Ok(())
}

fn reproduce_crash() -> Result<()> {
    println!("Attempting to reproduce MiniZinc FFI crash...");
    let project_root = paths::resolve_project_root()?;
    let _build_dir = paths::get_build_dir()?;

    let mut env_vars: Vec<(String, String)> = Vec::new();
    env_vars.push(("LD_LIBRARY_PATH".to_string(), format!("{}/tools/minizinc_c_wrapper/:{}/install/lib/", project_root.to_string_lossy(), project_root.to_string_lossy())));

    let output = subprocess::run_command_with_env("cargo", &["test", "--package", "minizinc_ffi"], &env_vars.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<(&str, &str)>>())?;


    if output.status.success() {
        println!("Tests passed. Crash not reproduced.");
    } else {
        println!("Tests failed. Crash likely reproduced.");
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

fn reproduce_ffi_bug() -> Result<()> {
    println!("Attempting to reproduce specific FFI bug...");
    let project_root = paths::resolve_project_root()?;
    let build_dir = paths::get_build_dir()?;

    let mut env_vars: Vec<(String, String)> = Vec::new();
    env_vars.push(("LD_LIBRARY_PATH".to_string(), format!("{}/tools/minizinc_c_wrapper/:{}/install/lib/", project_root.to_string_lossy(), project_root.to_string_lossy())));

    let output = subprocess::run_command_with_env("cargo", &["test", "--package", "minizinc_ffi", "--", "--nocapture"], &env_vars.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<Vec<(&str, &str)>>())?;

    if output.status.success() {
        println!("Tests passed. Bug not reproduced.");
    } else {
        println!("Tests failed. Bug likely reproduced.");
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
    }
    Ok(())
}

fn run_v7_debug_tests() -> Result<()> {
    println!("Running focused v7 debug tests...");
    // This logic was originally in scripts/run_v7_debug_tests.sh
    // It calls minizinc_test_runner_rs, which is a separate Rust executable.
    // We need to ensure minizinc_test_runner_rs is built.

    let minizinc_test_runner_exe = paths::resolve_project_root()?
        .join("tools")
        .join("minizinc_test_runner_rs")
        .join("target")
        .join("release")
        .join("minizinc_test_runner_rs");

    subprocess::run_command(&minizinc_test_runner_exe.to_string_lossy(), &[]).map(|_| ())?;

    println!("Focused v7 debug tests completed.");
    Ok(())
}
