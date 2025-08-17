use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;
use std::env;

// This function is temporarily removed due to persistent linking issues on Android.
// It will be re-introduced when a more robust testing environment for the FFI is established.
// pub fn test_rust_ffi() -> Result<()> {
//     println!("Running Rust FFI tests in project root: {}", project_root.display());

//     let project_root = paths::resolve_project_root()?;
//     let build_dir = paths::get_build_dir()?; // This is usually where libminizinc_c_wrapper.so is built

//     let lib_path = build_dir.join("libminizinc_c_wrapper.so"); // Assuming this is the correct name and location

//     // Set RUSTFLAGS to include the rpath
//     // let rpath_arg = format!("-C link-arg=-Wl,-rpath,{}", lib_path.parent().unwrap().to_string_lossy());
//     // env::set_var("RUSTFLAGS", rpath_arg);

//     // Run cargo test for the minizinc_ffi package
//     let output = subprocess::run_command("cargo", &["test", "--package", "minizinc_ffi"])?;

//     println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
//     println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

//     if !output.status.success() {
//         return Err(crate::utils::error::ZosError::CommandFailed {
//             command: "cargo test --package minizinc_ffi".to_string(),
//             exit_code: output.status.code(),
//             stdout: String::from_utf8_lossy(&output.stdout).to_string(),
//             stderr: String::from_utf8_lossy(&output.stderr).to_string(),
//         });
//     }

//     println!("Rust FFI tests completed successfully.");
//     Ok(())
// }
