use crate::environment::MiniZincEnvironment;
use crate::tests::GLOBAL_MINIZINC_ENV;
use crate::types::MiniZincBaseType;

#[test]
fn test_basic_int_var() {
    let env = GLOBAL_MINIZINC_ENV.lock().unwrap();
    let model_code = "var int: x; solve satisfy;";
    let filename = "basic_int_var.mzn";
    let model = env.parse_model(model_code, filename);
    assert!(model.is_ok(), "Failed to parse basic int var model: {:?}", model.err());
    let model = model.unwrap();

    println!("Parsed Model Filename: {}", model.filename());
    println!("Parsed Model Filepath: {}", model.filepath());
    println!("Parsed Model Num Items: {}", model.num_items());

    assert_eq!(model.num_items(), 2, "Basic int var model should have 2 items (var decl and solve)");

    // Further inspection of the variable declaration
    if let Some(item) = model.get_item_at_index(0) {
        assert!(item.is_vardecl(), "First item should be a variable declaration");
        if let Some(vardecl) = item.as_vardecl() {
            assert_eq!(vardecl.id(), "x", "Variable ID should be 'x'");
            let type_inst = vardecl.type_inst();
            assert_eq!(type_inst.base_type(), MiniZincBaseType::Int, "Variable base type should be Int");
        }
    }
}