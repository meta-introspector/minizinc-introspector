use crate::utils::error::Result;
use crate::utils::paths;
use crate::commands::build::{ensure_build_directory_exists, run_cmake_for_ffi, run_make_for_ffi, verify_ffi_library_exists};

pub fn test_rust_ffi() -> Result<()> {
    println!("--- Starting test_rust_ffi ---");
    println!("Running Rust FFI tests by calling modular build functions...");

    let build_dir = paths::get_build_dir()?;
    let project_root = paths::resolve_project_root()?;

    println!("Project Root: {}", project_root.display());
    println!("Build Dir: {}", build_dir.display());

    // Test ensure_build_directory_exists
    println!("\n--- Testing ensure_build_directory_exists ---");
    ensure_build_directory_exists::ensure_build_directory_exists(&build_dir)?;
    println!("ensure_build_directory_exists test passed.");

    // Test run_cmake_for_ffi
    println!("\n--- Testing run_cmake_for_ffi ---");
    run_cmake_for_ffi::run_cmake_for_ffi(&project_root, &build_dir)?;
    println!("run_cmake_for_ffi test passed.");

    // Test run_make_for_ffi (without strace)
    println!("\n--- Testing run_make_for_ffi (without strace) ---");
    run_make_for_ffi::run_make_for_ffi(&build_dir, false, &[])?;
    println!("run_make_for_ffi (without strace) test passed.");

    // Test verify_ffi_library_exists
    println!("\n--- Testing verify_ffi_library_exists ---");
    verify_ffi_library_exists::verify_ffi_library_exists(&build_dir)?;
    println!("verify_ffi_library_exists test passed.");

    // Test run_make_for_ffi (with strace)
    println!("\n--- Testing run_make_for_ffi (with strace) ---");
    run_make_for_ffi::run_make_for_ffi(&build_dir, true, &[])?;
    println!("run_make_for_ffi (with strace) test passed. Check make_strace.log in build dir.");

    println!("\n--- All Rust FFI modular tests completed successfully. ---");
    Ok(())
}
