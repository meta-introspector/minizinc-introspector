use crate::utils::error::{Result, ZosError};
use crate::utils::subprocess;
use crate::utils::paths;
use std::path::PathBuf;
use std::fs;

pub fn run_minimal_mzn(
    main_model_version: String,
    core_params_version: String,
    kappa_params_version: String,
    other_params_version: String,
    relations_version: String,
    vector_params_version: String,
) -> Result<()> {
    println!("Running minimal MiniZinc model...");
    let minizinc_models_dir = paths::get_minizinc_models_dir()?;
    let minizinc_data_dir = paths::get_minizinc_data_dir()?;
    let libminizinc_build_dir = paths::get_build_dir()?;

    let model_file = minizinc_models_dir.join(format!("embedding_sphere_{}.mzn", main_model_version));
    let core_params_file = minizinc_data_dir.join(format!("example_core_params_{}.dzn", core_params_version));
    let kappa_params_file = minizinc_data_dir.join(format!("example_kappa_params_{}.dzn", kappa_params_version));
    let other_params_file = minizinc_data_dir.join(format!("example_other_params_{}.dzn", other_params_version));
    let relations_file = minizinc_data_dir.join(format!("example_relations_{}.dzn", relations_version));
    let vector_params_file = minizinc_data_dir.join(format!("example_vector_params_{}.dzn", vector_params_version));

    let minizinc_exe = libminizinc_build_dir.join("minizinc");
    let args = vec![
        "--time-limit", "60000",
        &model_file.to_string_lossy(),
        &core_params_file.to_string_lossy(),
        &kappa_params_file.to_string_lossy(),
        &other_params_file.to_string_lossy(),
        &relations_file.to_string_lossy(),
        &vector_params_file.to_string_lossy(),
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

    println!("Minimal MiniZinc model run completed.");
    Ok(())
}
