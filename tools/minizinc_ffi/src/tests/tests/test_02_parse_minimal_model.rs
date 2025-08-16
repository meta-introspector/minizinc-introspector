#[cfg(test)]
mod test_02_parse_minimal_model {
    use crate::tests::tests::GLOBAL_MINIZINC_ENV;

    #[test]
    fn test_02_parse_minimal_model() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = "var int: x; solve satisfy;";
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        println!("Test 02: Parsed minimal model.");
    }
}
