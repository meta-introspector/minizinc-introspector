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

    let main_model_name = "llvm_blockchain_mesh_conceptual"; // New model name

    let mut results: Vec<(f64, String)> = Vec::new();

    // Single test case for the conceptual LLVM blockchain mesh model
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let current_proof_tape_dir = format!("{}/{}", proof_tapes_dir, timestamp);
    fs::create_dir_all(&current_proof_tape_dir)?;

    println!("Running: {} (conceptual model)", main_model_name);

    // Hardcoded data for the conceptual LLVM blockchain mesh model
    let hardcoded_data = r#"
num_nodes = 5;
node_types = [CPU, GPU, BlockchainNode, CPU, GPU];
splicing_active = [true, false, true];
abi_compatibility_score = 0.9;
network_fractal_dimension = 2.5;
active_networks = {Blockchain, GPU_Mesh, CPU_Mesh};
connectivity_density = 75;
cpu_cores_agent = 4;
gpu_memory_gb_agent = 8;
network_bandwidth_mbps_agent = 100;
"#;

    let generated_data_file = format!("{}/llvm_blockchain_mesh_conceptual_data.dzn", current_proof_tape_dir);
    fs::write(&generated_data_file, hardcoded_data)?;

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

    results.push((duration, format!("{} (conceptual model)", main_model_name)));

    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    println!("--- Test Results (Sorted by Time) ---");
    for (duration, line) in results {
        println!("{}", line);
    }

    Ok(())
}