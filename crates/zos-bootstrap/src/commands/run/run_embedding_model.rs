use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;
use std::fs;
use chrono::Local;
use crate::commands::run::vector_params_source::VectorParamsSource;



pub fn run_embedding_model(
    main_model_version: String,
    core_params_version: String,
    kappa_params_version: String,
    other_params_version: String,
    relations_version: String,
    vector_params_source: VectorParamsSource,
) -> Result<()> {
    println!("Running embedding model...");

    let project_root = paths::resolve_project_root()?;
    let minizinc_models_dir = project_root.join("minizinc_models");
    let minizinc_data_dir = project_root.join("minizinc_data");
    let proof_tapes_dir = project_root.join("proof_tapes");

    // Ensure proof_tapes directory exists
    fs::create_dir_all(&proof_tapes_dir)?;

    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let proof_tape_filename = format!("proof_tape_{}.dzn", timestamp);
    let proof_tape_path = proof_tapes_dir.join(&proof_tape_filename);

    let main_model_path = minizinc_models_dir.join(format!("embedding_model_v{}.mzn", main_model_version)).to_string_lossy().to_string();
    let core_params_path = minizinc_data_dir.join(format!("embedding_params_core_v{}.dzn", core_params_version)).to_string_lossy().to_string();
    let kappa_params_path = minizinc_data_dir.join(format!("embedding_params_kappa_v{}.dzn", kappa_params_version)).to_string_lossy().to_string();
    let other_params_path = minizinc_data_dir.join(format!("embedding_params_other_v{}.dzn", other_params_version)).to_string_lossy().to_string();
    let relations_path = minizinc_data_dir.join(format!("embedding_params_relations_v{}.dzn", relations_version)).to_string_lossy().to_string();
    let proof_tape_file_arg = format!("proof_tape_file=\"{}\";", proof_tape_path.to_string_lossy());

    let mut args: Vec<String> = vec![
        "--solver".to_string(), "gecode".to_string(),
        main_model_path,
        core_params_path,
        kappa_params_path,
        other_params_path,
        relations_path,
        "-D".to_string(), proof_tape_file_arg,
    ];

    match vector_params_source {
        VectorParamsSource::Version(version) => {
            let vector_params_path = minizinc_data_dir.join(format!("embedding_params_vector_v{}.dzn", version)).to_string_lossy().to_string();
            args.push(vector_params_path);
        },
        VectorParamsSource::NumVec(num) => {
            // Generate DZN for num_vec
            let dzn_content = format!("num_vec = {};", num);
            let temp_dzn_path = minizinc_data_dir.join(format!("temp_num_vec_{}.dzn", timestamp));
            fs::write(&temp_dzn_path, dzn_content)?;
            let temp_dzn_path_str = temp_dzn_path.to_string_lossy().to_string();
            args.push(temp_dzn_path_str);
        }
    }

    // Convert args to Vec<&str>
    let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    subprocess::run_command("minizinc", &args_str)?;

    println!("Embedding model run completed. Proof tape saved to {}.", proof_tape_path.to_string_lossy());
    Ok(())
}