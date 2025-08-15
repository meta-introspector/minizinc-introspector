#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::MiniZincEnvironment;
    use crate::coverage_report;

    #[test]
    fn test_get_version_string() {
        let env = MiniZincEnvironment::new().unwrap();
        let version = env.get_version_string();
        println!("MiniZinc Version: {}", version);
        assert_eq!(version, "2.9.4-introspector");
    }

    #[test]
    fn test_env_creation_and_free() {
        let env = MiniZincEnvironment::new();
        assert!(env.is_ok());
        // Drop will be called automatically when env goes out of scope
    }

    #[test]
    fn test_parse_model() {
        let env = MiniZincEnvironment::new().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let filename = ""; // Changed to empty string
        let model = env.parse_model(model_code, filename);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Parsed model filepath: {}", model_obj.filepath());
        println!("Parsed model num_items: {}", model_obj.num_items());
        assert_eq!(model_obj.filename(), filename);
        assert_eq!(model_obj.num_items(), 2); // var int: x; solve satisfy;

        // Generate C++ coverage report after tests
        coverage_report::generate_cpp_coverage_report().expect("Failed to generate C++ coverage report");
    }
}
