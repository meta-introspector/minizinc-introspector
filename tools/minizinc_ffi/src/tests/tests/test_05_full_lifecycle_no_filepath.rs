#[cfg(test)]
mod test_05_full_lifecycle_no_filepath {
    use crate::tests::tests::GLOBAL_MINIZINC_ENV;

    #[test]
    fn test_05_full_lifecycle_no_filepath() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let model_obj = model.unwrap();
        println!("model_obj filename: {}", model_obj.filename());
        println!("Test 05: Full lifecycle (no filepath) completed.");
        // Model and Env will be dropped here
    }
}
