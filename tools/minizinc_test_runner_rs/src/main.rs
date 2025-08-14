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

    let main_model_name = "sieve_embedding"; // New model name

    let num_vars_range = vec![1, 2, 3];
    let num_values_range = vec![2, 3];
    let num_partitions_range = vec![2, 3];

    let mut results: Vec<(f64, String)> = Vec::new();

    for num_vars in &num_vars_range {
        for num_values in &num_values_range {
            for num_partitions in &num_partitions_range {
                let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
                let current_proof_tape_dir = format!("{}/{}", proof_tapes_dir, timestamp);
                fs::create_dir_all(&current_proof_tape_dir)?;

                println!("Running: {} (num_vars={}, num_values={}, num_partitions={})",
                         main_model_name, num_vars, num_values, num_partitions);

                // Generate the DZN data dynamically using the Rust program
                let generated_data_file = format!("{}/generated_sieve_params.dzn", current_proof_tape_dir);
                let output = Command::new(&rust_data_generator_exe)
                    .arg(&num_vars.to_string())
                    .arg(&num_values.to_string())
                    .arg(&num_partitions.to_string())
                    .output()?;

                fs::write(&generated_data_file, output.stdout)?;

                if !fs::metadata(&generated_data_file)?.is_file() || fs::metadata(&generated_data_file)?.len() == 0 {
                    eprintln!("Error: Generated data file is empty or invalid: {}", generated_data_file);
                    std::process::exit(1);
                }

                let model_file = format!("{}/{}.mzn", minizinc_models_dir, main_model_name);

                let start_time = Instant::now();

                let minizinc_output = Command::new(format!("{}/minizinc", minizinc_build_dir))
                    .arg("-s")
                    .arg("--time-limit")
                    .arg("60000")
                    .arg("--json-stream")
                    .arg(&model_file)
                    .arg(&generated_data_file)
                    .output()?;

                let duration = start_time.elapsed().as_secs_f64();

                let full_output_log = format!("{}/full_output.log", current_proof_tape_dir);
                fs::write(&full_output_log, &minizinc_output.stdout)?;
                fs::write(format!("{}/stderr.log", current_proof_tape_dir), &minizinc_output.stderr)?;
                fs::write(format!("{}/stdout.log", current_proof_tape_dir), &minizinc_output.stdout)?;

                results.push((duration, format!("{} {} {} {} {}", duration, main_model_name, num_vars, num_values, num_partitions)));
            }
        }
    }

    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("--- Test Results (Sorted by Time) ---");
    for (duration, line) in results {
        println!("{}", line);
    }

    Ok(())
}