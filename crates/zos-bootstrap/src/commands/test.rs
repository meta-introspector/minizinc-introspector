use clap::Args;
use crate::utils::error::{Result, ZosError};
use crate::utils::subprocess;
use crate::utils::paths;

#[derive(Args)]
pub struct TestArgs {
    #[command(subcommand)]
    pub command: Option<TestCommands>,
}

#[derive(Subcommand)]
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
        Some(TestCommands::All {{}})
 => {
            println!("Running all tests...");
            test_c_abi()?;
            test_rust_ffi()?;
            test_minizinc_models()?;
            println!("All tests completed successfully.");
        }
        Some(TestCommands::CAbi {{}})
 => {
            test_c_abi()?;
        }
        Some(TestCommands::RustFfi {{}})
 => {
            test_rust_ffi()?;
        }
        Some(TestCommands::MinizincModels {{}})
 => {
            test_minizinc_models()?;
        }
        Some(TestCommands::DznGen {{ num_vec, base_size }})
 => {
            test_dzn_generation(num_vec, base_size)?;
        }
        Some(TestCommands::DznGenRust {{ num_vec }})
 => {
            test_dzn_gen_rust(num_vec)?;
        }
        Some(TestCommands::Coverage {{}})
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
    let c_test_file_content = format!(
        r##"#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "minizinc_ffi_declarations_v2.h"
#include "minizinc_opaque_types.h"

int main() {{
    printf(\"C ABI Test: Starting\n");

    MiniZincEnvWrapper* env_wrapper = minizinc_env_new();
    if (env_wrapper == NULL) {{
        fprintf(stderr, \"C ABI Test: Failed to create MiniZinc environment\n");
        return 1;
    }}
    printf(\"C ABI Test: MiniZinc environment created at %p\n\", (void*)env_wrapper);

    const char* model_str = \"var int: x; constraint x > 0; solve satisfy;\";
    MiniZincModel* model = minizinc_parse_string_only(env_wrapper, model_str);
    if (model == NULL) {{
        fprintf(stderr, \"C ABI Test: Failed to parse model\n");
        minizinc_env_free(env_wrapper);
        return 1;
    }}
    printf(\"C ABI Test: Model parsed successfully at %p\n\", (void*)model);

    minizinc_model_free(model);
    printf(\"C ABI Test: Model freed.\n");
    minizinc_env_free(env_wrapper);
    printf(\"C ABI Test: MiniZinc environment freed.\n");

    printf(\"C ABI Test: All C ABI tests passed successfully.\n");
    return 0;
}}
##",
    );

    let c_test_file_path = c_test_dir.join("test_c_abi.cpp");
    std::fs::write(&c_test_file_path, c_test_file_content)?;

    // Compile C test program
    let mut args_gpp = Vec::new();
    args_gpp.push("-o");
    args_gpp.push(&c_test_dir.join("test_c_abi").to_string_lossy());
    args_gpp.push(&c_test_file_path.to_string_lossy());
    args_gpp.push(&format!(