use crate::utils::error::{Result, ZosError};
use crate::utils::subprocess;
use crate::utils::paths;
use std::path::PathBuf;
use std::fs;

pub fn run_test_driver(num_vec: u32, base_size: u32) -> Result<()> {
    println!("Running MiniZinc test driver...");
    let minizinc_data_dir = paths::get_minizinc_data_dir()?;
    let minizinc_models_dir = paths::get_minizinc_models_dir()?;
    let libminizinc_build_dir = paths::get_build_dir()?;

    // --- Generate example_vector_params.dzn ---
    let vector_params_dzn_file = minizinc_data_dir.join(format!("example_vector_params_nv{}_bs{}.dzn", num_vec, base_size));
    let generate_args = vec![
        minizinc_models_dir.join("generate_vector_params_v2.mzn").to_string_lossy().to_string(),
        format!("-Dnum_vec={};", num_vec),
        format!("-Dbase_size={};", base_size),
    ];
    let generate_args_str: Vec<&str> = generate_args.iter().map(|s| s.as_ref()).collect();
    let generate_output = subprocess::run_command(&libminizinc_build_dir.join("minizinc").to_string_lossy(), &generate_args_str)?;
    // Remove last line (solver output separator)
    let stdout_lines: Vec<&str> = String::from_utf8_lossy(&generate_output.stdout).lines().collect();
    let content_without_last_line = stdout_lines[0..stdout_lines.len() - 1].join("\n");
    fs::write(&vector_params_dzn_file, content_without_last_line)?;

    // --- Create example_core_params.dzn ---
    let core_params_dzn_file = minizinc_data_dir.join(format!("example_core_params_nv{}.dzn", num_vec));
    let core_params_content = format!(
        "% File: example_core_params_nv{}.dzn\n% Purpose: Defines core parameters for testing MiniZinc model runtime.\n% This version sets num_vec to {}.\n\nnum_vec = {};\n",
        num_vec, num_vec, num_vec, num_vec
    );
    fs::write(&core_params_dzn_file, core_params_content)?;

    // --- Run the main MiniZinc model ---
    let main_model_version = "test_master"; // Our fully included model
    let kappa_params_version = "dummy_v1";
    let other_params_version = "dummy_v1";
    let relations_version = "test_v6"; // Our latest relations with beta reduction

    let model_file = minizinc_models_dir.join(format!("embedding_sphere_{}.mzn", main_model_version));
    let kappa_params_file = minizinc_data_dir.join(format!("example_kappa_params_{}.dzn", kappa_params_version));
    let other_params_file = minizinc_data_dir.join(format!("example_other_params_{}.dzn", other_params_version));
    let relations_file = minizinc_data_dir.join(format!("example_relations_{}.dzn", relations_version));

    let minizinc_exe = libminizinc_build_dir.join("minizinc");
    let args = vec![
        "--time-limit", "300000",
        "-I", &minizinc_models_dir.to_string_lossy(),
        &model_file.to_string_lossy(),
        &core_params_dzn_file.to_string_lossy(),
        &kappa_params_file.to_string_lossy(),
        &other_params_file.to_string_lossy(),
        &relations_file.to_string_lossy(),
        &vector_params_dzn_file.to_string_lossy(),
    ];

    let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
    let output = subprocess::run_command(&minizinc_exe.to_string_lossy(), &args_str)?;

    if !output.status.success() {
        return Err(ZosError::CommandFailed {
            command: format!("minizinc {}", args_str.join(" ")),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    println!("MiniZinc test driver run completed.");
    Ok(())
}
