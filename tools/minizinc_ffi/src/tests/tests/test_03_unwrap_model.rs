#[cfg(test)]
mod test_03_unwrap_model {
    use crate::tests::tests::GLOBAL_MINIZINC_ENV;

    #[test]
    fn test_03_unwrap_model() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        let _model_obj = model.unwrap();
        println!("Test 03: Unwrapped model.");
    }
}
