use crate::utils::error::{Result, ZosError};
use crate::utils::subprocess;
use crate::utils::paths;
use std::path::PathBuf;
use std::fs;
use chrono;

pub fn run_embedding_v6(
    main_model_version: String,
    core_params_version: String,
    kappa_params_version: String,
    other_params_version: String,
    relations_version: String,
    vector_params_version: String,
) -> Result<()> {
    println!("Running v6 embedding model...");
    let project_root = paths::resolve_project_root()?;
    let minizinc_models_dir = paths::get_minizinc_models_dir()?;
    let minizinc_data_dir = paths::get_minizinc_data_dir()?;
    let libminizinc_build_dir = paths::get_build_dir()?;
    let proof_tapes_dir = paths::get_proof_tapes_dir()?;

    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let proof_tape_dir = proof_tapes_dir.join(&timestamp);
    subprocess::run_command("mkdir", &["-p", &proof_tape_dir.to_string_lossy()])?;

    // Record version vector
    let version_vector_content = format!(
        "Main Model Version: {{}}
Core Params Version: {{}}
Kappa Params Version: {{}}
Other Params Version: {{}}
Relations Version: {{}}
Vector Params Version: {{}}
",
        main_model_version,
        core_params_version,
        kappa_params_version,
        other_params_version,
        relations_version,
        vector_params_version
    );
    fs::write(proof_tape_dir.join("version_vector.txt"), version_vector_content)?;

    let model_file = minizinc_models_dir.join(format!("embedding_sphere_final_{}.mzn", main_model_version));
    let core_params_file = minizinc_data_dir.join(format!("example_core_params_{}.dzn", core_params_version));
    let kappa_params_file = minizinc_data_dir.join(format!("example_kappa_params_{}.dzn", kappa_params_version));
    let other_params_file = minizinc_data_dir.join(format!("example_other_params_{}.dzn", other_params_version));
    let relations_file = minizinc_data_dir.join(format!("example_relations_{}.dzn", relations_version));
    let vector_params_file = minizinc_data_dir.join(format!("example_vector_params_{}.dzn", vector_params_version));

    // Copy files to proof tape
    fs::copy(&model_file, proof_tape_dir.join(model_file.file_name().unwrap()))?;
    fs::copy(&core_params_file, proof_tape_dir.join(core_params_file.file_name().unwrap()))?;
    fs::copy(&kappa_params_file, proof_tape_dir.join(kappa_params_file.file_name().unwrap()))?;
    fs::copy(&other_params_file, proof_tape_dir.join(other_params_file.file_name().unwrap()))?;
    fs::copy(&relations_file, proof_tape_dir.join(relations_file.file_name().unwrap()))?;
    fs::copy(&vector_params_file, proof_tape_dir.join(vector_params_file.file_name().unwrap()))?;

    let minizinc_exe = libminizinc_build_dir.join("minizinc");
    let args = vec![
        "-s",
        "--time-limit", "60000",
        "--json-stream",
        &model_file.to_string_lossy(),
        &core_params_file.to_string_lossy(),
        &kappa_params_file.to_string_lossy(),
        &other_params_file.to_string_lossy(),
        &relations_file.to_string_lossy(),
        &vector_params_file.to_string_lossy(),
    ];

    let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();
    let output = subprocess::run_command(&minizinc_exe.to_string_lossy(), &args_str)?;
    fs::write(proof_tape_dir.join("full_output.log"), &output.stdout)?;
    fs::write(proof_tape_dir.join("stderr.log"), &output.stderr)?;

    if !output.status.success() {
        return Err(ZosError::CommandFailed {
            command: format!("minizinc {{}}", args.join(" ")),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    println!("v6 embedding model run completed. Proof tape in: {{}}", proof_tape_dir.to_string_lossy());
    Ok(())
}
