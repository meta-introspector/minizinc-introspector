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
        let filename = ""; // Use empty string, parse_model now handles it
        let model = env.parse_model(model_code, filename); // parse_model now calls minizinc_parse_model_with_flags
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Parsed model filepath: {}", model_obj.filepath());
        println!("Parsed model num_items: {}", model_obj.num_items());
        // Note: filename returned by model_obj.filename() will be "dummy_model.mzn"
        // if an empty string was passed to parse_model.
        // So, we assert against the expected dummy filename.
        assert_eq!(model_obj.filename(), "dummy_model.mzn");
        assert_eq!(model_obj.num_items(), 2); // var int: x; solve satisfy;

        // Generate C++ coverage report after tests
        coverage_report::generate_cpp_coverage_report().expect("Failed to generate C++ coverage report");
    }
}
