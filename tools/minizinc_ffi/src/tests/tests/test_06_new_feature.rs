#[cfg(test)]
mod test_06_new_feature {
    use crate::tests::tests::GLOBAL_MINIZINC_ENV;

    #[test]
    fn test_06_parse_new_feature() {
        let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
        let model_code = include_str!("../../../../../tests/new_feature_test.mzn");
        let model = env.parse_string(model_code);
        assert!(model.is_ok());
        println!("Test 06: Parsed new feature model.");
    }
}
