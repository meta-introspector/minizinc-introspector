use clap::{Args, Subcommand};
use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;
use crate::constants;

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
            println!("{}", constants::MSG_BUILDING_ALL_COMPONENTS);
            build_gecode()?;
            build_libminizinc()?;
            build_ffi_declarations()?;
            build_ffi_wrapper(args.strace)?;
            build_rust_ffi_crate()?;
            build_solvers()?;
            println!("{}", constants::MSG_ALL_COMPONENTS_BUILT_SUCCESSFULLY);
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
            println!("{}", constants::MSG_NO_BUILD_COMMAND_PROVIDED);
        }
    }
    Ok(())
}

fn build_gecode() -> Result<()> {
    println!("{}", constants::MSG_BUILDING_GECODE);
    let gecode_build_dir = paths::get_gecode_build_dir()?;
    let gecode_vendor_dir = paths::get_gecode_vendor_dir()?;

    let mut args_mkdir: Vec<String> = Vec::new();
    args_mkdir.push(constants::ARG_MKDIR_P.to_string());
    args_mkdir.push(gecode_build_dir.to_string_lossy().to_string());
    subprocess::run_command("mkdir", &args_mkdir.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_cmake: Vec<String> = Vec::new();
    args_cmake.push("-B".to_string());
    args_cmake.push(gecode_build_dir.to_string_lossy().to_string());
    args_cmake.push(".".to_string()); // Source directory is current directory (gecode_vendor_dir)
    args_cmake.push(constants::ARG_CMAKE_POLICY_MINIMUM.to_string());
    subprocess::run_command_in_dir("cmake", &args_cmake.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &gecode_vendor_dir)?;

    let mut args_make: Vec<String> = Vec::new();
    args_make.push(constants::ARG_MAKE_C.to_string());
    args_make.push(gecode_build_dir.to_string_lossy().to_string());
    subprocess::run_command_in_dir("make", &args_make.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &gecode_build_dir)?;

    println!("{}", constants::MSG_GECODE_BUILT_SUCCESSFULLY);
    Ok(())
}

fn build_libminizinc() -> Result<()> {
    println!("{}", constants::MSG_BUILDING_LIBMINIZINC);
    let libminizinc_build_dir = paths::get_build_dir()?;
    let _project_root = paths::resolve_project_root()?;
    let gecode_build_dir = paths::get_gecode_build_dir()?;

    let mut args_mkdir: Vec<String> = Vec::new();
    args_mkdir.push(constants::ARG_MKDIR_P.to_string());
    args_mkdir.push(libminizinc_build_dir.to_string_lossy().to_string());
    subprocess::run_command("mkdir", &args_mkdir.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_cmake: Vec<String> = Vec::new();
    args_cmake.push(_project_root.to_string_lossy().to_string());
    args_cmake.push(format!("{}{}", constants::ARG_GECODE_ROOT.replace("{}", ""), gecode_build_dir.to_string_lossy()));
    subprocess::run_command("cmake", &args_cmake.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_make: Vec<String> = Vec::new();
    args_make.push(constants::ARG_MAKE_C.to_string());
    args_make.push(libminizinc_build_dir.to_string_lossy().to_string());
    subprocess::run_command("make", &args_make.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    println!("{}", constants::MSG_LIBMINIZINC_BUILT_SUCCESSFULLY);
    Ok(())
}

pub mod ensure_build_directory_exists;
pub mod run_cmake_for_ffi;
pub mod run_make_for_ffi;
pub mod verify_ffi_library_exists;

fn build_ffi_wrapper(strace_enabled: bool) -> Result<()> {
    println!("{}", constants::MSG_STARTING_BUILD_FFI_WRAPPER);
    println!("{}", constants::MSG_BUILDING_FFI_WRAPPER);

    let build_dir = paths::get_build_dir()?;
    let project_root = paths::resolve_project_root()?;

    println!("{}{}", constants::MSG_PROJECT_ROOT.replace("{}", ""), project_root.display());
    println!("{}{}", constants::MSG_BUILD_DIR.replace("{}", ""), build_dir.display());

    ensure_build_directory_exists::ensure_build_directory_exists(&build_dir)?;
    run_cmake_for_ffi::run_cmake_for_ffi(&project_root, &build_dir)?;
    run_make_for_ffi::run_make_for_ffi(&build_dir, strace_enabled)?;
    verify_ffi_library_exists::verify_ffi_library_exists(&build_dir)?;

    println!("{}", constants::MSG_FFI_WRAPPER_BUILT_SUCCESSFULLY);
    println!("{}", constants::MSG_FINISHED_BUILD_FFI_WRAPPER);
    Ok(())
}

fn build_rust_ffi_crate() -> Result<()> {
    println!("{}", constants::MSG_BUILDING_RUST_FFI_CRATE);
    let minizinc_ffi_crate_dir = paths::get_minizinc_ffi_crate_dir()?;
    println!("{}{}", constants::MSG_MINIZINC_FFI_CRATE_DIR.replace("{}", ""), minizinc_ffi_crate_dir.display());
    subprocess::run_command(constants::CMD_CARGO, &[constants::ARG_BUILD, constants::ARG_RELEASE])?;
    println!("{}", constants::MSG_RUST_FFI_CRATE_BUILT_SUCCESSFULLY);
    Ok(())
}

fn build_ffi_declarations() -> Result<()> {
    println!("{}", constants::MSG_GENERATING_FFI_HEADERS);
    let project_root = paths::resolve_project_root()?;
    let script_path = project_root.join(constants::SCRIPT_GENERATE_FFI_DECLARATIONS);
    subprocess::run_command(&script_path.to_string_lossy(), &[]).map(|_| ())?;
    println!("{}", constants::MSG_FFI_HEADERS_GENERATED_SUCCESSFULLY);
    Ok(())
}

fn build_solvers() -> Result<()> {
    println!("{}", constants::MSG_SETTING_UP_SOLVER_CONFIGS);
    let project_root = paths::resolve_project_root()?;
    let script_path = project_root.join(constants::SCRIPT_SETUP_MINIZINC_SOLVERS);
    subprocess::run_command(&script_path.to_string_lossy(), &[]).map(|_| ())?;
    println!("{}", constants::MSG_SOLVER_CONFIGS_SET_UP_SUCCESSFULLY);
    Ok(())
}

fn build_coverage() -> Result<()> {
    println!("{}", constants::MSG_BUILDING_COVERAGE_INSTRUMENTATION);
    let build_dir = paths::get_build_dir()?;
    let project_root = paths::resolve_project_root()?;

    let mut args_cmake: Vec<String> = Vec::new();
    args_cmake.push(project_root.to_string_lossy().to_string());
    args_cmake.push(constants::ARG_CMAKE_BUILD_TYPE_DEBUG.to_string());
    args_cmake.push(constants::ARG_CMAKE_CXX_FLAGS_COVERAGE.to_string());
    args_cmake.push(constants::ARG_CMAKE_C_FLAGS_COVERAGE.to_string());
    subprocess::run_command("cmake", &args_cmake.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    let mut args_make: Vec<String> = Vec::new();
    args_make.push(constants::ARG_MAKE_C.to_string());
    args_make.push(build_dir.to_string_lossy().to_string());
    subprocess::run_command("make", &args_make.iter().map(|s| s.as_str()).collect::<Vec<&str>>())?;

    println!("{}", constants::MSG_LIBMINIZINC_BUILT_WITH_COVERAGE);
    Ok(())
}


