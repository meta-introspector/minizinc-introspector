use crate::commands::test::test_rust_ffi::test_rust_ffi;
use crate::commands::test::test_minizinc_models::test_minizinc_models;
use crate::commands::test::test_dzn_generation::test_dzn_generation;
use crate::commands::test::test_dzn_gen_rust::test_dzn_gen_rust;
use crate::commands::test::test_coverage::test_coverage;

use clap::{Args, Subcommand};
use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;

pub mod test_rust_ffi;
pub mod test_minizinc_models;
pub mod test_dzn_generation;
pub mod test_dzn_gen_rust;
pub mod test_coverage;

#[derive(Args, Clone)]
pub struct TestArgs {
    #[command(subcommand)]
    pub command: Option<TestCommands>,
}

#[derive(Subcommand, Clone)]
pub enum TestCommands {
    /// Runs all tests (C ABI, Rust FFI, MiniZinc models)
    All {},
    /// Runs the standalone C ABI test
    CAbi {},
    /// Runs the Rust FFI tests
    RustFfi {},
    /// Runs the full MiniZinc model test suite
    MinizincModels {},
    /// Generates and verifies DZN data
    DznGen {
        num_vec: u32,
        base_size: u32,
    },
    /// Tests the Rust DZN generator
    DznGenRust {
        num_vec: u32,
    },
    /// Runs Rust FFI tests for C++ coverage data generation
    Coverage {},
}

pub fn handle_test_command(args: TestArgs) -> Result<()> {
    match args.command {
        Some(TestCommands::All {})
 => {
            println!("Running all tests...");
            test_c_abi()?;
            test_rust_ffi()?;
            test_minizinc_models()?;
            println!("All tests completed successfully.");
        }
        Some(TestCommands::CAbi {})
 => {
            test_c_abi()?;
        }
        Some(TestCommands::RustFfi {})
 => {
            test_rust_ffi()?;
        }
        Some(TestCommands::MinizincModels {})
 => {
            test_minizinc_models()?;
        }
        Some(TestCommands::DznGen { num_vec, base_size })
 => {
            test_dzn_generation(num_vec, base_size)?;
        }
        Some(TestCommands::DznGenRust { num_vec })
 => {
            test_dzn_gen_rust(num_vec)?;
        }
        Some(TestCommands::Coverage {})
 => {
            test_coverage()?;
        }
        None => {
            println!("No test command provided. Use --help for more information.");
        }
    }
    Ok(())
}

fn test_c_abi() -> Result<()> {
    println!("Running C ABI standalone test...");
    let c_test_dir = paths::resolve_project_root()?.join("c_abi_test");
    let minizinc_c_wrapper_include_dir = paths::get_minizinc_c_wrapper_dir()?;
    let build_dir = paths::get_build_dir()?;

    // Create C ABI test directory
    subprocess::run_command("mkdir", &["-p", &c_test_dir.to_string_lossy()])?;

    // Generate C test file content (from build_test_c_rust.sh)
    let c_test_file_content = format!("fixme read test_file.c");

    let c_test_file_path = c_test_dir.join("test_c_abi.cpp");
    std::fs::write(&c_test_file_path, c_test_file_content)?;

    // Compile C test program
    let mut args_gpp = Vec::new();
    args_gpp.push("-o");
    let c_test_abi_path = c_test_dir.join("test_c_abi").to_string_lossy().to_string();
    args_gpp.push(&c_test_abi_path);
    let c_test_file_path_str = c_test_file_path.to_string_lossy().to_string();
    args_gpp.push(&c_test_file_path_str);
    args_gpp.push(&format!(""));
    Ok(())
}
