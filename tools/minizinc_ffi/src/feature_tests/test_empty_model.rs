use crate::environment::MiniZincEnvironment;
use crate::tests::GLOBAL_MINIZINC_ENV;

#[test]
fn test_empty_model() {
    let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
    let model_code = "% This is an empty model";
    let filename = "empty.mzn";
    let model = env.parse_model(model_code, filename);
    assert!(model.is_ok(), "Failed to parse empty model: {:?}", model.err());
    let model = model.unwrap();

    println!("Parsed Model Filename: {}", model.filename());
    println!("Parsed Model Filepath: {}", model.filepath());
    println!("Parsed Model Num Items: {}", model.num_items());

    assert_eq!(model.num_items(), 0, "Empty model should have 0 items");
}