use std::process::{Command, Output};
use std::time::Instant;
use std::fs;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let minizinc_project_root = "/data/data/com.termux/files/home/storage/github/libminizinc";
    let minizinc_build_dir = format!("{}/build", minizinc_project_root);
    let minizinc_models_dir = format!("{}/minizinc_models", minizinc_project_root);
    let minizinc_data_dir = format!("{}/minizinc_data", minizinc_project_root);
    let proof_tapes_dir = format!("{}/proof_tapes", minizinc_project_root);
    let rust_data_generator_exe = format!("{}/tools/minizinc_data_generator_rs/minizinc_data_generator_rs/target/release/minizinc_data_generator_rs", minizinc_project_root);

    let main_model_version = "v6";
    let kappa_params_version = "v1";
    let other_params_version = "v1";
    let relations_version = "v1";

    let core_params_versions = vec![
        "v1", "nv1", "nv2", "nv3", "nv5", "nv7", "nv10", "nv11", "nv13", "nv17", "nv100", "nv1000", "nv10000"
    ];

    let mut results: Vec<(f64, String)> = Vec::new();

    for core_v in core_params_versions {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
        let current_proof_tape_dir = format!("{}/{}", proof_tapes_dir, timestamp);
        fs::create_dir_all(&current_proof_tape_dir)?;

        println!("Running: {} {} {} {} {} (vector_params_version will be {})",
                 main_model_version, core_v, kappa_params_version, other_params_version, relations_version, core_v);

        // Extract numerical value for num_vec
        let num_vec_value = if core_v == "v1" {
            "1".to_string()
        } else if core_v.starts_with("nv") {
            core_v.trim_start_matches("nv").to_string()
        } else {
            eprintln!("Error: Unknown CORE_V format: {}", core_v);
            std::process::exit(1);
        };

        // Generate the vector parameters dynamically using the Rust program
        let generated_vector_params_file = format!("{}/generated_vector_params.dzn", current_proof_tape_dir);
        let output = Command::new(&rust_data_generator_exe)
            .arg(&num_vec_value)
            .output()?;

        fs::write(&generated_vector_params_file, output.stdout)?;

        if !fs::metadata(&generated_vector_params_file)?.is_file() || fs::metadata(&generated_vector_params_file)?.len() == 0 {
            eprintln!("Error: Generated vector parameters file is empty or invalid: {}", generated_vector_params_file);
            std::process::exit(1);
        }

        let model_file = format!("{}/embedding_sphere_final_{}.mzn", minizinc_models_dir, main_model_version);
        let core_params_file = format!("{}/example_core_params_{}.dzn", minizinc_data_dir, core_v);
        let kappa_params_file = format!("{}/example_kappa_params_{}.dzn", minizinc_data_dir, kappa_params_version);
        let other_params_file = format!("{}/example_other_params_{}.dzn", minizinc_data_dir, other_params_version);
        let relations_file = format!("{}/example_relations_{}.dzn", minizinc_data_dir, relations_version);

        let start_time = Instant::now();

        let minizinc_output = Command::new(format!("{}/minizinc", minizinc_build_dir))
            .arg("-s")
            .arg("--time-limit")
            .arg("60000")
            .arg("--json-stream")
            .arg(&model_file)
            .arg(&core_params_file)
            .arg(&kappa_params_file)
            .arg(&other_params_file)
            .arg(&relations_file)
            .arg(&generated_vector_params_file)
            .output()?;

        let duration = start_time.elapsed().as_secs_f64();

        let full_output_log = format!("{}/full_output.log", current_proof_tape_dir);
        fs::write(&full_output_log, &minizinc_output.stdout)?;
        fs::write(format!("{}/stderr.log", current_proof_tape_dir), &minizinc_output.stderr)?;
        fs::write(format!("{}/stdout.log", current_proof_tape_dir), &minizinc_output.stdout)?;


        results.push((duration, format!("{} {} {} {} {} {} {}", duration, main_model_version, core_v, kappa_params_version, other_params_version, relations_version, num_vec_value)));
    }

    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("--- Test Results (Sorted by Time) ---");
    for (duration, line) in results {
        println!("{}", line);
    }

    Ok(())
}