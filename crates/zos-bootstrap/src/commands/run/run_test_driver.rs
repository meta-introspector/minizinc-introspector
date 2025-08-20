use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;

pub fn run_test_driver(num_vec: u32, base_size: u32) -> Result<()> {
    println!("Running MiniZinc test driver...");

    let project_root = paths::resolve_project_root()?;
    let minizinc_test_runner_exe = project_root
        .join("tools")
        .join("minizinc_test_runner_rs")
        .join("target")
        .join("release")
        .join("minizinc_test_runner_rs");

    let num_vec_str = num_vec.to_string();
    let base_size_str = base_size.to_string();

    let args = vec![
        "--num-vec", &num_vec_str,
        "--base-size", &base_size_str,
    ];

    // Convert args to Vec<&str>
    let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    subprocess::run_command(&minizinc_test_runner_exe.to_string_lossy(), &args_str)?;

    println!("MiniZinc test driver completed.");
    Ok(())
}