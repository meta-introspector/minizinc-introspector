use crate::utils::error::Result;
use crate::utils::subprocess;
use crate::utils::paths;

pub fn handle_extract_constants_command() -> Result<()> {
    println!("Extracting constant strings using MiniZinc...");

    let project_root = paths::resolve_project_root()?;
    let minizinc_models_dir = project_root.join("minizinc_models");
    let minizinc_data_dir = project_root.join("minizinc_data");
    let libminizinc_build_dir = paths::get_build_dir()?;

    let model_file = minizinc_models_dir.join("extract_constants.mzn");
    let data_file = minizinc_data_dir.join("raw_strings_data.dzn");

    let minizinc_exe = libminizinc_build_dir.join("minizinc");

    let args = vec![
        model_file.to_string_lossy().to_string(),
        data_file.to_string_lossy().to_string(),
    ];

    let args_str: Vec<&str> = args.iter().map(|s| s.as_ref()).collect();

    let output = subprocess::run_command(&minizinc_exe.to_string_lossy(), &args_str)?;

    println!("MiniZinc Output:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("MiniZinc Errors:\n{}", String::from_utf8_lossy(&output.stderr));

    if !output.status.success() {
        return Err(crate::utils::error::ZosError::CommandFailed {
            command: format!("minizinc {}", args_str.join(" ")),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }

    println!("Constant string extraction completed.");
    Ok(())
}
