#[cfg(test)]
mod minimal_crash_tests {
    use crate::environment::MiniZincEnvironment;
    use crate::tests::tests::GLOBAL_MINIZINC_ENV;

    // Test 1: Only initialize the global environment
    #[test]
    fn test_01_init_global_env() {
        let _env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        println!("Test 01: Initialized global environment.");
    }

    // Test 2: Initialize global environment and parse a minimal model
    #[test]
    fn test_02_parse_minimal_model() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        println!("Test 02: Parsed minimal model.");
    }

    // Test 3: Initialize global environment, parse model, and unwrap it
    #[test]
    fn test_03_unwrap_model() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let _model_obj = model.unwrap();
        println!("Test 03: Unwrapped model.");
    }

    // Test 4: Initialize global environment, parse model, unwrap, and get filename
    #[test]
    fn test_04_get_model_filename() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Test 04: Got model filename.");
    }

    // Test 5: Initialize global environment, parse model, unwrap, get filename, then let it drop
    #[test]
    fn test_05_full_lifecycle_no_filepath() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Test 05: Full lifecycle (no filepath) completed.");
        // Model and Env will be dropped here
    }
}