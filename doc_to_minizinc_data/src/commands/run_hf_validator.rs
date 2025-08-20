use crate::prelude::*; // Use the prelude
use crate::config::AppConfig; // Added AppConfig import

pub fn handle_run_hf_validator_command(config: &AppConfig, project_path: PathBuf, output_path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running hf-validator on project: {:?} to output: {:?}", project_path, output_path);

    // Construct the path to the hf-validator binary using config.project_root and config.build_target
    let hf_validator_path = config.project_root.join("target").join(&config.build_target).join("hf-validator");

    // Execute the hf-validator binary
    let status = ProcessCommand::new(&hf_validator_path)
        .arg("analyze-rust-to-ir")
        .arg(project_path)
        .arg(output_path)
        .status()?;

    if status.success() {
        println!("hf-validator executed successfully.");
    } else {
        eprintln!("hf-validator failed with status: {:?}", status);
        return Err(format!("hf-validator failed with status: {:?}", status).into());
    }

    Ok(())
}
