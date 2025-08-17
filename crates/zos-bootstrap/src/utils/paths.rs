use std::path::PathBuf;
use crate::ZosError;
use crate::Result;

pub fn resolve_project_root() -> Result<PathBuf> {
    let current_dir = std::env::current_dir()?;
    // Assuming project root is where Cargo.toml is located
    // Or a specific marker file like .zos_root
    // For now, let's assume it's the current working directory for simplicity
    Ok(current_dir)
}

pub fn get_build_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("build"))
}

pub fn get_gecode_build_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("vendor").join("gecode").join("build"))
}

pub fn get_gecode_vendor_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("vendor").join("gecode"))
}

pub fn get_minizinc_models_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("minizinc_models"))
}

pub fn get_minizinc_data_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("minizinc_data"))
}

pub fn get_minizinc_c_wrapper_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("tools").join("minizinc_c_wrapper_refactored"))
}

pub fn get_minizinc_ffi_crate_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("tools").join("minizinc_ffi"))
}

pub fn get_proof_tapes_dir() -> Result<PathBuf> {
    let root = resolve_project_root()?;
    Ok(root.join("proof_tapes"))
}

pub fn get_minizinc_user_solvers_dir() -> Result<PathBuf> {
    // This path is typically in the user's home directory, not project root
    // For Termux/Android, it might be different. Let's use a placeholder for now.
    // A more robust solution would involve reading from a config file or querying minizinc.
    let home_dir = dirs::home_dir().ok_or(ZosError::ConfigError("Home directory not found".to_string()))?;
    Ok(home_dir.join(".minizinc").join("solvers"))
}

// TODO: Implement set_ld_library_path dynamically for subprocesses