use super::*;

#[test]
fn test_parse_model_v2() {
    let env = MiniZincEnv::new();
    assert!(env.is_ok());
    let env = env.unwrap();

    let model_code = "var int: x; solve satisfy;";
    // Provide a valid, non-empty filename
    let filename = "test_model.mzn"; 
    let model = env.parse_model(model_code, filename);
    assert!(model.is_ok());
    let model_obj = model.unwrap();
    println!("Parsed model filename: {}", model_obj.filename());
    println!("Parsed model filepath: {}", model_obj.filepath());
    println!("Parsed model num_items: {}", model_obj.num_items());
    assert_eq!(model_obj.filename(), filename);
    assert_eq!(model_obj.num_items(), 2); // var int: x; solve satisfy;
}
