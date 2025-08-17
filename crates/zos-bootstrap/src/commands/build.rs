use clap::Args;
use crate::utils::error::{Result, ZosError};
use crate::utils::subprocess;
use crate::utils::paths;
use std::fs;

#[derive(Args)]
pub struct BuildArgs {
    #[command(subcommand)]
    pub command: Option<BuildCommands>,
}

#[derive(Subcommand)]
pub enum BuildCommands {
    /// Builds all core components (Gecode, libminizinc, FFI)
    All {},
    /// Builds Gecode
    Gecode {},
    /// Builds libminizinc
    Libminizinc {},
    /// Builds the C++ FFI wrapper
    Ffi {},
    /// Builds the Rust FFI crate
    RustFfi {},
    /// Generates FFI header declarations
    FfiDeclarations {},
    /// Sets up MiniZinc solver configurations
    Solvers {},
    /// Builds libminizinc with coverage instrumentation
    Coverage {},
}

pub fn handle_build_command(args: BuildArgs) -> Result<()> {
    match args.command {
        Some(BuildCommands::All {}) => {
            println!("Building all core components...");
            build_gecode()?;
            build_libminizinc()?;
            build_ffi_declarations()?;
            build_ffi_wrapper()?;
            build_rust_ffi_crate()?;
            build_solvers()?;
            println!("All core components built successfully.");
        }
        Some(BuildCommands::Gecode {}) => {
            build_gecode()?;
        }
        Some(BuildCommands::Libminizinc {}) => {
            build_libminizinc()?;
        }
        Some(BuildCommands::Ffi {}) => {
            build_ffi_wrapper()?;
        }
        Some(BuildCommands::RustFfi {}) => {
            build_rust_ffi_crate()?;
        }
        Some(BuildCommands::FfiDeclarations {}) => {
            build_ffi_declarations()?;
        }
        Some(BuildCommands::Solvers {}) => {
            build_solvers()?;
        }
        Some(BuildCommands::Coverage {}) => {
            build_coverage()?;
        }
        None => {
            println!("No build command provided. Use --help for more information.");
        }
    }
    Ok(())
}

fn build_gecode() -> Result<()> {
    println!("Building Gecode...");
    let gecode_build_dir = paths::get_gecode_build_dir()?;
    let project_root = paths::resolve_project_root()?;

    let mut args_mkdir = Vec::new();
    args_mkdir.push("-p");
    args_mkdir.push(&gecode_build_dir.to_string_lossy());
    subprocess::run_command("mkdir", &args_mkdir)?;

    let mut args_cmake = Vec::new();
    args_cmake.push(&format!("{}", gecode_build_dir.to_string_lossy()));
    args_cmake.push("-DCMAKE_POLICY_VERSION_MINIMUM=3.5");
    subprocess::run_command("cmake", &args_cmake)?;

    let mut args_make = Vec::new();
    args_make.push("-C");
    args_make.push(&gecode_build_dir.to_string_lossy());
    subprocess::run_command("make", &args_make)?;

    println!("Gecode built successfully.");
    Ok(())
}

fn build_libminizinc() -> Result<()> {
    println!("Building libminizinc...");
    let libminizinc_build_dir = paths::get_build_dir()?;
    let project_root = paths::resolve_project_root()?;
    let gecode_build_dir = paths::get_gecode_build_dir()?;

    let mut args_mkdir = Vec::new();
    args_mkdir.push("-p");
    args_mkdir.push(&libminizinc_build_dir.to_string_lossy());
    subprocess::run_command("mkdir", &args_mkdir)?;

    let mut args_cmake = Vec::new();
    args_cmake.push(&project_root.to_string_lossy());
    args_cmake.push(&format!(