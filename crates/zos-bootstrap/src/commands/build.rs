use clap::{Args, Subcommand};
use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;

#[derive(Args, Clone)]
pub struct BuildArgs {
    #[command(subcommand)]
    pub command: Option<BuildCommands>,
    #[arg(long)]
    pub strace: bool,
}

#[derive(Subcommand, Clone)]
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
            build_ffi_wrapper(args.strace)?;
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
            build_ffi_wrapper(args.strace)?;
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
    let _project_root = paths::resolve_project_root()?;

    let mut args_mkdir: Vec<String> = Vec::new();
    args_mkdir.push("-p".to_string());
    args_mkdir.push(gecode_build_dir.to_string_lossy().to_string());
    subprocess::run_command("mkdir", &args_mkdir.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_cmake: Vec<String> = Vec::new();
    args_cmake.push(format!("{}", gecode_build_dir.to_string_lossy()));
    args_cmake.push("-DCMAKE_POLICY_VERSION_MINIMUM=3.5".to_string());
    subprocess::run_command("cmake", &args_cmake.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_make: Vec<String> = Vec::new();
    args_make.push("-C".to_string());
    args_make.push(gecode_build_dir.to_string_lossy().to_string());
    subprocess::run_command("make", &args_make.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    println!("Gecode built successfully.");
    Ok(())
}

fn build_libminizinc() -> Result<()> {
    println!("Building libminizinc...");
    let libminizinc_build_dir = paths::get_build_dir()?;
    let _project_root = paths::resolve_project_root()?;
    let gecode_build_dir = paths::get_gecode_build_dir()?;

    let mut args_mkdir: Vec<String> = Vec::new();
    args_mkdir.push("-p".to_string());
    args_mkdir.push(libminizinc_build_dir.to_string_lossy().to_string());
    subprocess::run_command("mkdir", &args_mkdir.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_cmake: Vec<String> = Vec::new();
    args_cmake.push(_project_root.to_string_lossy().to_string());
    args_cmake.push(format!("-DGecode_ROOT={}", gecode_build_dir.to_string_lossy()));
    subprocess::run_command("cmake", &args_cmake.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_make: Vec<String> = Vec::new();
    args_make.push("-C".to_string());
    args_make.push(libminizinc_build_dir.to_string_lossy().to_string());
    subprocess::run_command("make", &args_make.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    println!("libminizinc built successfully.");
    Ok(())
}

pub mod ensure_build_directory_exists;
pub mod run_cmake_for_ffi;
pub mod run_make_for_ffi;
pub mod verify_ffi_library_exists;

fn build_ffi_wrapper(strace_enabled: bool) -> Result<()> {
    println!("--- Starting build_ffi_wrapper ---");
    println!("Building C++ FFI wrapper...");

    let build_dir = paths::get_build_dir()?;
    let project_root = paths::resolve_project_root()?;

    println!("Project Root: {}", project_root.display());
    println!("Build Dir: {}", build_dir.display());

    ensure_build_directory_exists::ensure_build_directory_exists(&build_dir)?;
    run_cmake_for_ffi::run_cmake_for_ffi(&project_root, &build_dir)?;
    run_make_for_ffi::run_make_for_ffi(&build_dir, strace_enabled)?;
    verify_ffi_library_exists::verify_ffi_library_exists(&build_dir)?;

    println!("C++ FFI wrapper built successfully.");
    println!("--- Finished build_ffi_wrapper ---");
    Ok(())
}

fn build_rust_ffi_crate() -> Result<()> {
    println!("Building Rust FFI crate...");
    let minizinc_ffi_crate_dir = paths::get_minizinc_ffi_crate_dir()?;
    subprocess::run_command("cargo", &["build", "--release"])?;
    println!("Rust FFI crate built successfully.");
    Ok(())
}

fn build_ffi_declarations() -> Result<()> {
    println!("Generating FFI header declarations...");
    let project_root = paths::resolve_project_root()?;
    let script_path = project_root.join("generate_ffi_declarations.sh");
    subprocess::run_command(&script_path.to_string_lossy(), &[]).map(|_| ())?;
    println!("FFI header declarations generated successfully.");
    Ok(())
}

fn build_solvers() -> Result<()> {
    println!("Setting up MiniZinc solver configurations...");
    let project_root = paths::resolve_project_root()?;
    let script_path = project_root.join("setup_minizinc_solvers.sh");
    subprocess::run_command(&script_path.to_string_lossy(), &[]).map(|_| ())?;
    println!("MiniZinc solver configurations set up successfully.");
    Ok(())
}

fn build_coverage() -> Result<()> {
    println!("Building libminizinc with coverage instrumentation...");
    let build_dir = paths::get_build_dir()?;
    let project_root = paths::resolve_project_root()?;

    let mut args_cmake: Vec<String> = Vec::new();
    args_cmake.push(project_root.to_string_lossy().to_string());
    args_cmake.push("-DCMAKE_BUILD_TYPE=Debug".to_string());
    args_cmake.push("-DCMAKE_CXX_FLAGS=\"--coverage\"".to_string());
    args_cmake.push("-DCMAKE_C_FLAGS=\"--coverage\"".to_string());
    subprocess::run_command("cmake", &args_cmake.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_make: Vec<String> = Vec::new();
    args_make.push("-C".to_string());
    args_make.push(build_dir.to_string_lossy().to_string());
    subprocess::run_command("make", &args_make.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    println!("libminizinc built with coverage instrumentation successfully.");
    Ok(())
}


