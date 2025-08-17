
use zos_bootstrap::utils::paths;
use zos_bootstrap::ZosError; // Import ZosError if needed for error handling

#[test]
fn test_get_minizinc_models_dir() -> Result<(), ZosError> {
    let path = paths::get_minizinc_models_dir()?;
    assert!(!path.to_str().unwrap().is_empty());
    assert!(path.to_str().unwrap().contains("minizinc_models"));
    Ok(())
}

#[test]
fn test_get_minizinc_data_dir() -> Result<(), ZosError> {
    let path = paths::get_minizinc_data_dir()?;
    assert!(!path.to_str().unwrap().is_empty());
    assert!(path.to_str().unwrap().contains("minizinc_data"));
    Ok(())
}

#[test]
fn test_get_minizinc_user_solvers_dir() -> Result<(), ZosError> {
    let path = paths::get_minizinc_user_solvers_dir()?;
    assert!(!path.to_str().unwrap().is_empty());
    // This path is user-specific, so we check for a common pattern
    assert!(path.to_str().unwrap().contains(".minizinc/solvers"));
    Ok(())
}
