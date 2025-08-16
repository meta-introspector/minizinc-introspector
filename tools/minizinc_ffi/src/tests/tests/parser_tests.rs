#[cfg(test)]
mod parser_tests {
    use crate::environment::MiniZincEnvironment;
    use crate::tests::tests::GLOBAL_MINIZINC_ENV;

    #[test]
    fn test_parse_simple_model() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Parsed model filepath: {}", model_obj.filepath());
        // Add more assertions here to check the parsed model structure if needed
    }

    #[test]
    fn test_parse_model_with_parameters() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "int: a; var int: x; constraint x > a; solve minimize x;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Parsed model filepath: {}", model_obj.filepath());
    }

    #[test]
    fn test_parse_model_with_array() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "array[1..3] of var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("Parsed model filename: {}", model_obj.filename());
        println!("Parsed model filepath: {}", model_obj.filepath());
    }
}
