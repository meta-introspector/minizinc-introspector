use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;

pub fn run_minimal_mzn(
    main_model_version: String,
    core_params_version: String,
    kappa_params_version: String,
    other_params_version: String,
    relations_version: String,
    vector_params_version: String,
) -> Result<()> {
    println!("Running minimal MiniZinc model...");

    let project_root = paths::resolve_project_root()?;
    let minizinc_models_dir = project_root.join("minizinc_models");
    let minizinc_data_dir = project_root.join("minizinc_data");

    let main_model_path = minizinc_models_dir.join(format!("embedding_model_v{}.mzn", main_model_version)).to_string_lossy().to_string();
    let core_params_path = minizinc_data_dir.join(format!("embedding_params_core_v{}.dzn", core_params_version)).to_string_lossy().to_string();
    let kappa_params_path = minizinc_data_dir.join(format!("embedding_params_kappa_v{}.dzn", kappa_params_version)).to_string_lossy().to_string();
    let other_params_path = minizinc_data_dir.join(format!("embedding_params_other_v{}.dzn", other_params_version)).to_string_lossy().to_string();
    let relations_path = minizinc_data_dir.join(format!("embedding_params_relations_v{}.dzn", relations_version)).to_string_lossy().to_string();
    let vector_params_path = minizinc_data_dir.join(format!("embedding_params_vector_v{}.dzn", vector_params_version)).to_string_lossy().to_string();

    let args = vec![
        "--solver", "gecode",
        &main_model_path,
        &core_params_path,
        &kappa_params_path,
        &other_params_path,
        &relations_path,
        &vector_params_path,
    ];

    // Convert args to Vec<&str>
    let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    subprocess::run_command("minizinc", &args_str)?;

    println!("Minimal MiniZinc model run completed.");
    Ok(())
}