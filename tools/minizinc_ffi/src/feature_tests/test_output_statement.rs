use crate::environment::MiniZincEnvironment;
use crate::tests::GLOBAL_MINIZINC_ENV;

#[test]
fn test_output_statement() {
    let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
    let model_code = "var int: x; solve satisfy; output [\"x = \\(x)\\n\"];";
    let filename = "output_statement.mzn";
    let model = env.parse_model(model_code, filename);
    assert!(model.is_ok(), "Failed to parse output statement model: {:?}", model.err());
    let model = model.unwrap();

    println!("Parsed Model Filename: {}", model.filename());
    println!("Parsed Model Filepath: {}", model.filepath());
    println!("Parsed Model Num Items: {}", model.num_items());

    // The number of items might vary depending on how MiniZinc parses, but it should be > 0
    assert!(model.num_items() > 0, "Output statement model should have items");

    // We can't easily inspect the output statement content via the current FFI, 
    // but we can assert that the model parsed successfully.
}